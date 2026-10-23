use std::collections::HashMap;
use std::sync::Arc;
use napi_derive::napi;
use napi::bindgen_prelude::*;
use crawlingo::engine::session::Session;
use crawlingo::parser::document::DomTree;
use crawlingo::engine::fetcher::{FetchRequest, FetcherTier};
use crawlingo::selector::{css, xpath, text_anchor, regex_selector};
use crawlingo::dataset::builder::{Dataset, DatasetField, DatasetResult};
use crawlingo::dataset::job::DatasetJob;
use crawlingo::crawl::crawler::Crawler;
use crawlingo::crawl::pagination::PaginationConfig;
use crawlingo::dataset::schema::{DatasetSchema, FieldType, FieldConstraint};
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
            _ => "Unknown".to_string(),
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
    #[napi(factory)]
    pub fn from_config(path: Option<String>) -> napi::Result<Self> {
        let config = crawlingo::config::CrawlingoConfig::load(path.as_deref().map(std::path::Path::new))
            .map_err(|e| to_napi_error(e, "", "configuration"))?;
        Ok(Self { inner: Arc::new(Session::from_config(&config)) })
    }

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
    pub fn get_cookies(&self) -> napi::Result<HashMap<String, String>> {
        Ok(self.inner.cookies.read().map_err(|e| napi::Error::from_reason(e.to_string()))?.clone())
    }

    #[napi]
    pub fn delete_cookie(&self, name: String) -> napi::Result<()> {
        self.inner.cookies.write().map_err(|e| napi::Error::from_reason(e.to_string()))?.remove(&name);
        Ok(())
    }

    #[napi]
    pub fn clear_cookies(&self) -> napi::Result<()> {
        self.inner.cookies.write().map_err(|e| napi::Error::from_reason(e.to_string()))?.clear();
        Ok(())
    }

    #[napi]
    pub fn retries(&self, count: f64) -> napi::Result<()> {
        if !count.is_finite() || count < 0.0 || count > u32::MAX as f64 || count.fract() != 0.0 {
            return Err(napi::Error::from_reason("retries must be an integer between 0 and 4294967295"));
        }
        *self.inner.retries.write().map_err(|e| napi::Error::from_reason(e.to_string()))? = Some(count as u32);
        Ok(())
    }

    #[napi]
    pub fn retry_backoff(&self, factor: f64) -> napi::Result<()> {
        if !factor.is_finite() || factor < 1.0 {
            return Err(napi::Error::from_reason("retryBackoff must be finite and >= 1"));
        }
        *self.inner.retry_backoff.write().map_err(|e| napi::Error::from_reason(e.to_string()))? = Some(factor);
        Ok(())
    }

    #[napi]
    pub fn retry_delay(&self, seconds: f64) -> napi::Result<()> {
        if !seconds.is_finite() || seconds < 0.0 || seconds > 86_400.0 || seconds.fract() != 0.0 {
            return Err(napi::Error::from_reason("retryDelay must be an integer between 0 and 86400 seconds"));
        }
        *self.inner.retry_delay.write().map_err(|e| napi::Error::from_reason(e.to_string()))? = Some(seconds as u32);
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
        let tier_enum = match tier.to_lowercase().as_str() {
            "stealthy" => FetcherTier::Stealthy,
            "standard" => FetcherTier::Standard,
            _ => return Err(napi::Error::from_reason("Unsupported fetcher tier: this Node build supports standard or stealthy; use the Playwright adapter for rendered HTML")),
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
        {
            let mut u = self.inner.proxy_provider_url.write()
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
            *u = url;
        }
        let _ = self.inner.fetch_provider_proxies();
        Ok(())
    }

    #[napi]
    pub fn clone(&self) -> napi::Result<JsSession> {
        let cloned = self.inner.clone_session();
        Ok(JsSession {
            inner: Arc::new(cloned),
        })
    }

    #[napi]
    pub fn destroy(&self) -> napi::Result<()> {
        self.inner.headers.write().unwrap().clear();
        self.inner.cookies.write().unwrap().clear();
        *self.inner.proxy.write().unwrap() = None;
        self.inner.proxy_pool.write().unwrap().clear();
        *self.inner.proxy_provider_url.write().unwrap() = None;
        *self.inner.fingerprint_store.write().unwrap() = None;
        *self.inner.fetch_manager.write().unwrap() = None;
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
            .map_err(|e| to_napi_error(e, &self.url, "selector"))?;
        Ok(JsElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        })
    }
}

fn to_napi_error(err: crawlingo::error::CrawlingoError, url: &str, stage: &str) -> napi::Error {
    use crawlingo::error::CrawlingoError as E;
    let error_type = match &err {
        E::FetchError(_) | E::HttpClientError(_) => "FetchError",
        E::ParseError(_) => "ParseError",
        E::SelectorError(_) => "SelectorError",
        E::AutoMatchFailed => "AutoMatchFailed",
        E::TimeoutError { .. } => "TimeoutError",
        E::RateLimitError { .. } => "RateLimitError",
        E::ChangeDetectionError(_) => "ChangeDetectionError",
        E::ExportError(_) | E::CsvError(_) | E::ParquetError(_) | E::ArrowError(_) => "ExportError",
        E::DnsError(_) => "DnsError",
        E::FingerprintStoreError(_) | E::SledError(_) | E::BincodeError(_) => "FingerprintStoreError",
        _ => "CrawlingoError",
    };
    let error_code = match &err {
        crawlingo::error::CrawlingoError::ParseError(_) => "INVALID_HTML",
        crawlingo::error::CrawlingoError::TimeoutError { .. } => "TIMEOUT",
        crawlingo::error::CrawlingoError::RateLimitError { .. } => "RATE_LIMIT",
        crawlingo::error::CrawlingoError::FetchError(_) | crawlingo::error::CrawlingoError::HttpClientError(_) => "FETCH_FAILED",
        crawlingo::error::CrawlingoError::AutoMatchFailed => "AUTO_MATCH_FAILED",
        E::SelectorError(_) => "SELECTOR_FAILED",
        E::DnsError(_) => "DNS_FAILED",
        E::ChangeDetectionError(_) => "CHANGE_DETECTION_FAILED",
        E::ExportError(_) | E::CsvError(_) | E::ParquetError(_) | E::ArrowError(_) => "EXPORT_FAILED",
        E::FingerprintStoreError(_) | E::SledError(_) | E::BincodeError(_) => "FINGERPRINT_STORE_FAILED",
        _ => "GENERIC_FAILURE",
    };

    let suggestion = match &err {
        crawlingo::error::CrawlingoError::ParseError(_) => "Check the response content type and HTML encoding.",
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
        "error_type": error_type,
        "seconds": match &err { E::TimeoutError { seconds } => Some(*seconds), _ => None },
        "host": match &err { E::RateLimitError { host } => Some(host.as_str()), _ => None },
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
    let active = session.map(|s| s.inner.clone()).unwrap_or_else(|| Arc::new(Session::new()));
    let mut headers_f = active.read_headers();
    headers_f.extend(headers.unwrap_or_default());
    let mut cookies_f = active.read_cookies();
    cookies_f.extend(cookies.unwrap_or_default());
    let timeout_f = timeout.map(u64::from).unwrap_or_else(|| *active.timeout_seconds.read().unwrap());
    let fetcher_tier_f = if session.is_none() && auto_match {
        FetcherTier::Stealthy
    } else {
        *active.fetcher_tier.read().unwrap()
    };
    let browser_profile_f = browser_profile.or_else(|| active.browser_profile.read().unwrap().clone());
    let proxy_f = proxy.or_else(|| active.get_next_proxy_with_auth());

    let req = FetchRequest {
        url: url.clone(),
        tier: fetcher_tier_f,
        browser_profile: browser_profile_f,
        headers: headers_f,
        cookies: cookies_f,
        proxy: proxy_f,
        timeout: std::time::Duration::from_secs(timeout_f),
        retries: active.retries.read().unwrap().unwrap_or(3) as usize,
    };

    let manager = active.fetch_manager();
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
            self.tree.get_outer_html(idx)
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
        self.tree.get_outer_html(self.node_idx)
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
pub struct JsDatasetJob { inner: DatasetJob }

#[napi]
impl JsDatasetJob {
    #[napi(constructor)]
    pub fn new(config_json: String, session: &JsSession) -> napi::Result<Self> {
        let config = serde_json::from_str(&config_json).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        let inner = DatasetJob::new(config, session.inner.clone()).map_err(|e| to_napi_error(e, "", "dataset_job"))?;
        Ok(Self { inner })
    }
    #[napi]
    pub async fn run(&self) -> napi::Result<String> {
        let report = self.inner.run().await.map_err(|e| to_napi_error(e, "", "dataset_job"))?;
        serde_json::to_string(&report).map_err(|e| napi::Error::from_reason(e.to_string()))
    }
    #[napi]
    pub fn cancel(&self) { self.inner.cancel(); }
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
    pub(crate) schema: Option<DatasetSchema>,
}

#[napi]
impl JsDataset {
    #[napi(constructor)]
    pub fn new(url: String, session: &JsSession) -> Self {
        Self {
            url,
            fields: Vec::new(),
            session: session.inner.clone(),
            schema: None,
        }
    }

    #[napi]
    pub fn field(
        &mut self,
        name: String,
        selector: String,
        selector_type: Option<String>,
        default_val: Option<String>,
        extract_type: Option<String>,
    ) {
        let ext = extract_type
            .as_deref()
            .map(crawlingo::extraction::ExtractionType::from_str_or_text)
            .unwrap_or_default();
        let field = DatasetField {
            name,
            selector,
            selector_type: selector_type.unwrap_or("css".to_string()),
            default: default_val,
            extract_type: ext,
        };
        self.fields.push(field);
    }

    #[napi]
    pub fn with_schema(&mut self, schema: &JsDatasetSchema) {
        self.schema = Some(schema.inner.clone());
    }

    #[napi]
    pub async fn build(&self) -> napi::Result<JsDatasetResult> {
        let mut dataset = Dataset::new(&self.url, self.session.clone());
        dataset.fields = self.fields.clone();
        dataset.schema = self.schema.clone();
        let res = dataset.build_async().await
            .map_err(|e| to_napi_error(e, &self.url, "dataset"))?;
        Ok(JsDatasetResult { inner: res })
    }

    /// Synchronously extract structured multi-row records from an already-fetched JsPage.
    /// Returns a Vec of HashMaps, one per row, zipped by element index across all selectors.
    #[napi]
    pub fn extract_structured(&self, page: &JsPage) -> napi::Result<Vec<HashMap<String, String>>> {
        let mut dataset = Dataset::new(&self.url, self.session.clone());
        dataset.fields = self.fields.clone();
        dataset.schema = self.schema.clone();
        dataset.extract_from_tree_checked(&page.tree).map_err(|e| to_napi_error(e, &self.url, "dataset"))
    }

    /// Fetch the URL, parse the page, and extract structured multi-row records entirely in Rust.
    #[napi]
    pub async fn build_structured(&self) -> napi::Result<Vec<HashMap<String, String>>> {
        let mut dataset = Dataset::new(&self.url, self.session.clone());
        dataset.fields = self.fields.clone();
        dataset.schema = self.schema.clone();
        dataset.build_structured().await
            .map_err(|e| to_napi_error(e, &self.url, "dataset"))
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

    #[napi(factory)]
    pub fn resumable(start_url: String, session: &JsSession, path: String) -> napi::Result<Self> {
        let crawler = Crawler::resumable(&start_url, session.inner.clone(), std::path::Path::new(&path))
            .map_err(|e| to_napi_error(e, &start_url, "crawler"))?;
        Ok(Self { crawler })
    }

    #[napi]
    pub fn with_pagination(&mut self, config: &JsPaginationConfig) {
        self.crawler.pagination = Some(config.inner.clone());
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
    pub fn field(
        &mut self,
        name: String,
        selector: String,
        selector_type: Option<String>,
        default_val: Option<String>,
        extract_type: Option<String>,
    ) {
        let ext = extract_type
            .as_deref()
            .map(crawlingo::extraction::ExtractionType::from_str_or_text)
            .unwrap_or_default();
        let field = DatasetField {
            name,
            selector,
            selector_type: selector_type.unwrap_or("css".to_string()),
            default: default_val,
            extract_type: ext,
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
        let results = res.results.into_iter().map(|item| JsDatasetResult { inner: item }).collect();
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
    pub fn field(
        &mut self,
        name: String,
        selector: String,
        selector_type: Option<String>,
        default_val: Option<String>,
        extract_type: Option<String>,
    ) {
        let ext = extract_type
            .as_deref()
            .map(crawlingo::extraction::ExtractionType::from_str_or_text)
            .unwrap_or_default();
        let field = DatasetField {
            name,
            selector,
            selector_type: selector_type.unwrap_or("css".to_string()),
            default: default_val,
            extract_type: ext,
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
                                let _ = tsfn.call(Err(to_napi_error(e, &url, "watch")), ThreadsafeFunctionCallMode::NonBlocking);
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

#[napi(object)]
pub struct JsDownloadResult {
    pub url: String,
    pub status: u16,
    pub bytes_written: f64,
    pub content_type: String,
    pub suggested_filename: Option<String>,
    pub resumed: bool,
}

impl From<crawlingo::engine::download::DownloadResult> for JsDownloadResult {
    fn from(res: crawlingo::engine::download::DownloadResult) -> Self {
        Self {
            url: res.url,
            status: res.status,
            bytes_written: res.bytes_written as f64,
            content_type: res.content_type,
            suggested_filename: res.suggested_filename,
            resumed: res.resumed,
        }
    }
}

#[napi(object)]
pub struct JsMemoryDownloadResult {
    pub result: JsDownloadResult,
    pub data: Buffer,
}

#[napi]
pub struct JsDownloader {
    session: Arc<Session>,
    chunk_size: usize,
    allow_resume: bool,
    max_bytes: Option<u64>,
}

#[napi]
impl JsDownloader {
    #[napi(constructor)]
    pub fn new(session: Option<&JsSession>) -> Self {
        let session = match session {
            Some(s) => s.inner.clone(),
            None => Arc::new(Session::new()),
        };
        Self {
            session,
            chunk_size: 65536,
            allow_resume: true,
            max_bytes: None,
        }
    }

    #[napi]
    pub fn chunk_size(&mut self, size: u32) {
        self.chunk_size = size as usize;
    }

    #[napi]
    pub fn allow_resume(&mut self, enabled: bool) {
        self.allow_resume = enabled;
    }

    #[napi]
    pub fn max_bytes(&mut self, n: u32) {
        self.max_bytes = Some(n as u64);
    }

    #[napi]
    pub async fn download(&self, url: String, dest: String) -> napi::Result<JsDownloadResult> {
        let downloader = crawlingo::engine::download::Downloader::new(self.session.clone())
            .with_chunk_size(self.chunk_size)
            .with_resume(self.allow_resume);
        let downloader = if let Some(max) = self.max_bytes {
            downloader.with_max_bytes(max)
        } else {
            downloader
        };

        let dest_path = std::path::PathBuf::from(dest);
        let res = downloader.download_to_file_async(&url, &dest_path).await
            .map_err(|e| to_napi_error(e, &url, "download"))?;
        Ok(JsDownloadResult::from(res))
    }

    #[napi]
    pub async fn download_to_memory(&self, url: String) -> napi::Result<JsMemoryDownloadResult> {
        let downloader = crawlingo::engine::download::Downloader::new(self.session.clone())
            .with_chunk_size(self.chunk_size)
            .with_resume(self.allow_resume);
        let downloader = if let Some(max) = self.max_bytes {
            downloader.with_max_bytes(max)
        } else {
            downloader
        };

        let (res, body) = downloader.download_to_memory_async(&url).await
            .map_err(|e| to_napi_error(e, &url, "download"))?;
        Ok(JsMemoryDownloadResult {
            result: JsDownloadResult::from(res),
            data: Buffer::from(body),
        })
    }
}

#[napi(object)]
pub struct JsSitemapEntry {
    pub loc: String,
    pub lastmod: Option<String>,
    pub changefreq: Option<String>,
    pub priority: Option<String>,
}

#[napi]
pub struct JsSitemap {
    sitemap_url: String,
    session: Arc<Session>,
    max_depth: usize,
    crawler_template: Crawler,
}

#[napi]
impl JsSitemap {
    #[napi(constructor)]
    pub fn new(sitemap_url: String, session: Option<&JsSession>) -> Self {
        let session = match session {
            Some(s) => s.inner.clone(),
            None => Arc::new(Session::new()),
        };
        let crawler_template = Crawler::new(&sitemap_url, session.clone());
        Self {
            sitemap_url,
            session,
            max_depth: 5,
            crawler_template,
        }
    }

    #[napi]
    pub fn max_depth(&mut self, depth: u32) {
        self.max_depth = depth as usize;
    }

    #[napi]
    pub fn follow(&mut self, selector: String) {
        self.crawler_template.follow_selector = selector;
    }

    #[napi]
    pub fn limit(&mut self, limit: u32) {
        self.crawler_template.limit = limit as usize;
    }

    #[napi]
    pub fn depth(&mut self, max_depth: u32) {
        self.crawler_template.max_depth = max_depth as usize;
    }

    #[napi]
    pub fn concurrency(&mut self, n: u32) {
        self.crawler_template.concurrency = n as usize;
    }

    #[napi]
    pub fn delay(&mut self, seconds: f64) {
        self.crawler_template.delay_seconds = seconds;
    }

    #[napi]
    pub fn field(&mut self, name: String, selector: String, selector_type: Option<String>, default_val: Option<String>) {
        let field = DatasetField {
            name,
            selector,
            selector_type: selector_type.unwrap_or("css".to_string()),
            default: default_val,
            extract_type: Default::default(),
        };
        self.crawler_template.fields.push(field);
    }

    #[napi]
    pub fn webhook(&mut self, url: String) {
        self.crawler_template.webhook_url = Some(url);
    }

    #[napi]
    pub async fn list_urls(&self) -> napi::Result<Vec<JsSitemapEntry>> {
        let sitemap_url = self.sitemap_url.clone();
        let session = self.session.clone();
        let max_depth = self.max_depth;

        let entries = tokio::spawn(async move {
            let crawler = crawlingo::crawl::sitemap::SitemapCrawler::new(&sitemap_url, session)
                .with_max_depth(max_depth);
            
            let mut results = Vec::new();
            let mut queue = vec![(sitemap_url.clone(), 0)];
            let mut visited_urls = std::collections::HashSet::new();

            while let Some((url, depth)) = queue.pop() {
                if depth > max_depth || visited_urls.contains(&url) {
                    continue;
                }
                visited_urls.insert(url.clone());

                let manager = crawler.session.fetch_manager();
                let req = FetchRequest {
                    url: url.clone(),
                    tier: FetcherTier::Standard,
                    browser_profile: None,
                    headers: crawler.session.headers.read().unwrap().clone(),
                    cookies: crawler.session.cookies.read().unwrap().clone(),
                    proxy: crawler.session.get_next_proxy(),
                    timeout: std::time::Duration::from_secs(*crawler.session.timeout_seconds.read().unwrap()),
                    retries: crawler.session.retries.read().unwrap().unwrap_or(2) as usize,
                };
                let resp = manager.dispatch(req).await
                    .map_err(|e| to_napi_error(e, &url, "sitemap"))?;
                let parsed = crawlingo::crawl::sitemap::parse_sitemap(&resp.body)
                    .map_err(|e| to_napi_error(e, &url, "sitemap"))?;
                match parsed {
                    crawlingo::crawl::sitemap::ParsedSitemap::Urlset(entries) => {
                        results.extend(entries.into_iter().map(|e| JsSitemapEntry {
                            loc: e.loc,
                            lastmod: e.lastmod,
                            changefreq: e.changefreq,
                            priority: e.priority,
                        }));
                    }
                    crawlingo::crawl::sitemap::ParsedSitemap::Index(entries) => {
                        for entry in entries {
                            queue.push((entry.loc, depth + 1));
                        }
                    }
                }
            }
            Ok::<_, napi::Error>(results)
        }).await.map_err(|e| napi::Error::from_reason(e.to_string()))?;

        entries
    }

    #[napi]
    pub async fn run(&self) -> napi::Result<Vec<JsDatasetResult>> {
        let sitemap_url = self.sitemap_url.clone();
        let session = self.session.clone();
        let max_depth = self.max_depth;
        let crawler_template = self.crawler_template.clone();

        let res = tokio::spawn(async move {
            let crawler = crawlingo::crawl::sitemap::SitemapCrawler::new(&sitemap_url, session)
                .with_max_depth(max_depth)
                .with_crawler_template(crawler_template);
            crawler.fetch_async().await
        }).await.map_err(|e| napi::Error::from_reason(e.to_string()))?
        .map_err(|e| to_napi_error(e, &self.sitemap_url, "sitemap"))?;

        let results = res.into_iter().map(|item| JsDatasetResult { inner: item }).collect();
        Ok(results)
    }
}

#[napi]
pub fn sitemap_url_for_origin(origin: String) -> String {
    crawlingo::crawl::sitemap::sitemap_url_for_origin(&origin)
}

#[napi]
pub struct JsPaginationConfig {
    pub(crate) inner: PaginationConfig,
}

#[napi]
impl JsPaginationConfig {
    #[napi(factory)]
    pub fn next_link(selector: String) -> Self {
        Self {
            inner: PaginationConfig::next_link(&selector),
        }
    }

    #[napi(factory)]
    pub fn page_number(url_template: String, start_page: u32, max_pages: u32) -> Self {
        Self {
            inner: PaginationConfig::page_number(&url_template, start_page as usize, max_pages as usize),
        }
    }

    #[napi(factory)]
    pub fn url_pattern(page_regex: String, max_page: u32) -> Self {
        Self {
            inner: PaginationConfig::url_pattern(&page_regex, max_page as usize),
        }
    }
}

#[napi]
pub enum JsFieldType {
    String,
    Integer,
    Float,
    Boolean,
}

impl From<JsFieldType> for FieldType {
    fn from(ft: JsFieldType) -> Self {
        match ft {
            JsFieldType::String => FieldType::String,
            JsFieldType::Integer => FieldType::Integer,
            JsFieldType::Float => FieldType::Float,
            JsFieldType::Boolean => FieldType::Boolean,
        }
    }
}

#[napi]
pub struct JsFieldConstraint {
    pub name: String,
    pub field_type: JsFieldType,
    pub required: bool,
}

#[napi]
impl JsFieldConstraint {
    #[napi(constructor)]
    pub fn new(name: String, field_type: JsFieldType, required: bool) -> Self {
        Self {
            name,
            field_type,
            required,
        }
    }
}

#[napi]
pub struct JsDatasetSchema {
    pub(crate) inner: DatasetSchema,
}

#[napi]
impl JsDatasetSchema {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: DatasetSchema::default(),
        }
    }

    #[napi]
    pub fn add_field(&mut self, name: String, field_type: JsFieldType, required: bool) {
        self.inner.fields.push(FieldConstraint {
            name,
            field_type: field_type.into(),
            required,
        });
    }

    #[napi]
    pub fn validate(&self, record: HashMap<String, String>) -> napi::Result<HashMap<String, String>> {
        self.inner.validate(&record).map_err(|e| to_napi_error(e, "", "schema"))
    }
}
