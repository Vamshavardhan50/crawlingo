use crate::crawl::frontier::{Frontier, MemoryFrontier};
use crate::crawl::pagination::{PaginationConfig, Paginator};
use crate::crawl::robots::RobotsCache;
use crate::dataset::builder::{DatasetField, DatasetResult};
use crate::engine::fetcher::FetchRequest;
use crate::engine::session::Session;
use crate::error::Result;
use crate::parser::streaming::HtmlParser;
use crate::selector::SelectorQuery;
use crate::util::url::canonicalize_relative_url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

/// Comprehensive result of a crawl run containing execution metrics and extracted dataset records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlResult {
    pub visited_count: usize,
    pub successful_count: usize,
    pub failed_count: usize,
    pub skipped_count: usize,
    pub duration_ms: u64,
    pub discovered_urls: Vec<String>,
    pub errors: Vec<String>,
    pub results: Vec<DatasetResult>,
}

impl Deref for CrawlResult {
    type Target = Vec<DatasetResult>;
    fn deref(&self) -> &Self::Target {
        &self.results
    }
}

/// Orchestrates multi-page, concurrent crawling with politeness, robots.txt, and resiliency.
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
    pub frontier: Option<Arc<dyn Frontier>>,
    pub pagination: Option<PaginationConfig>,
    pub respect_robots: bool,
    pub robots_cache: Arc<RobotsCache>,
    pub cancellation_token: CancellationToken,
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
            pagination: None,
            respect_robots: true,
            robots_cache: Arc::new(RobotsCache::new()),
            cancellation_token: CancellationToken::new(),
        }
    }

    /// Sets whether to respect robots.txt rules. Default: `true`.
    pub fn respect_robots(mut self, respect: bool) -> Self {
        self.respect_robots = respect;
        self
    }

    /// Sets the maximum depth to follow links (seed = depth 0).
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Sets the CSS selector for links to follow.
    pub fn follow(mut self, selector: &str) -> Self {
        self.follow_selector = selector.to_string();
        self
    }

    /// Sets max page limit.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Sets worker concurrency.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency.max(1);
        self
    }

    /// Sets politeness delay between requests in seconds.
    pub fn delay(mut self, seconds: f64) -> Self {
        self.delay_seconds = seconds;
        self
    }

    /// Adds an extraction field to the crawler.
    pub fn field(mut self, field: DatasetField) -> Self {
        self.fields.push(field);
        self
    }

    /// Sets a webhook URL for result delivery.
    pub fn webhook(mut self, url: &str) -> Self {
        self.webhook_url = Some(url.to_string());
        self
    }

    /// Sets the crawler's pagination config. Chainable.
    pub fn with_pagination(mut self, config: PaginationConfig) -> Self {
        self.pagination = Some(config);
        self
    }

    /// Uses `frontier` as the pending-URL queue/visited-set.
    pub fn with_frontier(mut self, frontier: Arc<dyn Frontier>) -> Self {
        self.frontier = Some(frontier);
        self
    }

    /// Triggers cancellation of an active crawl.
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    /// Builds a crawler backed by a [`crate::crawl::frontier::PersistentFrontier`] at `path`.
    pub fn resumable(
        start_url: &str,
        session: Arc<Session>,
        path: &std::path::Path,
    ) -> Result<Self> {
        let frontier: Arc<dyn Frontier> =
            Arc::new(crate::crawl::frontier::PersistentFrontier::open(path)?);
        Ok(Self::new(start_url, session).with_frontier(frontier))
    }

    /// Synchronous crawl returning `CrawlResult`.
    pub fn crawl(&self) -> Result<CrawlResult> {
        crate::TOKIO_RUNTIME.block_on(self.crawl_async())
    }

    /// Asynchronous crawling engine with worker synchronization and resilience.
    pub async fn crawl_async(&self) -> Result<CrawlResult> {
        let start_time = std::time::Instant::now();
        let results = Arc::new(Mutex::new(Vec::new()));
        let errors = Arc::new(Mutex::new(Vec::new()));
        let discovered_urls = Arc::new(Mutex::new(Vec::new()));

        let visited_count = Arc::new(AtomicUsize::new(0));
        let successful_count = Arc::new(AtomicUsize::new(0));
        let failed_count = Arc::new(AtomicUsize::new(0));
        let skipped_count = Arc::new(AtomicUsize::new(0));

        let frontier: Arc<dyn Frontier> = match &self.frontier {
            Some(f) => f.clone(),
            None => Arc::new(MemoryFrontier::new()),
        };

        if frontier.pending_len() == 0
            && frontier.visited_len() == 0
            && frontier.in_flight_len() == 0
        {
            frontier.enqueue(self.start_url.clone(), 0);
            discovered_urls.lock().await.push(self.start_url.clone());
        }

        let limit = self.limit;
        let max_depth = self.max_depth;
        let concurrency = self.concurrency.max(1);
        let follow_sel = self.follow_selector.clone();
        let delay = self.delay_seconds;
        let webhook_url = self.webhook_url.clone();
        let respect_robots = self.respect_robots;
        let robots_cache = self.robots_cache.clone();
        let cancellation_token = self.cancellation_token.clone();

        let manager = self.session.fetch_manager();
        let webhook_client = webhook_url.as_ref().map(|_| Arc::new(wreq::Client::new()));

        let mut fields_def = Vec::new();
        for f in &self.fields {
            fields_def.push(DatasetField {
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
        let paginator = self
            .pagination
            .as_ref()
            .map(|cfg| Arc::new(Paginator::new(cfg.clone())));

        // Synchronization across concurrent workers to avoid early exit on transient empty queue
        let active_workers = Arc::new(AtomicUsize::new(0));
        let notify = Arc::new(Notify::new());
        let scheduler = Arc::new(Mutex::new(()));

        let mut workers = JoinSet::new();

        for _ in 0..concurrency {
            let results = results.clone();
            let errors = errors.clone();
            let discovered_urls = discovered_urls.clone();
            let visited_count = visited_count.clone();
            let successful_count = successful_count.clone();
            let failed_count = failed_count.clone();
            let skipped_count = skipped_count.clone();
            let frontier = frontier.clone();
            let manager = manager.clone();
            let session = self.session.clone();
            let fields = fields_def_arc.clone();
            let follow_sel = follow_sel.clone();
            let webhook_url = webhook_url.clone();
            let webhook_client = webhook_client.clone();
            let paginator = paginator.clone();
            let active_workers = active_workers.clone();
            let notify = notify.clone();
            let robots_cache = robots_cache.clone();
            let cancellation_token = cancellation_token.clone();
            let scheduler = scheduler.clone();

            workers.spawn(async move {
                loop {
                    // Check cancellation
                    if cancellation_token.is_cancelled() {
                        break;
                    }

                    // 1. Check limit
                    let scheduling = scheduler.lock().await;
                    if visited_count.load(Ordering::SeqCst) >= limit {
                        notify.notify_waiters();
                        break;
                    }

                    // 2. Pop next URL
                    let (url_str, depth) = match frontier.dequeue() {
                        Some(task) => task,
                        None => {
                            drop(scheduling);
                            // Queue is empty. Check if any worker is actively processing work that might enqueue new links
                            if active_workers.load(Ordering::SeqCst) == 0 && frontier.pending_len() == 0 {
                                notify.notify_waiters();
                                break;
                            }
                            // Sleep briefly awaiting new work notification
                            tokio::select! {
                                _ = cancellation_token.cancelled() => break,
                                _ = notify.notified() => {},
                                _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {}
                            }
                            continue;
                        }
                    };

                    active_workers.fetch_add(1, Ordering::SeqCst);
                    visited_count.fetch_add(1, Ordering::SeqCst);
                    drop(scheduling);

                    let mut headers = session.read_headers();
                    let agent = headers.iter()
                        .find(|(key, _)| key.eq_ignore_ascii_case("user-agent"))
                        .map(|(_, value)| value.clone())
                        .unwrap_or_else(|| "CrawlingoBot".into());
                    if !headers.keys().any(|key| key.eq_ignore_ascii_case("user-agent")) {
                        headers.insert("User-Agent".into(), agent.clone());
                    }
                    // Robots.txt Check
                    if respect_robots {
                        let rules = robots_cache.get_or_fetch(&session, &url_str).await;
                        let path = url::Url::parse(&url_str)
                            .map(|u| u[url::Position::BeforePath..url::Position::AfterQuery].to_string())
                            .unwrap_or_else(|_| "/".to_string());
                        if !rules.is_allowed(&agent, &path) {
                            tracing::warn!("URL disallowed by robots.txt: {}", url_str);
                            skipped_count.fetch_add(1, Ordering::SeqCst);
                            frontier.mark_visited(&url_str);
                            active_workers.fetch_sub(1, Ordering::SeqCst);
                            notify.notify_waiters();
                            continue;
                        }
                        robots_cache.wait_delay(&url_str, &rules, &agent).await;
                    }

                    // Request parameters from Session
                    let cookies = session.cookies.read().unwrap().clone();
                    let proxy = session.get_next_proxy_with_auth();
                    let timeout_secs = *session.timeout_seconds.read().unwrap();
                    let fetcher_tier = *session.fetcher_tier.read().unwrap();
                    let browser_profile = session.browser_profile.read().unwrap().clone();
                    let retries = session.retries.read().unwrap().unwrap_or(3) as usize;

                    let req = FetchRequest {
                        url: url_str.clone(),
                        tier: fetcher_tier,
                        browser_profile,
                        headers,
                        cookies,
                        proxy,
                        timeout: std::time::Duration::from_secs(timeout_secs),
                        retries,
                    };

                    if delay > 0.0 {
                        tokio::time::sleep(std::time::Duration::from_secs_f64(delay)).await;
                    }

                    let response = tokio::select! {
                        _ = cancellation_token.cancelled() => {
                            frontier.unmark_in_flight(&url_str);
                            frontier.enqueue(url_str.clone(), depth);
                            active_workers.fetch_sub(1, Ordering::SeqCst);
                            notify.notify_waiters();
                            break;
                        }
                        response = manager.dispatch(req) => response,
                    };
                    match response {
                        Ok(response) => {
                            if response.status >= 400 {
                                failed_count.fetch_add(1, Ordering::SeqCst);
                                errors.lock().await.push(format!("HTTP {} for {}", response.status, url_str));
                            } else if let Ok(page) = HtmlParser::parse(response) {
                                successful_count.fetch_add(1, Ordering::SeqCst);

                                let mut fields_map = HashMap::new();
                                for f in fields.iter() {
                                    let query = SelectorQuery::from_type_str(
                                        &f.selector_type,
                                        &f.selector,
                                    );

                                    let matches = page.query(query).unwrap_or_default();
                                    let extracted = if matches.is_empty() {
                                        None
                                    } else {
                                        let value = match &f.extract_type {
                                            crate::extraction::ExtractionType::Attribute(attr) => {
                                                page.get_node_attr(&matches, attr)
                                            }
                                            other => {
                                                let combined_text =
                                                    page.get_nodes_combined_text(&matches);
                                                crate::extraction::ExtractionEngine::normalize_value(
                                                    &combined_text,
                                                    other,
                                                    page.url(),
                                                )
                                            }
                                        };
                                        if value.is_empty() { None } else { Some(value) }
                                    };
                                    let final_val =
                                        extracted.or_else(|| f.default.clone()).unwrap_or_default();
                                    fields_map.insert(f.name.clone(), final_val);
                                }

                                let result = DatasetResult {
                                    url: url_str.clone(),
                                    fields: fields_map,
                                    timestamp: chrono::Utc::now(),
                                };
                                results.lock().await.push(result.clone());

                                // Webhook delivery with best-effort retry
                                if let (Some(hook_url), Some(client)) =
                                    (webhook_url.as_ref(), webhook_client.as_ref())
                                {
                                    let hook_url = hook_url.clone();
                                    let client = client.clone();
                                    let result = result.clone();
                                    tokio::spawn(async move {
                                        for attempt in 1..=3 {
                                            let res = client
                                                .request(wreq::Method::POST, hook_url.clone())
                                                .json(&result)
                                                .send()
                                                .await;
                                            if res.is_ok() {
                                                break;
                                            }
                                            if attempt < 3 {
                                                tokio::time::sleep(std::time::Duration::from_millis(100 * attempt)).await;
                                            }
                                        }
                                    });
                                }

                                // Pagination
                                if let Some(ref pag) = paginator {
                                    let _ = pag.enqueue_next(&url_str, page.dom_tree(), &frontier);
                                }

                                // Link discovery within depth
                                if depth < max_depth && !follow_sel.is_empty() {
                                    let matches = page
                                        .query(SelectorQuery::Css(&follow_sel))
                                        .unwrap_or_default();
                                    for &link_idx in &matches {
                                        if let Some(href) =
                                            page.dom_tree().nodes[link_idx].attrs.get("href")
                                        {
                                            if let Some(abs_url) = canonicalize_relative_url(&url_str, href)
                                            {
                                                if !frontier.is_visited(&abs_url) {
                                                    frontier.enqueue(abs_url.clone(), depth + 1);
                                                    discovered_urls.lock().await.push(abs_url);
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                failed_count.fetch_add(1, Ordering::SeqCst);
                                errors.lock().await.push(format!("Parse failed for {}", url_str));
                            }
                        }
                        Err(e) => {
                            failed_count.fetch_add(1, Ordering::SeqCst);
                            errors.lock().await.push(format!("Fetch error for {}: {}", url_str, e));
                            tracing::error!("Crawler failed to fetch URL {}: {}", url_str, e);
                        }
                    }

                    frontier.mark_visited(&url_str);
                    active_workers.fetch_sub(1, Ordering::SeqCst);
                    notify.notify_waiters();
                }
            });
        }

        while let Some(result) = workers.join_next().await {
            result.map_err(|e| {
                crate::error::CrawlingoError::DatasetError(format!("crawler worker failed: {e}"))
            })?;
        }
        frontier.check_error()?;

        let duration_ms = start_time.elapsed().as_millis() as u64;
        let final_results = results.lock().await.clone();
        let final_errors = errors.lock().await.clone();
        let final_discovered = discovered_urls.lock().await.clone();

        Ok(CrawlResult {
            visited_count: visited_count.load(Ordering::SeqCst),
            successful_count: successful_count.load(Ordering::SeqCst),
            failed_count: failed_count.load(Ordering::SeqCst),
            skipped_count: skipped_count.load(Ordering::SeqCst),
            duration_ms,
            discovered_urls: final_discovered,
            errors: final_errors,
            results: final_results,
        })
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
                tracing::error!("Scheduled crawl error for {}: {}", crawler.start_url, e);
            }
            std::thread::sleep(std::time::Duration::from_secs(interval_seconds));
        });
    }
}

// PyO3 Bindings
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "CrawlResult")]
#[derive(Clone)]
pub struct PyCrawlResult {
    pub inner: CrawlResult,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyCrawlResult {
    #[getter]
    pub fn visited_count(&self) -> usize {
        self.inner.visited_count
    }
    #[getter]
    pub fn successful_count(&self) -> usize {
        self.inner.successful_count
    }
    #[getter]
    pub fn failed_count(&self) -> usize {
        self.inner.failed_count
    }
    #[getter]
    pub fn skipped_count(&self) -> usize {
        self.inner.skipped_count
    }
    #[getter]
    pub fn duration_ms(&self) -> u64 {
        self.inner.duration_ms
    }
    #[getter]
    pub fn discovered_urls(&self) -> Vec<String> {
        self.inner.discovered_urls.clone()
    }
    #[getter]
    pub fn errors(&self) -> Vec<String> {
        self.inner.errors.clone()
    }
}

#[cfg(feature = "python")]
use crate::engine::session::PySession;

#[cfg(feature = "python")]
#[pyclass(name = "Crawl")]
pub struct PyCrawl {
    pub inner: Crawler,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyCrawl {
    #[new]
    pub fn new(url: &str, session: &PySession) -> Self {
        Self {
            inner: Crawler::new(url, session.inner.clone()),
        }
    }

    #[staticmethod]
    pub fn resumable(url: &str, session: &PySession, path: &str) -> PyResult<Self> {
        let crawler = Crawler::resumable(url, session.inner.clone(), std::path::Path::new(path))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: crawler })
    }

    pub fn follow(&mut self, selector: &str) {
        self.inner.follow_selector = selector.to_string();
    }

    pub fn with_pagination(&mut self, config: &crate::crawl::pagination::PyPaginationConfig) {
        self.inner.pagination = Some(config.inner.clone());
    }

    pub fn limit(&mut self, limit: usize) {
        self.inner.limit = limit;
    }

    pub fn max_depth(&mut self, depth: usize) {
        self.inner.max_depth = depth;
    }

    pub fn concurrency(&mut self, concurrency: usize) {
        self.inner.concurrency = concurrency.max(1);
    }

    pub fn delay(&mut self, seconds: f64) {
        self.inner.delay_seconds = seconds;
    }

    pub fn respect_robots(&mut self, respect: bool) {
        self.inner.respect_robots = respect;
    }

    pub fn cancel(&self) {
        self.inner.cancel();
    }

    pub fn field(
        &mut self,
        name: &str,
        selector: &str,
        selector_type: Option<&str>,
        default: Option<&str>,
        extract_type: Option<&str>,
    ) {
        let st = selector_type.unwrap_or("css").to_string();
        let ext = extract_type
            .map(|t| match t {
                s if s.starts_with("attr:") => crate::extraction::ExtractionType::Attribute(
                    s.trim_start_matches("attr:").to_string(),
                ),
                "price" => crate::extraction::ExtractionType::Price,
                "datetime" => crate::extraction::ExtractionType::DateTime,
                "url" => crate::extraction::ExtractionType::NormalizedUrl,
                _ => crate::extraction::ExtractionType::Text,
            })
            .unwrap_or(crate::extraction::ExtractionType::Text);

        self.inner.fields.push(DatasetField {
            name: name.to_string(),
            selector: selector.to_string(),
            selector_type: st,
            transform: None,
            default: default.map(|s| s.to_string()),
            extract_type: ext,
        });
    }

    pub fn crawl(&self, py: Python<'_>) -> PyResult<Vec<HashMap<String, String>>> {
        let result = py
            .allow_threads(|| self.inner.crawl())
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(result.results.into_iter().map(|r| r.fields).collect())
    }

    pub fn build(&self, py: Python<'_>) -> PyResult<Vec<HashMap<String, String>>> {
        self.crawl(py)
    }
}
