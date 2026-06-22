use crate::engine::fetcher::FetcherTier;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

/// The internal shared state of a scraping session.
pub struct Session {
    pub headers: RwLock<HashMap<String, String>>,
    pub cookies: RwLock<HashMap<String, String>>,
    pub proxy: RwLock<Option<String>>,
    pub rate_limit_rps: RwLock<f64>,
    pub auto_match: RwLock<bool>,
    pub timeout_seconds: RwLock<u64>,
    pub fingerprint_path: RwLock<String>,
    pub fetcher_tier: RwLock<FetcherTier>,
    pub browser_profile: RwLock<Option<String>>,
    pub similarity_weights: RwLock<HashMap<String, f64>>,
    pub proxy_pool: RwLock<Vec<String>>,
    pub proxy_index: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    pub proxy_provider_url: RwLock<Option<String>>,
}

impl Session {
    /// Creates a new, empty `Session` with default settings.
    pub fn new() -> Self {
        Self {
            headers: RwLock::new(HashMap::new()),
            cookies: RwLock::new(HashMap::new()),
            proxy: RwLock::new(None),
            rate_limit_rps: RwLock::new(0.0),
            auto_match: RwLock::new(false),
            timeout_seconds: RwLock::new(30),
            fingerprint_path: RwLock::new(".crawlingo".to_string()),
            fetcher_tier: RwLock::new(FetcherTier::Standard),
            browser_profile: RwLock::new(None),
            similarity_weights: RwLock::new(HashMap::new()),
            proxy_pool: RwLock::new(Vec::new()),
            proxy_index: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            proxy_provider_url: RwLock::new(None),
        }
    }

    /// Selects the next proxy from the pool, or falls back to the static proxy setting.
    pub fn get_next_proxy(&self) -> Option<String> {
        if let Some(ref p) = *self.proxy.read().unwrap() {
            return Some(p.clone());
        }
        let pool = self.proxy_pool.read().unwrap();
        if pool.is_empty() {
            return None;
        }
        let idx = self
            .proxy_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Some(pool[idx % pool.len()].clone())
    }

    /// Fetch proxy list from provider URL.
    pub fn fetch_provider_proxies(&self) -> Result<(), String> {
        let provider_url = self.proxy_provider_url.read().unwrap().clone();
        if let Some(url) = provider_url {
            let res = crate::TOKIO_RUNTIME.block_on(async {
                let req = crate::engine::fetcher::FetchRequest {
                    url: url.clone(),
                    tier: FetcherTier::Standard,
                    browser_profile: None,
                    headers: HashMap::new(),
                    cookies: HashMap::new(),
                    proxy: None,
                    timeout: std::time::Duration::from_secs(10),
                    retries: 1,
                    rate_limit_rps: 0.0,
                };
                let rate_limiter = Arc::new(crate::engine::rate_limiter::HostRateLimiter::new());
                let fetcher = crate::engine::fetcher::Fetcher::new(
                    rate_limiter,
                    crate::engine::pool::ConnectionPoolConfig::default(),
                );
                let resp = fetcher.fetch(req).await.map_err(|e| e.to_string())?;
                let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
                let text = String::from_utf8_lossy(&bytes).to_string();
                let proxies: Vec<String> = text
                    .lines()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                Ok::<Vec<String>, String>(proxies)
            });
            if let Ok(proxies) = res {
                let mut pool = self.proxy_pool.write().unwrap();
                *pool = proxies;
            }
        }
        Ok(())
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

/// PyO3 Python wrapper for `Session` permitting shared state context.
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "Session")]
#[derive(Clone)]
pub struct PySession {
    pub inner: Arc<Session>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySession {
    #[new]
    pub fn new_py() -> Self {
        Self {
            inner: Arc::new(Session::new()),
        }
    }

    /// Set headers (returns self to enable fluent chaining)
    pub fn headers(self_: PyRef<'_, Self>, headers: HashMap<String, String>) -> PyResult<Py<Self>> {
        {
            let mut h = self_
                .inner
                .headers
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *h = headers;
        }
        Ok(self_.into())
    }

    /// Set cookies (returns self)
    pub fn cookies(self_: PyRef<'_, Self>, cookies: HashMap<String, String>) -> PyResult<Py<Self>> {
        {
            let mut c = self_
                .inner
                .cookies
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *c = cookies;
        }
        Ok(self_.into())
    }

    /// Set proxy string (returns self)
    #[pyo3(signature = (proxy_url=None))]
    pub fn proxy(self_: PyRef<'_, Self>, proxy_url: Option<String>) -> PyResult<Py<Self>> {
        {
            let mut p = self_
                .inner
                .proxy
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *p = proxy_url;
        }
        Ok(self_.into())
    }

    /// Set rate limit per second (returns self)
    pub fn rate_limit(self_: PyRef<'_, Self>, requests_per_second: f64) -> PyResult<Py<Self>> {
        {
            let mut r = self_
                .inner
                .rate_limit_rps
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *r = requests_per_second;
        }
        Ok(self_.into())
    }

    /// Enable or disable auto matcher recovery (returns self)
    pub fn auto_match(self_: PyRef<'_, Self>, enabled: bool) -> PyResult<Py<Self>> {
        {
            let mut a = self_
                .inner
                .auto_match
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *a = enabled;
        }
        Ok(self_.into())
    }

    /// Set timeout seconds (returns self)
    pub fn timeout(self_: PyRef<'_, Self>, seconds: u64) -> PyResult<Py<Self>> {
        {
            let mut t = self_
                .inner
                .timeout_seconds
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *t = seconds;
        }
        Ok(self_.into())
    }

    /// Set fingerprint storage database directory path (returns self)
    pub fn fingerprint_path(self_: PyRef<'_, Self>, path: String) -> PyResult<Py<Self>> {
        {
            let mut f = self_
                .inner
                .fingerprint_path
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *f = path;
        }
        Ok(self_.into())
    }

    /// Set fetcher tier standard vs stealthy (returns self)
    pub fn fetcher_tier(self_: PyRef<'_, Self>, tier: String) -> PyResult<Py<Self>> {
        let tier_enum = if tier.to_lowercase() == "stealthy" {
            FetcherTier::Stealthy
        } else {
            FetcherTier::Standard
        };
        {
            let mut t = self_
                .inner
                .fetcher_tier
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *t = tier_enum;
        }
        Ok(self_.into())
    }

    /// Set browser profile: "chrome", "firefox", "safari" (returns self)
    #[pyo3(signature = (profile=None))]
    pub fn browser_profile(self_: PyRef<'_, Self>, profile: Option<String>) -> PyResult<Py<Self>> {
        {
            let mut b = self_
                .inner
                .browser_profile
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *b = profile;
        }
        Ok(self_.into())
    }

    /// Set auto-match similarity weights dictionary (returns self)
    pub fn auto_match_weights(
        self_: PyRef<'_, Self>,
        weights: HashMap<String, f64>,
    ) -> PyResult<Py<Self>> {
        {
            let mut w = self_
                .inner
                .similarity_weights
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *w = weights;
        }
        Ok(self_.into())
    }

    /// Set proxy pool list of string URLs (returns self)
    pub fn proxy_pool(self_: PyRef<'_, Self>, proxies: Vec<String>) -> PyResult<Py<Self>> {
        {
            let mut p = self_
                .inner
                .proxy_pool
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *p = proxies;
        }
        Ok(self_.into())
    }

    /// Set proxy provider API URL (returns self)
    #[pyo3(signature = (url=None))]
    pub fn proxy_provider(self_: PyRef<'_, Self>, url: Option<String>) -> PyResult<Py<Self>> {
        {
            let mut u = self_
                .inner
                .proxy_provider_url
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *u = url;
        }
        // Fetch initially
        let _ = self_.inner.fetch_provider_proxies();
        Ok(self_.into())
    }

    // Support Context Manager (with Session() as session:)
    fn __enter__(self_: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        Ok(self_)
    }

    fn __exit__(
        &self,
        _exc_type: &pyo3::Bound<'_, pyo3::types::PyAny>,
        _exc_value: &pyo3::Bound<'_, pyo3::types::PyAny>,
        _traceback: &pyo3::Bound<'_, pyo3::types::PyAny>,
    ) -> PyResult<()> {
        Ok(())
    }
}
