# 04_TECHNICAL_DEBT.md

This document catalogues the current technical debt in the Crawlingo codebase, ranked by severity.

---

## High Severity Issues

### 1. Inefficient HTTP Client & Rate Limiter Lifecycles
- **Location:** [crawler.rs](file:///d:/Scraper/src/crawl/crawler.rs), [builder.rs](file:///d:/Scraper/src/dataset/builder.rs)
- **Problem:** Every crawler worker loop and every call to `Dataset::build_async` constructs a new `Fetcher` and a new `HostRateLimiter` instance rather than reusing a shared instance from the `Session`.
- **Impact:** Empties the HTTP connection pool, bypasses connection reuse, and defeats rate-limit enforcement across concurrent worker threads.

### 2. Sled Database Connection Bottleneck
- **Location:** [builder.rs](file:///d:/Scraper/src/dataset/builder.rs)
- **Problem:** `FingerprintStore::open()` is called inside `Dataset::build_async()` on every dataset execution.
- **Impact:** Sled is repeatedly opened and closed, which can lock the lockfile and significantly slows down extraction throughput.

### 3. Missing Network Abstractions & Mocks
- **Location:** [fetcher.rs](file:///d:/Scraper/src/engine/fetcher.rs)
- **Problem:** There is no generic `Fetcher` trait; the fetcher is a concrete struct tied directly to `wreq`.
- **Impact:** Impossible to mock HTTP responses or test network error recoveries offline. Integration tests have to rely entirely on loading local HTML files manually.

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
