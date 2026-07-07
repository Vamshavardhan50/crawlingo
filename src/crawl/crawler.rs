use crate::crawl::frontier::{Frontier, MemoryFrontier};
use crate::dataset::builder::{DatasetField, DatasetResult};
use crate::engine::fetcher::FetchRequest;
use crate::engine::session::Session;
use crate::error::Result;
use crate::parser::streaming::HtmlParser;
use crate::selector::SelectorQuery;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use url::Url;

/// Orchestrates multi-page, concurrent crawling with politeness limitations.
#[derive(Clone)]
pub struct Crawler {
    pub start_url: String,
    pub follow_selector: String,
    pub limit: usize,
    pub max_depth: usize,
    pub concurrency: usize,
    pub delay_seconds: f64,
    pub fields: Vec<DatasetField>,
    pub session: Arc<Session>,
    pub webhook_url: Option<String>,
    /// The pending-URL queue and visited set (see [`crate::crawl::frontier`]). `None` (the
    /// default) means an ephemeral [`MemoryFrontier`] is created fresh for each
    /// `crawl`/`crawl_async` call, seeded with `start_url` — the crawler's original behavior. Set
    /// via [`Crawler::with_frontier`] or [`Crawler::resumable`] to persist crawl state across runs.
    pub frontier: Option<Arc<dyn Frontier>>,
}

impl Crawler {
    /// Creates a new crawler instance.
    pub fn new(start_url: &str, session: Arc<Session>) -> Self {
        Self {
            start_url: start_url.to_string(),
            follow_selector: String::new(),
            limit: 10,
            max_depth: 3,
            concurrency: 2,
            delay_seconds: 0.0,
            fields: Vec::new(),
            session,
            webhook_url: None,
            frontier: None,
        }
    }

    /// Uses `frontier` as the pending-URL queue/visited-set instead of an ephemeral per-run one.
    /// Chainable. `start_url` is seeded automatically by `crawl`/`crawl_async` only if the
    /// frontier is completely fresh (nothing pending or visited yet) — a frontier already carrying
    /// state from a previous run is left alone, continuing from wherever it left off.
    pub fn with_frontier(mut self, frontier: Arc<dyn Frontier>) -> Self {
        self.frontier = Some(frontier);
        self
    }

    /// Builds a crawler backed by a [`crate::crawl::frontier::PersistentFrontier`] at `path`,
    /// resumable across process restarts: reopening the same path continues from wherever a
    /// previous run left off (its pending queue and visited set) instead of re-crawling
    /// everything from `start_url` again.
    pub fn resumable(
        start_url: &str,
        session: Arc<Session>,
        path: &std::path::Path,
    ) -> Result<Self> {
        let frontier: Arc<dyn Frontier> =
            Arc::new(crate::crawl::frontier::PersistentFrontier::open(path)?);
        Ok(Self::new(start_url, session).with_frontier(frontier))
    }

    /// Helper to resolve absolute URLs.
    fn resolve_url(base: &str, relative: &str) -> Option<String> {
        let base_url = Url::parse(base).ok()?;
        base_url.join(relative).ok().map(|u: Url| u.to_string())
    }

    /// Starts concurrent crawl.
    pub fn crawl(&self) -> Result<Vec<DatasetResult>> {
        crate::TOKIO_RUNTIME.block_on(self.crawl_async())
    }

    /// Asynchronous crawling engine using JoinSet.
    pub async fn crawl_async(&self) -> Result<Vec<DatasetResult>> {
        let results = Arc::new(Mutex::new(Vec::new()));

        let frontier: Arc<dyn Frontier> = match &self.frontier {
            Some(f) => f.clone(),
            None => Arc::new(MemoryFrontier::new()),
        };
        // Seed the start URL only for a genuinely fresh frontier — one carrying state from a
        // previous run (via Crawler::resumable) is left alone, continuing where it left off.
        if frontier.pending_len() == 0 && frontier.visited_len() == 0 {
            frontier.enqueue(self.start_url.clone(), 0);
        }

        // Crawl parameters
        let limit = self.limit;
        let max_depth = self.max_depth;
        let concurrency = self.concurrency;
        let follow_sel = self.follow_selector.clone();
        let delay = self.delay_seconds;
        let webhook_url = self.webhook_url.clone();

        // Share the session-wide fetch manager (and its rate limiter) across all workers.
        let manager = self.session.fetch_manager();

        // Build a single webhook client up front (wreq::Client is Arc-backed and cheap to clone),
        // instead of constructing a brand-new client for every delivered result.
        let webhook_client = webhook_url.as_ref().map(|_| Arc::new(wreq::Client::new()));

        // Create field extraction instructions
        let mut fields_def = Vec::new();
        for f in &self.fields {
            fields_def.push(crate::dataset::builder::DatasetField {
                name: f.name.clone(),
                selector: f.selector.clone(),
                selector_type: f.selector_type.clone(),
                #[cfg(feature = "python")]
                transform: f.transform.clone(),
                default: f.default.clone(),
                extract_type: f.extract_type.clone(),
            });
        }
        let fields_def_arc = Arc::new(fields_def);

        let mut workers = JoinSet::new();

        for _ in 0..concurrency {
            let results = results.clone();
            let frontier = frontier.clone();
            let manager = manager.clone();
            let session = self.session.clone();
            let fields = fields_def_arc.clone();
            let follow_sel = follow_sel.clone();
            let webhook_url = webhook_url.clone();
            let webhook_client = webhook_client.clone();

            workers.spawn(async move {
                loop {
                    // 1. Check if we hit limit
                    if results.lock().await.len() >= limit {
                        break;
                    }

                    // 2. Pop next URL
                    let (url_str, depth) = match frontier.dequeue() {
                        Some(task) => task,
                        None => break, // Queue is empty, worker can exit
                    };

                    // Check if already visited (each URL is only ever enqueued once in practice,
                    // but stay defensive rather than assume that invariant holds forever)
                    if frontier.is_visited(&url_str) {
                        continue;
                    }
                    frontier.mark_visited(&url_str);

                    // 3. Fetch configs (rotating proxy dynamically)
                    let headers = session.headers.read().unwrap().clone();
                    let cookies = session.cookies.read().unwrap().clone();
                    let proxy = session.get_next_proxy();
                    let rate_limit_rps = *session.rate_limit_rps.read().unwrap();
                    let timeout_secs = *session.timeout_seconds.read().unwrap();
                    let fetcher_tier = *session.fetcher_tier.read().unwrap();
                    let browser_profile = session.browser_profile.read().unwrap().clone();

                    let req = FetchRequest {
                        url: url_str.clone(),
                        tier: fetcher_tier,
                        browser_profile,
                        headers,
                        cookies,
                        proxy,
                        timeout: std::time::Duration::from_secs(timeout_secs),
                        retries: 2,
                        rate_limit_rps,
                    };

                    // Politeness delay
                    if delay > 0.0 {
                        tokio::time::sleep(std::time::Duration::from_secs_f64(delay)).await;
                    }

                    match manager.dispatch(req).await {
                        Ok(response) => {
                            if let Ok(page) = HtmlParser::parse(response) {
                                // Extract data using unified SelectorEngine queries
                                let mut fields_map = HashMap::new();
                                for f in fields.iter() {
                                    let query = match f.selector_type.as_str() {
                                        "xpath" => SelectorQuery::XPath(&f.selector),
                                        "regex" => SelectorQuery::Regex(&f.selector),
                                        "text" => SelectorQuery::TextAnchor(&f.selector),
                                        "after_text" => SelectorQuery::AfterText(&f.selector),
                                        "before_text" => SelectorQuery::BeforeText(&f.selector),
                                        _ => SelectorQuery::Css(&f.selector),
                                    };

                                    let matches = page.query(query).unwrap_or_default();
                                    let extracted = if matches.is_empty() {
                                        None
                                    } else {
                                        let combined_text = page.get_nodes_combined_text(&matches);
                                        let normalized = crate::extraction::ExtractionEngine::normalize_value(
                                            &combined_text,
                                            &f.extract_type,
                                            page.url(),
                                        );
                                        if normalized.is_empty() { None } else { Some(normalized) }
                                    };
                                    let final_val = extracted
                                        .or_else(|| f.default.clone())
                                        .unwrap_or_default();
                                    fields_map.insert(f.name.clone(), final_val);
                                }

                                let result = DatasetResult {
                                    url: url_str.clone(),
                                    fields: fields_map,
                                    timestamp: chrono::Utc::now(),
                                };
                                results.lock().await.push(result.clone());

                                // Deliver Webhook POST request if configured, reusing the
                                // shared client so connections can be pooled across deliveries.
                                if let (Some(hook_url), Some(client)) =
                                    (webhook_url.as_ref(), webhook_client.as_ref())
                                {
                                    let _ = client
                                        .request(wreq::Method::POST, hook_url.clone())
                                        .json(&result)
                                        .send()
                                        .await;
                                }

                                // Discover links to follow if depth limit is not reached
                                if depth < max_depth && !follow_sel.is_empty() {
                                    let matches = page
                                        .query(SelectorQuery::Css(&follow_sel))
                                        .unwrap_or_default();
                                    for &link_idx in &matches {
                                        if let Some(href) =
                                            page.dom_tree().nodes[link_idx].attrs.get("href")
                                        {
                                            if let Some(abs_url) = Self::resolve_url(&url_str, href)
                                            {
                                                frontier.enqueue(abs_url, depth + 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Crawler failed to fetch URL {}: {}", url_str, e);
                        }
                    }
                }
            });
        }

        // Wait for all workers to finish
        while workers.join_next().await.is_some() {}

        let final_results = results.lock().await.clone();
        Ok(final_results)
    }

    /// Spawns a background thread to repeat crawling on a fixed interval.
    pub fn run_scheduled(&self, interval_seconds: u64) {
        let crawler = self.clone();
        std::thread::spawn(move || loop {
            tracing::info!(
                "Executing scheduled crawl loop for start url: {}",
                crawler.start_url
            );
            if let Err(e) = crawler.crawl() {
                tracing::error!("Scheduled crawl encountered error: {}", e);
            }
            std::thread::sleep(std::time::Duration::from_secs(interval_seconds));
        });
    }
}

// PyO3 FFI Python classes
#[cfg(feature = "python")]
use crate::dataset::builder::PyDatasetResult;
#[cfg(feature = "python")]
use crate::engine::session::PySession;
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "Crawl")]
pub struct PyCrawl {
    pub inner: Crawler,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyCrawl {
    #[new]
    pub fn new_py(start_url: &str, session: &PySession) -> Self {
        Self {
            inner: Crawler::new(start_url, session.inner.clone()),
        }
    }

    /// Builds a crawler backed by a persistent, resumable frontier at `path` — reopening the
    /// same path continues from wherever a previous run left off instead of re-crawling
    /// everything from `start_url` again.
    #[staticmethod]
    pub fn resumable(start_url: &str, session: &PySession, path: &str) -> PyResult<Self> {
        let inner = Crawler::resumable(start_url, session.inner.clone(), std::path::Path::new(path))?;
        Ok(Self { inner })
    }

    pub fn follow(mut self_: PyRefMut<'_, Self>, selector: &str) -> PyResult<Py<Self>> {
        self_.inner.follow_selector = selector.to_string();
        Ok(self_.into())
    }

    pub fn limit(mut self_: PyRefMut<'_, Self>, pages: usize) -> PyResult<Py<Self>> {
        self_.inner.limit = pages;
        Ok(self_.into())
    }

    pub fn depth(mut self_: PyRefMut<'_, Self>, max_depth: usize) -> PyResult<Py<Self>> {
        self_.inner.max_depth = max_depth;
        Ok(self_.into())
    }

    #[pyo3(signature = (name, selector, selector_type=None, default=None))]
    pub fn field(
        mut self_: PyRefMut<'_, Self>,
        name: &str,
        selector: &str,
        selector_type: Option<&str>,
        default: Option<&str>,
    ) -> PyResult<Py<Self>> {
        let field = DatasetField {
            name: name.to_string(),
            selector: selector.to_string(),
            selector_type: selector_type.unwrap_or("css").to_string(),
            #[cfg(feature = "python")]
            transform: None,
            default: default.map(|s| s.to_string()),
            extract_type: Default::default(),
        };
        self_.inner.fields.push(field);
        Ok(self_.into())
    }

    pub fn concurrency(mut self_: PyRefMut<'_, Self>, n: usize) -> PyResult<Py<Self>> {
        self_.inner.concurrency = n;
        Ok(self_.into())
    }

    pub fn delay(mut self_: PyRefMut<'_, Self>, seconds: f64) -> PyResult<Py<Self>> {
        self_.inner.delay_seconds = seconds;
        Ok(self_.into())
    }

    /// Set webhook endpoint URL (returns self)
    pub fn webhook(mut self_: PyRefMut<'_, Self>, url: &str) -> PyResult<Py<Self>> {
        self_.inner.webhook_url = Some(url.to_string());
        Ok(self_.into())
    }

    /// Run crawling recurringly in background (non-blocking)
    pub fn schedule(self_: PyRef<'_, Self>, interval_seconds: u64) -> PyResult<()> {
        self_.inner.run_scheduled(interval_seconds);
        Ok(())
    }

    pub fn build(self_: PyRef<'_, Self>) -> PyResult<Vec<PyDatasetResult>> {
        let res = self_.inner.crawl()?;
        let py_res = res
            .into_iter()
            .map(|r| PyDatasetResult { inner: r })
            .collect();
        Ok(py_res)
    }
}
