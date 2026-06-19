use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use url::Url;
use crate::error::{CrawlingoError, Result};
use crate::engine::session::Session;
use crate::engine::fetcher::{Fetcher, FetchRequest, FetcherTier};
use crate::engine::pool::ConnectionPoolConfig;
use crate::parser::streaming::parse_html;
use crate::selector::css;
use crate::dataset::builder::{Dataset, DatasetField, DatasetResult};

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
        }
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
        let visited = Arc::new(Mutex::new(HashSet::new()));
        let results = Arc::new(Mutex::new(Vec::new()));
        
        let pending_queue = Arc::new(Mutex::new(vec![(self.start_url.clone(), 0)]));

        // Crawl parameters
        let limit = self.limit;
        let max_depth = self.max_depth;
        let concurrency = self.concurrency;
        let follow_sel = self.follow_selector.clone();
        let delay = self.delay_seconds;
        let webhook_url = self.webhook_url.clone();
        
        let rate_limiter = Arc::new(crate::engine::rate_limiter::HostRateLimiter::new());
        let fetcher = Arc::new(Fetcher::new(rate_limiter, ConnectionPoolConfig::default()));

        // Create field extraction instructions
        let mut fields_def = Vec::new();
        for f in &self.fields {
            fields_def.push(crate::dataset::builder::DatasetField {
                name: f.name.clone(),
                selector: f.selector.clone(),
                selector_type: f.selector_type.clone(),
                #[cfg(feature = "python")]
                transform: None,
                default: f.default.clone(),
            });
        }
        let fields_def_arc = Arc::new(fields_def);

        let mut workers = JoinSet::new();

        for _ in 0..concurrency {
            let visited = visited.clone();
            let results = results.clone();
            let pending_queue = pending_queue.clone();
            let fetcher = fetcher.clone();
            let session = self.session.clone();
            let fields = fields_def_arc.clone();
            let follow_sel = follow_sel.clone();
            let webhook_url = webhook_url.clone();

            workers.spawn(async move {
                loop {
                    // 1. Check if we hit limit
                    {
                        let res_count = results.lock().await.len();
                        if res_count >= limit {
                            break;
                        }
                    }

                    // 2. Pop next URL
                    let next_task = {
                        let mut queue = pending_queue.lock().await;
                        queue.pop()
                    };

                    let (url_str, depth) = match next_task {
                        Some(task) => task,
                        None => break, // Queue is empty, worker can exit
                    };

                    // Check if already visited
                    {
                        let mut vis = visited.lock().await;
                        if vis.contains(&url_str) {
                            continue;
                        }
                        vis.insert(url_str.clone());
                    }

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

                    match fetcher.fetch(req).await {
                        Ok(response) => {
                            if let Ok(bytes) = response.bytes().await {
                                if let Ok(tree) = parse_html(&bytes) {
                                    // Extract data
                                    let mut fields_map = HashMap::new();
                                    for f in fields.iter() {
                                        let matches = css::query(&tree, &f.selector);
                                        let text_val = matches.iter()
                                            .map(|&idx| tree.get_text(idx))
                                            .collect::<Vec<String>>()
                                            .join(" ");
                                        fields_map.insert(f.name.clone(), text_val);
                                    }

                                    let result = DatasetResult {
                                        url: url_str.clone(),
                                        fields: fields_map,
                                        timestamp: chrono::Utc::now(),
                                    };
                                    results.lock().await.push(result.clone());

                                    // Deliver Webhook POST request if configured
                                    if let Some(ref hook_url) = webhook_url {
                                        let client = wreq::Client::new();
                                        let _ = client.request(wreq::Method::POST, hook_url.clone())
                                            .json(&result)
                                            .send()
                                            .await;
                                    }

                                    // Discover links to follow if depth limit is not reached
                                    if depth < max_depth && !follow_sel.is_empty() {
                                        let links = css::query(&tree, &follow_sel);
                                        let mut new_links = Vec::new();
                                        for &link_idx in &links {
                                            if let Some(href) = tree.nodes[link_idx].attrs.get("href") {
                                                if let Some(abs_url) = Self::resolve_url(&url_str, href) {
                                                    new_links.push(abs_url);
                                                }
                                            }
                                        }

                                        let mut queue = pending_queue.lock().await;
                                        for link in new_links {
                                            queue.push((link, depth + 1));
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
        std::thread::spawn(move || {
            loop {
                tracing::info!("Executing scheduled crawl loop for start url: {}", crawler.start_url);
                if let Err(e) = crawler.crawl() {
                    tracing::error!("Scheduled crawl encountered error: {}", e);
                }
                std::thread::sleep(std::time::Duration::from_secs(interval_seconds));
            }
        });
    }
}

// PyO3 FFI Python classes
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use crate::engine::session::PySession;
#[cfg(feature = "python")]
use crate::dataset::builder::PyDatasetResult;

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
        let py_res = res.into_iter()
            .map(|r| PyDatasetResult { inner: r })
            .collect();
        Ok(py_res)
    }
}
