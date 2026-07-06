# 04_TECHNICAL_DEBT.md

This document catalogues the current technical debt in the Crawlingo codebase, ranked by severity.

---

## High Severity Issues

### 1. Inefficient HTTP Client & Rate Limiter Lifecycles
- **Location:** [crawler.rs](file:///d:/Scraper/src/crawl/crawler.rs), [builder.rs](file:///d:/Scraper/src/dataset/builder.rs)
- **Problem:** Every crawler worker loop and every call to `Dataset::build_async` constructs a new `Fetcher` and a new `HostRateLimiter` instance rather than reusing a shared instance from the `Session`.
- **Impact:** Empties the HTTP connection pool, bypasses connection reuse, and defeats rate-limit enforcement across concurrent worker threads.
- **Status (rate limiter): RESOLVED.** `Session` now owns a single lazily-built `FetchManager` (`Session::fetch_manager()`), reused by `Dataset::build_async`, `Crawler::crawl_async`, and proxy-provider fetches. Host rate limits are therefore shared per-session. The crawler webhook also reuses one `wreq::Client` instead of building one per delivered result.
- **Remaining follow-up (pooling):** `HttpFetcher::execute` still builds a fresh `wreq::Client` on **every** request, so connection pooling never actually engages regardless of `ConnectionPoolConfig`. Caching a client per (proxy, tier, profile) key is the next performance task.

### 2. Sled Database Connection Bottleneck
- **Location:** [builder.rs](file:///d:/Scraper/src/dataset/builder.rs)
- **Problem:** `FingerprintStore::open()` is called inside `Dataset::build_async()` on every dataset execution.
- **Impact:** Sled is repeatedly opened and closed, which can lock the lockfile and significantly slows down extraction throughput.

### 3. Missing Network Abstractions & Mocks
- **Location:** [fetcher.rs](file:///d:/Scraper/src/engine/fetcher.rs)
- **Problem:** There is no generic `Fetcher` trait; the fetcher is a concrete struct tied directly to `wreq`.
- **Impact:** Impossible to mock HTTP responses or test network error recoveries offline. Integration tests have to rely entirely on loading local HTML files manually.
- **Status: RESOLVED.** The old RPITIT `FetchStrategy` trait (not dyn-compatible) was replaced with an object-safe `Transport` trait returning a `BoxFuture`. `FetchManager` now holds `Arc<dyn Transport>` and gains `FetchManager::with_transport(...)`. A public `MockTransport` (canned per-URL/default responses, call recording, `failing_first(n)` for retry simulation) can be injected via `Session::set_transport(...)`. The dead legacy `Fetcher` struct (~60 duplicated lines) was removed. New offline tests cover the full fetch → retry → parse → extract pipeline with zero network access (`tests/mock_transport_test.rs`, unit tests in `fetcher.rs`).

---

## Medium Severity Issues

### 4. Dead Code: Unused `RequestQueue`
- **Location:** [request_queue.rs](file:///d:/Scraper/src/queue/request_queue.rs)
- **Problem:** The `RequestQueue` struct (an lock-free queue with priority tiers) is fully implemented but never imported or utilized by the `Crawler` or `Dataset` engines.
- **Impact:** Increases binary compile times and codebase noise.

### 5. FFI Layer Violations in Core Structures
- **Location:** [builder.rs](file:///d:/Scraper/src/dataset/builder.rs)
- **Problem:** The core Rust `DatasetField` contains a `transform` field defined as `Option<PyObject>` under `#[cfg(feature = "python")]`.
- **Impact:** Blurs the boundary between core Rust logic and Python's GIL. Node.js bindings cannot utilize this field or have to implement separate workarounds.

### 6. Logic Duplication between Python and Node.js FFI Engines
- **Location:** `sdk/nodejs/native/src/lib.rs` and `src/lib.rs` (PyO3 bindings)
- **Problem:** Structs like `Page`, `Dataset`, `Crawl`, and `Watch` are implemented and wrapped separately for NAPI-RS and PyO3.
- **Impact:** Any change to the core API signature requires updating multiple FFI mapping files manually, leading to diverging SDK features.

### 7. Crawler Extraction is Hardcoded to CSS
- **Location:** [crawler.rs](file:///d:/Scraper/src/crawl/crawler.rs)
- **Problem:** The crawler extraction loop only calls `css::query()`, ignoring the configured `selector_type` parameter.
- **Impact:** Crawler fails if a schema uses XPath, regex, or text anchor selectors.

### 12. Retry Engine Ignored Retryable HTTP Statuses
- **Location:** [fetcher.rs](file:///d:/Scraper/src/engine/fetcher.rs) (`FetchManager::dispatch`)
- **Problem:** The retry loop only fired on transport-level `Err`s. A `429 Too Many Requests` or `5xx` server error is an `Ok(NormalizedResponse)` as far as the `Transport` is concerned, so these responses were returned to the caller on the first attempt with no retry at all.
- **Impact:** Callers hitting rate limits or transient server errors got an immediate failed-looking response instead of the automatic recovery the "retry engine" is supposed to provide.
- **Status: RESOLVED.** Added a `RetryPolicy` (`src/engine/retry.rs`) consulted by `FetchManager::dispatch` on every attempt, not just on `Err`. Defaults to retrying `{429, 500, 502, 503, 504}` with exponential backoff (500ms base, ×2, capped at 30s), honoring a `Retry-After` response header when present in place of the computed backoff. Non-retryable statuses (e.g. `404`) and successes return immediately, as before. Configurable per `FetchManager` via `.with_retry_policy(...)`. Covered by unit tests in `retry.rs` and dispatch-level integration tests in `fetcher.rs` (`manager_retries_503_then_succeeds`, `manager_does_not_retry_404`, `manager_honors_retry_after_header`).

---

## Low Severity Issues

### 8. SDK Feature Discrepancies
- **Location:** `sdk/python/crawlingo/` vs `sdk/nodejs/`
- **Problem:** Node.js exposes `build_structured()` and `extract_structured()` helper methods on its FFI class, whereas Python's SDK handles transformations differently.
- **Impact:** Inconsistent developer experience when switching between languages.

### 9. Version Synchronization
- **Location:** `Cargo.toml`, `sdk/python/pyproject.toml`, `sdk/nodejs/package.json`
- **Problem:** The project version (`0.1.0`) is defined in three separate files.
- **Impact:** High risk of drift. CI checks enforce parity, but updates require manual edits across three package configuration manifests.

### 10. `PyDataset.build_async` Is Not Actually Async
- **Location:** [builder.rs](file:///d:/Scraper/src/dataset/builder.rs) (`PyDataset::build_async`)
- **Problem:** The method blocks on the Tokio runtime and returns an already-computed `PyDatasetResult`, not an awaitable. `await`ing it in Python is misleading.
- **Impact:** False async API surface. A truthful fix needs `pyo3-async-runtimes` (integrate the future with the Python event loop) or the method should be documented/renamed as synchronous.

### 11. PyO3 0.23 Cannot Build Against Python ≥ 3.14
- **Location:** `Cargo.toml` (`pyo3 = "0.23"`)
- **Problem:** PyO3 0.23 supports CPython ≤ 3.13; `cargo build --features python` fails outright on newer interpreters ("configured Python interpreter version (3.14) is newer than PyO3's maximum supported version").
- **Impact:** Blocks local Python-feature builds/tests on modern toolchains. Fix: bump PyO3 to a release supporting 3.14, or set `PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1` as an interim workaround.

### 13. Orphaned Modules Cleaned Up (Duplicate Extraction Engines, Dead DNS Cache, Unwired Schema/Streaming, Unexposed Markdown)
- **Location:** `src/extraction/mod.rs`, `src/dataset/extractor.rs` (removed), `src/engine/dns_cache.rs` (removed), `src/dataset/schema.rs`, `src/dataset/stream.rs`, `src/dataset/export.rs`, `src/parser/document.rs`
- **Problem:** An audit found ~600 lines of code that compiled but had no callers anywhere in the crate: two near-duplicate `ExtractionEngine` implementations (one in `extraction/mod.rs`, a strictly weaker copy in `dataset/extractor.rs`), a fully-implemented `DnsCache` never wired into the fetcher, a `DatasetSchema` validation subsystem, a `DatasetStream`/`compile_stream`/streaming-exporter trio with no producer, and a working `Page::markdown()` never exposed to either SDK.
- **Status: RESOLVED, case-by-case.**
  - **Deleted** `dataset/extractor.rs` (true duplicate of `extraction/mod.rs`'s `ExtractionEngine`, missing `DateTime` support) and `engine/dns_cache.rs` (redundant — `wreq`'s `hickory-dns` feature already resolves and caches DNS; confirmed via `Cargo.toml`). Removed the now-unused direct `hickory-resolver` dependency.
  - **Wired in** `ExtractionEngine`: `DatasetField` gained an `extract_type: ExtractionType` (default `Text`, backward-compatible), used by `Dataset::extract_from_page` and the crawler's inline extraction loop to normalize `Price`/`DateTime`/`NormalizedUrl` values instead of returning raw trimmed text. `PyDataset.field(..., extract_type=...)` exposes it to Python via `ExtractionType::from_str_or_text`.
  - **Wired in** `DatasetSchema`: `Dataset::with_schema(...)` attaches required-field/type-coercion constraints; `extract_from_page` validates and overlays coerced values after extraction, returning a `DatasetError` if a required field is missing or fails to parse (fields outside the schema are left untouched, not dropped).
  - **Wired in** `DatasetStream`/streaming exporters: added `Dataset::build_many_streamed(urls, concurrency)`, which fetches many URLs concurrently and streams each extracted record onto a `DatasetStream` as it completes (each record gets a `"url"` key injected). Added `DatasetStream::write_parquet`, which forwards records into `export::write_parquet_stream`. Deleted the now-redundant `export::write_csv_stream` (superseded by `DatasetStream::write_csv`). Added a test exercising `compile_stream` directly (previously untested).
  - **Found and fixed a latent deadlock while wiring `DatasetStream` in:** the struct bundles both channel ends, so any drain loop relying on `recv()` returning `None` (`write_csv`, `write_parquet`, or a caller's own loop over a `DatasetStream` it holds) would wait forever — the struct itself keeps one `sender` alive for its whole lifetime, so the channel could never report "closed" even after every real producer finished. Never caught before because nothing called these methods. Fixed by making the field `Option`al with a new `DatasetStream::detach_sender()` (called once producers have their own `DatasetStreamHandle`, e.g. in `build_many_streamed`), plus destructuring `self` in `write_csv`/`write_parquet` to explicitly drop their own copy before draining. Regression-tested with a 5s `tokio::time::timeout` guard so a reintroduction fails fast instead of hanging CI.
  - **Wired in** `Page::markdown()`: extracted the DOM-walk into a public `Page::render_markdown(&DomTree)` so bindings that don't hold a full `Page` (e.g. `PyPage`) can call it; exposed as `PyPage.markdown()`. Node.js `JsPage.markdown()` exposure is tracked as a follow-up alongside the rest of the SDK parity work (see item 8).

### 14. Configuration Loading (New Subsystem)
- **Location:** `src/config/mod.rs`, `src/engine/session.rs` (`Session::from_config`), `sdk/python/crawlingo/session.py` (`Session.from_config`)
- **Problem:** The charter's "Configuration" subsystem was entirely absent — all `Session` state had to be set imperatively via chained setters; there was no way to configure a session from a file or environment variables (e.g. for containerized deployments).
- **Status: RESOLVED.** Added `CrawlingoConfig` (TOML/JSON via `.toml`/`.json` extension) with precedence defaults → file → `CRAWLINGO_*` environment variables (`CRAWLINGO_PROXY`, `CRAWLINGO_PROXY_PROVIDER_URL`, `CRAWLINGO_RATE_LIMIT_RPS`, `CRAWLINGO_AUTO_MATCH`, `CRAWLINGO_TIMEOUT_SECONDS`, `CRAWLINGO_FINGERPRINT_PATH`, `CRAWLINGO_FETCHER_TIER`, `CRAWLINGO_BROWSER_PROFILE`); malformed env values are ignored with a `tracing::warn!` rather than failing the whole load. `PoolConfigSpec`/`RetryConfigSpec` mirror `ConnectionPoolConfig`/`RetryPolicy` in a serializable (plain-seconds) shape and convert via `From`. `Session::from_config` applies the loaded config as a session's *initial* state and pre-builds its `FetchManager` with the configured pool/retry settings (previously always hardcoded to `::default()` regardless of what a config system might specify) — any setter called afterward still overrides it, same as on a plain `Session::new()`. Exposed to Python as `Session.from_config(path=None)`. Node.js exposure and a Rust-native CLI consumer are tracked as follow-ups (see items 8 and the Go/CLI roadmap).

### 15. Request/Response Middleware (New Subsystem)
- **Location:** `src/engine/middleware.rs`, `src/engine/fetcher.rs` (`FetchManager::with_middleware`), `src/engine/session.rs` (`Session::add_middleware`/`Session::middleware`)
- **Problem:** The charter lists "Request/response middleware" as a core subsystem; there was no way to add cross-cutting fetch behavior (caching, auth, metrics, logging) without editing `HttpFetcher`/`FetchManager` directly for each concern.
- **Status: RESOLVED (foundation).** Added a `Layer` trait — a `Transport` decorator — and a `MiddlewareStack` that composes an ordered list of layers around a base `Transport`, applied outermost-first in insertion order (mirrors the pattern `MockTransport` injection already established). `FetchManager::with_middleware(&stack)` wraps both tiers; `Session::add_middleware(layer)` queues a layer, applied when the session's `FetchManager` is next built (`fetch_manager()`, `set_transport()`, and `from_config()` all now apply the stack). This is the keystone the response-caching and auth-helper subsystems are meant to ride as `Layer` implementations, rather than each hacking into `HttpFetcher` separately (metrics and caching already do — items 16, 17). **Caveat, consistent with `pool_config`/`retry_policy`:** middleware must be added before the session's first fetch — the manager is built once and cached, so a layer added afterward has no effect. Auth is still open follow-up work.

### 16. Metrics (New Subsystem)
- **Location:** `src/metrics/mod.rs`, `src/engine/session.rs` (`Session::metrics`), `PySession.metrics()`
- **Problem:** The charter lists "Metrics" as a core subsystem; there was no way to observe fetch volume, success/failure rates, per-host/per-status breakdowns, or latency for a session.
- **Status: RESOLVED.** Added `Metrics` (lock-free: atomics + `DashMap`, `Ordering::Relaxed` — counters don't need to synchronize with anything else) and `MetricsLayer`, the first real consumer of the middleware backbone (item 15). Every `Session::new()` seeds a `MetricsLayer` as the *outermost* middleware layer automatically — metrics are always-on, matching rate limiting and retry, not an opt-in the caller has to wire up. Tracks total/per-host/per-status request counts, success vs. failure, bytes received, and average latency; `Session::metrics()` (Rust) / `Session.metrics()` (Python, returns a dict via a JSON round-trip) return a plain-data `MetricsSnapshot`. Note: "a request" is counted per attempt the `Transport` actually receives, including retries — a dispatch that fails twice then succeeds records 3 requests, which is intentional (each retry is a real network attempt). Node.js exposure is tracked as a follow-up alongside the rest of the SDK parity work (item 8).

### 17. HTTP Response Caching (New Subsystem)
- **Location:** `src/engine/cache.rs`, `src/engine/session.rs` (`Session::enable_response_cache`), `PySession.enable_response_cache()`
- **Problem:** The charter lists "Caching" as a core subsystem; there was no way to avoid re-fetching an unchanged page — every `Dataset`/`Crawler` operation hit the network, even for `304`-eligible or long-`max-age` responses.
- **Status: RESOLVED.** Added `CachingLayer` (second consumer of the middleware backbone, item 15) backed by a pluggable `ResponseCache` trait, with an in-memory `moka`-backed `InMemoryCache` implementation (bounded by entry count + a hard TTL safety net independent of any single response's `max-age`). Honors `Cache-Control: no-store` (never cached), `no-cache` (cached but always revalidated, never served without asking first), `max-age` (freshness window, defaulting to a caller-configured TTL when absent), and `ETag`/`Last-Modified` conditional revalidation (`If-None-Match`/`If-Modified-Since`, serving the cached body on a `304`). Every fetch in this crate is a `GET` (`HttpFetcher::execute` hardcodes it), so the cache key is just the URL — no `Vary` handling needed. **Opt-in** (unlike metrics) via `Session::enable_response_cache(max_entries, default_ttl)` / `PySession.enable_response_cache(...)`, since caching trades freshness for fewer requests, which isn't always desirable for a crawler. Same before-first-fetch caveat as other middleware. A disk-backed (`sled`) `ResponseCache` implementation, and Node.js exposure, are tracked as follow-ups.

### 18. Authentication Helpers (New Subsystem)
- **Location:** `src/engine/auth.rs`, `src/engine/session.rs` (`Session::set_auth`), `PySession.basic_auth`/`.bearer_auth`/`.header_auth`/`.api_key_auth`
- **Problem:** The charter lists "Authentication helpers" as a core subsystem; authenticating against a target required manually building the right header/query param on every `Session::headers()`/`.proxy()` call site, with no dedicated, reusable mechanism.
- **Status: RESOLVED.** Added `AuthLayer` (third middleware-backbone consumer, item 15) supporting `AuthScheme::{Basic, Bearer, Header, ApiKeyQuery, Dynamic}`. `Basic` base64-encodes `username:password` (added `base64` as a direct dependency — already transitively present via other deps, per `Cargo.lock`); `ApiKeyQuery` parses and rewrites the request URL via the existing `url` crate to append a query parameter. `Dynamic(Arc<dyn TokenProvider>)` is the escape hatch for schemes whose credential must itself be fetched (e.g. OAuth2 client-credentials): on a `401 Unauthorized` response, the layer calls `TokenProvider::refresh()` and retries **once** with the new token — static schemes (`Basic`/`Bearer`/`Header`/`ApiKeyQuery`) do not retry on `401` since there's nothing to refresh. Exposed to Python as `basic_auth`/`bearer_auth`/`header_auth`/`api_key_auth`; `Dynamic`'s `TokenProvider` trait is Rust-only (no Python callback bridge built). Same before-first-fetch middleware caveat applies.
