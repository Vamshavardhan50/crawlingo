# 14_CHANGELOG_INTERNAL.md

This document records the internal release log and history for the Crawlingo project.

---

## [0.1.0] - 2026-06-26

### Added
- **Core Rust Engine:**
  - Fast streaming HTML parser based on `lol_html`.
  - Contiguous flat vector structure (`Vec<DomNode>`) for cache-friendly DOM representations.
  - Multi-query selectors (CSS, XPath, Regex, and SIMD text search).
  - Parallel auto-healing match scoring using `Rayon`.
  - Embedded `Sled` fingerprint database.
  - Arrow and Parquet export modules.
- **Python SDK:**
  - Maturin PyO3 integration bindings.
  - Custom before/after fetch, parse, and extraction pipeline hooks.
  - Development CLI with extract commands and interactive shell wrappers.
  - SSE Model Context Protocol server.
- **Node.js SDK:**
  - NAPI-RS bindings for Node environments.
  - Native `.node` builds with asynchronous JS tasks.
- **CI/CD Pipelines:**
  - Multi-platform compiler testing actions.
  - Automatic release packing workflows publishing wheels and tarballs to NPM and PyPI.
