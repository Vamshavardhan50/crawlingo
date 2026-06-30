# docs/CORE_REFACTOR_ANALYSIS.md

This document presents a comprehensive architectural audit and dependency coupling review of Crawlingo's current core engine, highlighting responsibilities, coupling, ownership, and circular design patterns.

---

## 1. Module-by-Module Audit

### A. Fetch Module
- **Location:** `src/engine/fetcher.rs`, `src/engine/session.rs`, `src/engine/pool.rs`, `src/engine/rate_limiter.rs`, `src/engine/dns_cache.rs`
- **Current Responsibilities:**
  - `Fetcher` constructs `wreq` clients on the fly, applies rate-limiting pacing based on the target host, rotates proxies, configures stealth/browser impersonation, and performs exponential backoff retries.
  - `Session` manages global state headers, cookies, proxies, and rate limit settings.
- **Coupling & Architectural Flaws:**
  - **High Transport Coupling:** `Fetcher` is hard-coupled to the `wreq` client and returns a concrete `wreq::Response` object. There is no trait or abstraction boundary, making it impossible to mock the transport layer for offline unit testing.
  - **Hardcoded Stealth Options:** Standard vs. Stealthy modes are hardcoded in a `FetcherTier` enum. Impersonation settings are resolved inside the fetch function using structural matches rather than polymorphism.
  - **Session Mutability & Cloning:** Session state is wrapped in `RwLock` and cloned frequently. Settings like rate-limit RPS are retrieved and passed as manual parameters rather than standardizing requests.

### B. Parser Module
- **Location:** `src/parser/streaming.rs`, `src/parser/document.rs`
- **Current Responsibilities:**
  - Tokenizes raw HTML bytes via `lol_html`.
  - Builds a flat, index-routed `DomTree` (`Vec<DomNode>`) allocating parent-child indices to avoid reference cycles.
- **Coupling & Architectural Flaws:**
  - **Lack of Input Isolation:** `parse_html` takes a byte slice (`&[u8]`) and directly constructs the DOM. However, it lacks parsing of HTTP headers or metadata (like charset encodings or content types), which can lead to malformed text outputs when encodings are not UTF-8.
  - **Mixed FFI Types:** `document.rs` contains PyO3 wrapper classes (`PyElement` and `PyElementCollection`) conditional on `#[cfg(feature = "python")]` mixed directly alongside the core Rust `DomTree` struct.

### C. Selector Module
- **Location:** `src/selector/css.rs`, `src/selector/xpath.rs`, `src/selector/text_anchor.rs`, `src/selector/regex_selector.rs`
- **Current Responsibilities:**
  - Queries nodes inside a `DomTree` and returns flat lists of matching `usize` indices.
- **Coupling & Architectural Flaws:**
  - **Direct Node Exposure:** Selectors rely directly on public fields of `DomNode` (like `tag`, `text`, `children`, and `parent`). There are no helper getters on `DomTree` to isolate traversal. If the internal layout of `DomNode` changes, all selectors break.
  - **Lack of Interface Unification:** Each selector type is implemented as a standalone query function (`css::query`, `xpath::query`, etc.). There is no unified trait interface for selectors.

### D. Extractor Module
- **Location:** *Non-existent as a standalone module.* Currently embedded directly inside the `Dataset` builder (`src/dataset/builder.rs`).
- **Current Responsibilities:**
  - Extracts text from selector matches and falls back to a default value if empty.
- **Coupling & Architectural Flaws:**
  - **Tight Integration:** Extraction is tightly coupled to the dataset building loop. You cannot extract structured text from a DOM tree without building a dataset, loading a proxy, or performing a fetch.
  - **Inline Transform Callbacks:** Python-specific transform callbacks (`transform: Option<PyObject>`) are stored inside the core `DatasetField` struct and invoked inside `PyDataset::build` using the GIL. This directly leaks Python types into the Rust core engine.

### E. Dataset Module
- **Location:** `src/dataset/builder.rs`, `src/dataset/export.rs`
- **Current Responsibilities:**
  - Runs the fetch-parse-extract loop for a target URL and exports findings to CSV/JSON/Parquet.
- **Coupling & Architectural Flaws:**
  - **God Object Anti-Pattern:** The `Dataset` struct violates the Single Responsibility Principle. It initiates the HTTP request, invokes the parser, opens the Sled fingerprint store, runs the extraction selectors, and manages data exports.
  - **Temporary Database Handles:** Opens and closes the Sled fingerprint database (`FingerprintStore::open`) on every single dataset build invocation. This results in heavy I/O overhead and potential locking errors.

### F. Watch Module
- **Location:** `src/watch/monitor.rs`, `src/change/detector.rs`
- **Current Responsibilities:**
  - Polls a target URL at regular intervals, constructs dataset fields, compares old vs. new data in parallel, and runs callbacks.
- **Coupling & Architectural Flaws:**
  - **GIL Blocking:** The Rust watch monitor manages Python callback objects (`PyObject`) and executes them synchronously. If a Python callback hangs or blocks, the entire async tokio monitor block can stall.
  - **Poller-Dataset Coupling:** The watcher must instantiate and run a complete `Dataset` loop on every tick, inheriting all of its network, parser, and database coupling.

---

## 2. Dependency & Ownership Analysis

### A. Duplicated Logic
- **Request Setup:** Header assembly, cookie processing, and proxy configuration are repeated across `Fetcher`, `Dataset::build_async`, and `Crawler::crawl_async`.
- **Text Aggregation:** Constructing a space-separated string from a list of matched element indices is duplicated in both `crawler.rs` and `builder.rs`.

### B. Incorrect Ownership
- **Network Handles:** Localized HTTP clients and connection pools are created dynamically inside loops rather than being shared globally from a single long-lived Session container.
- **FFI Type Leaks:** Core structs contain `PyObject` fields, forcing the engine to compile conditional on python flags and exposing raw FFI types to the library core.

### C. Circular Dependencies
- No direct Rust compiler-level circular imports exist (which would fail compilation), but there is a heavy conceptual loop:
  - `Dataset` depends on `Session` -> `Session` defines FFI wrappers -> FFI wrappers return `DatasetResult` -> `DatasetResult` depends on export formats.
  - `Crawler` depends on `DatasetField` -> `DatasetField` depends on Python bindings -> Python bindings wrap `Crawler`.
