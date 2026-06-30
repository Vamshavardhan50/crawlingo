# 02_CODEBASE_MAP.md

This document maps every single source file in the Crawlingo project, explaining its exact purpose, its dependencies, and how it fits into the broader architecture.

---

## Core Rust Library (`src/`)

### Core Entry & FFI
- **[lib.rs](file:///d:/Scraper/src/lib.rs)**
  - *Purpose:* Entry point for the Rust crate. Defines module hierarchy, starts the shared global `TOKIO_RUNTIME`, and exposes PyO3 bindings (`_crawlingo_core`) when the `python` feature flag is enabled.
  - *Dependencies:* `once_cell`, `tokio::runtime::Runtime`, `pyo3`.
- **[error.rs](file:///d:/Scraper/src/error.rs)**
  - *Purpose:* Centralizes all error variants into the `CrawlingoError` enum. Uses `thiserror` for clean formatting.
  - *Dependencies:* `thiserror`, `url::ParseError`, `regex::Error`, `bincode::Error`.

### Network & Engine
- **[engine/fetcher.rs](file:///d:/Scraper/src/engine/fetcher.rs)**
  - *Purpose:* Executes HTTP/Stealth network requests using the `wreq` client.
  - *Dependencies:* `wreq`, `tokio`, `url`.
- **[engine/session.rs](file:///d:/Scraper/src/engine/session.rs)**
  - *Purpose:* Implements `Session` struct to manage HTTP states, cookies, proxy rotating pools, user-agent profiles, and authentication headers.
  - *Dependencies:* `std::sync::RwLock`, `std::sync::atomic::AtomicUsize`.
- **[engine/rate_limiter.rs](file:///d:/Scraper/src/engine/rate_limiter.rs)**
  - *Purpose:* Applies host-based rate limiting using the `governor` crate and a `DashMap` cache to track client requests.
  - *Dependencies:* `governor`, `dashmap`.
- **[engine/pool.rs](file:///d:/Scraper/src/engine/pool.rs)**
  - *Purpose:* Contains configuration structures (`ConnectionPoolConfig`) for HTTP connection pooling parameters.
  - *Dependencies:* `serde`.
- **[engine/dns_cache.rs](file:///d:/Scraper/src/engine/dns_cache.rs)**
  - *Purpose:* Wraps `hickory-resolver` with a `moka` cache to optimize network connection times by skipping recurrent DNS calls.
  - *Dependencies:* `hickory-resolver`, `moka`.

### Parsing
- **[parser/document.rs](file:///d:/Scraper/src/parser/document.rs)**
  - *Purpose:* Implements `DomTree` and `DomNode` using a flat `Vec` structure. Exposes `PyElement` and `PyElementCollection` wrappers.
  - *Dependencies:* `std::sync::Arc`, `pyo3`.
- **[parser/streaming.rs](file:///d:/Scraper/src/parser/streaming.rs)**
  - *Purpose:* Uses `lol_html` to read raw bytes and streamingly construct a flat list of `DomNode` indices.
  - *Dependencies:* `lol_html`.

### Selector Matchers
- **[selector/css.rs](file:///d:/Scraper/src/selector/css.rs)**
  - *Purpose:* Evaluates CSS selectors against the `DomTree` using `cssparser` and `selectors` crates. Caches compiled selectors using `DashMap`.
  - *Dependencies:* `cssparser`, `selectors`, `dashmap`.
- **[selector/xpath.rs](file:///d:/Scraper/src/selector/xpath.rs)**
  - *Purpose:* Traversing-based evaluation of raw XPath queries against the flat `DomTree`.
  - *Dependencies:* `cssparser`.
- **[selector/text_anchor.rs](file:///d:/Scraper/src/selector/text_anchor.rs)**
  - *Purpose:* Fast exact/fuzzy search of nodes by content using `memchr` SIMD operations (includes before/after anchoring).
  - *Dependencies:* `memchr`.
- **[selector/regex_selector.rs](file:///d:/Scraper/src/selector/regex_selector.rs)**
  - *Purpose:* Regex queries matching patterns against DOM text nodes. Caches regexes in a `DashMap`.
  - *Dependencies:* `regex`, `dashmap`.

### Crawling, Datasets, Watching, Changes
- **[crawl/crawler.rs](file:///d:/Scraper/src/crawl/crawler.rs)**
  - *Purpose:* Performs multi-threaded web crawling with limits (concurrency, depth, max pages). Spawns tasks via `tokio::task::JoinSet`.
  - *Dependencies:* `tokio`, `dashmap`.
- **[dataset/builder.rs](file:///d:/Scraper/src/dataset/builder.rs)**
  - *Purpose:* Structured tabular dataset extractor from parsed Pages using defined schemas.
  - *Dependencies:* `serde`, `pyo3`.
- **[dataset/export.rs](file:///d:/Scraper/src/dataset/export.rs)**
  - *Purpose:* Export pipelines mapping results to Arrow `RecordBatch` formats, writing CSV, Parquet, and JSON files.
  - *Dependencies:* `arrow`, `parquet`, `csv`, `serde_json`.
- **[change/detector.rs](file:///d:/Scraper/src/change/detector.rs)**
  - *Purpose:* Compares structural or content modifications in pages over time.
  - *Dependencies:* `serde`, `pyo3`.
- **[watch/monitor.rs](file:///d:/Scraper/src/watch/monitor.rs)**
  - *Purpose:* Background polling monitor that triggers events when monitored page changes.
  - *Dependencies:* `tokio::time`, `tokio_util::sync::CancellationToken`.
- **[queue/request_queue.rs](file:///d:/Scraper/src/queue/request_queue.rs)**
  - *Purpose:* Implements a multi-priority (High, Normal, Low) thread-safe queue.
  - *Dependencies:* `crossbeam::queue::SegQueue`.

### Intelligent Selector Healing
- **[fingerprint/dom.rs](file:///d:/Scraper/src/fingerprint/dom.rs)**
  - *Purpose:* Calculates structural fingerprinted path maps of DOM nodes using xxhash64 hashes.
  - *Dependencies:* `xxhash-rust`.
- **[fingerprint/store.rs](file:///d:/Scraper/src/fingerprint/store.rs)**
  - *Purpose:* Embedded DB wrapper for DOM fingerprints using `sled`.
  - *Dependencies:* `sled`, `bincode`.
- **[matcher/scorer.rs](file:///d:/Scraper/src/matcher/scorer.rs)**
  - *Purpose:* Scores the structural similarity of two DOM nodes using weights (Jaro-Winkler for text, Jaccard for attributes, classes, tags, and hierarchy depth).
  - *Dependencies:* `strsim` (Jaro-Winkler).
- **[matcher/auto_matcher.rs](file:///d:/Scraper/src/matcher/auto_matcher.rs)**
  - *Purpose:* Automatic selector healing that runs in parallel via Rayon when a static selector query fails to find elements.
  - *Dependencies:* `rayon`.

---

## Python Wrapper SDK (`sdk/python/`)

- **[crawlingo/exceptions.py](file:///d:/Scraper/sdk/python/crawlingo/exceptions.py)**
  - *Purpose:* Defines Python-side custom exception wrappers and converts core FFI rust errors into Python runtime errors.
- **[crawlingo/__init__.py](file:///d:/Scraper/sdk/python/crawlingo/__init__.py)**
  - *Purpose:* Exposes python classes (`Page`, `Dataset`, `Crawl`, `Session`, `Watch`) and custom exception classes.
- **[crawlingo/page.py](file:///d:/Scraper/sdk/python/crawlingo/page.py)**
  - *Purpose:* Python wrapper for `PyPage` that injects lifecycle hooks (`before_fetch`, `after_fetch`, `before_parse`, `after_extract`).
- **[crawlingo/dataset.py](file:///d:/Scraper/sdk/python/crawlingo/dataset.py)**
  - *Purpose:* Python wrapper for `PyDataset`.
- **[crawlingo/crawl.py](file:///d:/Scraper/sdk/python/crawlingo/crawl.py)**
  - *Purpose:* Python wrapper for `PyCrawl` crawler.
- **[crawlingo/watch.py](file:///d:/Scraper/sdk/python/crawlingo/watch.py)**
  - *Purpose:* Python wrapper for `PyWatch` site monitor.
- **[crawlingo/session.py](file:///d:/Scraper/sdk/python/crawlingo/session.py)**
  - *Purpose:* Context manager wrapper for PySession.
- **[crawlingo/element.py](file:///d:/Scraper/sdk/python/crawlingo/element.py)**
  - *Purpose:* Python wrappers for native `PyElement` and `PyElementCollection`.
- **[crawlingo/hooks.py](file:///d:/Scraper/sdk/python/crawlingo/hooks.py)**
  - *Purpose:* Provides common data transformers (whitespace stripping, casing conversions) and request/response logging callbacks.
- **[crawlingo/cli.py](file:///d:/Scraper/sdk/python/crawlingo/cli.py)**
  - *Purpose:* Command line interface supporting page extraction, interactive python shells, and MCP SSE server commands.
- **[crawlingo/mcp.py](file:///d:/Scraper/sdk/python/crawlingo/mcp.py)**
  - *Purpose:* Model Context Protocol SSE JSON-RPC 2.0 Web server facilitating LLM agent interactions.

---

## Node.js Wrapper SDK (`sdk/nodejs/`)

- **[native/src/lib.rs](file:///d:/Scraper/sdk/nodejs/native/src/lib.rs)**
  - *Purpose:* Defines NAPI-RS bindings for Node.js. Duplicates many of the struct definitions from Python's PyO3 files to map Rust struct lifetimes to JavaScript engines.
  - *Dependencies:* `napi`, `napi-derive`.
