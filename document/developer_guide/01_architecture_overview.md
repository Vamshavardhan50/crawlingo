# Architecture Overview

Crawlingo is a multi-language web scraping and data extraction engine with a Rust core and Python / Node.js SDKs.

## High-Level Architecture

```
User Application (Python / Node.js)
        │
        ▼
  ┌─────────────────────┐
  │    SDK Wrappers      │  PyO3 (Python) / NAPI-RS (Node.js)
  │ (Session, Page,      │
  │  Dataset, Crawler)   │
  └─────────┬───────────┘
            │ FFI Boundary
            ▼
  ┌─────────────────────┐
  │    Rust Core         │
  │                      │
  │  ┌────────────────┐  │
  │  │  Fetch Manager  │  │  HTTP, Browser, Cache strategies
  │  │  Rate Limiter   │  │  Token-bucket per-host limiter
  │  │  DNS Cache      │  │  Thread-safe DNS resolver cache
  │  └────────────────┘  │
  │         │            │
  │         ▼            │
  │  ┌────────────────┐  │
  │  │  Parser         │  │  lol_html streaming HTML parser
  │  │  DomTree        │  │  Flat Vec<DomNode> representation
  │  └────────────────┘  │
  │         │            │
  │         ▼            │
  │  ┌────────────────┐  │
  │  │  Selector       │  │  CSS, XPath, Regex, Text Anchor
  │  │  Engine         │  │  SIMD memchr, DashMap caches
  │  └────────────────┘  │
  │         │            │
  │         ▼            │
  │  ┌────────────────┐  │
  │  │  AutoMatcher    │  │  Rayon-parallel fingerprint scoring
  │  │  FingerprintDB  │  │  Sled embedded key-value store
  │  └────────────────┘  │
  │         │            │
  │         ▼            │
  │  ┌────────────────┐  │
  │  │  Dataset        │  │  Schema extraction, Arrow/Parquet export
  │  │  Exporter       │  │  CSV, JSON, Parquet serialisation
  │  └────────────────┘  │
  │         │            │
  │         ▼            │
  │  ┌────────────────┐  │
  │  │  Crawler        │  │  Multi-threaded page discovery
  │  │  Watcher        │  │  Periodic change detection
  │  └────────────────┘  │
  └─────────────────────┘
```

## Language Stack

| Layer | Language | Key Crates / Tools |
|-------|----------|-------------------|
| Core | Rust | tokio, wreq, lol_html, sled, rayon, arrow |
| Python SDK | Python | maturin, PyO3 |
| Node.js SDK | TypeScript | napi-rs |
| CLI | Python | argparse |

## Key Design Decisions

- **Flat DomTree** (`Vec<DomNode>`) over pointer-based DOM for cache locality and clean FFI.
- **Streaming parser** (lol_html) for zero-copy HTML processing.
- **Sled embedded database** for lock-free, transactional fingerprint storage.
- **Rayon** for parallel fingerprint scoring across CPU cores.
- **PyO3 / NAPI-RS** for native FFI bindings (avoids HTTP IPC overhead).

## Module Dependency Rules

- `DomTree` must not depend on `Fetcher`.
- `AutoMatcher` must not depend on FFI wrappers.
- `Watcher` must not depend on the database layer directly.

## See Also

- [Codebase Map](../developer_guide/03_codebase_map.md): File-by-file source guide.
- [Design Decisions](../developer_guide/07_design_decisions.md): Detailed rationale for architectural choices.
