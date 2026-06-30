use once_cell::sync::Lazy;
#[cfg(feature = "python")]
use std::collections::HashMap;
#[cfg(feature = "python")]
use std::sync::Arc;
use tokio::runtime::Runtime;

pub mod change;
pub mod crawl;
pub mod dataset;
pub mod engine;
pub mod error;
pub mod fingerprint;
pub mod matcher;
pub mod parser;
pub mod selector;
pub mod watch;

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
    Ok(())
}
