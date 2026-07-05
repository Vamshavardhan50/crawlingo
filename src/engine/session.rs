use crate::engine::fetcher::{FetchManager, FetcherTier, Transport};
use crate::engine::pool::ConnectionPoolConfig;
use crate::engine::rate_limiter::HostRateLimiter;
use crate::fingerprint::store::FingerprintStore;
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
    pub fingerprint_store: RwLock<Option<(String, Arc<FingerprintStore>)>>,
    /// A single, session-wide fetch manager shared by all `Dataset` and `Crawler` operations.
    ///
    /// Sharing one manager (and therefore one [`HostRateLimiter`]) is what makes rate limiting
    /// actually global per session, instead of being silently reset on every build. Lazily
    /// initialised by [`Session::fetch_manager`]; overridable in tests via
    /// [`Session::set_transport`].
    pub fetch_manager: RwLock<Option<Arc<FetchManager>>>,
}

impl Session {
    /// Builds a `Session` from a loaded [`crate::config::CrawlingoConfig`].
    ///
    /// Applies the config's values as the session's *initial* state; any `Session`/`PySession`
    /// setter called afterward simply overwrites the corresponding field as normal, so explicit
    /// configuration always wins over what was loaded from a file or environment variable.
    pub fn from_config(config: &crate::config::CrawlingoConfig) -> Self {
        let session = Self::new();
        *session.headers.write().unwrap() = config.headers.clone();
        *session.proxy.write().unwrap() = config.proxy.clone();
        *session.proxy_pool.write().unwrap() = config.proxy_pool.clone();
        *session.proxy_provider_url.write().unwrap() = config.proxy_provider_url.clone();
        *session.rate_limit_rps.write().unwrap() = config.rate_limit_rps;
        *session.auto_match.write().unwrap() = config.auto_match;
        if config.timeout_seconds > 0 {
            *session.timeout_seconds.write().unwrap() = config.timeout_seconds;
        }
        if let Some(ref path) = config.fingerprint_path {
            *session.fingerprint_path.write().unwrap() = path.clone();
        }
        if let Some(ref tier) = config.fetcher_tier {
            *session.fetcher_tier.write().unwrap() = if tier.eq_ignore_ascii_case("stealthy") {
                FetcherTier::Stealthy
            } else {
                FetcherTier::Standard
            };
        }
        *session.browser_profile.write().unwrap() = config.browser_profile.clone();

        // Pre-build the shared FetchManager using the config's pool/retry settings, rather than
        // leaving fetch_manager() to lazily build one from ConnectionPoolConfig::default() and
        // RetryPolicy::default() on first use.
        let manager = FetchManager::new(
            Arc::new(HostRateLimiter::new()),
            (&config.pool).into(),
        )
        .with_retry_policy((&config.retry).into());
        *session.fetch_manager.write().unwrap() = Some(Arc::new(manager));

        session
    }

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
            fingerprint_store: RwLock::new(None),
            fetch_manager: RwLock::new(None),
        }
    }

    /// Returns the session-wide [`FetchManager`], creating it on first use.
    ///
    /// All fetch traffic for a session flows through this single instance so that connection
    /// state and, critically, host rate limits are shared rather than reset per operation.
    pub fn fetch_manager(&self) -> Arc<FetchManager> {
        {
            let guard = self.fetch_manager.read().unwrap();
            if let Some(ref manager) = *guard {
                return manager.clone();
            }
        }

        let mut guard = self.fetch_manager.write().unwrap();
        // Re-check: another thread may have initialised it between the locks.
        if let Some(ref manager) = *guard {
            return manager.clone();
        }
        let manager = Arc::new(FetchManager::new(
            Arc::new(HostRateLimiter::new()),
            ConnectionPoolConfig::default(),
        ));
        *guard = Some(manager.clone());
        manager
    }

    /// Replaces the session's fetch transport — primarily to inject a mock in tests.
    ///
    /// The supplied transport is used for both the standard and stealth tiers, wrapped in a
    /// fresh [`FetchManager`] so retry and rate-limit logic still apply.
    pub fn set_transport(&self, transport: Arc<dyn Transport>) {
        let manager = Arc::new(FetchManager::with_transport(
            Arc::new(HostRateLimiter::new()),
            transport,
        ));
        *self.fetch_manager.write().unwrap() = Some(manager);
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
            let manager = self.fetch_manager();
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
                let resp = manager.dispatch(req).await.map_err(|e| e.to_string())?;
                let text = String::from_utf8_lossy(&resp.body).to_string();
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

    /// Retrieves the long-lived fingerprint store cached connection.
    pub fn get_fingerprint_store(
        &self,
    ) -> Result<Arc<FingerprintStore>, crate::error::CrawlingoError> {
        let path = self.fingerprint_path.read().unwrap().clone();

        {
            let store_opt = self.fingerprint_store.read().unwrap();
            if let Some((ref cached_path, ref store)) = *store_opt {
                if cached_path == &path {
                    return Ok(store.clone());
                }
            }
        }

        let mut store_opt = self.fingerprint_store.write().unwrap();
        if let Some((ref cached_path, ref store)) = *store_opt {
            if cached_path == &path {
                return Ok(store.clone());
            }
        }

        let new_store = Arc::new(FingerprintStore::open(std::path::Path::new(&path))?);
        *store_opt = Some((path, new_store.clone()));
        Ok(new_store)
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CrawlingoConfig;

    #[test]
    fn from_config_applies_loaded_values() {
        let config = CrawlingoConfig {
            proxy: Some("http://proxy.example.com:8080".to_string()),
            rate_limit_rps: 4.0,
            auto_match: true,
            timeout_seconds: 45,
            fetcher_tier: Some("stealthy".to_string()),
            browser_profile: Some("firefox".to_string()),
            ..Default::default()
        };

        let session = Session::from_config(&config);

        assert_eq!(
            session.proxy.read().unwrap().as_deref(),
            Some("http://proxy.example.com:8080")
        );
        assert_eq!(*session.rate_limit_rps.read().unwrap(), 4.0);
        assert!(*session.auto_match.read().unwrap());
        assert_eq!(*session.timeout_seconds.read().unwrap(), 45);
        assert_eq!(*session.fetcher_tier.read().unwrap(), FetcherTier::Stealthy);
        assert_eq!(
            session.browser_profile.read().unwrap().as_deref(),
            Some("firefox")
        );
    }

    #[test]
    fn from_config_zero_timeout_keeps_default() {
        // timeout_seconds: 0 is CrawlingoConfig's Default (unset), not a real "0 second timeout" —
        // Session::new()'s default of 30s should be preserved instead of being zeroed out.
        let config = CrawlingoConfig::default();
        let session = Session::from_config(&config);
        assert_eq!(*session.timeout_seconds.read().unwrap(), 30);
    }

    #[test]
    fn from_config_prebuilds_fetch_manager_reused_by_fetch_manager_accessor() {
        let config = CrawlingoConfig::default();
        let session = Session::from_config(&config);
        let first = session.fetch_manager();
        let second = session.fetch_manager();
        assert!(Arc::ptr_eq(&first, &second));
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

    /// Builds a `Session` from a config file (`.toml`/`.json`) layered with `CRAWLINGO_*`
    /// environment variable overrides. Pass `path=None` to skip the file layer and load from
    /// defaults + environment only.
    #[staticmethod]
    #[pyo3(signature = (path=None))]
    pub fn from_config(path: Option<&str>) -> PyResult<Self> {
        let config = crate::config::CrawlingoConfig::load(path.map(std::path::Path::new))
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(Session::from_config(&config)),
        })
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
