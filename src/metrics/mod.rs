//! Fetch metrics aggregation.
//!
//! [`MetricsLayer`] is a [`Layer`](crate::engine::middleware::Layer) that wraps a `Transport` and
//! records every fetch attempt into a shared [`Metrics`] sink. Every [`crate::engine::session::Session`]
//! carries its own `Metrics` (see `Session::metrics()`), with a `MetricsLayer` wired in as the
//! outermost middleware layer automatically — no opt-in required, matching rate limiting and retry
//! being always-on.
//!
//! Note on what counts as "a request": the layer wraps the transport that
//! [`crate::engine::fetcher::FetchManager::dispatch`] calls, which includes every retry attempt,
//! not just the first. A dispatch that fails twice then succeeds records three requests (two
//! failures, one success) — this is intentional, since each retry is a real network attempt worth
//! accounting for.

use crate::engine::fetcher::{BoxFuture, FetchRequest, NormalizedResponse, Transport};
use crate::engine::middleware::Layer;
use crate::error::Result;
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Per-host request/success/failure counters.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct HostMetrics {
    pub requests: u64,
    pub successes: u64,
    pub failures: u64,
}

/// A point-in-time, plain-data copy of [`Metrics`] — safe to serialize, print, or return across
/// an FFI boundary (unlike `Metrics` itself, which holds atomics/`DashMap`s).
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct MetricsSnapshot {
    pub requests: u64,
    pub successes: u64,
    pub failures: u64,
    pub bytes_in: u64,
    pub status_counts: HashMap<u16, u64>,
    pub per_host: HashMap<String, HostMetrics>,
    pub avg_latency_ms: f64,
}

/// Thread-safe, lock-free aggregation of fetch outcomes across a session's lifetime.
///
/// Counters are plain atomics/`DashMap` entries updated with `Ordering::Relaxed` — metrics don't
/// need to synchronize with any other memory access, only be eventually consistent with each
/// other, which relaxed ordering guarantees for independent counters.
#[derive(Default)]
pub struct Metrics {
    requests: AtomicU64,
    successes: AtomicU64,
    failures: AtomicU64,
    bytes_in: AtomicU64,
    total_latency_ms: AtomicU64,
    status_counts: DashMap<u16, u64>,
    per_host: DashMap<String, HostMetrics>,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    fn record_success(&self, host: &str, status: u16, bytes: u64, elapsed: Duration) {
        self.requests.fetch_add(1, Ordering::Relaxed);
        self.successes.fetch_add(1, Ordering::Relaxed);
        self.bytes_in.fetch_add(bytes, Ordering::Relaxed);
        self.total_latency_ms
            .fetch_add(elapsed.as_millis() as u64, Ordering::Relaxed);
        *self.status_counts.entry(status).or_insert(0) += 1;

        let mut entry = self.per_host.entry(host.to_string()).or_default();
        entry.requests += 1;
        entry.successes += 1;
    }

    fn record_failure(&self, host: &str, elapsed: Duration) {
        self.requests.fetch_add(1, Ordering::Relaxed);
        self.failures.fetch_add(1, Ordering::Relaxed);
        self.total_latency_ms
            .fetch_add(elapsed.as_millis() as u64, Ordering::Relaxed);

        let mut entry = self.per_host.entry(host.to_string()).or_default();
        entry.requests += 1;
        entry.failures += 1;
    }

    /// Takes a consistent-enough point-in-time snapshot of all counters.
    ///
    /// Not perfectly atomic across fields under concurrent writers (each field is read
    /// independently), which is an acceptable tradeoff for observability data — a snapshot taken
    /// mid-burst may show e.g. `requests` slightly ahead of `successes + failures` by whatever
    /// updates land between reads, never a lost or duplicated update.
    pub fn snapshot(&self) -> MetricsSnapshot {
        let requests = self.requests.load(Ordering::Relaxed);
        let total_latency_ms = self.total_latency_ms.load(Ordering::Relaxed);
        MetricsSnapshot {
            requests,
            successes: self.successes.load(Ordering::Relaxed),
            failures: self.failures.load(Ordering::Relaxed),
            bytes_in: self.bytes_in.load(Ordering::Relaxed),
            status_counts: self
                .status_counts
                .iter()
                .map(|e| (*e.key(), *e.value()))
                .collect(),
            per_host: self
                .per_host
                .iter()
                .map(|e| (e.key().clone(), e.value().clone()))
                .collect(),
            avg_latency_ms: if requests > 0 {
                total_latency_ms as f64 / requests as f64
            } else {
                0.0
            },
        }
    }
}

/// A [`Layer`] that records every fetch attempt passing through it into a shared [`Metrics`].
pub struct MetricsLayer {
    metrics: Arc<Metrics>,
}

impl MetricsLayer {
    pub fn new(metrics: Arc<Metrics>) -> Self {
        Self { metrics }
    }
}

struct MetricsTransport {
    metrics: Arc<Metrics>,
    inner: Arc<dyn Transport>,
}

impl Transport for MetricsTransport {
    fn fetch<'a>(&'a self, request: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse>> {
        Box::pin(async move {
            let host = url::Url::parse(&request.url)
                .ok()
                .and_then(|u| u.host_str().map(String::from))
                .unwrap_or_default();
            let start = Instant::now();
            let result = self.inner.fetch(request).await;
            let elapsed = start.elapsed();
            match &result {
                Ok(resp) => self
                    .metrics
                    .record_success(&host, resp.status, resp.body.len() as u64, elapsed),
                Err(_) => self.metrics.record_failure(&host, elapsed),
            }
            result
        })
    }
}

impl Layer for MetricsLayer {
    fn wrap(&self, inner: Arc<dyn Transport>) -> Arc<dyn Transport> {
        Arc::new(MetricsTransport {
            metrics: self.metrics.clone(),
            inner,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::fetcher::{FetchManager, FetcherTier, MockTransport};
    use crate::engine::middleware::MiddlewareStack;
    use crate::engine::rate_limiter::HostRateLimiter;

    fn mock_request(url: &str) -> FetchRequest {
        FetchRequest {
            url: url.to_string(),
            tier: FetcherTier::Standard,
            browser_profile: None,
            headers: Default::default(),
            cookies: Default::default(),
            proxy: None,
            timeout: Duration::from_secs(5),
            retries: 0,
            rate_limit_rps: 0.0,
        }
    }

    #[test]
    fn snapshot_of_fresh_metrics_is_all_zero() {
        let metrics = Metrics::new();
        let snap = metrics.snapshot();
        assert_eq!(snap.requests, 0);
        assert_eq!(snap.avg_latency_ms, 0.0);
    }

    #[tokio::test]
    async fn records_successes_and_failures_per_host() {
        let mock = Arc::new(
            MockTransport::new()
                .with_html("https://a.example.com/ok", "<p>ok</p>")
                .with_response(
                    "https://b.example.com/err",
                    crate::engine::fetcher::MockResponse::with_status(500, "err"),
                ),
        );

        let metrics = Arc::new(Metrics::new());
        let stack = MiddlewareStack::new().with_layer(Arc::new(MetricsLayer::new(metrics.clone())));
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock)
            .with_middleware(&stack);

        manager
            .dispatch(mock_request("https://a.example.com/ok"))
            .await
            .unwrap();
        // 500 is Ok(NormalizedResponse) from the transport's perspective, not an Err — recorded
        // as a "success" fetch here (the metrics layer counts completed requests, not business
        // outcomes; a retryable-status decision is RetryPolicy's job, one layer up).
        manager
            .dispatch(mock_request("https://b.example.com/err"))
            .await
            .unwrap();

        let snap = metrics.snapshot();
        assert_eq!(snap.requests, 2);
        assert_eq!(snap.successes, 2);
        assert_eq!(snap.failures, 0);
        assert_eq!(snap.status_counts.get(&200), Some(&1));
        assert_eq!(snap.status_counts.get(&500), Some(&1));
        assert_eq!(snap.per_host.get("a.example.com").unwrap().requests, 1);
        assert_eq!(snap.per_host.get("b.example.com").unwrap().requests, 1);
    }

    #[tokio::test]
    async fn records_transport_errors_as_failures() {
        let mock = Arc::new(MockTransport::new()); // no routes registered -> every fetch errors
        let metrics = Arc::new(Metrics::new());
        let stack = MiddlewareStack::new().with_layer(Arc::new(MetricsLayer::new(metrics.clone())));
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock)
            .with_middleware(&stack);

        let _ = manager
            .dispatch(mock_request("https://unrouted.example.com"))
            .await;

        let snap = metrics.snapshot();
        assert_eq!(snap.requests, 1);
        assert_eq!(snap.failures, 1);
        assert_eq!(snap.successes, 0);
    }

    #[tokio::test]
    async fn retries_count_as_additional_requests() {
        let mock = Arc::new(
            MockTransport::new()
                .with_default_html("<p>recovered</p>")
                .failing_first(2),
        );
        let metrics = Arc::new(Metrics::new());
        let stack = MiddlewareStack::new().with_layer(Arc::new(MetricsLayer::new(metrics.clone())));
        let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock)
            .with_middleware(&stack);

        let mut req = mock_request("https://flaky.example.com");
        req.retries = 2;
        manager.dispatch(req).await.unwrap();

        let snap = metrics.snapshot();
        // 2 failed attempts + 1 successful attempt, each a distinct pass through the layer.
        assert_eq!(snap.requests, 3);
        assert_eq!(snap.failures, 2);
        assert_eq!(snap.successes, 1);
    }
}
