# docs/MASTER_ROADMAP.md

This master roadmap defines the target path for transforming Crawlingo into a production-grade web data engine.

---

## Milestone v0.1: Architecture Clean-up & Refactoring
- **Goal:** Resolve core client connection bottlenecks, remove dead code, and optimize database connections.
- **Tasks:**
  1. Delete `src/queue/request_queue.rs`.
  2. Implement `Fetcher` traits to support transport mocking.
  3. Modify the `Crawler` and `Dataset` builder to reuse session connections and rate limiters.
  4. Cache the Sled DB instance at the Session level.
- **Files Affected:**
  - `src/queue/request_queue.rs` (Delete)
  - `src/engine/fetcher.rs`
  - `src/crawl/crawler.rs`
  - `src/dataset/builder.rs`
- **Estimated Complexity:** Medium
- **Risks:** FFI API signatures will change, requiring careful updates in Python and Node.js wrappers.
- **Testing Requirements:** Implement unit tests verifying connection reuse.
- **Documentation Requirements:** Document the new session initialization parameters.

---

## Milestone v0.2: Extensible Fetch Strategies
- **Goal:** Implement the `FetchStrategy` trait and add support for headless browser and local disk caches.
- **Tasks:**
  1. Refactor the `FetchManager` to leverage strategy traits.
  2. Implement `BrowserFetcher` utilizing headless Playwright processes.
  3. Implement `CacheFetcher` for local file reads.
- **Files Affected:**
  - `src/engine/fetcher.rs`
  - `src/engine/strategies/` (New folder)
- **Estimated Complexity:** High
- **Risks:** Managing external headless browser processes securely without memory leaks.
- **Testing Requirements:** Add browser-based integration tests.
- **Documentation Requirements:** Strategy configuration guides.

---

## Milestone v0.3: Unified DOM & Selector API
- **Goal:** Unify node extraction methods and align Python/JS features.
- **Tasks:**
  1. Add utility extraction helpers (text, markdown, links, images, tables) directly to the `Page` struct.
  2. Unify selectors to support XPath, Regex, and text queries.
- **Files Affected:**
  - `src/parser/document.rs`
  - `sdk/python/crawlingo/page.py`
  - `sdk/nodejs/native/src/lib.rs`
- **Estimated Complexity:** Medium
- **Risks:** Introduces breaking changes for existing client scripts.
- **Testing Requirements:** Run cross-language extraction checks.
- **Documentation Requirements:** Update API reference tables.

---

## Milestone v0.4: Stream-Based Dataset Engine
- **Goal:** Build streaming dataset exporters to process large volumes of pages with low memory overhead.
- **Tasks:**
  1. Implement streaming row exports directly to Parquet/CSV files.
  2. Implement Bloom filters for multi-page deduplication.
- **Files Affected:**
  - `src/dataset/builder.rs`
  - `src/dataset/export.rs`
- **Estimated Complexity:** High
- **Risks:** Performance bottlenecks during disk I/O flushes.
- **Testing Requirements:** Run memory benchmarks on 10,000+ pages.
- **Documentation Requirements:** Streaming dataset guides.

---

## Milestone v1.0: Observability & Production Stability
- **Goal:** Make Crawlingo production-ready with metrics, telemetry, and logging instrumentation.
- **Tasks:**
  1. Integrate `tracing` spans and export Prometheus metrics for network performance and selector failure rates.
  2. Implement graceful shutdown handlers for crawls.
- **Files Affected:**
  - `src/lib.rs`
  - `src/crawl/crawler.rs`
- **Estimated Complexity:** Medium
- **Risks:** Performance degradation from logging overhead.
- **Testing Requirements:** Throttling tests under high network loads.
- **Documentation Requirements:** Production deployment checklists.
