# Codebase Map

## Source Tree

```
crawlingo/
├── src/
│   ├── lib.rs           # Public FFI API surface (Session, Page, Dataset, Crawler)
│   ├── error.rs         # Unified error types
│   ├── engine/
│   │   └── mod.rs       # FetchManager, RateLimiter, DnsCache
│   ├── parser/
│   │   └── mod.rs       # HtmlParser (lol_html streaming wrapper)
│   ├── selector/
│   │   └── mod.rs       # CssSelector, XPathSelector, RegexSelector, TextAnchor
│   ├── matcher/
│   │   └── mod.rs       # AutoMatcher, Scorer (Jaro-Winkler, Jaccard)
│   ├── fingerprint/
│   │   └── mod.rs       # FingerprintStore (Sled wrapper)
│   ├── dataset/
│   │   ├── mod.rs       # Dataset, Schema, Extraction
│   │   └── export.rs    # Arrow/Parquet/CSV/JSON export
│   ├── crawl/
│   │   └── mod.rs       # Crawler (multi-threaded page discovery)
│   ├── watch/
│   │   └── mod.rs       # Watcher (periodic change detection)
│   └── change/
│       └── mod.rs       # ChangeDetector (fingerprint comparison)
├── sdk/
│   ├── python/
│   │   ├── crawlingo/
│   │   │   ├── __init__.py
│   │   │   ├── session.py
│   │   │   ├── page.py
│   │   │   ├── dataset.py
│   │   │   ├── crawler.py
│   │   │   └── cli.py
│   │   ├── pyproject.toml
│   │   └── Cargo.toml    # PyO3 bindings
│   └── nodejs/
│       ├── src/
│       │   ├── lib.rs     # NAPI-RS bindings
│       │   └── index.ts   # TypeScript types
│       ├── package.json
│       └── Cargo.toml
├── tests/
│   ├── integration_test.rs
│   └── data/              # Test fixtures
└── benches/
    └── selector_bench.rs
```

## Core Crates (Cargo.toml)

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime (static, initialised once) |
| `wreq` | HTTP client with rustls, JA3 fingerprinting |
| `lol_html` | Streaming HTML parser |
| `sled` | Embedded key-value store (fingerprint DB) |
| `rayon` | Parallel fingerprint scoring |
| `arrow` / `parquet` | Columnar data export |
| `pyo3` | Python FFI bindings |
| `napi` / `napi-derive` | Node.js FFI bindings |
