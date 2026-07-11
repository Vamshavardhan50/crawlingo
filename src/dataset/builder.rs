use crate::dataset::schema::DatasetSchema;
use crate::engine::fetcher::FetchRequest;
#[cfg(feature = "python")]
use crate::engine::session::PySession;
use crate::engine::session::Session;
use crate::error::Result;
use crate::extraction::{ExtractionEngine, ExtractionType};
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
    /// How the raw matched text is cleaned and typed (plain text, price, date, absolute URL, ...).
    /// Defaults to [`ExtractionType::Text`], which simply trims the combined text — identical to
    /// this field's behavior before typed extraction existed.
    pub extract_type: ExtractionType,
}

/// A fluent builder for structured data extraction.
#[derive(Clone)]
pub struct Dataset {
    pub url: String,
    pub fields: Vec<DatasetField>,
    pub session: Arc<Session>,
    /// Optional constraints (required fields, type coercion) checked against the extracted field
    /// map before a build is considered successful. See [`DatasetSchema`].
    pub schema: Option<DatasetSchema>,
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
            schema: None,
        }
    }

    /// Adds a field rule to the dataset.
    pub fn add_field(&mut self, field: DatasetField) {
        self.fields.push(field);
    }

    /// Attaches a [`DatasetSchema`] that extracted fields must satisfy. When set, `build`/
    /// `build_async` return a [`crate::error::CrawlingoError::DatasetError`] if a required field
    /// is missing/empty or a value fails to parse as its declared [`crate::dataset::schema::FieldType`],
    /// instead of silently returning the raw (or default) string.
    pub fn with_schema(mut self, schema: DatasetSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Fetches and extracts all fields synchronously from the current thread.
    pub fn build(&self) -> Result<DatasetResult> {
        crate::TOKIO_RUNTIME.block_on(self.build_async())
    }

    /// Core logic: zip selector matches from an already-parsed DomTree into structured records.
    pub fn extract_from_tree(&self, tree: &crate::parser::document::DomTree) -> Vec<HashMap<String, String>> {
        use crate::selector::{css, xpath, text_anchor, regex_selector};

        // Collect all match index lists per field
        let collections: Vec<Vec<usize>> = self.fields.iter().map(|f| {
            match f.selector_type.as_str() {
                "xpath"       => xpath::query(tree, &f.selector),
                "text"        => text_anchor::find(tree, &f.selector),
                "after_text"  => text_anchor::after(tree, &f.selector),
                "before_text" => text_anchor::before(tree, &f.selector),
                "regex"       => regex_selector::query(tree, &f.selector).unwrap_or_default(),
                _             => css::query(tree, &f.selector),
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


    pub async fn build_structured(&self) -> Result<Vec<HashMap<String, String>>> {
        use crate::engine::fetcher::{FetchRequest, FetchManager};
        use crate::engine::pool::ConnectionPoolConfig;
        use crate::parser::streaming::HtmlParser;

        let headers = self.session.headers.read().unwrap().clone();
        let cookies = self.session.cookies.read().unwrap().clone();
        let proxy = self.session.get_next_proxy();
        let rate_limit_rps = *self.session.rate_limit_rps.read().unwrap();
        let timeout_secs = *self.session.timeout_seconds.read().unwrap();
        let fetcher_tier = *self.session.fetcher_tier.read().unwrap();
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

        let rate_limiter = std::sync::Arc::new(crate::engine::rate_limiter::HostRateLimiter::new());
        let manager = FetchManager::new(rate_limiter, ConnectionPoolConfig::default());
        let resp = manager.dispatch(req).await?;
        let page = HtmlParser::parse(resp)?;

        Ok(self.extract_from_tree(page.dom_tree()))
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

        // 3. Fetch using the session-wide manager so connection state and host rate limits
        //    are shared across every build rather than reset on each call.
        let manager = self.session.fetch_manager();
        let response = manager.dispatch(req).await?;

        // 4. Parse using HtmlParser producing Page
        let page = HtmlParser::parse(response)?;

        // 5. Extract fields from Page
        self.extract_from_page(&page).await
    }

    /// Extracts fields directly from a pre-parsed Page object.
    pub async fn extract_from_page(&self, page: &Page) -> Result<DatasetResult> {
        let auto_match_enabled = *self.session.auto_match.read().unwrap();

        // The fingerprint store is only needed for auto-match recovery. Open it lazily so that
        // the common path (auto_match disabled) never touches Sled — this avoids both the I/O
        // cost and lockfile contention when many datasets extract concurrently.
        let mut store = None;

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
                // Open (and cache) the fingerprint store on first actual use.
                if store.is_none() {
                    store = Some(self.session.get_fingerprint_store()?);
                }
                let store_ref = store.as_ref().unwrap();
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
                    store_ref,
                    weights_opt,
                ) {
                    matches = vec![recovered_idx];
                }
            }

            // Extract combined text, cleaned and typed per the field's `extract_type` (plain
            // text, price, date, or absolute URL normalization).
            if !matches.is_empty() {
                let combined_text = page.get_nodes_combined_text(&matches);
                let normalized =
                    ExtractionEngine::normalize_value(&combined_text, &f.extract_type, page.url());
                if !normalized.is_empty() {
                    extracted_val = Some(normalized);
                }
            }

            // Fallback to default
            let final_val = extracted_val
                .or_else(|| f.default.clone())
                .unwrap_or_default();
            fields_map.insert(f.name.clone(), final_val);
        }

        // Apply schema validation/type-coercion, if attached. Only overlays the fields the
        // schema declares constraints for; fields outside the schema are left untouched. Returns
        // an error (aborting the build) if a required field is missing or fails to parse.
        if let Some(ref schema) = self.schema {
            let validated = schema.validate(&fields_map)?;
            fields_map.extend(validated);
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
        let schema = self.schema.clone();

        tokio::spawn(async move {
            let temp_dataset = Dataset {
                url: String::new(),
                fields,
                session,
                schema,
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

    /// Builds this dataset's configured fields against many URLs concurrently, streaming each
    /// completed record onto a [`DatasetStream`] as it finishes rather than collecting everything
    /// into memory first — useful for large URL lists where results should be written out (see
    /// [`DatasetStream::write_csv`]/[`DatasetStream::write_parquet`]) incrementally.
    ///
    /// Each pushed record's field map additionally carries a `"url"` entry (unless a field named
    /// `"url"` is already configured) so the source page remains identifiable once flattened into
    /// a plain `HashMap`.
    pub fn build_many_streamed(
        &self,
        urls: Vec<String>,
        concurrency: usize,
    ) -> crate::dataset::stream::DatasetStream {
        let mut stream = crate::dataset::stream::DatasetStream::new();
        let handle = stream.handle();
        // The spawned producer below owns the only remaining sender (via `handle` and its
        // per-request clones); detach the stream's own copy so the caller's `recv()` loop can
        // observe the channel closing once every in-flight fetch has pushed its result.
        stream.detach_sender();
        let fields = self.fields.clone();
        let session = self.session.clone();
        let schema = self.schema.clone();
        let concurrency = concurrency.max(1);

        tokio::spawn(async move {
            use futures::stream::{self, StreamExt};

            stream::iter(urls)
                .for_each_concurrent(concurrency, move |url| {
                    let fields = fields.clone();
                    let session = session.clone();
                    let schema = schema.clone();
                    let handle = handle.clone();
                    async move {
                        let dataset = Dataset {
                            url,
                            fields,
                            session,
                            schema,
                        };
                        let result = dataset.build_async().await.map(|r| {
                            let mut fields = r.fields;
                            fields.entry("url".to_string()).or_insert(r.url);
                            fields
                        });
                        handle.push(result);
                    }
                })
                .await;
        });

        stream
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

    /// Add a field to be extracted (supports Python mapping callback).
    ///
    /// `extract_type` selects built-in cleaning/typing applied to the raw matched text before
    /// `transform` (if any) runs: `"text"` (default), `"price"`, `"datetime"`, `"url"`, or
    /// `"attr:<name>"` for an HTML attribute value.
    #[pyo3(signature = (name, selector, selector_type=None, transform=None, default=None, extract_type=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn field(
        mut self_: PyRefMut<'_, Self>,
        name: &str,
        selector: &str,
        selector_type: Option<&str>,
        transform: Option<PyObject>,
        default: Option<&str>,
        extract_type: Option<&str>,
    ) -> PyResult<Py<Self>> {
        let field = DatasetField {
            name: name.to_string(),
            selector: selector.to_string(),
            selector_type: selector_type.unwrap_or("css").to_string(),
            transform,
            default: default.map(|s| s.to_string()),
            extract_type: extract_type
                .map(ExtractionType::from_str_or_text)
                .unwrap_or_default(),
        };
        self_.inner.add_field(field);
        Ok(self_.into())
    }

    /// Sync build method
    pub fn build(self_: PyRef<'_, Self>) -> PyResult<PyDatasetResult> {
        let py = self_.py();
        let inner = self_.inner.clone();
        let res = py.allow_threads(move || inner.build())?;

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
        let inner = self_.inner.clone();
        let result = py.allow_threads(move || inner.build())?;
        Py::new(py, PyDatasetResult { inner: result }).map(|py_res| py_res.into_any())
    }

    #[pyo3(signature = (page))]
    pub fn extract_structured(&self, page: &crate::PyPage) -> Vec<HashMap<String, String>> {
        self.inner.extract_from_tree(&page.tree)
    }

    pub fn build_structured(&self, py: Python<'_>) -> PyResult<Vec<HashMap<String, String>>> {
        let inner = self.inner.clone();
        let res = py.allow_threads(move || {
            crate::TOKIO_RUNTIME.block_on(async {
                inner.build_structured().await
            })
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(res)
    }


    #[staticmethod]
    pub fn save_json(records: Vec<HashMap<String, String>>, path: &str) -> PyResult<()> {
        let json = serde_json::to_string_pretty(&records)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    #[staticmethod]
    pub fn save_csv(records: Vec<HashMap<String, String>>, path: &str) -> PyResult<()> {
        let mut writer = csv::Writer::from_path(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        if let Some(first) = records.first() {
            let keys: Vec<&str> = first.keys().map(|k| k.as_str()).collect();
            writer.write_record(&keys)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
            for r in &records {
                let values: Vec<&str> = keys.iter().map(|k| r.get(*k).map(|s| s.as_str()).unwrap_or("")).collect();
                writer.write_record(&values)
                    .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
            }
        }
        writer.flush()?;
        Ok(())
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
