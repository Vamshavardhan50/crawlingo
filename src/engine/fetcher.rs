use crate::engine::pool::ConnectionPoolConfig;
use crate::engine::rate_limiter::HostRateLimiter;
use crate::error::{CrawlingoError, Result};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use wreq::{Client, Method};
use wreq_util::Emulation;

/// The fetcher tier mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetcherTier {
    Standard,
    Stealthy,
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

/// The unified interface that all fetching strategies must implement.
pub trait FetchStrategy: Send + Sync {
    /// Executes the fetch operation for a single request, returning a normalized response.
    fn fetch(
        &self,
        request: &FetchRequest,
    ) -> impl std::future::Future<Output = Result<NormalizedResponse>> + Send;
}

/// Strategy for executing fetches via standard HTTP client pooling and emulation.
pub struct HttpFetcher {
    pool_config: ConnectionPoolConfig,
}

impl HttpFetcher {
    pub fn new(pool_config: ConnectionPoolConfig) -> Self {
        Self { pool_config }
    }
}

impl FetchStrategy for HttpFetcher {
    async fn fetch(&self, request: &FetchRequest) -> Result<NormalizedResponse> {
        let mut builder = Client::builder()
            .pool_max_idle_per_host(self.pool_config.max_idle_per_host)
            .pool_idle_timeout(self.pool_config.idle_timeout)
            .tcp_keepalive(self.pool_config.tcp_keepalive)
            .timeout(request.timeout)
            .cookie_store(true);

        if let Some(ref proxy_url) = request.proxy {
            let proxy = wreq::Proxy::all(proxy_url)
                .map_err(|e| CrawlingoError::FetchError(format!("Proxy error: {}", e)))?;
            builder = builder.proxy(proxy);
        }

        if request.tier == FetcherTier::Stealthy {
            let emulation = match request.browser_profile.as_deref() {
                Some("firefox") => Emulation::Firefox142,
                Some("safari") => Emulation::Safari26,
                _ => Emulation::Chrome147,
            };
            builder = builder.emulation(emulation);
        }

        let client = builder
            .build()
            .map_err(|e| CrawlingoError::FetchError(format!("Failed to build client: {}", e)))?;

        let mut req_builder = client.request(Method::GET, request.url.clone());

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

impl FetchStrategy for BrowserFetcher {
    async fn fetch(&self, request: &FetchRequest) -> Result<NormalizedResponse> {
        // Delegates to fallback HTTP fetcher for Phase 3 (browser logic is placeholder for now)
        self.fallback.fetch(request).await
    }
}

/// The main orchestrator managing fetching strategies, retries, and rate limiting.
pub struct FetchManager {
    rate_limiter: Arc<HostRateLimiter>,
    http_strategy: Arc<HttpFetcher>,
    browser_strategy: Arc<BrowserFetcher>,
}

impl FetchManager {
    pub fn new(rate_limiter: Arc<HostRateLimiter>, pool_config: ConnectionPoolConfig) -> Self {
        Self {
            rate_limiter,
            http_strategy: Arc::new(HttpFetcher::new(pool_config.clone())),
            browser_strategy: Arc::new(BrowserFetcher::new(pool_config)),
        }
    }

    /// Dispatches the request through the correct strategy, applying rate limiting and exponential backoff.
    pub async fn dispatch(&self, request: FetchRequest) -> Result<NormalizedResponse> {
        let parsed_url = url::Url::parse(&request.url)
            .map_err(|e| CrawlingoError::FetchError(format!("Invalid URL: {}", e)))?;
        let host = parsed_url.host_str().unwrap_or("");

        self.rate_limiter.wait(host, request.rate_limit_rps).await;

        let mut attempt = 0;
        let mut delay = Duration::from_millis(500);

        loop {
            let res = if request.tier == FetcherTier::Stealthy {
                self.browser_strategy.fetch(&request).await
            } else {
                self.http_strategy.fetch(&request).await
            };

            match res {
                Ok(response) => {
                    // Successful or normal non-retryable response
                    return Ok(response);
                }
                Err(err) => {
                    if attempt < request.retries {
                        tracing::warn!("Fetch error: {}. Retrying in {:?}", err, delay);
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        delay *= 2;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
    }
}

/// Backward-compatibility wrapper representing the deprecated Fetcher client interface.
pub struct Fetcher {
    manager: FetchManager,
}

impl Fetcher {
    pub fn new(rate_limiter: Arc<HostRateLimiter>, pool_config: ConnectionPoolConfig) -> Self {
        Self {
            manager: FetchManager::new(rate_limiter, pool_config),
        }
    }

    pub async fn fetch(&self, request: FetchRequest) -> Result<wreq::Response> {
        // Emulates returning a wreq::Response by doing a direct fetch for backwards compatibility.
        // This is only retained for any direct crate dependencies that haven't migrated to FetchManager yet.
        let mut builder = Client::builder()
            .pool_max_idle_per_host(self.manager.http_strategy.pool_config.max_idle_per_host)
            .pool_idle_timeout(self.manager.http_strategy.pool_config.idle_timeout)
            .tcp_keepalive(self.manager.http_strategy.pool_config.tcp_keepalive)
            .timeout(request.timeout)
            .cookie_store(true);

        if let Some(ref proxy_url) = request.proxy {
            let proxy = wreq::Proxy::all(proxy_url)
                .map_err(|e| CrawlingoError::FetchError(format!("Proxy error: {}", e)))?;
            builder = builder.proxy(proxy);
        }

        if request.tier == FetcherTier::Stealthy {
            let emulation = match request.browser_profile.as_deref() {
                Some("firefox") => Emulation::Firefox142,
                Some("safari") => Emulation::Safari26,
                _ => Emulation::Chrome147,
            };
            builder = builder.emulation(emulation);
        }

        let client = builder
            .build()
            .map_err(|e| CrawlingoError::FetchError(format!("Failed to build client: {}", e)))?;

        let mut req_builder = client.request(Method::GET, request.url.clone());

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

        req_builder
            .send()
            .await
            .map_err(|e| CrawlingoError::FetchError(e.to_string()))
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
}
