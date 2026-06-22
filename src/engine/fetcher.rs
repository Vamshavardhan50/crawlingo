use crate::engine::pool::ConnectionPoolConfig;
use crate::engine::rate_limiter::HostRateLimiter;
use crate::error::{CrawlingoError, Result};
use std::sync::Arc;
use std::time::Duration;
use wreq::{Client, Method, Response};
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
    pub headers: std::collections::HashMap<String, String>,
    pub cookies: std::collections::HashMap<String, String>,
    pub proxy: Option<String>,
    pub timeout: Duration,
    pub retries: usize,
    pub rate_limit_rps: f64,
}

/// The Fetcher engine that handles HTTP requests with connection pooling, rate limiting, and browser impersonation.
pub struct Fetcher {
    rate_limiter: Arc<HostRateLimiter>,
    pool_config: ConnectionPoolConfig,
}

impl Fetcher {
    /// Creates a new `Fetcher` with the given rate limiter and connection pool config.
    pub fn new(rate_limiter: Arc<HostRateLimiter>, pool_config: ConnectionPoolConfig) -> Self {
        Self {
            rate_limiter,
            pool_config,
        }
    }

    /// Fetches the target URL with the requested options, utilizing retries and rate limiting.
    pub async fn fetch(&self, request: FetchRequest) -> Result<Response> {
        // 1. Rate Limiting Check
        let parsed_url = url::Url::parse(&request.url)
            .map_err(|e| CrawlingoError::FetchError(format!("Invalid URL: {}", e)))?;
        let host = parsed_url.host_str().unwrap_or("");

        self.rate_limiter.wait(host, request.rate_limit_rps).await;

        // 2. Build the wreq Client
        let mut builder = Client::builder()
            .pool_max_idle_per_host(self.pool_config.max_idle_per_host)
            .pool_idle_timeout(self.pool_config.idle_timeout)
            .tcp_keepalive(self.pool_config.tcp_keepalive)
            .timeout(request.timeout)
            .cookie_store(true);

        // Configure Proxy
        if let Some(ref proxy_url) = request.proxy {
            let proxy = wreq::Proxy::all(proxy_url)
                .map_err(|e| CrawlingoError::FetchError(format!("Proxy error: {}", e)))?;
            builder = builder.proxy(proxy);
        }

        // Configure Stealth Emulation Tier
        if request.tier == FetcherTier::Stealthy {
            let emulation = match request.browser_profile.as_deref() {
                Some("firefox") => Emulation::Firefox142,
                Some("safari") => Emulation::Safari26,
                _ => Emulation::Chrome147, // Default to Chrome
            };
            builder = builder.emulation(emulation);
        }

        let client = builder
            .build()
            .map_err(|e| CrawlingoError::FetchError(format!("Failed to build client: {}", e)))?;

        // 3. Dispatch the Request with Exponential Backoff
        let mut attempt = 0;
        let mut delay = Duration::from_millis(500);

        loop {
            let mut req_builder = client.request(Method::GET, request.url.clone());

            // Attach Headers
            for (key, val) in &request.headers {
                req_builder = req_builder.header(key, val);
            }

            // Attach Cookies manually if present (wreq manages cookie jar internally, but we support session initialization cookies)
            if !request.cookies.is_empty() {
                let cookie_str = request
                    .cookies
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<String>>()
                    .join("; ");
                req_builder = req_builder.header("Cookie", cookie_str);
            }

            match req_builder.send().await {
                Ok(resp) => {
                    // Check if response is successful status or a retryable code (like 5xx, 429)
                    let status = resp.status();
                    if (status.is_server_error() || status.as_u16() == 429)
                        && attempt < request.retries
                    {
                        tracing::warn!(
                            "Retryable status code received: {}. Retrying in {:?}",
                            status,
                            delay
                        );
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        delay *= 2;
                    } else {
                        return Ok(resp);
                    }
                }
                Err(err) => {
                    if attempt < request.retries {
                        tracing::warn!("Network error: {}. Retrying in {:?}", err, delay);
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        delay *= 2;
                    } else {
                        return Err(CrawlingoError::FetchError(format!(
                            "Fetch execution failed: {}",
                            err
                        )));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetcher_standard() {
        let rl = Arc::new(HostRateLimiter::new());
        let fetcher = Fetcher::new(rl, ConnectionPoolConfig::default());
        let req = FetchRequest {
            url: "https://httpbin.org/get".to_string(),
            tier: FetcherTier::Standard,
            browser_profile: None,
            headers: std::collections::HashMap::new(),
            cookies: std::collections::HashMap::new(),
            proxy: None,
            timeout: Duration::from_secs(10),
            retries: 2,
            rate_limit_rps: 0.0,
        };

        let res = fetcher.fetch(req).await;
        assert!(res.is_ok());
        let status = res.unwrap().status().as_u16();
        assert!(
            status == 200
                || status == 503
                || status == 403
                || status == 429
                || status == 301
                || status == 302,
            "Unexpected status code: {}",
            status
        );
    }
}
