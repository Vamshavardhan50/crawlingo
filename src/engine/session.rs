use crate::engine::fetcher::{FetchManager, FetcherTier, Transport};
use crate::engine::middleware::{Layer, MiddlewareStack};
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
    /// Cross-cutting layers (caching, auth, metrics, ...) wrapped around every fetch. Must be
    /// populated via [`Session::add_middleware`] *before* [`Session::fetch_manager`] is first
    /// called (directly or via a `Dataset`/`Crawler` operation) — like `pool_config`/
    /// `retry_policy`, the manager is built once and cached.
    pub middleware: RwLock<MiddlewareStack>,
    /// Aggregated fetch metrics for this session (requests, successes/failures, per-host and
    /// per-status counts, average latency). Always present and updated automatically — a
    /// [`crate::metrics::MetricsLayer`] wrapping this same `Metrics` is seeded as the outermost
    /// middleware layer in [`Session::new`], so no opt-in is required. Read via
    /// [`Session::metrics`].
    pub metrics: Arc<crate::metrics::Metrics>,
    pub retries: RwLock<Option<u32>>,
    pub retry_backoff: RwLock<Option<f64>>,
    pub retry_delay: RwLock<Option<u32>>,
    pub proxy_username: RwLock<Option<String>>,
    pub proxy_password: RwLock<Option<String>>,
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
        let manager = FetchManager::new(Arc::new(HostRateLimiter::new()), (&config.pool).into())
            .with_retry_policy((&config.retry).into())
            .with_middleware(&session.middleware.read().unwrap());
        *session.fetch_manager.write().unwrap() = Some(Arc::new(manager));

        session
    }

    /// Creates a new, empty `Session` with default settings.
    pub fn new() -> Self {
        let metrics = Arc::new(crate::metrics::Metrics::new());
        let mut middleware = MiddlewareStack::new();
        middleware.push(Arc::new(crate::metrics::MetricsLayer::new(metrics.clone())));

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
            middleware: RwLock::new(middleware),
            metrics,
            retries: RwLock::new(None),
            retry_backoff: RwLock::new(None),
            retry_delay: RwLock::new(None),
            proxy_username: RwLock::new(None),
            proxy_password: RwLock::new(None),
        }
    }

    /// Adds a middleware layer around all fetches for this session (see
    /// [`crate::engine::middleware`]). Must be called before the session's `FetchManager` is
    /// first built — see the field doc on [`Session::middleware`].
    ///
    /// Layers added here run *inside* the always-on [`crate::metrics::MetricsLayer`] seeded by
    /// [`Session::new`] (which is outermost), so their effect on latency/outcome is still
    /// reflected in [`Session::metrics`].
    pub fn add_middleware(&self, layer: Arc<dyn Layer>) {
        self.middleware.write().unwrap().push(layer);
    }

    /// Returns a snapshot of this session's aggregated fetch metrics.
    pub fn metrics(&self) -> crate::metrics::MetricsSnapshot {
        self.metrics.snapshot()
    }

    /// Enables in-memory HTTP response caching for this session, honoring `Cache-Control`
    /// (`no-store`/`no-cache`/`max-age`) and `ETag`/`Last-Modified` conditional revalidation (see
    /// [`crate::engine::cache`]). Opt-in, unlike metrics — caching trades freshness for fewer
    /// requests, which isn't always what a scraper wants. Must be called before the session's
    /// first fetch, like [`Session::add_middleware`].
    ///
    /// `max_entries` bounds the number of distinct cached URLs; `default_ttl` is the freshness
    /// window applied to a cacheable response with no explicit `Cache-Control: max-age`.
    pub fn enable_response_cache(&self, max_entries: u64, default_ttl: std::time::Duration) {
        let cache = Arc::new(crate::engine::cache::InMemoryCache::new(
            max_entries,
            std::time::Duration::from_secs(3600).max(default_ttl),
        ));
        self.add_middleware(Arc::new(crate::engine::cache::CachingLayer::new(
            cache,
            default_ttl,
        )));
    }

    /// Authenticates every fetch made through this session per the given [`crate::engine::auth::AuthScheme`]
    /// (see [`crate::engine::auth`]). Must be called before the session's first fetch, like
    /// [`Session::add_middleware`].
    pub fn set_auth(&self, scheme: crate::engine::auth::AuthScheme) {
        self.add_middleware(Arc::new(crate::engine::auth::AuthLayer::new(scheme)));
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
        let manager = Arc::new(
            FetchManager::new(Arc::new(HostRateLimiter::new()), ConnectionPoolConfig::default())
                .with_middleware(&self.middleware.read().unwrap()),
        );
        *guard = Some(manager.clone());
        manager
    }

    /// Replaces the session's fetch transport — primarily to inject a mock in tests.
    ///
    /// The supplied transport is used for both the standard and stealth tiers, wrapped in a
    /// fresh [`FetchManager`] so retry, rate-limit, and middleware logic still apply.
    pub fn set_transport(&self, transport: Arc<dyn Transport>) {
        let manager = Arc::new(
            FetchManager::with_transport(Arc::new(HostRateLimiter::new()), transport)
                .with_middleware(&self.middleware.read().unwrap()),
        );
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

    #[tokio::test]
    async fn add_middleware_wraps_every_fetch_through_a_mock_transport() {
        use crate::engine::fetcher::{
            BoxFuture, FetchRequest, MockTransport, NormalizedResponse,
        };
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct CountingLayer(Arc<AtomicUsize>);
        struct CountingTransport(Arc<AtomicUsize>, Arc<dyn Transport>);
        impl Transport for CountingTransport {
            fn fetch<'a>(
                &'a self,
                request: &'a FetchRequest,
            ) -> BoxFuture<'a, crate::error::Result<NormalizedResponse>> {
                self.0.fetch_add(1, Ordering::SeqCst);
                self.1.fetch(request)
            }
        }
        impl Layer for CountingLayer {
            fn wrap(&self, inner: Arc<dyn Transport>) -> Arc<dyn Transport> {
                Arc::new(CountingTransport(self.0.clone(), inner))
            }
        }

        let session = Session::new();
        let count = Arc::new(AtomicUsize::new(0));
        session.add_middleware(Arc::new(CountingLayer(count.clone())));

        let mock = Arc::new(MockTransport::new().with_default_html("<h1>hi</h1>"));
        session.set_transport(mock);

        let manager = session.fetch_manager();
        let req = FetchRequest {
            url: "https://example.com".to_string(),
            tier: FetcherTier::Standard,
            browser_profile: None,
            headers: Default::default(),
            cookies: Default::default(),
            proxy: None,
            timeout: std::time::Duration::from_secs(5),
            retries: 0,
            rate_limit_rps: 0.0,
        };
        manager.dispatch(req).await.expect("mock fetch should succeed");

        assert_eq!(
            count.load(Ordering::SeqCst),
            1,
            "middleware added before set_transport must still wrap the injected mock"
        );
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

    /// Enable in-memory HTTP response caching (honors `Cache-Control`/`ETag`/`Last-Modified`).
    /// Must be called before the first fetch made through this session. Returns self.
    #[pyo3(signature = (max_entries=1024, default_ttl_seconds=300))]
    pub fn enable_response_cache(
        self_: PyRef<'_, Self>,
        max_entries: u64,
        default_ttl_seconds: u64,
    ) -> PyResult<Py<Self>> {
        self_
            .inner
            .enable_response_cache(max_entries, std::time::Duration::from_secs(default_ttl_seconds));
        Ok(self_.into())
    }

    /// Authenticate every fetch with HTTP Basic auth. Must be called before the first fetch made
    /// through this session. Returns self.
    pub fn basic_auth(self_: PyRef<'_, Self>, username: &str, password: &str) -> PyResult<Py<Self>> {
        self_.inner.set_auth(crate::engine::auth::AuthScheme::Basic {
            username: username.to_string(),
            password: password.to_string(),
        });
        Ok(self_.into())
    }

    /// Authenticate every fetch with a fixed `Authorization: Bearer <token>` header. Must be
    /// called before the first fetch made through this session. Returns self.
    pub fn bearer_auth(self_: PyRef<'_, Self>, token: &str) -> PyResult<Py<Self>> {
        self_
            .inner
            .set_auth(crate::engine::auth::AuthScheme::Bearer(token.to_string()));
        Ok(self_.into())
    }

    /// Authenticate every fetch with a fixed custom header (e.g. an API key header). Must be
    /// called before the first fetch made through this session. Returns self.
    pub fn header_auth(self_: PyRef<'_, Self>, name: &str, value: &str) -> PyResult<Py<Self>> {
        self_.inner.set_auth(crate::engine::auth::AuthScheme::Header {
            name: name.to_string(),
            value: value.to_string(),
        });
        Ok(self_.into())
    }

    /// Authenticate every fetch by appending an API key query parameter to the request URL. Must
    /// be called before the first fetch made through this session. Returns self.
    pub fn api_key_auth(self_: PyRef<'_, Self>, name: &str, value: &str) -> PyResult<Py<Self>> {
        self_
            .inner
            .set_auth(crate::engine::auth::AuthScheme::ApiKeyQuery {
                name: name.to_string(),
                value: value.to_string(),
            });
        Ok(self_.into())
    }

    /// Returns a snapshot of this session's aggregated fetch metrics as a dict: `requests`,
    /// `successes`, `failures`, `bytes_in`, `status_counts` (status code -> count), `per_host`
    /// (host -> `{requests, successes, failures}`), and `avg_latency_ms`.
    pub fn metrics(&self, py: Python<'_>) -> PyResult<PyObject> {
        let snapshot = self.inner.metrics();
        let json = serde_json::to_string(&snapshot)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        let json_module = py.import("json")?;
        let parsed = json_module.call_method1("loads", (json,))?;
        Ok(parsed.into())
    }

    /// Clone the session (returns a new Session)
    pub fn clone(&self) -> PyResult<Self> {
        let cloned = Session::new();
        *cloned.headers.write().unwrap() = self.inner.headers.read().unwrap().clone();
        *cloned.cookies.write().unwrap() = self.inner.cookies.read().unwrap().clone();
        *cloned.proxy.write().unwrap() = self.inner.proxy.read().unwrap().clone();
        *cloned.rate_limit_rps.write().unwrap() = *self.inner.rate_limit_rps.read().unwrap();
        *cloned.auto_match.write().unwrap() = *self.inner.auto_match.read().unwrap();
        *cloned.timeout_seconds.write().unwrap() = *self.inner.timeout_seconds.read().unwrap();
        *cloned.fingerprint_path.write().unwrap() = self.inner.fingerprint_path.read().unwrap().clone();
        *cloned.fetcher_tier.write().unwrap() = *self.inner.fetcher_tier.read().unwrap();
        *cloned.browser_profile.write().unwrap() = self.inner.browser_profile.read().unwrap().clone();
        *cloned.similarity_weights.write().unwrap() = self.inner.similarity_weights.read().unwrap().clone();
        *cloned.proxy_pool.write().unwrap() = self.inner.proxy_pool.read().unwrap().clone();
        cloned.proxy_index.store(
            self.inner.proxy_index.load(std::sync::atomic::Ordering::SeqCst),
            std::sync::atomic::Ordering::SeqCst,
        );
        *cloned.proxy_provider_url.write().unwrap() = self.inner.proxy_provider_url.read().unwrap().clone();
        *cloned.retries.write().unwrap() = *self.inner.retries.read().unwrap();
        *cloned.retry_backoff.write().unwrap() = *self.inner.retry_backoff.read().unwrap();
        *cloned.retry_delay.write().unwrap() = *self.inner.retry_delay.read().unwrap();
        *cloned.proxy_username.write().unwrap() = self.inner.proxy_username.read().unwrap().clone();
        *cloned.proxy_password.write().unwrap() = self.inner.proxy_password.read().unwrap().clone();
        Ok(Self {
            inner: Arc::new(cloned),
        })
    }

    /// Destroy/invalidate the session resources
    pub fn destroy(&self) -> PyResult<()> {
        self.inner.headers.write().unwrap().clear();
        self.inner.cookies.write().unwrap().clear();
        *self.inner.proxy.write().unwrap() = None;
        self.inner.proxy_pool.write().unwrap().clear();
        *self.inner.proxy_provider_url.write().unwrap() = None;
        *self.inner.fingerprint_store.write().unwrap() = None;
        *self.inner.fetch_manager.write().unwrap() = None;
        Ok(())
    }

    /// Remove a header from session headers
    pub fn remove_header(self_: PyRef<'_, Self>, name: &str) -> PyResult<Py<Self>> {
        {
            let mut h = self_
                .inner
                .headers
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            h.remove(name);
        }
        Ok(self_.into())
    }

    /// Merge headers into session headers
    pub fn merge_headers(self_: PyRef<'_, Self>, headers: HashMap<String, String>) -> PyResult<Py<Self>> {
        {
            let mut h = self_
                .inner
                .headers
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            h.extend(headers);
        }
        Ok(self_.into())
    }

    /// Get cookies
    pub fn get_cookies(&self) -> PyResult<HashMap<String, String>> {
        let c = self.inner.cookies.read()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(c.clone())
    }

    /// Clear cookies
    pub fn clear_cookies(self_: PyRef<'_, Self>) -> PyResult<Py<Self>> {
        {
            let mut c = self_
                .inner
                .cookies
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            c.clear();
        }
        Ok(self_.into())
    }

    /// Delete a cookie
    pub fn delete_cookie(self_: PyRef<'_, Self>, name: &str) -> PyResult<Py<Self>> {
        {
            let mut c = self_
                .inner
                .cookies
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            c.remove(name);
        }
        Ok(self_.into())
    }

    /// Set retries
    pub fn retries(self_: PyRef<'_, Self>, count: u32) -> PyResult<Py<Self>> {
        {
            let mut r = self_
                .inner
                .retries
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *r = Some(count);
        }
        Ok(self_.into())
    }

    /// Set retry backoff
    pub fn retry_backoff(self_: PyRef<'_, Self>, factor: f64) -> PyResult<Py<Self>> {
        {
            let mut r = self_
                .inner
                .retry_backoff
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *r = Some(factor);
        }
        Ok(self_.into())
    }

    /// Set retry delay
    pub fn retry_delay(self_: PyRef<'_, Self>, seconds: u32) -> PyResult<Py<Self>> {
        {
            let mut r = self_
                .inner
                .retry_delay
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *r = Some(seconds);
        }
        Ok(self_.into())
    }

    /// Set proxy authentication
    #[pyo3(signature = (username=None, password=None))]
    pub fn proxy_auth(self_: PyRef<'_, Self>, username: Option<String>, password: Option<String>) -> PyResult<Py<Self>> {
        {
            let mut u = self_
                .inner
                .proxy_username
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *u = username;
            let mut p = self_
                .inner
                .proxy_password
                .write()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            *p = password;
        }
        Ok(self_.into())
    }

    /// Rotate to next proxy
    pub fn proxy_rotate(&self) -> PyResult<Option<String>> {
        Ok(self.inner.get_next_proxy())
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
