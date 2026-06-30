use std::collections::HashMap;
use std::sync::Arc;
use napi_derive::napi;
use napi::bindgen_prelude::*;
use crawlingo::engine::session::Session;
use crawlingo::parser::document::DomTree;
use crawlingo::engine::fetcher::{FetchRequest, FetcherTier};
use crawlingo::engine::pool::ConnectionPoolConfig;
use crawlingo::selector::{css, xpath, text_anchor, regex_selector};
use crawlingo::dataset::builder::{Dataset, DatasetField, DatasetResult};
use crawlingo::crawl::crawler::Crawler;
use crawlingo::change::detector::{detect_changes, ChangeType};

#[napi(object)]
pub struct JsChangeEvent {
    pub url: String,
    pub field: String,
    pub change_type: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

impl From<crawlingo::change::detector::ChangeEvent> for JsChangeEvent {
    fn from(evt: crawlingo::change::detector::ChangeEvent) -> Self {
        let change_type = match evt.change_type {
            ChangeType::ContentChange => "ContentChange".to_string(),
            ChangeType::PriceChange { .. } => "PriceChange".to_string(),
            ChangeType::StockChange { .. } => "StockChange".to_string(),
            ChangeType::ElementAdded => "ElementAdded".to_string(),
            ChangeType::ElementRemoved => "ElementRemoved".to_string(),
            ChangeType::LayoutChange => "LayoutChange".to_string(),
        };

        Self {
            url: evt.url,
            field: evt.field,
            change_type,
            old_value: Some(evt.old_value),
            new_value: Some(evt.new_value),
        }
    }
}

#[napi]
pub struct JsSession {
    pub(crate) inner: Arc<Session>,
}

#[napi]
impl JsSession {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Session::new()),
        }
    }

    #[napi]
    pub fn headers(&self, headers: HashMap<String, String>) -> napi::Result<()> {
        let mut h = self.inner.headers.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *h = headers;
        Ok(())
    }

    #[napi]
    pub fn cookies(&self, cookies: HashMap<String, String>) -> napi::Result<()> {
        let mut c = self.inner.cookies.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *c = cookies;
        Ok(())
    }

    #[napi]
    pub fn proxy(&self, proxy_url: String) -> napi::Result<()> {
        let mut p = self.inner.proxy.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *p = Some(proxy_url);
        Ok(())
    }

    #[napi]
    pub fn rate_limit(&self, requests_per_second: f64) -> napi::Result<()> {
        let mut r = self.inner.rate_limit_rps.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *r = requests_per_second;
        Ok(())
    }

    #[napi]
    pub fn auto_match(&self, enabled: bool) -> napi::Result<()> {
        let mut a = self.inner.auto_match.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *a = enabled;
        Ok(())
    }

    #[napi]
    pub fn timeout(&self, seconds: u32) -> napi::Result<()> {
        let mut t = self.inner.timeout_seconds.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *t = seconds as u64;
        Ok(())
    }

    #[napi]
    pub fn fingerprint_path(&self, path: String) -> napi::Result<()> {
        let mut f = self.inner.fingerprint_path.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *f = path;
        Ok(())
    }

    #[napi]
    pub fn fetcher_tier(&self, tier: String) -> napi::Result<()> {
        let tier_enum = if tier.to_lowercase() == "stealthy" {
            FetcherTier::Stealthy
        } else {
            FetcherTier::Standard
        };
        let mut t = self.inner.fetcher_tier.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *t = tier_enum;
        Ok(())
    }

    #[napi]
    pub fn browser_profile(&self, profile: Option<String>) -> napi::Result<()> {
        let mut b = self.inner.browser_profile.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *b = profile;
        Ok(())
    }

    #[napi]
    pub fn auto_match_weights(&self, weights: HashMap<String, f64>) -> napi::Result<()> {
        let mut w = self.inner.similarity_weights.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *w = weights;
        Ok(())
    }

    #[napi]
    pub fn proxy_pool(&self, proxies: Vec<String>) -> napi::Result<()> {
        let mut p = self.inner.proxy_pool.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *p = proxies;
        Ok(())
    }

    #[napi]
    pub fn proxy_provider(&self, url: Option<String>) -> napi::Result<()> {
        let mut u = self.inner.proxy_provider_url.write()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        *u = url;
        let _ = self.inner.fetch_provider_proxies();
        Ok(())
    }
}

#[napi]
pub struct JsPage {
    pub url: String,
    pub status: u16,
    pub html: String,
    pub(crate) tree: Arc<DomTree>,
}

#[napi]
impl JsPage {
    #[napi]
    pub fn title(&self) -> String {
        let matched = css::query(&self.tree, "title");
        if !matched.is_empty() {
            self.tree.get_text(matched[0])
        } else {
            String::new()
        }
    }

    #[napi]
    pub fn css(&self, selector: String) -> JsElementCollection {
        let indices = css::query(&self.tree, &selector);
        JsElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    #[napi]
    pub fn xpath(&self, query: String) -> JsElementCollection {
        let indices = xpath::query(&self.tree, &query);
        JsElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    #[napi]
    pub fn find_text(&self, text: String) -> JsElementCollection {
        let indices = text_anchor::find(&self.tree, &text);
        JsElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    #[napi]
    pub fn after_text(&self, text: String) -> JsElementCollection {
        let indices = text_anchor::after(&self.tree, &text);
        JsElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    #[napi]
    pub fn before_text(&self, text: String) -> JsElementCollection {
        let indices = text_anchor::before(&self.tree, &text);
        JsElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    #[napi]
    pub fn regex(&self, pattern: String) -> napi::Result<JsElementCollection> {
        let indices = regex_selector::query(&self.tree, &pattern)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(JsElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        })
    }
}

fn to_napi_error(err: crawlingo::error::CrawlingoError, url: &str, stage: &str) -> napi::Error {
    let error_code = match &err {
        crawlingo::error::CrawlingoError::ParseError(_) => "INVALID_HTML",
        crawlingo::error::CrawlingoError::TimeoutError { .. } => "TIMEOUT",
        crawlingo::error::CrawlingoError::RateLimitError { .. } => "RATE_LIMIT",
        crawlingo::error::CrawlingoError::FetchError(_) | crawlingo::error::CrawlingoError::HttpClientError(_) => "FETCH_FAILED",
        crawlingo::error::CrawlingoError::AutoMatchFailed => "AUTO_MATCH_FAILED",
        _ => "GENERIC_FAILURE",
    };

    let suggestion = match &err {
        crawlingo::error::CrawlingoError::ParseError(_) => "Falling back to HTML5 parser.",
        crawlingo::error::CrawlingoError::TimeoutError { .. } => "Increase request timeout or check target server responsiveness.",
        crawlingo::error::CrawlingoError::RateLimitError { .. } => "Implement exponential backoff or reduce request rate.",
        crawlingo::error::CrawlingoError::FetchError(_) | crawlingo::error::CrawlingoError::HttpClientError(_) => "Check network connectivity, headers, proxy configuration, or target URL status.",
        crawlingo::error::CrawlingoError::AutoMatchFailed => "Verify if page structure has changed significantly or update selectors.",
        _ => "Inspect server logs and try repeating the operation.",
    };

    let recoverable = match &err {
        crawlingo::error::CrawlingoError::ParseError(_) |
        crawlingo::error::CrawlingoError::TimeoutError { .. } |
        crawlingo::error::CrawlingoError::RateLimitError { .. } |
        crawlingo::error::CrawlingoError::FetchError(_) |
        crawlingo::error::CrawlingoError::HttpClientError(_) => true,
        _ => false,
    };

    let detailed = serde_json::json!({
        "success": false,
        "url": url,
        "stage": stage,
        "error_code": error_code,
        "message": err.to_string(),
        "recoverable": recoverable,
        "suggestion": suggestion,
    });

    let detailed_str = serde_json::to_string(&detailed).unwrap_or_else(|_| err.to_string());
    napi::Error::from_reason(detailed_str)
}

#[napi]
pub async fn fetch_page(
    url: String,
    auto_match: bool,
    timeout: Option<u32>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
    proxy: Option<String>,
    browser_profile: Option<String>,
    session: Option<&JsSession>,
) -> napi::Result<JsPage> {
    let headers_val = headers.unwrap_or_default();
    let cookies_val = cookies.unwrap_or_default();
    let timeout_val = timeout.unwrap_or(30) as u64;

    let (headers_f, cookies_f, proxy_f, rate_limit_rps_f, timeout_f, fetcher_tier_f, browser_profile_f) = if let Some(s) = session {
        let h = s.inner.headers.read().unwrap().clone();
        let c = s.inner.cookies.read().unwrap().clone();
        let p = s.inner.proxy.read().unwrap().clone();
        let r = *s.inner.rate_limit_rps.read().unwrap();
        let t = *s.inner.timeout_seconds.read().unwrap();
        let tier = *s.inner.fetcher_tier.read().unwrap();
        let b = s.inner.browser_profile.read().unwrap().clone();
        (h, c, p, r, t, tier, b)
    } else {
        (
            headers_val,
            cookies_val,
            proxy,
            0.0,
            timeout_val,
            if auto_match { FetcherTier::Stealthy } else { FetcherTier::Standard },
            browser_profile,
        )
    };

    let req = FetchRequest {
        url: url.clone(),
        tier: fetcher_tier_f,
        browser_profile: browser_profile_f,
        headers: headers_f,
        cookies: cookies_f,
        proxy: proxy_f,
        timeout: std::time::Duration::from_secs(timeout_f),
        retries: 3,
        rate_limit_rps: rate_limit_rps_f,
    };

    let rate_limiter = Arc::new(crawlingo::engine::rate_limiter::HostRateLimiter::new());
    let manager = crawlingo::engine::fetcher::FetchManager::new(rate_limiter, ConnectionPoolConfig::default());
    let resp = manager.dispatch(req).await
        .map_err(|e| to_napi_error(e, &url, "network"))?;
    let page = crawlingo::parser::streaming::HtmlParser::parse(resp)
        .map_err(|e| to_napi_error(e, &url, "parser"))?;

    Ok(JsPage {
        url,
        status: page.status(),
        html: page.html().to_string(),
        tree: page.dom_tree().clone(),
    })
}

#[napi]
pub struct JsElementCollection {
    pub(crate) tree: Arc<DomTree>,
    pub(crate) node_indices: Vec<usize>,
}

#[napi]
impl JsElementCollection {
    #[napi]
    pub fn length(&self) -> u32 {
        self.node_indices.len() as u32
    }

    #[napi]
    pub fn text(&self) -> Vec<String> {
        self.node_indices.iter().map(|&idx| self.tree.get_text(idx)).collect()
    }

    #[napi]
    pub fn html(&self) -> Vec<String> {
        self.node_indices.iter().map(|&idx| {
            if let Some(node) = self.tree.nodes.get(idx) {
                node.html_snippet.clone()
            } else {
                String::new()
            }
        }).collect()
    }

    #[napi]
    pub fn attr(&self, name: String) -> Vec<Option<String>> {
        self.node_indices.iter().map(|&idx| {
            self.tree.nodes.get(idx).and_then(|n| n.attrs.get(&name).cloned())
        }).collect()
    }

    #[napi]
    pub fn at(&self, index: u32) -> Option<JsElement> {
        self.node_indices.get(index as usize).map(|&idx| JsElement {
            tree: self.tree.clone(),
            node_idx: idx,
        })
    }
}

#[napi]
pub struct JsElement {
    pub(crate) tree: Arc<DomTree>,
    pub(crate) node_idx: usize,
}

#[napi]
impl JsElement {
    #[napi]
    pub fn text(&self) -> String {
        self.tree.get_text(self.node_idx)
    }

    #[napi]
    pub fn html(&self) -> String {
        if let Some(node) = self.tree.nodes.get(self.node_idx) {
            node.html_snippet.clone()
        } else {
            String::new()
        }
    }

    #[napi]
    pub fn attr(&self, name: String) -> Option<String> {
        self.tree.nodes.get(self.node_idx).and_then(|n| n.attrs.get(&name).cloned())
    }
}

#[napi]
pub struct JsDatasetResult {
    pub(crate) inner: DatasetResult,
}

#[napi]
impl JsDatasetResult {
    #[napi]
    pub fn to_dict(&self) -> HashMap<String, String> {
        self.inner.fields.clone()
    }

    #[napi]
    pub async fn to_json(&self, path: String) -> napi::Result<()> {
        let file = std::fs::File::create(&path)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        serde_json::to_writer_pretty(file, &self.inner.fields)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(())
    }

    #[napi]
    pub async fn to_csv(&self, path: String) -> napi::Result<()> {
        let mut writer = csv::Writer::from_path(&path)
            .map_err(|e: csv::Error| napi::Error::from_reason(e.to_string()))?;
        
        let keys: Vec<&str> = self.inner.fields.keys().map(|k| k.as_str()).collect();
        writer.write_record(&keys)
            .map_err(|e: csv::Error| napi::Error::from_reason(e.to_string()))?;
        
        let values: Vec<&str> = self.inner.fields.values().map(|v| v.as_str()).collect();
        writer.write_record(&values)
            .map_err(|e: csv::Error| napi::Error::from_reason(e.to_string()))?;
        
        writer.flush()
            .map_err(|e: std::io::Error| napi::Error::from_reason(e.to_string()))?;
        Ok(())
    }

    #[napi]
    pub async fn to_parquet(&self, path: String) -> napi::Result<()> {
        crawlingo::dataset::export::write_parquet(&path, &self.inner.fields).await
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(())
    }
}

#[napi]
pub struct JsDataset {
    pub(crate) url: String,
    pub(crate) fields: Vec<DatasetField>,
    pub(crate) session: Arc<Session>,
}

impl JsDataset {
    /// Core logic: zip selector matches from an already-parsed DomTree into structured records.
    fn extract_from_tree(&self, tree: &crawlingo::parser::document::DomTree) -> Vec<HashMap<String, String>> {
        use crawlingo::selector::{css, xpath, text_anchor, regex_selector};

        // Collect all match index lists per field
        let collections: Vec<Vec<usize>> = self.fields.iter().map(|f| {
            match f.selector_type.as_str() {
                "xpath"       => xpath::query(tree, &f.selector),
                "text"        => text_anchor::find(tree, &f.selector),
                "after_text"  => text_anchor::after(tree, &f.selector),
                "before_text" => text_anchor::before(tree, &f.selector),
                "regex"       => regex_selector::query(tree, &f.selector).unwrap_or_default(),
                _             => css::query(tree, &f.selector),  // default: css
            }
        }).collect();

        let max_len = collections.iter().map(|c| c.len()).max().unwrap_or(0);
        let mut records = Vec::with_capacity(max_len);

        for row_idx in 0..max_len {
            let mut record = HashMap::new();
            for (field_idx, field) in self.fields.iter().enumerate() {
                let text = collections[field_idx]
                    .get(row_idx)
                    .map(|&node_idx| tree.get_text(node_idx).trim().to_string())
                    .unwrap_or_default();
                record.insert(field.name.clone(), text);
            }
            records.push(record);
        }

        records
    }
}

#[napi]
impl JsDataset {
    #[napi(constructor)]
    pub fn new(url: String, session: &JsSession) -> Self {
        Self {
            url,
            fields: Vec::new(),
            session: session.inner.clone(),
        }
    }

    #[napi]
    pub fn field(&mut self, name: String, selector: String, selector_type: Option<String>, default_val: Option<String>) {
        let field = DatasetField {
            name,
            selector,
            selector_type: selector_type.unwrap_or("css".to_string()),
            default: default_val,
        };
        self.fields.push(field);
    }

    #[napi]
    pub async fn build(&self) -> napi::Result<JsDatasetResult> {
        let mut dataset = Dataset::new(&self.url, self.session.clone());
        dataset.fields = self.fields.clone();
        let res = dataset.build_async().await
            .map_err(|e| to_napi_error(e, &self.url, "dataset"))?;
        Ok(JsDatasetResult { inner: res })
    }

    /// Synchronously extract structured multi-row records from an already-fetched JsPage.
    /// Returns a Vec of HashMaps, one per row, zipped by element index across all selectors.
    #[napi]
    pub fn extract_structured(&self, page: &JsPage) -> Vec<HashMap<String, String>> {
        self.extract_from_tree(&page.tree)
    }

    /// Fetch the URL, parse the page, and extract structured multi-row records entirely in Rust.
    #[napi]
    pub async fn build_structured(&self) -> napi::Result<Vec<HashMap<String, String>>> {
        use crawlingo::engine::fetcher::{FetchRequest, FetchManager};
        use crawlingo::engine::pool::ConnectionPoolConfig;
        use crawlingo::parser::streaming::HtmlParser;

        let headers = self.session.headers.read().unwrap().clone();
        let cookies = self.session.cookies.read().unwrap().clone();
        let proxy   = self.session.get_next_proxy();
        let rate_limit_rps = *self.session.rate_limit_rps.read().unwrap();
        let timeout_secs   = *self.session.timeout_seconds.read().unwrap();
        let fetcher_tier   = *self.session.fetcher_tier.read().unwrap();
        let browser_profile = self.session.browser_profile.read().unwrap().clone();

        let req = FetchRequest {
            url: self.url.clone(),
            tier: fetcher_tier,
            browser_profile,
            headers,
            cookies,
            proxy,
            timeout: std::time::Duration::from_secs(timeout_secs),
            retries: 3,
            rate_limit_rps,
        };

        let rate_limiter = Arc::new(crawlingo::engine::rate_limiter::HostRateLimiter::new());
        let manager = FetchManager::new(rate_limiter, ConnectionPoolConfig::default());
        let resp  = manager.dispatch(req).await
            .map_err(|e| to_napi_error(e, &self.url, "network"))?;
        let page = HtmlParser::parse(resp)
            .map_err(|e| to_napi_error(e, &self.url, "parser"))?;

        Ok(self.extract_from_tree(page.dom_tree()))
    }
}

/// Write structured records as a pretty-printed JSON array to `path`.
/// Each record is a flat object (field_name → value).
#[napi]
pub fn save_structured_json(records: Vec<HashMap<String, String>>, path: String) -> napi::Result<()> {
    let json = serde_json::to_string_pretty(&records)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    std::fs::write(&path, json)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(())
}

/// Write structured records as a clean CSV file to `path`.
/// The first row is the header (field names); subsequent rows contain values.
#[napi]
pub fn save_structured_csv(records: Vec<HashMap<String, String>>, path: String) -> napi::Result<()> {
    let mut writer = csv::Writer::from_path(&path)
        .map_err(|e: csv::Error| napi::Error::from_reason(e.to_string()))?;

    if records.is_empty() {
        writer.flush()
            .map_err(|e: std::io::Error| napi::Error::from_reason(e.to_string()))?;
        return Ok(());
    }

    let headers: Vec<String> = records[0].keys().cloned().collect();
    let header_refs: Vec<&str> = headers.iter().map(|s| s.as_str()).collect();
    writer.write_record(&header_refs)
        .map_err(|e: csv::Error| napi::Error::from_reason(e.to_string()))?;

    for record in &records {
        let values: Vec<String> = headers.iter()
            .map(|h| record.get(h).cloned().unwrap_or_default())
            .collect();
        let value_refs: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
        writer.write_record(&value_refs)
            .map_err(|e: csv::Error| napi::Error::from_reason(e.to_string()))?;
    }

    writer.flush()
        .map_err(|e: std::io::Error| napi::Error::from_reason(e.to_string()))?;
    Ok(())
}

#[napi]
pub struct JsCrawl {
    pub(crate) crawler: Crawler,
}

#[napi]
impl JsCrawl {
    #[napi(constructor)]
    pub fn new(start_url: String, session: &JsSession) -> Self {
        Self {
            crawler: Crawler::new(&start_url, session.inner.clone()),
        }
    }

    #[napi]
    pub fn follow(&mut self, selector: String) {
        self.crawler.follow_selector = selector;
    }

    #[napi]
    pub fn limit(&mut self, limit: u32) {
        self.crawler.limit = limit as usize;
    }

    #[napi]
    pub fn depth(&mut self, depth: u32) {
        self.crawler.max_depth = depth as usize;
    }

    #[napi]
    pub fn concurrency(&mut self, n: u32) {
        self.crawler.concurrency = n as usize;
    }

    #[napi]
    pub fn delay(&mut self, seconds: f64) {
        self.crawler.delay_seconds = seconds;
    }

    #[napi]
    pub fn field(&mut self, name: String, selector: String, selector_type: Option<String>, default_val: Option<String>) {
        let field = DatasetField {
            name,
            selector,
            selector_type: selector_type.unwrap_or("css".to_string()),
            default: default_val,
        };
        self.crawler.fields.push(field);
    }

    #[napi]
    pub fn webhook(&mut self, url: String) {
        self.crawler.webhook_url = Some(url);
    }

    #[napi]
    pub fn schedule(&self, interval_seconds: u32) {
        self.crawler.run_scheduled(interval_seconds as u64);
    }

    #[napi]
    pub async fn run(&self) -> napi::Result<Vec<JsDatasetResult>> {
        let res = self.crawler.crawl_async().await
            .map_err(|e| to_napi_error(e, &self.crawler.start_url, "crawler"))?;
        let results = res.into_iter().map(|item| JsDatasetResult { inner: item }).collect();
        Ok(results)
    }
}

use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};

#[napi]
pub struct JsWatch {
    url: String,
    fields: Vec<DatasetField>,
    interval_seconds: u32,
    session: Arc<Session>,
    cancellation_token: tokio_util::sync::CancellationToken,
}

#[napi]
impl JsWatch {
    #[napi(constructor)]
    pub fn new(url: String, session: &JsSession) -> Self {
        Self {
            url,
            fields: Vec::new(),
            interval_seconds: 60,
            session: session.inner.clone(),
            cancellation_token: tokio_util::sync::CancellationToken::new(),
        }
    }

    #[napi]
    pub fn field(&mut self, name: String, selector: String, selector_type: Option<String>, default_val: Option<String>) {
        let field = DatasetField {
            name,
            selector,
            selector_type: selector_type.unwrap_or("css".to_string()),
            default: default_val,
        };
        self.fields.push(field);
    }

    #[napi]
    pub fn interval(&mut self, seconds: u32) {
        self.interval_seconds = seconds;
    }

    #[napi]
    pub fn run(
        &self,
        #[napi(ts_arg_type = "(err: Error | null, event: JsChangeEvent) => void")]
        callback: JsFunction,
    ) -> napi::Result<()> {
        let tsfn: ThreadsafeFunction<JsChangeEvent, ErrorStrategy::CalleeHandled> = callback
            .create_threadsafe_function(0, |ctx| {
                Ok(vec![ctx.value])
            })?;

        let url = self.url.clone();
        let session = self.session.clone();
        let interval_sec = self.interval_seconds as u64;
        let token = self.cancellation_token.clone();
        let fields = self.fields.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(interval_sec));
            let mut previous_data = HashMap::new();

            loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        break;
                    }
                    _ = interval.tick() => {
                        let mut dataset = Dataset::new(&url, session.clone());
                        dataset.fields = fields.clone();

                        match dataset.build_async().await {
                            Ok(res) => {
                                if !previous_data.is_empty() {
                                    let changes = detect_changes(&url, &previous_data, &res.fields);
                                    for change in changes {
                                        let js_evt = JsChangeEvent::from(change);
                                        let _ = tsfn.call(Ok(js_evt), ThreadsafeFunctionCallMode::NonBlocking);
                                    }
                                }
                                previous_data = res.fields;
                            }
                            Err(e) => {
                                let err_msg = format!("Watch check failed for {}: {}", url, e);
                                let _ = tsfn.call(Err(napi::Error::from_reason(err_msg)), ThreadsafeFunctionCallMode::NonBlocking);
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    #[napi]
    pub fn stop(&self) {
        self.cancellation_token.cancel();
    }
}
