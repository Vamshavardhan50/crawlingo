# document/30_DIRECTORY_GUIDE.md

This document maps all directories and key files in Crawlingo, evaluating their location and recommending restructuring where necessary.

---

## 1. Directory Structure Map

```
.
├── src/                      # Core Rust Engine
│   ├── change/               # Structural Change Detection
│   ├── crawl/                # Parallel Crawler Engines
│   ├── dataset/              # Tabular extraction & file serialization
│   ├── engine/               # Network Transport (HTTP, Rate Limiters, Sessions)
│   ├── fingerprint/          # Fingerprinting and Sled DB Storage
│   ├── matcher/              # Similarity scoring selector auto-healing
│   ├── parser/               # streaming HTML parsing (lol_html)
│   ├── selector/             # CSS, XPath, Regex, SIMD search engines
│   └── watch/                # Periodic site polling monitors
├── sdk/                      # Multi-Language FFI wrappers
│   ├── nodejs/               # Node.js NAPI-RS packages
│   └── python/               # Python maturin/pip packages
├── tests/                    # Integration & Edge-Case test runs
├── benches/                  # Criterion benchmarks
└── docs/                     # Next.js developer documentation
```

---

## 2. Important File Analysis & Cleanups

### A. Core Rust files (`src/`)
- **`src/lib.rs`**
  - *Responsibilities:* Configures crate submodules and PyO3 classes.
  - *Recommendation:* **Should be split.** The FFI binding classes should be moved to `src/ffi/python.rs` to keep the core library entry point clean.
- **`src/dataset/builder.rs`**
  - *Responsibilities:* Compiles rows and handles selector auto-healing.
  - *Recommendation:* **Should be split.** The FFI wrapper classes (`PyDataset`, `PyDatasetResult`) should be moved to the FFI layer.

### B. Python SDK files (`sdk/python/`)
- **`sdk/python/crawlingo/page.py`**
  - *Responsibilities:* Python `Page` interface and hook lifecycles.
  - *Recommendation:* Keep. It provides a clean interface for Python developers.
- **`sdk/python/crawlingo/exceptions.py`**
  - *Responsibilities:* Exception definitions and Rust runtime error conversions.
  - *Recommendation:* Keep. It separates error handling logic from the FFI wrapper classes.

### C. Node.js SDK files (`sdk/nodejs/`)
- **`sdk/nodejs/native/src/lib.rs`**
  - *Responsibilities:* NAPI-RS native bindings.
  - *Recommendation:* **Should be simplified.** It duplicates many of the FFI wrappers from the PyO3 module. The wrappers should be refactored to call shared library helper methods directly, minimizing code duplication.
