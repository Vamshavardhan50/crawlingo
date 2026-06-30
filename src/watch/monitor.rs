#[cfg(feature = "python")]
use crate::change::detector::{detect_changes, ChangeType};
#[cfg(feature = "python")]
use crate::dataset::builder::Dataset;
use crate::dataset::builder::DatasetField;
use crate::engine::session::Session;
#[cfg(feature = "python")]
use std::collections::HashMap;
use std::sync::Arc;
#[cfg(feature = "python")]
use std::time::Duration;
use tokio_util::sync::CancellationToken;

/// Polling monitor that checks for webpage data changes at set intervals.
pub struct Watch {
    pub url: String,
    pub fields: Vec<DatasetField>,
    pub interval_seconds: u64,
    pub session: Arc<Session>,
    pub cancellation_token: CancellationToken,
}

impl Watch {
    /// Creates a new `Watch` poller.
    pub fn new(url: &str, session: Arc<Session>) -> Self {
        Self {
            url: url.to_string(),
            fields: Vec::new(),
            interval_seconds: 60,
            session,
            cancellation_token: CancellationToken::new(),
        }
    }
}

// PyO3 FFI Python classes
#[cfg(feature = "python")]
use crate::engine::session::PySession;
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "Watch")]
pub struct PyWatch {
    pub inner: Watch,
    pub on_change_cb: Option<PyObject>,
    pub on_price_change_cb: Option<PyObject>,
    pub on_stock_change_cb: Option<PyObject>,
    pub on_added_cb: Option<PyObject>,
    pub on_removed_cb: Option<PyObject>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyWatch {
    #[new]
    pub fn new_py(url: &str, session: &PySession) -> Self {
        Self {
            inner: Watch::new(url, session.inner.clone()),
            on_change_cb: None,
            on_price_change_cb: None,
            on_stock_change_cb: None,
            on_added_cb: None,
            on_removed_cb: None,
        }
    }

    #[pyo3(signature = (name, selector, selector_type="css", default=None))]
    pub fn field(
        mut self_: PyRefMut<'_, Self>,
        name: &str,
        selector: &str,
        selector_type: &str,
        default: Option<String>,
    ) -> PyResult<Py<Self>> {
        let field = DatasetField {
            name: name.to_string(),
            selector: selector.to_string(),
            selector_type: selector_type.to_string(),
            transform: None,
            default,
        };
        self_.inner.fields.push(field);
        Ok(self_.into())
    }

    pub fn interval(mut self_: PyRefMut<'_, Self>, seconds: u64) -> PyResult<Py<Self>> {
        self_.inner.interval_seconds = seconds;
        Ok(self_.into())
    }

    pub fn on_change(mut self_: PyRefMut<'_, Self>, cb: PyObject) -> PyResult<Py<Self>> {
        self_.on_change_cb = Some(cb);
        Ok(self_.into())
    }

    pub fn on_price_change(mut self_: PyRefMut<'_, Self>, cb: PyObject) -> PyResult<Py<Self>> {
        self_.on_price_change_cb = Some(cb);
        Ok(self_.into())
    }

    pub fn on_stock_change(mut self_: PyRefMut<'_, Self>, cb: PyObject) -> PyResult<Py<Self>> {
        self_.on_stock_change_cb = Some(cb);
        Ok(self_.into())
    }

    pub fn on_element_added(mut self_: PyRefMut<'_, Self>, cb: PyObject) -> PyResult<Py<Self>> {
        self_.on_added_cb = Some(cb);
        Ok(self_.into())
    }

    pub fn on_element_removed(mut self_: PyRefMut<'_, Self>, cb: PyObject) -> PyResult<Py<Self>> {
        self_.on_removed_cb = Some(cb);
        Ok(self_.into())
    }

    /// Run the poll loop synchronously (blocking the Python thread)
    pub fn run(&self, py: Python<'_>) -> PyResult<()> {
        let on_change = self.on_change_cb.clone();
        let on_price = self.on_price_change_cb.clone();
        let on_stock = self.on_stock_change_cb.clone();
        let on_added = self.on_added_cb.clone();
        let on_removed = self.on_removed_cb.clone();

        let url = self.inner.url.clone();
        let session = self.inner.session.clone();
        let interval_sec = self.inner.interval_seconds;
        let token = self.inner.cancellation_token.clone();

        let mut fields = Vec::new();
        for f in &self.inner.fields {
            fields.push(crate::dataset::builder::DatasetField {
                name: f.name.clone(),
                selector: f.selector.clone(),
                selector_type: f.selector_type.clone(),
                transform: None,
                default: f.default.clone(),
            });
        }

        // Release the GIL during sleep and fetches, re-acquiring it only to run callbacks
        py.allow_threads(move || {
            crate::TOKIO_RUNTIME.block_on(async {
                let mut interval = tokio::time::interval(Duration::from_secs(interval_sec));
                let mut previous_data = HashMap::new();

                loop {
                    tokio::select! {
                        _ = token.cancelled() => {
                            break;
                        }
                        _ = interval.tick() => {
                            tracing::info!("Watch tick checking: {}", url);
                            let mut dataset = Dataset::new(&url, session.clone());
                            for f in &fields {
                                dataset.add_field(crate::dataset::builder::DatasetField {
                                    name: f.name.clone(),
                                    selector: f.selector.clone(),
                                    selector_type: f.selector_type.clone(),
                                    transform: None,
                                    default: f.default.clone(),
                                });
                            }

                            if let Ok(res) = dataset.build_async().await {
                                if !previous_data.is_empty() {
                                    let changes = detect_changes(&url, &previous_data, &res.fields);
                                    for change in changes {
                                        Python::with_gil(|py| {
                                            let py_evt = crate::change::detector::PyChangeEvent::from(change.clone());
                                            let py_evt_obj = py_evt.into_pyobject(py).unwrap();

                                            if let Some(ref cb) = on_change {
                                                let _ = cb.call1(py, (&py_evt_obj,));
                                            }

                                            match change.change_type {
                                                ChangeType::PriceChange { .. } => {
                                                    if let Some(ref cb) = on_price {
                                                        let _ = cb.call1(py, (&py_evt_obj,));
                                                    }
                                                }
                                                ChangeType::StockChange { .. } => {
                                                    if let Some(ref cb) = on_stock {
                                                        let _ = cb.call1(py, (&py_evt_obj,));
                                                    }
                                                }
                                                ChangeType::ElementAdded => {
                                                    if let Some(ref cb) = on_added {
                                                        let _ = cb.call1(py, (&py_evt_obj,));
                                                    }
                                                }
                                                ChangeType::ElementRemoved => {
                                                    if let Some(ref cb) = on_removed {
                                                        let _ = cb.call1(py, (&py_evt_obj,));
                                                    }
                                                }
                                                _ => {}
                                            }
                                        });
                                    }
                                }
                                previous_data = res.fields;
                            }
                        }
                    }
                }
            });
        });

        Ok(())
    }

    /// Stops the watcher loop.
    pub fn stop(&self) {
        self.inner.cancellation_token.cancel();
    }
}
