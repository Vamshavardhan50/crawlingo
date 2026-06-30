# docs/FETCH_DESIGN.md

This document designs the fetch layer for Crawlingo, moving away from concrete HTTP implementations to a strategy-based transport model.

---

## 1. The `FetchStrategy` Trait

Decouples request dispatch from the transport implementation.

```rust
pub trait FetchStrategy: Send + Sync {
    /// Executes the fetch request, returning a normalized response.
    fn fetch<'a>(&'a self, req: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse, CrawlingoError>>;
}
```

---

## 2. Core Fetch Strategies

### A. HTTP Fetcher (`HttpFetcher`)
- **Transport:** Built using the `wreq` client.
- **Role:** Handles high-performance HTTP requests, cookie jars, header maps, and proxies.
- **Use Case:** Scrapes raw static HTML documents rapidly with low memory overhead.

### B. Browser Fetcher (`BrowserFetcher`)
- **Transport:** Playwright or headless Chromium process wrapper.
- **Role:** Executes dynamic client-side JavaScript, handles heavy hydration, and mimics human actions.
- **Use Case:** Scrapes Single Page Applications (SPAs) or sites protected by advanced anti-bot challenges.

### C. API Fetcher (`ApiFetcher`)
- **Transport:** Direct raw socket or HTTP JSON client.
- **Role:** Fetches REST or GraphQL endpoints directly. Bypasses lol_html parsing entirely.
- **Use Case:** Scrapes data from dynamic backend APIs.

### D. Session Fetcher (`SessionFetcher`)
- **Transport:** Stateful client session manager.
- **Role:** Coordinates sequential request workflows (e.g. logging into a dashboard, storing auth tokens, navigating to private elements).
- **Use Case:** Scrapes websites requiring active user authentication or sequential form submissions.

### E. Cache Fetcher (`CacheFetcher`)
- **Transport:** Local disk directory storage (e.g. sled cache, flat filesystem).
- **Role:** Intercepts outgoing requests to return cached page files, preventing redundant network requests.
- **Use Case:** Offline local development, schema testing, and debugging.

---

## 3. The `FetchManager`

Coordinates proxy rotation, DNS caching, rate limiting, and strategy delegation.

```rust
pub struct FetchManager {
    session: Arc<Session>,
    rate_limiter: Arc<HostRateLimiter>,
    strategies: DashMap<String, Arc<dyn FetchStrategy>>,
    default_strategy: String,
}

impl FetchManager {
    pub async fn fetch(&self, req: FetchRequest) -> Result<NormalizedResponse, CrawlingoError> {
        // 1. Check rate limits
        let host = req.host()?;
        self.rate_limiter.wait_for(host).await?;

        // 2. Select strategy
        let strategy_name = req.strategy.unwrap_or(self.default_strategy.clone());
        let strategy = self.strategies.get(&strategy_name)
            .ok_or_else(|| CrawlingoError::ConfigurationError(format!("Strategy not found: {}", strategy_name)))?;

        // 3. Execute with retries
        let mut attempts = 0;
        loop {
            match strategy.fetch(&req).await {
                Ok(resp) => return Ok(resp),
                Err(e) if attempts < req.retries => {
                    attempts += 1;
                    // Apply exponential backoff delay
                    tokio::time::sleep(Duration::from_millis(attempts * 500)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

---

## 4. How Strategy Selection Works

The strategy selection is determined dynamically using the `FetchRequest` options:
1. **Explicit Selection:** If `req.strategy` is specified (e.g. `"browser"`), the FetchManager retrieves that strategy from its active registry.
2. **Implicit Fallback:** If not specified, the engine falls back to `default_strategy` (usually `"http"` for raw static fetches).
3. **Fuzzy selector healing triggers:** If a standard HTTP scrape fails to extract target elements, and `auto_match` is enabled, the client can fall back to the browser-based strategy to ensure the page has been fully loaded/rendered.

---

## 5. How Retries & Error Propagation Work

- **Exponential Backoff:** If a fetch strategy fails with a retriable error (e.g., a connection time-out or network reset), the manager sleeps for `attempt * 500ms` before calling the strategy again.
- **Error Types:** Non-retriable errors (e.g. `404 Not Found` or `401 Unauthorized`) propagate immediately.
- **FFI Mapping:** Errors are converted to `CrawlingoError` variants, which are translated into Python or JS exception instances at the outer FFI layers.

---

## 6. Normalizing Responses

Different fetchers (HTTP, browser, cache) output varying response models. The engine maps these to a unified `NormalizedResponse`:

```rust
pub struct NormalizedResponse {
    pub url: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: bytes::Bytes,
}
```

---

## 7. Plugin Registration

Users can register custom strategies at runtime:

```rust
impl FetchManager {
    pub fn register_strategy(&self, name: &str, strategy: Arc<dyn FetchStrategy>) {
        self.strategies.insert(name.to_string(), strategy);
    }
}
```
This allows developers to build and use proprietary extraction tools (such as headless browser containers or third-party web scraping APIs) within Crawlingo's pipeline.
