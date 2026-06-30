# 05_ROADMAP.md

This document defines the roadmap for transforming Crawlingo into a production-grade, modular web data engine.

---

## Milestone v0.1: Architecture Clean-up & Refactoring
- **Goal:** Fix core connection bottlenecks and clean up dead code.
- **Tasks:**
  1. Remove the unused `src/queue/request_queue.rs`.
  2. Implement a unified `Fetcher` trait and decouple the concrete implementation.
  3. Modify the `Crawler` and `Dataset` builder to accept a shared `Arc<Session>` reference, enabling connection pool and rate-limiter reuse.
  4. Ensure `FingerprintStore::open()` is called once and shared across dataset extractions, rather than opening/closing on every call.
- **Files Affected:**
  - `src/queue/request_queue.rs` (Delete)
  - `src/engine/fetcher.rs` (Refactor)
  - `src/crawl/crawler.rs` (Modify)
  - `src/dataset/builder.rs` (Modify)
  - `src/lib.rs` (Modify)
- **Estimated Complexity:** Medium
- **Risks:** FFI layers may break if lifetime signatures of shared sessions are not correctly exposed.
- **Testing Requirements:** Add unit tests verifying that connections are reused and rate limits are respected across concurrent requests.
- **Documentation Requirements:** Document the new FFI initialization signatures.

---

## Milestone v0.2: Extensible Fetch Strategies
- **Goal:** Support diverse extraction engines (HTTP, Browser rendering, Cache layer) via modular strategies.
- **Tasks:**
  1. Define a `FetchStrategy` trait.
  2. Implement a `BrowserFetcher` strategy (using headless chromium/playwright wrappers) for SPA/JS heavy web pages.
  3. Implement a local `CacheFetcher` strategy to serve cached pages during local development or debugging.
  4. Implement an dynamic strategy selector based on page performance or parsing failures.
- **Files Affected:**
  - `src/engine/fetcher.rs`
  - `src/engine/strategies/` (New folder)
- **Estimated Complexity:** High
- **Risks:** Headless browser dependencies increase build times and introduce potential memory leaks.
- **Testing Requirements:** Integration tests using mock browser instances.
- **Documentation Requirements:** Guide on selecting and configuring fetch strategies.

---

## Milestone v0.3: Unified DOM & Selector API
- **Goal:** Clean up the DOM element manipulation API and normalize behavior across Node.js and Python.
- **Tasks:**
  1. Add `.html()`, `.text()`, `.markdown()`, `.links()`, `.images()`, `.tables()` methods directly to `Page` and `Element`.
  2. Unify selector extraction methods to support CSS, XPath, Regex, and text anchors transparently.
  3. Align Python hooks and Node.js extraction builders.
- **Files Affected:**
  - `src/parser/document.rs`
  - `sdk/python/crawlingo/page.py`
  - `sdk/nodejs/native/src/lib.rs`
- **Estimated Complexity:** Medium
- **Risks:** Breaking changes for existing scripts using older element APIs.
- **Testing Requirements:** Cross-language verification tests.
- **Documentation Requirements:** Complete API reference tables.

---

## Milestone v0.4: Stream-Based Dataset Engine
- **Goal:** Support massive dataset extractions without high memory overhead.
- **Tasks:**
  1. Implement streaming dataset extraction where rows are emitted to Parquet/CSV chunks immediately, rather than collecting all results in memory.
  2. Implement multi-page deduplication filters using Bloom filters.
  3. Add JSON schema validation support for extracted records.
- **Files Affected:**
  - `src/dataset/builder.rs`
  - `src/dataset/export.rs`
- **Estimated Complexity:** High
- **Risks:** Intermittent disk I/O bottlenecks when writing data streams.
- **Testing Requirements:** Benchmark memory usage on 10,000+ pages.
- **Documentation Requirements:** Streaming dataset export guides.

---

## Milestone v1.0: Production Reliability & Observability
- **Goal:** Make Crawlingo stable, safe, observable, and ready for enterprise workloads.
- **Tasks:**
  1. Integrate `tracing` spans and export Prometheus metrics for network duration, parser throughput, and selector failure rates.
  2. Implement graceful shutdown handlers for long-running crawls.
  3. Package and publish final SDK bindings to PyPI and NPM.
- **Files Affected:**
  - `src/lib.rs`
  - `src/crawl/crawler.rs`
- **Estimated Complexity:** Medium
- **Risks:** Trace outputs could slow down parsing speeds if not structured efficiently.
- **Testing Requirements:** Endurance testing under connection throttling.
- **Documentation Requirements:** Production checklist, deployment guides.
