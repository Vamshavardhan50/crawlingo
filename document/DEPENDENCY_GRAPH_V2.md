# docs/DEPENDENCY_GRAPH_V2.md

This document maps out the clean, circular-free dependency graph of the redesigned Crawlingo V2 core engine.

---

## 1. Clean Core Dependency Graph

```
                   +---------------------------------------+
                   |           FFI / Wrapper Layer         |
                   |   (pyo3 Python / napi-rs Node.js)     |
                   +---------------------------------------+
                                       |
                                       v
                   +---------------------------------------+
                   |          Public API Interface         |
                   |             (Page Struct)             |
                   +---------------------------------------+
                    /                  |                  \
                   /                   v                   \
                  /         +--------------------+          \
                 /          |    Parser Layer    |           \
                /           | (HtmlParser / DOM) |            \
               v            +--------------------+             v
    +--------------------+             |            +--------------------+
    |   Fetch Manager    |             |            |  Selector Engine   |
    | (FetchStrategies)  |             v            | (CSS/XPath/Regex)  |
    +--------------------+    +------------------+  +--------------------+
               |              |    Extraction    |             |
               |              |      Engine      |             |
               v              +------------------+             v
    +--------------------+             |            +--------------------+
    |  Network Client    |             v            |   Dataset Engine   |
    | (wreq / reqwest)   |    +------------------+  | (Schema/Validation)|
    +--------------------+    |  Tabular Records |  +--------------------+
                              +------------------+             |
                                                               v
                                                    +--------------------+
                                                    |  Export / Exporter |
                                                    | (CSV/JSON/Parquet) |
                                                    +--------------------+
```

---

## 2. Dependency Hierarchy Rules

To maintain strict modularity and high testability, the dependency hierarchy must flow in a single direction:

1. **FFI Wrappers** sit at the outer perimeter. Core files must **never** reference Python-specific `PyObject` or Node-specific `JsValue` structures.
2. **Page** is the core data transfer exchange. All query and extraction steps depend on `Page`.
3. **Fetch Manager** depends only on `FetchStrategy` traits. Concrete network client crates (`wreq`, `hickory-resolver`, `governor`) are hidden behind strategies.
4. **Parser** depends only on raw HTTP byte responses (`NormalizedResponse`) and compiles the internal DOM.
5. **Selector Engine** depends only on `DomTree` structural node layers.
6. **Extraction Engine** depends on `Page` and `SelectorEngine` to query and parse local fields.
7. **Dataset Engine** collects records emitted by the Extraction Engine and passes them to target file exporters.

---

## 3. Shared Utilities Layer

Cross-cutting helper modules reside in a separate base utility tier that has no upstream dependencies:
- **`error`:** Exposes the unified enum-based `CrawlingoError`.
- **`session`:** Holds shared long-lived variables (proxy lists, rate limit governors, Sled fingerprint connection pools).
- **`rate_limiter`:** Implements host-level pacing via `governor`.
