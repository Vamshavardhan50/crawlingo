# 15_AGENT_MEMORY.md

> [!NOTE]
> This is a living context document for AI agent sessions. Do not modify the structure or delete historical gotchas unless they are verified as fixed.

---

## 1. Project Context & Constraints
- **Core Strategy:** Crawlingo is a Rust web scraping library wrapped with PyO3 (Python) and NAPI-RS (Node.js).
- **Core Constraint:** **Do NOT modify any source files (`.rs`, `.ts`, `.js`, `.py`)** during design/planning phases. Documentation and architecture design always come first.
- **FFI Boundary:** Python and Node.js wrapper files duplicate FFI signatures separately. Changes to Rust core method inputs/outputs must be manually mirrored across `src/lib.rs` (PyO3) and `sdk/nodejs/native/src/lib.rs` (NAPI-RS).
- **DOM Architecture:** Uses a flat `Vec<DomNode>` (`DomTree`) with `usize` index markers. Do NOT attempt to build a traditional recursive tree in Rust.

---

## 2. Key Codebase Findings & Gotchas

1. **Dead Code:** `src/queue/request_queue.rs` is implemented but unused. Do not attempt to integrate or modify it without cleaning up the crawler pipeline first.
2. **Rate Limiting Leak:** Crawler worker threads and dataset extractions spawn localized `HostRateLimiter` and `Fetcher` instances inline. Global rate limits are bypassed across threads. They should share a single `Arc<Session>` instance instead.
3. **Database Locks:** Sled DB is opened and closed per dataset extraction, which is a major database bottleneck. Open the database once at the session level and pass it down.
4. **Hardcoded Selectors:** The Crawler's extraction loop calls CSS query paths directly, bypassing XPath, regex, and text anchor selections.
5. **GIL Handling:** When making blocking calls from Python, release the GIL using `py.allow_threads(...)` to prevent blocking Python's async loop.

---

## 3. Active File Registry

- **Rust Core:** `src/lib.rs`, `src/error.rs`, `src/engine/fetcher.rs`, `src/engine/session.rs`, `src/engine/rate_limiter.rs`, `src/parser/document.rs`, `src/parser/streaming.rs`, `src/selector/`, `src/matcher/`, `src/fingerprint/`, `src/dataset/`, `src/crawl/`.
- **Node.js FFI:** `sdk/nodejs/native/src/lib.rs`.
- **Python FFI:** `sdk/python/crawlingo/`.
