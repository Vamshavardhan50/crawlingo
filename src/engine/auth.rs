//! Authentication helpers, as a [`Layer`](crate::engine::middleware::Layer).
//!
//! [`AuthLayer`] injects credentials into every request before it reaches the network — a
//! `Basic`/`Bearer`/custom header, an API key query parameter, or a dynamically-fetched token via
//! [`TokenProvider`] (retried once with a freshly-fetched token after a `401 Unauthorized`).

use crate::engine::fetcher::{BoxFuture, FetchRequest, NormalizedResponse, Transport};
use crate::engine::middleware::Layer;
use crate::error::{CrawlingoError, Result};
use base64::Engine;
use std::sync::Arc;

/// Supplies a bearer-style token on demand — for auth schemes where the credential must itself be
/// fetched (and periodically refreshed) rather than being a fixed string, e.g. OAuth2 client
/// credentials.
pub trait TokenProvider: Send + Sync {
    /// Returns the current token, using a cached value if the implementation keeps one.
    fn token<'a>(&'a self) -> BoxFuture<'a, Result<String>>;

    /// Forces a fresh token (called after a `401`), replacing anything cached.
    fn refresh<'a>(&'a self) -> BoxFuture<'a, Result<String>>;
}

/// How to authenticate outgoing requests.
#[derive(Clone)]
pub enum AuthScheme {
    /// `Authorization: Basic base64(username:password)`.
    Basic { username: String, password: String },
    /// `Authorization: Bearer <token>`, a fixed, unchanging token.
    Bearer(String),
    /// An arbitrary fixed header, e.g. `Header { name: "X-Api-Key".into(), value: "...".into() }`.
    Header { name: String, value: String },
    /// A query-string API key (e.g. `?api_key=...`), appended to every request URL.
    ApiKeyQuery { name: String, value: String },
    /// A `Bearer` token fetched — and refreshed on `401` — via [`TokenProvider`], e.g. OAuth2.
    Dynamic(Arc<dyn TokenProvider>),
}

/// A [`Layer`] that authenticates every request per its [`AuthScheme`].
pub struct AuthLayer {
    scheme: AuthScheme,
}

impl AuthLayer {
    pub fn new(scheme: AuthScheme) -> Self {
        Self { scheme }
    }
}

struct AuthTransport {
    scheme: AuthScheme,
    inner: Arc<dyn Transport>,
}

impl AuthTransport {
    /// Applies the auth scheme to `request` in place. `token_override`, when given, is used
    /// instead of calling [`TokenProvider::token`] again — the retry-after-401 path already has a
    /// freshly-refreshed token and shouldn't immediately re-fetch (and potentially re-cache) it.
    async fn apply(&self, request: &mut FetchRequest, token_override: Option<&str>) -> Result<()> {
        match &self.scheme {
            AuthScheme::Basic { username, password } => {
                let encoded = base64::engine::general_purpose::STANDARD
                    .encode(format!("{username}:{password}"));
                request
                    .headers
                    .insert("Authorization".to_string(), format!("Basic {encoded}"));
            }
            AuthScheme::Bearer(token) => {
                request
                    .headers
                    .insert("Authorization".to_string(), format!("Bearer {token}"));
            }
            AuthScheme::Header { name, value } => {
                request.headers.insert(name.clone(), value.clone());
            }
            AuthScheme::ApiKeyQuery { name, value } => {
                let mut url = url::Url::parse(&request.url).map_err(|e| {
                    CrawlingoError::FetchError(format!("invalid URL for auth query parameter: {e}"))
                })?;
                url.query_pairs_mut().append_pair(name, value);
                request.url = url.to_string();
            }
            AuthScheme::Dynamic(provider) => {
                let token = match token_override {
                    Some(t) => t.to_string(),
                    None => provider.token().await?,
                };
                request
                    .headers
                    .insert("Authorization".to_string(), format!("Bearer {token}"));
            }
        }
        Ok(())
    }
}

impl Transport for AuthTransport {
    fn fetch<'a>(&'a self, request: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse>> {
        Box::pin(async move {
            let mut authed = request.clone();
            self.apply(&mut authed, None).await?;
            let result = self.inner.fetch(&authed).await?;

            // Only a Dynamic scheme can do anything useful with a 401 — static credentials
            // (Basic/Bearer/Header/ApiKeyQuery) would just fail identically a second time.
            if result.status == 401 {
                if let AuthScheme::Dynamic(ref provider) = self.scheme {
                    let fresh = provider.refresh().await?;
                    let mut retried = request.clone();
                    self.apply(&mut retried, Some(&fresh)).await?;
                    return self.inner.fetch(&retried).await;
                }
            }
            Ok(result)
        })
    }
}

impl Layer for AuthLayer {
    fn wrap(&self, inner: Arc<dyn Transport>) -> Arc<dyn Transport> {
        Arc::new(AuthTransport {
            scheme: self.scheme.clone(),
            inner,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::fetcher::{mock_request, MockResponse, MockTransport};
    use crate::engine::middleware::MiddlewareStack;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A `Transport` that echoes back the `Authorization` header (or the request URL, for the
    /// query-param scheme) as the response body, so tests can assert on exactly what was sent
    /// without `MockTransport` needing to understand headers.
    struct EchoAuthTransport;
    impl Transport for EchoAuthTransport {
        fn fetch<'a>(
            &'a self,
            request: &'a FetchRequest,
        ) -> BoxFuture<'a, Result<NormalizedResponse>> {
            Box::pin(async move {
                let body = format!(
                    "auth={} url={}",
                    request
                        .headers
                        .get("Authorization")
                        .cloned()
                        .unwrap_or_default(),
                    request.url,
                );
                Ok(NormalizedResponse {
                    url: request.url.clone(),
                    status: 200,
                    headers: HashMap::new(),
                    cookies: HashMap::new(),
                    body: body.into(),
                    content_type: "text/plain".to_string(),
                    encoding: "utf-8".to_string(),
                    timings: Default::default(),
                })
            })
        }
    }

    fn wrap(inner: Arc<dyn Transport>, scheme: AuthScheme) -> Arc<dyn Transport> {
        MiddlewareStack::new()
            .with_layer(Arc::new(AuthLayer::new(scheme)))
            .build(inner)
    }

    #[tokio::test]
    async fn basic_auth_sets_base64_authorization_header() {
        let transport = wrap(
            Arc::new(EchoAuthTransport),
            AuthScheme::Basic {
                username: "alice".to_string(),
                password: "wonderland".to_string(),
            },
        );
        let resp = transport
            .fetch(&mock_request("https://example.com"))
            .await
            .unwrap();
        let body = String::from_utf8(resp.body.to_vec()).unwrap();
        // base64("alice:wonderland") = "YWxpY2U6d29uZGVybGFuZA=="
        assert!(
            body.contains("auth=Basic YWxpY2U6d29uZGVybGFuZA=="),
            "{body}"
        );
    }

    #[tokio::test]
    async fn bearer_auth_sets_authorization_header() {
        let transport = wrap(
            Arc::new(EchoAuthTransport),
            AuthScheme::Bearer("tok123".to_string()),
        );
        let resp = transport
            .fetch(&mock_request("https://example.com"))
            .await
            .unwrap();
        let body = String::from_utf8(resp.body.to_vec()).unwrap();
        assert!(body.contains("auth=Bearer tok123"), "{body}");
    }

    #[tokio::test]
    async fn header_auth_sets_arbitrary_header() {
        struct EchoHeaderTransport;
        impl Transport for EchoHeaderTransport {
            fn fetch<'a>(
                &'a self,
                request: &'a FetchRequest,
            ) -> BoxFuture<'a, Result<NormalizedResponse>> {
                Box::pin(async move {
                    let body = request
                        .headers
                        .get("X-Api-Key")
                        .cloned()
                        .unwrap_or_default();
                    Ok(NormalizedResponse {
                        url: request.url.clone(),
                        status: 200,
                        headers: HashMap::new(),
                        cookies: HashMap::new(),
                        body: body.into(),
                        content_type: "text/plain".to_string(),
                        encoding: "utf-8".to_string(),
                        timings: Default::default(),
                    })
                })
            }
        }

        let transport = wrap(
            Arc::new(EchoHeaderTransport),
            AuthScheme::Header {
                name: "X-Api-Key".to_string(),
                value: "secret-key".to_string(),
            },
        );
        let resp = transport
            .fetch(&mock_request("https://example.com"))
            .await
            .unwrap();
        assert_eq!(&resp.body[..], b"secret-key");
    }

    #[tokio::test]
    async fn api_key_query_appends_query_parameter() {
        let transport = wrap(
            Arc::new(EchoAuthTransport),
            AuthScheme::ApiKeyQuery {
                name: "api_key".to_string(),
                value: "secret".to_string(),
            },
        );
        let resp = transport
            .fetch(&mock_request("https://example.com/data?x=1"))
            .await
            .unwrap();
        let body = String::from_utf8(resp.body.to_vec()).unwrap();
        assert!(body.contains("x=1"), "{body}");
        assert!(body.contains("api_key=secret"), "{body}");
    }

    /// A `TokenProvider` that starts with a stale token and only returns the valid one after
    /// `refresh()` is called — used to prove the auth layer actually retries with a fresh token
    /// on a `401`, rather than giving up or looping.
    struct FlakyTokenProvider {
        refreshed: std::sync::atomic::AtomicBool,
        refresh_calls: AtomicUsize,
    }

    impl TokenProvider for FlakyTokenProvider {
        fn token<'a>(&'a self) -> BoxFuture<'a, Result<String>> {
            Box::pin(async move {
                Ok(if self.refreshed.load(Ordering::SeqCst) {
                    "fresh-token".to_string()
                } else {
                    "stale-token".to_string()
                })
            })
        }

        fn refresh<'a>(&'a self) -> BoxFuture<'a, Result<String>> {
            Box::pin(async move {
                self.refreshed.store(true, Ordering::SeqCst);
                self.refresh_calls.fetch_add(1, Ordering::SeqCst);
                Ok("fresh-token".to_string())
            })
        }
    }

    struct UnauthorizedUntilFreshTransport;
    impl Transport for UnauthorizedUntilFreshTransport {
        fn fetch<'a>(
            &'a self,
            request: &'a FetchRequest,
        ) -> BoxFuture<'a, Result<NormalizedResponse>> {
            Box::pin(async move {
                let authed = request.headers.get("Authorization").map(String::as_str)
                    == Some("Bearer fresh-token");
                Ok(NormalizedResponse {
                    url: request.url.clone(),
                    status: if authed { 200 } else { 401 },
                    headers: HashMap::new(),
                    cookies: HashMap::new(),
                    body: if authed {
                        "welcome".into()
                    } else {
                        "denied".into()
                    },
                    content_type: "text/plain".to_string(),
                    encoding: "utf-8".to_string(),
                    timings: Default::default(),
                })
            })
        }
    }

    #[tokio::test]
    async fn dynamic_scheme_refreshes_and_retries_once_on_401() {
        let provider = Arc::new(FlakyTokenProvider {
            refreshed: std::sync::atomic::AtomicBool::new(false),
            refresh_calls: AtomicUsize::new(0),
        });
        let transport = wrap(
            Arc::new(UnauthorizedUntilFreshTransport),
            AuthScheme::Dynamic(provider.clone()),
        );

        let resp = transport
            .fetch(&mock_request("https://example.com"))
            .await
            .unwrap();

        assert_eq!(resp.status, 200);
        assert_eq!(&resp.body[..], b"welcome");
        assert_eq!(provider.refresh_calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn static_schemes_do_not_retry_on_401() {
        let mock = Arc::new(
            MockTransport::new()
                .with_response("https://example.com/", MockResponse::with_status(401, "no")),
        );
        let transport = wrap(mock.clone(), AuthScheme::Bearer("tok".to_string()));

        let resp = transport
            .fetch(&mock_request("https://example.com/"))
            .await
            .unwrap();

        assert_eq!(resp.status, 401);
        assert_eq!(
            mock.call_count(),
            1,
            "a static scheme has nothing to refresh, so must not retry"
        );
    }
}
