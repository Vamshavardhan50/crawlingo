# docs/FETCH_REDESIGN.md

This document details the redesign of the Fetching Layer in Crawlingo around a polymorphic, strategy-based architecture.

---

## 1. The `FetchStrategy` Trait

Every network client backend must implement the standard `FetchStrategy` trait. This encapsulates the transport layer and exposes a uniform async interface.

```rust
use crate::error::Result;
use std::time::Duration;
use bytes::Bytes;
use std::collections::HashMap;

/// A normalized request payload passed to fetch strategies.
#[derive(Debug, Clone)]
pub struct FetchOptions {
    pub url: String,
    pub timeout: Duration,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub proxy: Option<String>,
    pub user_agent: Option<String>,
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

#[derive(Debug, Clone, Default)]
pub struct ResponseTimings {
    pub dns_lookup: Duration,
    pub tcp_connect: Duration,
    pub tls_handshake: Duration,
    pub total_duration: Duration,
}

/// Unified fetch strategy interface.
#[async_trait::async_trait]
pub trait FetchStrategy: Send + Sync {
    /// Executes the fetch operation according to the strategy's rules.
    async fn fetch(&self, options: &FetchOptions) -> Result<NormalizedResponse>;
}
```

---

## 2. Fetch Strategy Backends

### A. `HttpFetcher` (Standard HTTP Client)
- **Engine:** Built using `wreq` (client with HTTP/2, cookies, TLS emulation).
- **Use Case:** High-throughput, lightweight crawling of standard HTML pages and API endpoints. Excellent performance and low memory foot-print.

### B. `BrowserFetcher` (Stealth / Headless Engine)
- **Engine:** Headless browser integration (e.g. Playwright or Chrome DevTools Protocol bindings).
- **Use Case:** Single Page Applications (SPAs) requiring JavaScript execution, DOM manipulation, or bypass of advanced anti-bot challenges (Stealthy tier).

### C. `SessionFetcher` (Stateful Wrapper)
- **Engine:** Intercepts requests and injects cookie store handles or persistent session headers.
- **Use Case:** Authenticated crawler tasks where login credentials or state tokens must be preserved across redirects and requests.

### D. `ApiFetcher` (Stealth Proxy / Middleware Routing)
- **Engine:** Routes requests through commercial proxy APIs or custom proxy rotational gates.
- **Use Case:** Large-scale commercial scraping that offloads IP rotation, proxy retry pacing, and captcha solving to third-party microservices.

---

## 3. Fetch Manager & Automatic Routing

The `FetchManager` holds references to the available fetch strategies and acts as the orchestrator.

```rust
use std::sync::Arc;

pub struct FetchManager {
    http_strategy: Arc<dyn FetchStrategy>,
    browser_strategy: Arc<dyn FetchStrategy>,
    // Other strategy backends...
}

impl FetchManager {
    /// Evaluates request parameters and routes the request to the correct strategy.
    pub async fn dispatch(&self, request: &FetchRequest, session: &Session) -> Result<NormalizedResponse> {
        // 1. Resolve politeness rate limit wait
        let parsed_url = url::Url::parse(&request.url)?;
        let host = parsed_url.host_str().unwrap_or("");
        session.rate_limiter.wait(host, request.rate_limit_rps).await;

        // 2. Map session configs to FetchOptions
        let options = FetchOptions {
            url: request.url.clone(),
            timeout: request.timeout,
            headers: request.headers.clone(),
            cookies: request.cookies.clone(),
            proxy: request.proxy.clone(),
            user_agent: request.browser_profile.clone(),
        };

        // 3. Selection routing algorithm:
        // - If request requires dynamic JS, captcha bypass, or tier is Stealthy, use Browser Strategy.
        // - Else, fall back to the high-performance HttpStrategy.
        let strategy = if request.tier == FetcherTier::Stealthy {
            &self.browser_strategy
        } else {
            &self.http_strategy
        };

        // 4. Dispatch with retry loop
        let mut attempt = 0;
        let mut delay = Duration::from_millis(500);

        loop {
            match strategy.fetch(&options).await {
                Ok(response) => return Ok(response),
                Err(err) if err.is_retryable() && attempt < request.retries => {
                    tracing::warn!("Fetch attempt {} failed with: {}. Retrying...", attempt, err);
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                    delay *= 2;
                }
                Err(err) => return Err(err),
            }
        }
    }
}
```
