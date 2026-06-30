# document/35_ENGINE_SCORECARD.md

This document presents a brutally honest scorecard of Crawlingo's current codebase, with each rating backed by direct evidence from the source files.

---

## 1. Engine Scorecard Summary

| Category | Score | Primary Codebase Evidence |
| :--- | :--- | :--- |
| **Architecture** | `5 / 10` | The `Fetcher` and `HostRateLimiter` are instantiated inline on every crawler worker loop and dataset call, bypassing connection pooling and rate limits. |
| **Maintainability** | `5 / 10` | The Node.js bindings (`sdk/nodejs/native/src/lib.rs`) duplicate the structures and functions mapped in the Python PyO3 modules, making API updates tedious. |
| **Readability** | `8 / 10` | Consistent formatting using standard Rust syntax. Module boundaries are clearly defined. |
| **Performance** | `7 / 10` | Uses `lol_html` for streaming parsing, `memchr` SIMD for text search, and `Rayon` for parallel similarity matching. However, lack of client reuse degrades throughput. |
| **Scalability** | `4 / 10` | Large dataset extractions collect all records in memory before writing to disk, leading to high memory pressure. |
| **API Design** | `6 / 10` | The `Page` API is clean, but FFI structures are inconsistent between Python and Node.js wrapper classes. |
| **Testing** | `4 / 10` | The integration test suite (`tests/integration_test.rs`) relies entirely on loading local HTML files. There are no mocks or live network tests. |
| **CI/CD** | `9 / 10` | Comprehensive GitHub Actions workflow building native modules and publishing wheels/tarballs across Windows, macOS, and Linux runners. |
| **Documentation** | `3 / 10` | Lacked internal design blueprints or developer manuals until the current documentation overhaul. |
| **Release Process** | `9 / 10` | Automatic version checks and multi-platform packaging workflows triggered on Git tags. |
| **SDK Quality** | `6 / 10` | Wrapper classes provide a clean developer experience, but FFI boundaries are tightly coupled with the core library. |
| **Production Readiness** | `4 / 10` | Lacks metrics, observability logging, or tracing spans. Opening Sled DB per call causes database locks under concurrent tasks. |
| **Open Source Readiness**| `6 / 10` | Clean code and targets, but lacks detailed contribution guidelines or architecture blueprints. |

---

## 2. Evidence-Based Evaluations

### Architecture (`5 / 10`)
- **Evidence:** Lines 87-88 of `src/dataset/builder.rs` initialize a new fetcher inline:
  ```rust
  let rate_limiter = Arc::new(crate::engine::rate_limiter::HostRateLimiter::new());
  let fetcher = Fetcher::new(rate_limiter, ConnectionPoolConfig::default());
  ```
  Additionally, `Fetcher::fetch` instantiates a new `Client::builder()` on every call. This violates connection reuse.

### Maintainability (`5 / 10`)
- **Evidence:** `sdk/nodejs/native/src/lib.rs` contains 800+ lines duplicating FFI definitions and struct mapping setups that are already defined in PyO3 modules. This makes API modifications tedious.

### Testing (`4 / 10`)
- **Evidence:** The integration test suite (`tests/integration_test.rs`) uses static offline HTML data:
  ```rust
  let html = b"<html>...</html>";
  ```
  There are no mock fetcher strategies or tests verifying network error recovery or concurrent connection loads.

### Production Readiness (`4 / 10`)
- **Evidence:** `FingerprintStore::open()` is called inside `Dataset::build_async()` on every dataset extraction call. Sled database locks are exclusive, so concurrent calls lead to file lock race conditions and database bottlenecks.
