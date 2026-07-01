use crate::engine::fetcher::{FetchManager, FetchRequest};
use crate::engine::pool::ConnectionPoolConfig;
#[cfg(feature = "python")]
use crate::engine::session::PySession;
use crate::engine::session::Session;
use crate::error::Result;
use crate::matcher::auto_matcher;
use crate::parser::document::Page;
use crate::parser::streaming::HtmlParser;
use crate::selector::SelectorQuery;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;

/// Represents a field to extract.
#[derive(Debug, Clone)]
pub struct DatasetField {
    pub name: String,
    pub selector: String,
    pub selector_type: String, // "css", "xpath", "text", "regex", "after_text", "before_text"
    #[cfg(feature = "python")]
    pub transform: Option<pyo3::PyObject>,
    pub default: Option<String>,
}

/// A fluent builder for structured data extraction.
pub struct Dataset {
    pub url: String,
    pub fields: Vec<DatasetField>,
    pub session: Arc<Session>,
}

/// Holds the output fields and metadata of a dataset build.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DatasetResult {
    pub url: String,
    pub fields: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

impl Dataset {
    /// Creates a new `Dataset` builder.
    pub fn new(url: &str, session: Arc<Session>) -> Self {
        Self {
            url: url.to_string(),
            fields: Vec::new(),
            session,
        }
    }

    /// Adds a field rule to the dataset.
    pub fn add_field(&mut self, field: DatasetField) {
        self.fields.push(field);
    }

    /// Fetches and extracts all fields synchronously from the current thread.
    pub fn build(&self) -> Result<DatasetResult> {
        crate::TOKIO_RUNTIME.block_on(self.build_async())
    }

    /// Asynchronous core of the dataset build operation.
    pub async fn build_async(&self) -> Result<DatasetResult> {
        // 1. Gather config from Session
        let headers = self.session.headers.read().unwrap().clone();
        let cookies = self.session.cookies.read().unwrap().clone();
        let proxy = self.session.get_next_proxy();
        let rate_limit_rps = *self.session.rate_limit_rps.read().unwrap();
        let timeout_secs = *self.session.timeout_seconds.read().unwrap();
        let fetcher_tier = *self.session.fetcher_tier.read().unwrap();
        let browser_profile = self.session.browser_profile.read().unwrap().clone();

        // 2. Build Fetch Request options
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

        // 3. Fetch using Fetch Manager with no keepalive (test-safe)
        let rate_limiter = Arc::new(crate::engine::rate_limiter::HostRateLimiter::new());
        let manager = FetchManager::new(rate_limiter, ConnectionPoolConfig::no_keepalive());
        let response = manager.dispatch(req).await?;

        // 4. Parse using HtmlParser producing Page
        let page = HtmlParser::parse(response)?;

        // 5. Extract fields from Page
        self.extract_from_page(&page).await
    }

    /// Extracts fields directly from a pre-parsed Page object.
    pub async fn extract_from_page(&self, page: &Page) -> Result<DatasetResult> {
        let auto_match_enabled = *self.session.auto_match.read().unwrap();

        // Retrieve long-lived fingerprint store from Session
        let store = self.session.get_fingerprint_store()?;

        let mut fields_map = HashMap::new();

        for f in &self.fields {
            let mut extracted_val = None;

            let query = match f.selector_type.as_str() {
                "xpath" => SelectorQuery::XPath(&f.selector),
                "regex" => SelectorQuery::Regex(&f.selector),
                "text" => SelectorQuery::TextAnchor(&f.selector),
                "after_text" => SelectorQuery::AfterText(&f.selector),
                "before_text" => SelectorQuery::BeforeText(&f.selector),
                _ => SelectorQuery::Css(&f.selector),
            };

            // Resolve selector matches
            let mut matches = page.query(query).unwrap_or_default();

            // Auto-matching recovery logic
            if matches.is_empty() && auto_match_enabled && f.selector_type == "css" {
                let weights = self.session.similarity_weights.read().unwrap();
                let weights_opt = if weights.is_empty() {
                    None
                } else {
                    Some(&*weights)
                };
                if let Ok(recovered_idx) = auto_matcher::auto_match(
                    page.dom_tree(),
                    page.url(),
                    &f.selector,
                    &store,
                    weights_opt,
                ) {
                    matches = vec![recovered_idx];
                }
            }

            // Extract combined text
            if !matches.is_empty() {
                let combined_text = page.get_nodes_combined_text(&matches);
                let trimmed = combined_text.trim().to_string();
                if !trimmed.is_empty() {
                    extracted_val = Some(trimmed);
                }
            }

            // Fallback to default
            let final_val = extracted_val
                .or_else(|| f.default.clone())
                .unwrap_or_default();
            fields_map.insert(f.name.clone(), final_val);
        }

        Ok(DatasetResult {
            url: page.url().to_string(),
            fields: fields_map,
            timestamp: Utc::now(),
        })
    }

    /// Compiles a stream of Page objects into a stream of DatasetResult records.
    pub fn compile_stream(
        &self,
        mut page_receiver: tokio::sync::mpsc::Receiver<Page>,
    ) -> tokio::sync::mpsc::Receiver<Result<DatasetResult>> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let fields = self.fields.clone();
        let session = self.session.clone();

        tokio::spawn(async move {
            let temp_dataset = Dataset {
                url: String::new(),
                fields,
                session,
            };
            while let Some(page) = page_receiver.recv().await {
                let res = temp_dataset.extract_from_page(&page).await;
                if tx.send(res).await.is_err() {
                    break;
                }
            }
        });

        rx
    }
}

// PyO3 Bindings
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "Dataset")]
pub struct PyDataset {
    pub inner: Dataset,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDataset {
    #[new]
    pub fn new_py(url: &str, session: &PySession) -> Self {
        Self {
            inner: Dataset::new(url, session.inner.clone()),
        }
    }

    /// Add a field to be extracted (supports Python mapping callback)
    #[pyo3(signature = (name, selector, selector_type=None, transform=None, default=None))]
    pub fn field(
        mut self_: PyRefMut<'_, Self>,
        name: &str,
        selector: &str,
        selector_type: Option<&str>,
        transform: Option<PyObject>,
        default: Option<&str>,
    ) -> PyResult<Py<Self>> {
        let field = DatasetField {
            name: name.to_string(),
            selector: selector.to_string(),
            selector_type: selector_type.unwrap_or("css").to_string(),
            transform,
            default: default.map(|s| s.to_string()),
        };
        self_.inner.add_field(field);
        Ok(self_.into())
    }

    /// Sync build method
    pub fn build(self_: PyRef<'_, Self>) -> PyResult<PyDatasetResult> {
        let py = self_.py();
        let res = self_.inner.build()?;

        // Apply python transforms if present
        let mut final_fields = res.fields.clone();
        for field_def in &self_.inner.fields {
            if let Some(ref trans_fn) = field_def.transform {
                if let Some(val) = final_fields.get_mut(&field_def.name) {
                    let py_val = val.as_str().into_pyobject(py)?;
                    let py_res = trans_fn.call1(py, (py_val,))?;
                    let new_val: String = py_res.extract(py)?;
                    *val = new_val;
                }
            }
        }

        Ok(PyDatasetResult {
            inner: DatasetResult {
                url: res.url,
                fields: final_fields,
                timestamp: res.timestamp,
            },
        })
    }

    /// Async build method returning coroutine/future
    pub fn build_async(self_: PyRef<'_, Self>) -> PyResult<PyObject> {
        let py = self_.py();
        let result = self_.inner.build()?;
        Py::new(py, PyDatasetResult { inner: result }).map(|py_res| py_res.into_any())
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "DatasetResult")]
#[derive(Clone)]
pub struct PyDatasetResult {
    pub inner: DatasetResult,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDatasetResult {
    /// Export result to JSON
    pub fn to_json(&self, path: &str) -> PyResult<()> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, &self.inner.fields)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Export result to CSV
    pub fn to_csv(&self, path: &str) -> PyResult<()> {
        let mut writer = csv::Writer::from_path(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        // Header
        let keys: Vec<&str> = self.inner.fields.keys().map(|k| k.as_str()).collect();
        writer
            .write_record(&keys)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        // Row values
        let values: Vec<&str> = self.inner.fields.values().map(|v| v.as_str()).collect();
        writer
            .write_record(&values)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        writer.flush()?;
        Ok(())
    }

    /// Export result to Parquet
    pub fn to_parquet(&self, path: &str) -> PyResult<()> {
        crate::TOKIO_RUNTIME
            .block_on(crate::dataset::export::write_parquet(
                path,
                &self.inner.fields,
            ))
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Returns the fields dictionary to Python
    pub fn to_dict(&self) -> HashMap<String, String> {
        self.inner.fields.clone()
    }

    fn __getitem__(&self, key: &str) -> Option<String> {
        self.inner.fields.get(key).cloned()
    }

    fn __repr__(&self) -> String {
        format!(
            "DatasetResult(url='{}', fields={:?})",
            self.inner.url, self.inner.fields
        )
    }
}
