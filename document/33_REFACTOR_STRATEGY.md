# document/33_REFACTOR_STRATEGY.md

This document outlines the refactoring strategy for Crawlingo, prioritizing tasks to resolve the client connection bottlenecks, database locks, and FFI coupling issues identified in the audit.

---

## 1. Refactor Priorities Table

| Priority | Task Name | Affected Files | Estimated Effort | Risk Level | Dependencies |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Critical** | Reconstruct client & rate-limiter lifecycles | `crawler.rs`, `builder.rs`, `fetcher.rs` | 2 days | High | `Fetcher` trait |
| **Critical** | Cache Sled database connection in Session | `builder.rs`, `session.rs` | 1 day | Medium | Sled setup |
| **High** | Introduce dynamic `FetchStrategy` traits | `fetcher.rs`, `lib.rs` | 3 days | High | None |
| **Medium** | Decouple Python transform GIL fields | `builder.rs`, `sdk/python/crawlingo/dataset.py` | 1 day | Medium | Python SDK |
| **Medium** | Delete unused dead code `request_queue.rs` | `request_queue.rs` (Delete), `lib.rs` | 0.5 days | Low | None |
| **Low** | Separate FFI wrappers into dedicated folder | `lib.rs`, `sdk/nodejs/native/src/lib.rs` | 2 days | Low | None |

---

## 2. In-Depth Strategy Explanations

### Critical: Reconstruct Client & Rate-Limiter Lifecycles
- **Why:** Every fetch request inline-instantiates a new `wreq::Client` and `HostRateLimiter` inside the `fetch()` method. This destroys connection reuse, bypasses connection pooling, and defeats host rate-limit enforcement across concurrent worker threads.
- **Action:**
  1. Define a shared `Session` reference.
  2. Instantiate the `wreq::Client` and `HostRateLimiter` once at the Session level.
  3. Pass the shared `Arc<Session>` reference down to the Fetcher, Crawler, and Dataset builders.
- **Risk:** High. Changing constructor signatures will require updating FFI mappings in both Python and Node.js SDKs.

### Critical: Cache Sled Database Connection in Session
- **Why:** `FingerprintStore::open()` is called inside `Dataset::build_async()` on every dataset extraction call. Sled database locks are exclusive, so concurrent calls lead to file lock race conditions and database bottlenecks.
- **Action:** Initialize the Sled database connection once at the Session level and pass the shared connection wrapper down to the dataset builder.
- **Risk:** Medium. Requires updating the `Session` struct initialization code.

### High: Introduce Dynamic `FetchStrategy` Traits
- **Why:** The fetcher is currently a concrete struct tied directly to `wreq`, making it impossible to mock network transport for offline testing.
- **Action:** Define a `FetchStrategy` trait and implement `HttpFetcher`, `BrowserFetcher`, and `CacheFetcher` as modular strategies.
- **Risk:** High. Modifies the core transport execution pipeline.
