//! A composable middleware stack for [`Transport`].
//!
//! Rather than growing `FetchManager`/`HttpFetcher` with an ad-hoc `bool`/`Option` field per new
//! cross-cutting concern (caching, auth, metrics, logging, ...), each concern is a [`Layer`] that
//! *decorates* a `Transport` with another `Transport` wrapping it — the same pattern
//! [`crate::engine::fetcher::MockTransport`] already gets injected through. This keeps
//! `HttpFetcher` focused purely on performing the actual network call, and lets any combination of
//! layers be composed (or skipped entirely) per [`crate::engine::session::Session`].

use crate::engine::fetcher::Transport;
use std::sync::Arc;

/// A single cross-cutting concern that wraps a [`Transport`] with another `Transport`.
///
/// Implementations should call through to `inner` — a `Layer` that never calls `inner.fetch(..)`
/// would break every fetch passing through it.
pub trait Layer: Send + Sync {
    /// Wraps `inner`, returning a new `Transport` that adds this layer's behavior around it.
    fn wrap(&self, inner: Arc<dyn Transport>) -> Arc<dyn Transport>;
}

/// An ordered stack of [`Layer`]s applied around a base [`Transport`].
///
/// Layers run in the order they were added: the first layer added is the *outermost* — it sees
/// each request first (e.g. a cache layer added first can short-circuit the network entirely) and
/// the response last (e.g. a metrics layer added last records only what actually reached the
/// network, after a cache hit already returned).
#[derive(Clone, Default)]
pub struct MiddlewareStack {
    layers: Vec<Arc<dyn Layer>>,
}

impl MiddlewareStack {
    /// An empty stack — [`MiddlewareStack::build`] returns `base` unchanged.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a layer. Chainable, for building a stack in one expression.
    pub fn with_layer(mut self, layer: Arc<dyn Layer>) -> Self {
        self.layers.push(layer);
        self
    }

    /// Appends a layer in place.
    pub fn push(&mut self, layer: Arc<dyn Layer>) {
        self.layers.push(layer);
    }

    /// Whether any layers have been added.
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    /// Wraps `base` with every layer in this stack, outermost-first.
    ///
    /// Folds in reverse insertion order so the first-added layer ends up as the outermost
    /// wrapper: for layers `[A, B, C]` (added in that order), the result is `A(B(C(base)))`.
    pub fn build(&self, base: Arc<dyn Transport>) -> Arc<dyn Transport> {
        self.layers
            .iter()
            .rev()
            .fold(base, |acc, layer| layer.wrap(acc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::fetcher::{BoxFuture, FetchRequest, MockTransport, NormalizedResponse};
    use crate::error::Result;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    /// A test layer that records its position in a shared call-order log, then delegates.
    struct RecordingLayer {
        name: &'static str,
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    struct RecordingTransport {
        name: &'static str,
        log: Arc<Mutex<Vec<&'static str>>>,
        inner: Arc<dyn Transport>,
    }

    impl Transport for RecordingTransport {
        fn fetch<'a>(
            &'a self,
            request: &'a FetchRequest,
        ) -> BoxFuture<'a, Result<NormalizedResponse>> {
            Box::pin(async move {
                self.log.lock().unwrap().push(self.name);
                self.inner.fetch(request).await
            })
        }
    }

    impl Layer for RecordingLayer {
        fn wrap(&self, inner: Arc<dyn Transport>) -> Arc<dyn Transport> {
            Arc::new(RecordingTransport {
                name: self.name,
                log: self.log.clone(),
                inner,
            })
        }
    }

    fn mock_request(url: &str) -> FetchRequest {
        FetchRequest {
            url: url.to_string(),
            tier: crate::engine::fetcher::FetcherTier::Standard,
            browser_profile: None,
            headers: Default::default(),
            cookies: Default::default(),
            proxy: None,
            timeout: std::time::Duration::from_secs(5),
            retries: 0,
            rate_limit_rps: 0.0,
        }
    }

    #[tokio::test]
    async fn empty_stack_returns_base_unchanged() {
        let stack = MiddlewareStack::new();
        assert!(stack.is_empty());
        let base: Arc<dyn Transport> =
            Arc::new(MockTransport::new().with_default_html("<h1>hi</h1>"));
        let wrapped = stack.build(base.clone());
        // Same underlying transport (no layers to add), proven by both producing identical
        // responses from the same mock, since Arc<dyn Transport> can't be pointer-compared
        // across a trait object boundary meaningfully otherwise.
        let resp = wrapped
            .fetch(&mock_request("https://example.com"))
            .await
            .unwrap();
        assert_eq!(&resp.body[..], b"<h1>hi</h1>");
    }

    #[tokio::test]
    async fn layers_run_outermost_first_in_insertion_order() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let stack = MiddlewareStack::new()
            .with_layer(Arc::new(RecordingLayer {
                name: "A",
                log: log.clone(),
            }))
            .with_layer(Arc::new(RecordingLayer {
                name: "B",
                log: log.clone(),
            }))
            .with_layer(Arc::new(RecordingLayer {
                name: "C",
                log: log.clone(),
            }));

        let base: Arc<dyn Transport> =
            Arc::new(MockTransport::new().with_default_html("<p>ok</p>"));
        let wrapped = stack.build(base);

        wrapped
            .fetch(&mock_request("https://example.com"))
            .await
            .unwrap();

        assert_eq!(*log.lock().unwrap(), vec!["A", "B", "C"]);
    }

    #[tokio::test]
    async fn push_mutates_an_existing_stack_in_place() {
        let mut stack = MiddlewareStack::new();
        let count = Arc::new(AtomicUsize::new(0));

        struct CountingLayer(Arc<AtomicUsize>);
        struct CountingTransport(Arc<AtomicUsize>, Arc<dyn Transport>);
        impl Transport for CountingTransport {
            fn fetch<'a>(
                &'a self,
                request: &'a FetchRequest,
            ) -> BoxFuture<'a, Result<NormalizedResponse>> {
                self.0.fetch_add(1, Ordering::SeqCst);
                self.1.fetch(request)
            }
        }
        impl Layer for CountingLayer {
            fn wrap(&self, inner: Arc<dyn Transport>) -> Arc<dyn Transport> {
                Arc::new(CountingTransport(self.0.clone(), inner))
            }
        }

        stack.push(Arc::new(CountingLayer(count.clone())));
        assert!(!stack.is_empty());

        let base: Arc<dyn Transport> =
            Arc::new(MockTransport::new().with_default_html("<p>ok</p>"));
        let wrapped = stack.build(base);
        wrapped
            .fetch(&mock_request("https://example.com"))
            .await
            .unwrap();

        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
}
