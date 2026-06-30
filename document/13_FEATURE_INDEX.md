# 13_FEATURE_INDEX.md

This document lists the implemented and planned features of Crawlingo, mapped to their implementation paths.

---

## Implemented Features

### 1. High-Performance HTML Parser
- **Description:** Fast streaming HTML parser and document tree constructor using `lol_html`.
- **Status:** Production-ready.
- **Paths:** [streaming.rs](file:///d:/Scraper/src/parser/streaming.rs), [document.rs](file:///d:/Scraper/src/parser/document.rs).

### 2. Multi-Selector Matcher Engine
- **Description:** Resolves elements using CSS, XPath, Regex patterns, or SIMD-accelerated text anchor boundaries.
- **Status:** Production-ready.
- **Paths:** [selector/](file:///d:/Scraper/src/selector/).

### 3. Self-Healing Selectors (Auto-Matcher)
- **Description:** Evaluates Jaro-Winkler, Jaccard tag similarity, attribute intersection, and DOM hierarchy depth to repair failed CSS selectors.
- **Status:** Active / Ready.
- **Paths:** [scorer.rs](file:///d:/Scraper/src/matcher/scorer.rs), [auto_matcher.rs](file:///d:/Scraper/src/matcher/auto_matcher.rs).

### 4. DOM Fingerprint Store
- **Description:** Embedded database for archiving node fingerprints, built using the `sled` transactional engine.
- **Status:** Active.
- **Paths:** [store.rs](file:///d:/Scraper/src/fingerprint/store.rs), [dom.rs](file:///d:/Scraper/src/fingerprint/dom.rs).

### 5. Multi-Threaded Crawler
- **Description:** Crawls sites concurrently using worker pools and Tokio `JoinSet` handles.
- **Status:** Active.
- **Paths:** [crawler.rs](file:///d:/Scraper/src/crawl/crawler.rs).

### 6. Tabular Exporter
- **Description:** Serializes data models directly into Apache Parquet, CSV, or JSON formats.
- **Status:** Active.
- **Paths:** [export.rs](file:///d:/Scraper/src/dataset/export.rs).

### 7. CLI & Interactive Shell
- **Description:** Command line interface supporting structured page extractions and preloaded interactive Python shells.
- **Status:** Active.
- **Paths:** [cli.py](file:///d:/Scraper/sdk/python/crawlingo/cli.py).

### 8. Model Context Protocol (MCP) Server
- **Description:** SSE-based JSON-RPC 2.0 interface enabling LLM agents to fetch pages, extract schemas, and crawl websites.
- **Status:** Active.
- **Paths:** [mcp.py](file:///d:/Scraper/sdk/python/crawlingo/mcp.py).

---

## Planned Features

### 9. Headless Browser Fetch Strategy
- **Description:** Playwright or Puppeteer bindings to extract SPAs and dynamic client-side JS-rendered pages.
- **Status:** In Design (v0.2).
- **Paths:** `src/engine/strategies/browser.rs`.

### 10. Streaming Datasets
- **Description:** Write page extraction records immediately to disk streams, avoiding holding thousands of rows in memory.
- **Status:** In Design (v0.4).
- **Paths:** `src/dataset/streaming.rs`.
