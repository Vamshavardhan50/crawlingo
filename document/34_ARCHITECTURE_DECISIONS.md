# document/34_ARCHITECTURE_DECISIONS.md

This document records the architectural design decisions made during the development of Crawlingo, evaluating their effectiveness.

---

## 1. Rationale Behind Core Architecture Choices

### Why Rust?
- **Performance:** Scrapers process massive volumes of HTML data. Rust provides low-level speed, SIMD acceleration, and efficient memory management.
- **Safety:** Eliminates data races and memory corruption bugs in multi-threaded crawling environments.

### Why Async (Tokio)?
- **Concurrency:** Scrapers spend most of their time waiting on network responses. Async tasks allow a single thread pool to manage thousands of concurrent connections efficiently.

### Why the contiguous Flat `DomTree`?
- **Cache Locality:** contiguous vectors fit efficiently inside CPU cache lines.
- **FFI Sharing:** The DOM tree is shared with Python and Node.js wrappers as a list of `usize` index offsets, avoiding memory copies across FFI boundaries.

### Why `lol_html` Parser?
- **Streaming:** Ingests raw bytes streamingly as they arrive, avoiding fully reading the HTML text string before parsing.
- **Low Overhead:** Fast and memory-efficient.

### Why `wreq` Fetcher?
- **Stealth compliance:** Implements TLS JA3 fingerprints and user-agent emulation profiles to mimic browser request styles.

---

## 2. Evaluation: Good Choices vs. Recommended Changes

### Architectural Decisions to Retain
1. **Contiguous DOM Tree vector:** A brilliant design choice that keeps traversals cache-friendly and FFI boundaries simple.
2. **Streaming parser (`lol_html`):** The streaming tokenizer ensures low memory overhead during raw page ingestion.
3. **Maturin/PyO3 and NAPI-RS wrappers:** Native binary extensions provide direct memory access, avoiding inter-process overhead.

### Architectural Decisions to Change
1. **Concrete Fetcher implementation:** Decouple the fetcher from the concrete `wreq` client. Introduce a unified `FetchStrategy` trait to support browser rendering, local caches, and mock transports for offline testing.
2. **Localized client initialization:** Stop instantiating new `wreq::Client` and `HostRateLimiter` instances inside the `fetch()` loop. Initialize them once at the Session level to reuse connection pools and enforce global rate limits.
3. **FFI types in core Rust structures:** Remove Python-specific GIL fields (such as `Option<PyObject>`) from the core Rust structures inside `builder.rs` to maintain clean library boundaries.
