use crate::engine::pool::ConnectionPoolConfig;
use crate::engine::rate_limiter::HostRateLimiter;
use crate::engine::retry::{RetryDecision, RetryPolicy};
use crate::error::{CrawlingoError, Result};
use bytes::Bytes;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use wreq::{Client, Method};
use wreq_util::Emulation;

/// A boxed, `Send` future — the return type used by the object-safe [`Transport`] trait.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// The fetcher tier mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FetcherTier {
    Standard,
    Stealthy,
}

/// The identity of a reusable HTTP client.
///
/// Two requests that produce the same `TransportKey` can safely share a single [`wreq::Client`]
/// (and therefore its connection pool). It captures exactly the request attributes that alter how
/// the client is *constructed* — proxy, tier, and browser/TLS emulation profile. Per-request
/// concerns that do **not** affect client identity (timeout, headers, cookies, target URL) are
/// deliberately excluded so that clients are shared as widely as possible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TransportKey {
    tier: FetcherTier,
    /// The browser/TLS fingerprint emulation profile ("chrome"/"firefox"/"safari"). Only
    /// materially affects the client in the stealth tier, but is always included for determinism.
    browser_profile: Option<String>,
    /// The proxy URL, if any. Proxy is a client-level setting in wreq, so distinct proxies
    /// require distinct clients.
    proxy: Option<String>,
}

impl TransportKey {
    fn from_request(request: &FetchRequest) -> Self {
        Self {
            tier: request.tier,
            browser_profile: request.browser_profile.clone(),
            proxy: request.proxy.clone(),
        }
    }
}

/// Parameters for a single fetch request.
#[derive(Debug, Clone)]
pub struct FetchRequest {
    pub url: String,
    pub tier: FetcherTier,
    pub browser_profile: Option<String>, // "chrome", "firefox", "safari"
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub proxy: Option<String>,
    pub timeout: Duration,
    pub retries: usize,
    pub rate_limit_rps: f64,
}

/// Normalized timing measurements for fetch operations.
#[derive(Debug, Clone, Default)]
pub struct ResponseTimings {
    pub dns_lookup: Duration,
    pub tcp_connect: Duration,
    pub tls_handshake: Duration,
    pub total_duration: Duration,
}

/// A normalized response object returned by all strategies.
#[derive(Debug, Clone)]
pub struct NormalizedResponse {
    pub url: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Bytes,
    pub content_type: String,
    pub encoding: String,
    pub timings: ResponseTimings,
}

/// The unified, object-safe interface that all fetching transports must implement.
///
/// Returning a [`BoxFuture`] (rather than an `impl Future` via RPITIT) keeps the trait
/// **dyn-compatible**, which is what allows [`FetchManager`] to hold an `Arc<dyn Transport>`
/// and lets callers inject a [`MockTransport`] to exercise the full fetch → parse → extract
/// pipeline offline, with no network access.
pub trait Transport: Send + Sync {
    /// Executes the fetch operation for a single request, returning a normalized response.
    fn fetch<'a>(&'a self, request: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse>>;
}

/// Strategy for executing fetches via standard HTTP client pooling and emulation.
///
/// Holds a bounded, thread-safe cache of [`wreq::Client`]s keyed by [`TransportKey`]. Each client
/// owns a connection pool configured from the [`ConnectionPoolConfig`], so reusing a client across
/// requests is what makes keep-alive connection reuse actually happen. Clients are created lazily
/// on first use for a given configuration and evicted in LRU order once the cache exceeds
/// `pool_config.max_clients`.
pub struct HttpFetcher {
    pool_config: ConnectionPoolConfig,
    clients: moka::future::Cache<TransportKey, Client>,
}

impl HttpFetcher {
    pub fn new(pool_config: ConnectionPoolConfig) -> Self {
        let clients = moka::future::Cache::builder()
            .max_capacity(pool_config.max_clients)
            .build();
        Self {
            pool_config,
            clients,
        }
    }

    /// Constructs a fresh client for a given transport configuration, applying the pool settings.
    ///
    /// Note: cookies are handled explicitly per-request (via the `Cookie` header and manual
    /// `set-cookie` parsing), so the client-level cookie store is disabled. This is required for a
    /// shared/pooled client — an automatic jar would otherwise bleed cookies across unrelated
    /// requests that happen to reuse the same client.
    fn build_client(&self, key: &TransportKey) -> Result<Client> {
        let mut builder = Client::builder()
            .pool_max_idle_per_host(self.pool_config.max_idle_per_host)
            .pool_idle_timeout(self.pool_config.idle_timeout)
            .tcp_keepalive(self.pool_config.tcp_keepalive)
            .cookie_store(false);

        if let Some(ref proxy_url) = key.proxy {
            let proxy = wreq::Proxy::all(proxy_url)
                .map_err(|e| CrawlingoError::FetchError(format!("Proxy error: {}", e)))?;
            builder = builder.proxy(proxy);
        }

        if key.tier == FetcherTier::Stealthy {
            let emulation = match key.browser_profile.as_deref() {
                Some("firefox") => Emulation::Firefox142,
                Some("safari") => Emulation::Safari26,
                _ => Emulation::Chrome147,
            };
            builder = builder.emulation(emulation);
        }

        builder
            .build()
            .map_err(|e| CrawlingoError::FetchError(format!("Failed to build client: {}", e)))
    }

    /// Returns a client for the request's transport configuration, reusing a cached one when the
    /// configuration matches. Under concurrency, at most one client is built per key.
    async fn get_client(&self, request: &FetchRequest) -> Result<Client> {
        let key = TransportKey::from_request(request);
        self.clients
            .try_get_with(key.clone(), async { self.build_client(&key) })
            .await
            .map_err(|e: Arc<CrawlingoError>| {
                CrawlingoError::FetchError(format!("Client init failed: {}", e))
            })
    }

    /// Number of distinct clients currently cached. Primarily for tests/observability.
    ///
    /// moka maintains counters asynchronously, so pending maintenance is flushed first to make
    /// the returned value deterministic immediately after inserts/evictions.
    pub async fn cached_client_count(&self) -> u64 {
        self.clients.run_pending_tasks().await;
        self.clients.entry_count()
    }

    /// Returns the pooled [`wreq::Client`] that would serve this request, creating and caching it
    /// on first use. Exposed for benchmarking and advanced observability; normal callers should go
    /// through [`FetchManager::dispatch`], which additionally applies rate limiting and retries.
    pub async fn pooled_client(&self, request: &FetchRequest) -> Result<Client> {
        self.get_client(request).await
    }
}

impl HttpFetcher {
    /// Performs the actual network fetch. Kept as an inherent `async fn` for ergonomics;
    /// the object-safe [`Transport`] impl simply boxes this future.
    async fn execute(&self, request: &FetchRequest) -> Result<NormalizedResponse> {
        // Reuse a pooled client for this transport configuration. The per-request timeout is
        // applied on the request builder (below) rather than the client, so a single client can
        // serve requests with differing timeouts without defeating reuse.
        let client = self.get_client(request).await?;

        let mut req_builder = client
            .request(Method::GET, request.url.clone())
            .timeout(request.timeout);

        for (key, val) in &request.headers {
            req_builder = req_builder.header(key, val);
        }

        if !request.cookies.is_empty() {
            let cookie_str = request
                .cookies
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("; ");
            req_builder = req_builder.header("Cookie", cookie_str);
        }

        let start_time = std::time::Instant::now();
        let resp = req_builder
            .send()
            .await
            .map_err(|e| CrawlingoError::FetchError(e.to_string()))?;
        let elapsed = start_time.elapsed();

        let timings = ResponseTimings {
            dns_lookup: Duration::default(),
            tcp_connect: Duration::default(),
            tls_handshake: Duration::default(),
            total_duration: elapsed,
        };

        let status = resp.status().as_u16();
        let url_str = request.url.clone();

        let mut headers = HashMap::new();
        for (name, value) in resp.headers() {
            if let Ok(val_str) = value.to_str() {
                headers.insert(name.as_str().to_string(), val_str.to_string());
            }
        }

        let mut cookies = HashMap::new();
        for value in resp.headers().get_all("set-cookie") {
            if let Ok(val_str) = value.to_str() {
                if let Some(cookie_part) = val_str.split(';').next() {
                    let mut parts = cookie_part.splitn(2, '=');
                    if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                        cookies.insert(k.trim().to_string(), v.trim().to_string());
                    }
                }
            }
        }

        let content_type = headers.get("content-type").cloned().unwrap_or_default();
        let encoding = if content_type.contains("charset=") {
            content_type
                .split("charset=")
                .nth(1)
                .unwrap_or("utf-8")
                .trim()
                .to_string()
        } else {
            "utf-8".to_string()
        };

        let body = resp
            .bytes()
            .await
            .map_err(|e| CrawlingoError::FetchError(e.to_string()))?;

        Ok(NormalizedResponse {
            url: url_str,
            status,
            headers,
            cookies,
            body,
            content_type,
            encoding,
            timings,
        })
    }
}

impl Transport for HttpFetcher {
    fn fetch<'a>(&'a self, request: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse>> {
        Box::pin(self.execute(request))
    }
}

/// Placeholder strategy for executing fetches in a headless stealth browser.
pub struct BrowserFetcher {
    fallback: HttpFetcher,
}

impl BrowserFetcher {
    pub fn new(pool_config: ConnectionPoolConfig) -> Self {
        Self {
            fallback: HttpFetcher::new(pool_config),
        }
    }
}

impl Transport for BrowserFetcher {
    fn fetch<'a>(&'a self, request: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse>> {
        // Delegates to fallback HTTP fetcher for Phase 3 (browser logic is placeholder for now)
        self.fallback.fetch(request)
    }
}

/// The main orchestrator managing fetching strategies, retries, and rate limiting.
pub struct FetchManager {
    rate_limiter: Arc<HostRateLimiter>,
    http_strategy: Arc<dyn Transport>,
    browser_strategy: Arc<dyn Transport>,
    retry_policy: RetryPolicy,
}

impl FetchManager {
    /// Builds a manager backed by the real HTTP/browser transports.
    pub fn new(rate_limiter: Arc<HostRateLimiter>, pool_config: ConnectionPoolConfig) -> Self {
        Self {
            rate_limiter,
            http_strategy: Arc::new(HttpFetcher::new(pool_config.clone())),
            browser_strategy: Arc::new(BrowserFetcher::new(pool_config)),
            retry_policy: RetryPolicy::default(),
        }
    }

    /// Builds a manager backed by a single caller-supplied [`Transport`], used for both the
    /// standard and stealth tiers.
    ///
    /// This is the injection point for tests: pass a [`MockTransport`] to drive the entire
    /// fetch → retry → parse pipeline deterministically and offline.
    pub fn with_transport(
        rate_limiter: Arc<HostRateLimiter>,
        transport: Arc<dyn Transport>,
    ) -> Self {
        Self {
            rate_limiter,
            http_strategy: transport.clone(),
            browser_strategy: transport,
            retry_policy: RetryPolicy::default(),
        }
    }

    /// Overrides the manager's [`RetryPolicy`]. Chainable.
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Wraps both the standard and stealth transports with the given
    /// [`crate::engine::middleware::MiddlewareStack`] (caching, auth, metrics, ...). Chainable.
    ///
    /// A no-op for an empty stack. Must be called after [`FetchManager::new`]/
    /// [`FetchManager::with_transport`], since it wraps whatever transport is already set — any
    /// transport swapped in afterward (there is no such method today) would bypass it.
    pub fn with_middleware(mut self, stack: &crate::engine::middleware::MiddlewareStack) -> Self {
        if !stack.is_empty() {
            self.http_strategy = stack.build(self.http_strategy);
            self.browser_strategy = stack.build(self.browser_strategy);
        }
        self
    }

    /// Dispatches the request through the correct strategy, applying rate limiting and retries.
    ///
    /// Retries are governed by [`RetryPolicy`]: transport errors are always retried, and by
    /// default so are `429` and `5xx` responses (a non-2xx status is still an `Ok` result from
    /// the transport's perspective). A `Retry-After` response header takes precedence over the
    /// computed exponential backoff when present.
    pub async fn dispatch(&self, request: FetchRequest) -> Result<NormalizedResponse> {
        let parsed_url = url::Url::parse(&request.url)
            .map_err(|e| CrawlingoError::FetchError(format!("Invalid URL: {}", e)))?;
        let host = parsed_url.host_str().unwrap_or("");

        self.rate_limiter.wait(host, request.rate_limit_rps).await;

        let mut attempt = 0;

        loop {
            let res = if request.tier == FetcherTier::Stealthy {
                self.browser_strategy.fetch(&request).await
            } else {
                self.http_strategy.fetch(&request).await
            };

            match self.retry_policy.decide(attempt, request.retries, &res) {
                RetryDecision::Return => return res,
                RetryDecision::Retry(delay) => {
                    tracing::warn!(
                        "Fetch attempt {} not successful ({:?}), retrying in {:?}",
                        attempt,
                        res.as_ref().map(|r| r.status),
                        delay
                    );
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                }
            }
        }
    }
}

/// A programmable, in-memory [`Transport`] for offline testing.
///
/// Register canned responses per-URL (or a single default) and inject it via
/// [`FetchManager::with_transport`]. The mock records every requested URL so tests can assert
/// on call counts (e.g. retry behaviour, crawler de-duplication). It can also be told to fail a
/// fixed number of times before succeeding, to exercise the retry/backoff path deterministically.
///
/// # Example
/// ```
/// # use std::sync::Arc;
/// # use crawlingo::engine::fetcher::{FetchManager, FetchRequest, FetcherTier, MockTransport};
/// # use crawlingo::engine::rate_limiter::HostRateLimiter;
/// let mock = Arc::new(MockTransport::new().with_html("https://example.com", "<h1>Hi</h1>"));
/// let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock.clone());
/// ```
#[derive(Default)]
pub struct MockTransport {
    routes: std::sync::Mutex<HashMap<String, MockResponse>>,
    default: std::sync::Mutex<Option<MockResponse>>,
    calls: std::sync::Mutex<Vec<String>>,
    /// Number of leading calls that should fail with a `FetchError` before a real response.
    failures_before_success: std::sync::atomic::AtomicUsize,
}

/// A canned response used by [`MockTransport`].
#[derive(Debug, Clone)]
pub struct MockResponse {
    pub status: u16,
    pub body: Bytes,
    pub content_type: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
}

impl MockResponse {
    /// A `200 OK` HTML response with the given body.
    pub fn html(body: impl Into<Bytes>) -> Self {
        Self {
            status: 200,
            body: body.into(),
            content_type: "text/html; charset=utf-8".to_string(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
        }
    }

    /// A response with an arbitrary status code and body.
    pub fn with_status(status: u16, body: impl Into<Bytes>) -> Self {
        Self {
            status,
            ..Self::html(body)
        }
    }

    /// Sets a response header. Chainable.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

impl MockTransport {
    /// Creates an empty mock transport. Without any routes or a default, every fetch fails.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an HTML `200` response for an exact URL. Chainable.
    pub fn with_html(self, url: impl Into<String>, html: impl Into<String>) -> Self {
        self.routes
            .lock()
            .unwrap()
            .insert(url.into(), MockResponse::html(html.into()));
        self
    }

    /// Registers an arbitrary response for an exact URL. Chainable.
    pub fn with_response(self, url: impl Into<String>, response: MockResponse) -> Self {
        self.routes.lock().unwrap().insert(url.into(), response);
        self
    }

    /// Sets a fallback response returned for any URL without an explicit route. Chainable.
    pub fn with_default_html(self, html: impl Into<String>) -> Self {
        *self.default.lock().unwrap() = Some(MockResponse::html(html.into()));
        self
    }

    /// Makes the first `n` fetches fail (to exercise retry/backoff), then serve normally. Chainable.
    pub fn failing_first(self, n: usize) -> Self {
        self.failures_before_success
            .store(n, std::sync::atomic::Ordering::SeqCst);
        self
    }

    /// Returns the list of URLs that have been requested, in order.
    pub fn calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }

    /// Returns how many fetches have been attempted.
    pub fn call_count(&self) -> usize {
        self.calls.lock().unwrap().len()
    }
}

impl Transport for MockTransport {
    fn fetch<'a>(&'a self, request: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse>> {
        Box::pin(async move {
            self.calls.lock().unwrap().push(request.url.clone());

            // Simulate transient failures for retry testing.
            let remaining = self
                .failures_before_success
                .load(std::sync::atomic::Ordering::SeqCst);
            if remaining > 0 {
                self.failures_before_success
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                return Err(CrawlingoError::FetchError(format!(
                    "MockTransport: simulated transient failure ({} remaining)",
                    remaining - 1
                )));
            }

            let response = {
                let routes = self.routes.lock().unwrap();
                routes
                    .get(&request.url)
                    .cloned()
                    .or_else(|| self.default.lock().unwrap().clone())
            };

            match response {
                Some(mock) => Ok(NormalizedResponse {
                    url: request.url.clone(),
                    status: mock.status,
                    headers: mock.headers,
                    cookies: mock.cookies,
                    body: mock.body,
                    content_type: mock.content_type,
                    encoding: "utf-8".to_string(),
                    timings: ResponseTimings::default(),
                }),
                None => Err(CrawlingoError::FetchError(format!(
                    "MockTransport: no route registered for {}",
                    request.url
                ))),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    async fn spawn_test_server() -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind local test server");
        let addr = listener.local_addr().expect("read local test server addr");

        tokio::spawn(async move {
            // Accept multiple connections for retry logic (max 5)
            for _ in 0..5 {
                match listener.accept().await {
                    Ok((mut socket, _)) => {
                        let mut buf = [0_u8; 4096];
                        // Read the HTTP request
                        match socket.read(&mut buf).await {
                            Ok(n) if n > 0 => {
                                let body = b"<html><title>OK</title></html>";
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=utf-8\r\nset-cookie: session=test\r\ncontent-length: {}\r\n\r\n{}",
                                    body.len(),
                                    std::str::from_utf8(body).unwrap()
                                );
                                let _ = socket.write_all(response.as_bytes()).await;
                            }
                            _ => break,
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        format!("http://{addr}")
    }

    #[tokio::test]
    async fn test_fetcher_standard() {
        let url = spawn_test_server().await;

        // Test through FetchManager
        let rl = Arc::new(HostRateLimiter::new());
        let manager = FetchManager::new(rl, ConnectionPoolConfig::default());
        let req = FetchRequest {
            url,
            tier: FetcherTier::Standard,
            browser_profile: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            proxy: None,
            timeout: Duration::from_secs(10),
            retries: 2,
            rate_limit_rps: 0.0,
        };

        let res = manager.dispatch(req).await;
        assert!(res.is_ok(), "Fetch failed: {res:?}");
        let resp = res.unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.content_type, "text/html; charset=utf-8");
        assert_eq!(
            resp.cookies.get("session").map(String::as_str),
            Some("test")
        );
        assert_eq!(&resp.body[..], b"<html><title>OK</title></html>");
    }

    fn mock_request(url: &str) -> FetchRequest {
        FetchRequest {
            url: url.to_string(),
            tier: FetcherTier::Standard,
            browser_profile: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            proxy: None,
            timeout: Duration::from_secs(5),
            retries: 0,
            rate_limit_rps: 0.0,
        }
    }

    #[tokio::test]
    async fn mock_transport_serves_registered_route() {
        let mock =
            Arc::new(MockTransport::new().with_html("https://example.com/a", "<h1>Alpha</h1>"));
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock.clone());

        let resp = manager
            .dispatch(mock_request("https://example.com/a"))
            .await
            .expect("registered route should succeed");

        assert_eq!(resp.status, 200);
        assert_eq!(&resp.body[..], b"<h1>Alpha</h1>");
        assert_eq!(mock.calls(), vec!["https://example.com/a".to_string()]);
    }

    #[tokio::test]
    async fn mock_transport_default_and_status_passthrough() {
        let mock = Arc::new(
            MockTransport::new()
                .with_response(
                    "https://example.com/404",
                    MockResponse::with_status(404, "nope"),
                )
                .with_default_html("<p>default</p>"),
        );
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock);

        let missing = manager
            .dispatch(mock_request("https://example.com/unknown"))
            .await
            .expect("default route should serve unknown URLs");
        assert_eq!(&missing.body[..], b"<p>default</p>");

        let not_found = manager
            .dispatch(mock_request("https://example.com/404"))
            .await
            .expect("status codes are surfaced, not turned into errors");
        assert_eq!(not_found.status, 404);
    }

    #[tokio::test]
    async fn mock_transport_without_route_errors() {
        let mock = Arc::new(MockTransport::new());
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock);
        let res = manager
            .dispatch(mock_request("https://example.com/x"))
            .await;
        assert!(res.is_err(), "unrouted request should fail");
    }

    #[tokio::test]
    async fn manager_retries_transient_failures_then_succeeds() {
        let mock = Arc::new(
            MockTransport::new()
                .with_default_html("<h1>recovered</h1>")
                .failing_first(2),
        );
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock.clone());

        // retries=2 → one initial attempt + two retries; the first two fail, the third succeeds.
        let mut req = mock_request("https://example.com/flaky");
        req.retries = 2;
        let resp = manager
            .dispatch(req)
            .await
            .expect("should recover after retries");

        assert_eq!(&resp.body[..], b"<h1>recovered</h1>");
        assert_eq!(mock.call_count(), 3, "expected 2 failures then 1 success");
    }

    #[tokio::test]
    async fn manager_gives_up_after_exhausting_retries() {
        let mock = Arc::new(
            MockTransport::new()
                .with_default_html("<h1>late</h1>")
                .failing_first(5),
        );
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock.clone());

        let mut req = mock_request("https://example.com/dead");
        req.retries = 2;
        let res = manager.dispatch(req).await;

        assert!(res.is_err(), "should fail once retries are exhausted");
        assert_eq!(mock.call_count(), 3, "1 initial + 2 retries, all failing");
    }

    #[tokio::test]
    async fn manager_retries_503_then_succeeds() {
        let mock = Arc::new(MockTransport::new().with_response(
            "https://example.com/flaky-status",
            MockResponse::with_status(503, "unavailable"),
        ));
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock.clone())
            .with_retry_policy(RetryPolicy {
                base_delay: Duration::from_millis(1),
                max_delay: Duration::from_millis(5),
                ..RetryPolicy::default()
            });

        // The mock always answers 503 for this route; with 2 retries dispatch gives up and
        // surfaces the last (non-error) response rather than looping forever.
        let mut req = mock_request("https://example.com/flaky-status");
        req.retries = 2;
        let resp = manager.dispatch(req).await.expect("status is not an Err");
        assert_eq!(resp.status, 503);
        assert_eq!(mock.call_count(), 3, "1 initial + 2 retries on 503");
    }

    #[tokio::test]
    async fn manager_does_not_retry_404() {
        let mock = Arc::new(MockTransport::new().with_response(
            "https://example.com/missing",
            MockResponse::with_status(404, "nope"),
        ));
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock.clone());

        let mut req = mock_request("https://example.com/missing");
        req.retries = 3;
        let resp = manager.dispatch(req).await.expect("404 is not an Err");
        assert_eq!(resp.status, 404);
        assert_eq!(
            mock.call_count(),
            1,
            "non-retryable status must not be retried"
        );
    }

    #[tokio::test]
    async fn manager_honors_retry_after_header() {
        let mock = Arc::new(MockTransport::new().with_response(
            "https://example.com/rate-limited",
            MockResponse::with_status(429, "slow down").with_header("retry-after", "0"),
        ));
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock.clone());

        let mut req = mock_request("https://example.com/rate-limited");
        req.retries = 1;
        let resp = manager.dispatch(req).await.expect("429 is not an Err");
        assert_eq!(resp.status, 429);
        assert_eq!(
            mock.call_count(),
            2,
            "1 initial + 1 retry honoring Retry-After"
        );
    }

    // ---- Client-pool tests ---------------------------------------------------------------

    fn req_with(
        url: &str,
        tier: FetcherTier,
        profile: Option<&str>,
        proxy: Option<&str>,
    ) -> FetchRequest {
        FetchRequest {
            url: url.to_string(),
            tier,
            browser_profile: profile.map(String::from),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            proxy: proxy.map(String::from),
            timeout: Duration::from_secs(5),
            retries: 0,
            rate_limit_rps: 0.0,
        }
    }

    #[tokio::test]
    async fn identical_config_reuses_one_client() {
        let fetcher = HttpFetcher::new(ConnectionPoolConfig::default());
        // Same transport config, different URLs and timeouts → must share one client.
        let mut a = req_with("https://a.example.com/1", FetcherTier::Standard, None, None);
        let mut b = req_with("https://b.example.com/2", FetcherTier::Standard, None, None);
        a.timeout = Duration::from_secs(1);
        b.timeout = Duration::from_secs(30);

        fetcher.pooled_client(&a).await.unwrap();
        fetcher.pooled_client(&b).await.unwrap();

        assert_eq!(
            fetcher.cached_client_count().await,
            1,
            "URL and timeout must not affect client identity"
        );
    }

    #[tokio::test]
    async fn distinct_configs_create_distinct_clients() {
        let fetcher = HttpFetcher::new(ConnectionPoolConfig::default());

        // Baseline standard client.
        fetcher
            .pooled_client(&req_with(
                "https://x.example.com",
                FetcherTier::Standard,
                None,
                None,
            ))
            .await
            .unwrap();
        // Tier change.
        fetcher
            .pooled_client(&req_with(
                "https://x.example.com",
                FetcherTier::Stealthy,
                None,
                None,
            ))
            .await
            .unwrap();
        // Browser-profile change (within stealth tier).
        fetcher
            .pooled_client(&req_with(
                "https://x.example.com",
                FetcherTier::Stealthy,
                Some("firefox"),
                None,
            ))
            .await
            .unwrap();
        // Proxy change.
        fetcher
            .pooled_client(&req_with(
                "https://x.example.com",
                FetcherTier::Standard,
                None,
                Some("http://127.0.0.1:9"),
            ))
            .await
            .unwrap();

        assert_eq!(
            fetcher.cached_client_count().await,
            4,
            "tier, profile, and proxy each define a separate client"
        );
    }

    #[tokio::test]
    async fn concurrent_requests_build_single_client_per_key() {
        let fetcher = Arc::new(HttpFetcher::new(ConnectionPoolConfig::default()));
        let mut handles = Vec::new();
        for i in 0..64 {
            let f = fetcher.clone();
            handles.push(tokio::spawn(async move {
                let req = req_with(
                    &format!("https://host.example.com/{i}"),
                    FetcherTier::Standard,
                    None,
                    None,
                );
                f.pooled_client(&req).await.map(|_| ())
            }));
        }
        for h in handles {
            h.await.unwrap().unwrap();
        }
        assert_eq!(
            fetcher.cached_client_count().await,
            1,
            "concurrent identical-config requests must not stampede distinct clients"
        );
    }

    #[tokio::test]
    async fn cache_is_bounded_and_evicts() {
        let cfg = ConnectionPoolConfig::default().with_max_clients(2);
        let fetcher = HttpFetcher::new(cfg);
        // Three distinct proxies → three distinct keys, but capacity is 2.
        for port in ["1", "2", "3"] {
            let proxy = format!("http://127.0.0.1:{port}");
            fetcher
                .pooled_client(&req_with(
                    "https://y.example.com",
                    FetcherTier::Standard,
                    None,
                    Some(&proxy),
                ))
                .await
                .unwrap();
        }
        assert!(
            fetcher.cached_client_count().await <= 2,
            "cache must not exceed the configured max_clients bound"
        );
    }
}
