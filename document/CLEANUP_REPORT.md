# Crawlingo Codebase Cleanup Report

This report summarizes the actions taken and structural audits performed during the codebase cleanup of Crawlingo. The objective was to clean, simplify, and standardize the codebase while preserving all runtime behavior and public API contracts.

---

## 1. Phase 1: Repository Cleanup (Completed)
- **Unused Modules Removed:** Deleted the abandoned `src/queue/` directory containing the lock-free priority `RequestQueue`. Removed the corresponding `pub mod queue;` declaration from `src/lib.rs`.
- **Redundant Documentation Removed:** Deleted duplicate documentation `document/DATASET_ENGINE.md` (which is already properly tracked in `document/28_DATASET_ENGINE.md` and `docs/content/docs/api-reference.mdx`).
- **Scratch & Generated Files Cleaned:** Deleted temporary testing output files (e.g. `waitlist.csv`, `test-crawlingo-install/*.js`, `test-crawlingo-install/*.csv`, `test-crawlingo-install/*.json`) and added these patterns to `.gitignore`.
- **Documentation:** Created [CODEBASE_CLEANUP.md](file:///d:/Scraper/docs/CODEBASE_CLEANUP.md) detailing candidates.

---

## 2. Phase 2: Rust Source Cleanup (Completed)
- **Verification:** Ran `cargo test --workspace` confirming that all 17 integration and unit tests pass successfully.
- **Linting & Quality Check:** Ran `cargo clippy --workspace --all-targets -- -D warnings` which passed with zero warnings.
- **Code Style Alignment:** Ran `cargo fmt --all -- --check` which confirmed all Rust files adhere to standard formatting.

---

## 3. Phase 3: Module Cleanup & Structural Suggestions
- **FFI Separation Suggestion:**
  - *Current Status:* Python FFI bindings (`PyPage`, `PyCrawl`, `PyDataset`, etc.) are currently defined inside the core `crawlingo` crate using `#[cfg(feature = "python")]` and `pyo3` attributes. Conversely, the Node.js FFI bindings are isolated in a separate crate `sdk/nodejs/native` (`crawlingo-native`).
  - *Recommendation:* Split the Python bindings out of the core `crawlingo` crate into a separate crate (e.g. `sdk/python/native`), identical to the Node.js wrapper layout. This keeps the core library clean of PyO3 dependencies, simplifies the cargo feature configuration, and speeds up core compilation times.

---

## 4. Phase 4: Public API Cleanup Evaluation
- **Visibility Constraints:**
  - *Current Status:* The internal DOM representations `DomNode` and `DomTree` expose all of their struct fields as public (`pub`). While this is necessary because traversal modules (selectors, auto-matchers, and scorers) span multiple distinct subcrates/modules, it exposes internal memory representations (like flat-vector indices) to Rust API consumers.
  - *Recommendation:* Convert internal fields of `DomNode` (such as `parent`, `children`, and `index`) to `pub(crate)` to restrict access to the crawlingo workspace only. External Rust API consumers should query elements via `Page` and public query APIs, preventing downstream code from relying on private indexing details.

---

## 5. Phase 5: Dependency Cleanup (Completed)
- **Unused Dependencies Removed:** Removed 7 unused crates from the root `Cargo.toml`:
  1. `crossbeam` (previously used by deleted `RequestQueue`)
  2. `axum` (no HTTP/SSE server running in core engine)
  3. `clap` (CLI arguments parsed directly by SDK/Python wrappers)
  4. `indicatif` (no progress bars in core engine)
  5. `colored` (no colorized terminal logging in core engine)
  6. `tokio-stream` (async streams are not used in core)
  7. `tower` (rate limiting is handled natively via `governor`)
- **Documentation:** Created [DEPENDENCY_AUDIT.md](file:///d:/Scraper/docs/DEPENDENCY_AUDIT.md).

---

## 6. Phase 6: SDK Cleanup
- **CLI & MCP Audit:** Inspected `sdk/python/crawlingo/cli.py` and `sdk/python/crawlingo/mcp.py`. Verified that:
  - `cli.py` is configured as a `project.scripts` entry point `crawlingo = "crawlingo.cli:main"` in `pyproject.toml`.
  - `mcp.py` is dynamically imported and utilized by the `crawlingo mcp` command.
  - No dead or obsolete SDK helpers are present.
- **FFI Verification:** Verified that FFI contracts remain completely unchanged, ensuring pre-compiled native modules and bindings remain fully compatible.

---

## 7. Phase 7: Documentation Cleanup (Completed)
- **Broken Path Fixes:**
  - Fixed 15 broken paths in [MASTER_INDEX.md](file:///d:/Scraper/docs/MASTER_INDEX.md) where reverse-engineered design documents (21-35) were erroneously linked to `docs/` instead of the correct `document/` folder.
  - Cleaned up path-based headers in all 15 operational guides in the `document/` directory (from `document/21_EXECUTION_FLOW.md` to `document/35_ENGINE_SCORECARD.md`) to point to `document/` instead of `docs/`.

---

## 8. Remaining Technical Debt
1. **Arrow & Parquet Heavyweight Feature Flag:**
   - *Issue:* The `arrow` and `parquet` crates are extremely heavy dependencies that increase build times and compile overhead, but are only used inside `src/dataset/export.rs` for Parquet exporting.
   - *Action:* Isolate these under a cargo feature flag (e.g. `parquet = ["dep:arrow", "dep:parquet"]`) in the next release to allow lighter builds of the engine.
