use once_cell::sync::Lazy;
#[cfg(feature = "python")]
use std::collections::HashMap;
#[cfg(feature = "python")]
use std::sync::Arc;
use tokio::runtime::Runtime;

pub mod change;
pub mod config;
pub mod crawl;
pub mod dataset;
pub mod engine;
pub mod error;
pub mod extraction;
pub mod fingerprint;
pub mod matcher;
pub mod metrics;
pub mod parser;
pub mod selector;
pub mod watch;

/// Convenience re-export of the core Page type.
pub use parser::document::Page;

#[cfg(feature = "python")]
use crate::engine::fetcher::{FetchManager, FetchRequest, FetcherTier};
#[cfg(feature = "python")]
use crate::engine::pool::ConnectionPoolConfig;
#[cfg(feature = "python")]
use crate::error::CrawlingoError;
#[cfg(feature = "python")]
use crate::parser::document::DomTree;
#[cfg(feature = "python")]
use crate::parser::document::PyElementCollection;
#[cfg(feature = "python")]
use crate::parser::streaming::HtmlParser;
#[cfg(feature = "python")]
use crate::selector::{css, regex_selector, text_anchor, xpath};

/// Shared static Tokio runtime used to run async futures synchronously for the Python GIL thread.
pub static TOKIO_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Failed to initialize static Tokio runtime"));

// PyO3 bindings
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "Page")]
pub struct PyPage {
    #[pyo3(get)]
    pub url: String,
    pub tree: Arc<DomTree>,
    #[pyo3(get)]
    pub status: u16,
    pub html: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyPage {
    #[new]
    #[pyo3(signature = (url, auto_match=false, timeout=30, retries=3, headers=None, cookies=None, proxy=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        py: Python<'_>,
        url: &str,
        auto_match: bool,
        timeout: u64,
        retries: usize,
        headers: Option<HashMap<String, String>>,
        cookies: Option<HashMap<String, String>>,
        proxy: Option<String>,
    ) -> PyResult<Self> {
        let url_str = url.to_string();
        let headers_val = headers.unwrap_or_default();
        let cookies_val = cookies.unwrap_or_default();

        let result: Result<crate::parser::document::Page, CrawlingoError> =
            py.allow_threads(move || {
                TOKIO_RUNTIME.block_on(async {
                    let req = FetchRequest {
                        url: url_str.clone(),
                        tier: if auto_match {
                            FetcherTier::Stealthy
                        } else {
                            FetcherTier::Standard
                        },
                        browser_profile: None,
                        headers: headers_val,
                        cookies: cookies_val,
                        proxy,
                        timeout: std::time::Duration::from_secs(timeout),
                        retries,
                        rate_limit_rps: 0.0,
                    };
                    let rate_limiter =
                        Arc::new(crate::engine::rate_limiter::HostRateLimiter::new());
                    let manager = FetchManager::new(rate_limiter, ConnectionPoolConfig::default());
                    let resp = manager.dispatch(req).await?;
                    let page = HtmlParser::parse(resp)?;
                    Ok(page)
                })
            });

        let page = result?;

        Ok(Self {
            url: url.to_string(),
            tree: page.dom_tree().clone(),
            status: page.status(),
            html: page.html().to_string(),
        })
    }

    /// Query element using CSS selector.
    pub fn css(&self, selector: &str) -> PyElementCollection {
        let indices = css::query(&self.tree, selector);
        PyElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    /// Query element using XPath.
    pub fn xpath(&self, query: &str) -> PyElementCollection {
        let indices = xpath::query(&self.tree, query);
        PyElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    /// Query element using fuzzy/exact text content.
    pub fn find_text(&self, text: &str) -> PyElementCollection {
        let indices = text_anchor::find(&self.tree, text);
        PyElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    /// Query element following the anchor text.
    pub fn after_text(&self, text: &str) -> PyElementCollection {
        let indices = text_anchor::after(&self.tree, text);
        PyElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    /// Query element preceding the anchor text.
    pub fn before_text(&self, text: &str) -> PyElementCollection {
        let indices = text_anchor::before(&self.tree, text);
        PyElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        }
    }

    /// Query element using Regex pattern.
    pub fn regex(&self, pattern: &str) -> PyResult<PyElementCollection> {
        let indices = regex_selector::query(&self.tree, pattern)?;
        Ok(PyElementCollection {
            tree: self.tree.clone(),
            node_indices: indices,
        })
    }

    /// Retrieves the page document HTML contents.
    pub fn html(&self) -> String {
        self.html.clone()
    }

    /// Converts the page's DOM to clean Markdown text.
    pub fn markdown(&self) -> String {
        crate::parser::document::Page::render_markdown(&self.tree)
    }

    /// Extract page title tag text.
    pub fn title(&self) -> String {
        let matched = css::query(&self.tree, "title");
        if !matched.is_empty() {
            self.tree.get_text(matched[0])
        } else {
            String::new()
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "DownloadResult")]
#[derive(Clone)]
pub struct PyDownloadResult {
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub status: u16,
    #[pyo3(get)]
    pub bytes_written: u64,
    #[pyo3(get)]
    pub content_type: String,
    #[pyo3(get)]
    pub suggested_filename: Option<String>,
    #[pyo3(get)]
    pub resumed: bool,
}

#[cfg(feature = "python")]
#[pyclass(name = "Downloader")]
pub struct PyDownloader {
    session: Arc<crate::engine::session::Session>,
    chunk_size: usize,
    allow_resume: bool,
    max_bytes: Option<u64>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDownloader {
    #[new]
    #[pyo3(signature = (session=None))]
    pub fn new(session: Option<&crate::engine::session::PySession>) -> Self {
        let session = match session {
            Some(s) => s.inner.clone(),
            None => Arc::new(crate::engine::session::Session::new()),
        };
        Self {
            session,
            chunk_size: 65536,
            allow_resume: true,
            max_bytes: None,
        }
    }

    pub fn chunk_size(mut self_: PyRefMut<'_, Self>, size: usize) -> PyResult<Py<Self>> {
        self_.chunk_size = size;
        Ok(self_.into())
    }

    pub fn allow_resume(mut self_: PyRefMut<'_, Self>, enabled: bool) -> PyResult<Py<Self>> {
        self_.allow_resume = enabled;
        Ok(self_.into())
    }

    pub fn max_bytes(mut self_: PyRefMut<'_, Self>, n: u64) -> PyResult<Py<Self>> {
        self_.max_bytes = Some(n);
        Ok(self_.into())
    }

    pub fn download(
        self_: PyRef<'_, Self>,
        py: Python<'_>,
        url: &str,
        dest: &str,
    ) -> PyResult<PyDownloadResult> {
        let downloader = crate::engine::download::Downloader::new(self_.session.clone())
            .with_chunk_size(self_.chunk_size)
            .with_resume(self_.allow_resume);
        let downloader = if let Some(max) = self_.max_bytes {
            downloader.with_max_bytes(max)
        } else {
            downloader
        };

        let dest_path = std::path::Path::new(dest);
        let url_str = url.to_string();

        let res = py.allow_threads(move || downloader.download_to_file(&url_str, dest_path))?;
        Ok(PyDownloadResult {
            url: res.url,
            status: res.status,
            bytes_written: res.bytes_written,
            content_type: res.content_type,
            suggested_filename: res.suggested_filename,
            resumed: res.resumed,
        })
    }

    pub fn download_to_memory(
        self_: PyRef<'_, Self>,
        py: Python<'_>,
        url: &str,
    ) -> PyResult<(PyDownloadResult, Vec<u8>)> {
        let downloader = crate::engine::download::Downloader::new(self_.session.clone())
            .with_chunk_size(self_.chunk_size)
            .with_resume(self_.allow_resume);
        let downloader = if let Some(max) = self_.max_bytes {
            downloader.with_max_bytes(max)
        } else {
            downloader
        };

        let url_str = url.to_string();
        let (res, body) = py.allow_threads(move || downloader.download_to_memory(&url_str))?;
        Ok((
            PyDownloadResult {
                url: res.url,
                status: res.status,
                bytes_written: res.bytes_written,
                content_type: res.content_type,
                suggested_filename: res.suggested_filename,
                resumed: res.resumed,
            },
            body,
        ))
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "SitemapEntry")]
#[derive(Clone)]
pub struct PySitemapEntry {
    #[pyo3(get)]
    pub loc: String,
    #[pyo3(get)]
    pub lastmod: Option<String>,
    #[pyo3(get)]
    pub changefreq: Option<String>,
    #[pyo3(get)]
    pub priority: Option<String>,
}

#[cfg(feature = "python")]
#[pyclass(name = "Sitemap")]
pub struct PySitemap {
    sitemap_url: String,
    session: Arc<crate::engine::session::Session>,
    max_depth: usize,
    crawler_template: crate::crawl::crawler::Crawler,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySitemap {
    #[new]
    #[pyo3(signature = (sitemap_url, session=None))]
    pub fn new(sitemap_url: &str, session: Option<&crate::engine::session::PySession>) -> Self {
        let session = match session {
            Some(s) => s.inner.clone(),
            None => Arc::new(crate::engine::session::Session::new()),
        };
        let crawler_template = crate::crawl::crawler::Crawler::new(sitemap_url, session.clone());
        Self {
            sitemap_url: sitemap_url.to_string(),
            session,
            max_depth: 5,
            crawler_template,
        }
    }

    pub fn max_depth(mut self_: PyRefMut<'_, Self>, depth: usize) -> PyResult<Py<Self>> {
        self_.max_depth = depth;
        Ok(self_.into())
    }

    pub fn follow(mut self_: PyRefMut<'_, Self>, selector: &str) -> PyResult<Py<Self>> {
        self_.crawler_template.follow_selector = selector.to_string();
        Ok(self_.into())
    }

    pub fn limit(mut self_: PyRefMut<'_, Self>, limit: usize) -> PyResult<Py<Self>> {
        self_.crawler_template.limit = limit;
        Ok(self_.into())
    }

    pub fn depth(mut self_: PyRefMut<'_, Self>, max_depth: usize) -> PyResult<Py<Self>> {
        self_.crawler_template.max_depth = max_depth;
        Ok(self_.into())
    }

    pub fn concurrency(mut self_: PyRefMut<'_, Self>, n: usize) -> PyResult<Py<Self>> {
        self_.crawler_template.concurrency = n;
        Ok(self_.into())
    }

    pub fn delay(mut self_: PyRefMut<'_, Self>, seconds: f64) -> PyResult<Py<Self>> {
        self_.crawler_template.delay_seconds = seconds;
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
        let field = crate::dataset::builder::DatasetField {
            name: name.to_string(),
            selector: selector.to_string(),
            selector_type: selector_type.unwrap_or("css").to_string(),
            transform: None,
            default: default.map(|s| s.to_string()),
            extract_type: Default::default(),
        };
        self_.crawler_template.fields.push(field);
        Ok(self_.into())
    }

    pub fn webhook(mut self_: PyRefMut<'_, Self>, url: &str) -> PyResult<Py<Self>> {
        self_.crawler_template.webhook_url = Some(url.to_string());
        Ok(self_.into())
    }

    pub fn list_urls(self_: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Vec<PySitemapEntry>> {
        let sitemap_url = self_.sitemap_url.clone();
        let session = self_.session.clone();
        let max_depth = self_.max_depth;

        let entries = py.allow_threads(move || {
            let crawler = crate::crawl::sitemap::SitemapCrawler::new(&sitemap_url, session)
                .with_max_depth(max_depth);
            
            crate::TOKIO_RUNTIME.block_on(async {
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
                        retries: 2,
                        rate_limit_rps: 0.0,
                    };
                    if let Ok(resp) = manager.dispatch(req).await {
                        if let Ok(parsed) = crate::crawl::sitemap::parse_sitemap(&resp.body) {
                            match parsed {
                                crate::crawl::sitemap::ParsedSitemap::Urlset(entries) => {
                                    results.extend(entries.into_iter().map(|e| PySitemapEntry {
                                        loc: e.loc,
                                        lastmod: e.lastmod,
                                        changefreq: e.changefreq,
                                        priority: e.priority,
                                    }));
                                }
                                crate::crawl::sitemap::ParsedSitemap::Index(entries) => {
                                    for entry in entries {
                                        queue.push((entry.loc, depth + 1));
                                    }
                                }
                            }
                        }
                    }
                }
                Ok::<Vec<PySitemapEntry>, CrawlingoError>(results)
            })
        })?;

        Ok(entries)
    }

    pub fn build(self_: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Vec<crate::dataset::builder::PyDatasetResult>> {
        let sitemap_url = self_.sitemap_url.clone();
        let session = self_.session.clone();
        let max_depth = self_.max_depth;
        let crawler_template = self_.crawler_template.clone();

        let res = py.allow_threads(move || {
            let crawler = crate::crawl::sitemap::SitemapCrawler::new(&sitemap_url, session)
                .with_max_depth(max_depth)
                .with_crawler_template(crawler_template);
            crawler.fetch()
        })?;

        let py_res = res
            .into_iter()
            .map(|r| crate::dataset::builder::PyDatasetResult { inner: r })
            .collect();
        Ok(py_res)
    }
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn sitemap_url_for_origin(origin: &str) -> String {
    crate::crawl::sitemap::sitemap_url_for_origin(origin)
}

/// The core FFI PyO3 binary module.
#[cfg(feature = "python")]
#[pymodule]
fn _crawlingo_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPage>()?;
    m.add_class::<crate::parser::document::PyElement>()?;
    m.add_class::<crate::parser::document::PyElementCollection>()?;
    m.add_class::<crate::dataset::builder::PyDataset>()?;
    m.add_class::<crate::dataset::builder::PyDatasetResult>()?;
    m.add_class::<crate::crawl::crawler::PyCrawl>()?;
    m.add_class::<crate::watch::monitor::PyWatch>()?;
    m.add_class::<crate::engine::session::PySession>()?;
    m.add_class::<crate::change::detector::PyChangeEvent>()?;
    m.add_class::<PyDownloader>()?;
    m.add_class::<PyDownloadResult>()?;
    m.add_class::<PySitemap>()?;
    m.add_class::<PySitemapEntry>()?;
    m.add_function(wrap_pyfunction!(sitemap_url_for_origin, m)?)?;
    Ok(())
}
