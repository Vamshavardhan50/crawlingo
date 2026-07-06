//! HTTP response caching, as a [`Layer`](crate::engine::middleware::Layer).
//!
//! [`CachingLayer`] wraps a `Transport` with a [`ResponseCache`], honoring the subset of standard
//! HTTP caching semantics that matters for a crawler: `Cache-Control: no-store` (never cache),
//! `no-cache` (cache for revalidation but never serve without asking first), `max-age` (freshness
//! window), and `ETag`/`Last-Modified` conditional revalidation (`If-None-Match`/
//! `If-Modified-Since`, honoring a `304 Not Modified` by serving the cached body).
//!
//! Every fetch in this crate is a GET (see [`crate::engine::fetcher::HttpFetcher::execute`]), so
//! the cache key is simply the request URL — no method or `Vary` component is needed.

use crate::engine::fetcher::{BoxFuture, FetchRequest, NormalizedResponse, Transport};
use crate::engine::middleware::Layer;
use crate::error::Result;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// A cached response plus the freshness/validator metadata needed to decide whether it can still
/// be served as-is, must be revalidated, or has expired outright.
#[derive(Clone)]
pub struct CacheEntry {
    pub response: NormalizedResponse,
    pub stored_at: Instant,
    pub max_age: Duration,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    /// Set by `Cache-Control: no-cache` — the entry is retained as a revalidation candidate but
    /// treated as immediately stale, so it is never served without a conditional round-trip first.
    pub must_revalidate: bool,
}

impl CacheEntry {
    fn is_fresh(&self) -> bool {
        !self.must_revalidate && self.stored_at.elapsed() < self.max_age
    }

    fn has_validator(&self) -> bool {
        self.etag.is_some() || self.last_modified.is_some()
    }
}

/// A pluggable store for cached responses, keyed by request URL.
pub trait ResponseCache: Send + Sync {
    fn get<'a>(&'a self, url: &'a str) -> BoxFuture<'a, Option<CacheEntry>>;
    fn put<'a>(&'a self, url: &'a str, entry: CacheEntry) -> BoxFuture<'a, ()>;
}

/// An in-memory [`ResponseCache`] backed by `moka`, bounded by entry count and a hard TTL.
pub struct InMemoryCache {
    entries: moka::future::Cache<String, CacheEntry>,
}

impl InMemoryCache {
    /// `max_capacity` bounds the number of distinct cached URLs (LRU-evicted beyond it);
    /// `hard_ttl` upper-bounds how long any entry is kept regardless of its own `max_age`, as a
    /// safety net against an unreasonably large `Cache-Control: max-age` pinning memory forever.
    pub fn new(max_capacity: u64, hard_ttl: Duration) -> Self {
        Self {
            entries: moka::future::Cache::builder()
                .max_capacity(max_capacity)
                .time_to_live(hard_ttl)
                .build(),
        }
    }
}

impl Default for InMemoryCache {
    fn default() -> Self {
        Self::new(1024, Duration::from_secs(3600))
    }
}

impl ResponseCache for InMemoryCache {
    fn get<'a>(&'a self, url: &'a str) -> BoxFuture<'a, Option<CacheEntry>> {
        Box::pin(async move { self.entries.get(url).await })
    }

    fn put<'a>(&'a self, url: &'a str, entry: CacheEntry) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            self.entries.insert(url.to_string(), entry).await;
        })
    }
}

#[derive(Debug, Default)]
struct CacheControl {
    no_store: bool,
    no_cache: bool,
    max_age: Option<Duration>,
}

fn parse_cache_control(headers: &std::collections::HashMap<String, String>) -> CacheControl {
    let mut cc = CacheControl::default();
    let Some(value) = headers.get("cache-control") else {
        return cc;
    };
    for directive in value.split(',') {
        let directive = directive.trim();
        if directive.eq_ignore_ascii_case("no-store") {
            cc.no_store = true;
        } else if directive.eq_ignore_ascii_case("no-cache") {
            cc.no_cache = true;
        } else if let Some(secs) = directive
            .strip_prefix("max-age=")
            .and_then(|s| s.trim().parse::<u64>().ok())
        {
            cc.max_age = Some(Duration::from_secs(secs));
        }
    }
    cc
}

/// A [`Layer`] that caches `GET` responses, honoring `Cache-Control`/`ETag`/`Last-Modified`.
pub struct CachingLayer {
    cache: Arc<dyn ResponseCache>,
    /// Freshness window applied to a cacheable response that has no explicit `max-age`.
    default_ttl: Duration,
}

impl CachingLayer {
    pub fn new(cache: Arc<dyn ResponseCache>, default_ttl: Duration) -> Self {
        Self { cache, default_ttl }
    }
}

struct CachingTransport {
    cache: Arc<dyn ResponseCache>,
    default_ttl: Duration,
    inner: Arc<dyn Transport>,
}

impl CachingTransport {
    /// Stores `response` if it's cacheable (`200 OK` and not `Cache-Control: no-store`).
    async fn maybe_store(&self, url: &str, response: &NormalizedResponse) {
        if response.status != 200 {
            return;
        }
        let cc = parse_cache_control(&response.headers);
        if cc.no_store {
            return;
        }
        let entry = CacheEntry {
            response: response.clone(),
            stored_at: Instant::now(),
            max_age: cc.max_age.unwrap_or(self.default_ttl),
            etag: response.headers.get("etag").cloned(),
            last_modified: response.headers.get("last-modified").cloned(),
            must_revalidate: cc.no_cache,
        };
        self.cache.put(url, entry).await;
    }
}

impl Transport for CachingTransport {
    fn fetch<'a>(&'a self, request: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse>> {
        Box::pin(async move {
            if let Some(entry) = self.cache.get(&request.url).await {
                if entry.is_fresh() {
                    return Ok(entry.response.clone());
                }

                if entry.has_validator() {
                    let mut revalidate_req = request.clone();
                    if let Some(ref etag) = entry.etag {
                        revalidate_req
                            .headers
                            .insert("If-None-Match".to_string(), etag.clone());
                    }
                    if let Some(ref last_modified) = entry.last_modified {
                        revalidate_req
                            .headers
                            .insert("If-Modified-Since".to_string(), last_modified.clone());
                    }

                    return match self.inner.fetch(&revalidate_req).await {
                        Ok(resp) if resp.status == 304 => {
                            // Not Modified — refresh the freshness clock and serve the body we
                            // already had; a 304 has no body of its own to replace it with.
                            let mut refreshed = entry.clone();
                            refreshed.stored_at = Instant::now();
                            let body = refreshed.response.clone();
                            self.cache.put(&request.url, refreshed).await;
                            Ok(body)
                        }
                        Ok(resp) => {
                            self.maybe_store(&request.url, &resp).await;
                            Ok(resp)
                        }
                        Err(e) => Err(e),
                    };
                }
                // Stale with no validator to revalidate against — fall through to a normal fetch.
            }

            let result = self.inner.fetch(request).await;
            if let Ok(ref resp) = result {
                self.maybe_store(&request.url, resp).await;
            }
            result
        })
    }
}

impl Layer for CachingLayer {
    fn wrap(&self, inner: Arc<dyn Transport>) -> Arc<dyn Transport> {
        Arc::new(CachingTransport {
            cache: self.cache.clone(),
            default_ttl: self.default_ttl,
            inner,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::fetcher::{FetcherTier, MockResponse, MockTransport};
    use crate::engine::middleware::MiddlewareStack;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

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

    fn wrap(inner: Arc<dyn Transport>, cache: Arc<dyn ResponseCache>) -> Arc<dyn Transport> {
        MiddlewareStack::new()
            .with_layer(Arc::new(CachingLayer::new(cache, Duration::from_secs(60))))
            .build(inner)
    }

    #[tokio::test]
    async fn fresh_response_is_served_from_cache_without_a_second_fetch() {
        let mock = Arc::new(MockTransport::new().with_response(
            "https://example.com/a",
            MockResponse::html("<p>hi</p>").with_header("cache-control", "max-age=60"),
        ));
        let cache: Arc<dyn ResponseCache> = Arc::new(InMemoryCache::default());
        let transport = wrap(mock.clone(), cache);

        transport.fetch(&mock_request("https://example.com/a")).await.unwrap();
        let second = transport.fetch(&mock_request("https://example.com/a")).await.unwrap();

        assert_eq!(&second.body[..], b"<p>hi</p>");
        assert_eq!(mock.call_count(), 1, "second fetch should be served from cache");
    }

    #[tokio::test]
    async fn no_store_response_is_never_cached() {
        let mock = Arc::new(MockTransport::new().with_response(
            "https://example.com/b",
            MockResponse::html("<p>fresh each time</p>").with_header("cache-control", "no-store"),
        ));
        let cache: Arc<dyn ResponseCache> = Arc::new(InMemoryCache::default());
        let transport = wrap(mock.clone(), cache);

        transport.fetch(&mock_request("https://example.com/b")).await.unwrap();
        transport.fetch(&mock_request("https://example.com/b")).await.unwrap();

        assert_eq!(mock.call_count(), 2, "no-store must bypass the cache entirely");
    }

    #[tokio::test]
    async fn stale_without_validator_refetches_and_updates_body() {
        let mock = Arc::new(MockTransport::new().with_response(
            "https://example.com/c",
            MockResponse::html("<p>v1</p>").with_header("cache-control", "max-age=0"),
        ));
        let cache: Arc<dyn ResponseCache> = Arc::new(InMemoryCache::default());
        let transport = wrap(mock.clone(), cache);

        transport.fetch(&mock_request("https://example.com/c")).await.unwrap();
        // Immediately stale (max-age=0), no ETag/Last-Modified -> must hit the network again.
        transport.fetch(&mock_request("https://example.com/c")).await.unwrap();

        assert_eq!(mock.call_count(), 2, "stale entry with no validator must be refetched");
    }

    /// A tiny hand-rolled `Transport` (rather than extending `MockTransport`) so the test can
    /// inspect the conditional-request headers `CachingTransport` sends and answer with a real
    /// `304 Not Modified` — behavior `MockTransport` doesn't model.
    struct ConditionalTransport {
        calls: AtomicUsize,
        conditional_calls: AtomicUsize,
    }

    impl Transport for ConditionalTransport {
        fn fetch<'a>(
            &'a self,
            request: &'a FetchRequest,
        ) -> BoxFuture<'a, Result<NormalizedResponse>> {
            Box::pin(async move {
                self.calls.fetch_add(1, Ordering::SeqCst);
                if request.headers.contains_key("If-None-Match") {
                    self.conditional_calls.fetch_add(1, Ordering::SeqCst);
                    return Ok(NormalizedResponse {
                        url: request.url.clone(),
                        status: 304,
                        headers: HashMap::new(),
                        cookies: HashMap::new(),
                        body: bytes::Bytes::new(),
                        content_type: String::new(),
                        encoding: "utf-8".to_string(),
                        timings: Default::default(),
                    });
                }
                let mut headers = HashMap::new();
                headers.insert("cache-control".to_string(), "max-age=0".to_string());
                headers.insert("etag".to_string(), "\"v1\"".to_string());
                Ok(NormalizedResponse {
                    url: request.url.clone(),
                    status: 200,
                    headers,
                    cookies: HashMap::new(),
                    body: "<p>original</p>".into(),
                    content_type: "text/html".to_string(),
                    encoding: "utf-8".to_string(),
                    timings: Default::default(),
                })
            })
        }
    }

    #[tokio::test]
    async fn etag_revalidation_serves_cached_body_on_304() {
        let inner = Arc::new(ConditionalTransport {
            calls: AtomicUsize::new(0),
            conditional_calls: AtomicUsize::new(0),
        });
        let cache: Arc<dyn ResponseCache> = Arc::new(InMemoryCache::default());
        let transport = wrap(inner.clone(), cache);

        let first = transport.fetch(&mock_request("https://example.com/d")).await.unwrap();
        assert_eq!(&first.body[..], b"<p>original</p>");

        // Immediately stale (max-age=0) but has an ETag -> must revalidate, not blindly refetch.
        let second = transport.fetch(&mock_request("https://example.com/d")).await.unwrap();

        assert_eq!(
            &second.body[..],
            b"<p>original</p>",
            "a 304 must serve the previously cached body"
        );
        assert_eq!(inner.calls.load(Ordering::SeqCst), 2);
        assert_eq!(
            inner.conditional_calls.load(Ordering::SeqCst),
            1,
            "the second request must carry If-None-Match"
        );
    }
}
