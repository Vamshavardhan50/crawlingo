# Dependency Audit Report

This document audits the dependencies declared in `Cargo.toml` to identify unused crates, heavy dependencies, and feature optimization opportunities.

---

## 1. Unused Dependencies

The following dependencies are declared in `Cargo.toml` but are never imported or used anywhere in the Rust core codebase:
1. **`crossbeam`** (`0.8`)
   - *Status:* Unused. Formerly utilized by the deleted priority `RequestQueue`.
   - *Action:* Remove from `Cargo.toml`.
2. **`axum`** (`0.7`)
   - *Status:* Unused. No web server exists in the core Rust engine.
   - *Action:* Remove from `Cargo.toml`.
3. **`clap`** (`4`)
   - *Status:* Unused. The core library does not process CLI commands.
   - *Action:* Remove from `Cargo.toml`.
4. **`indicatif`** (`0.17`)
   - *Status:* Unused. No progress bar UI exists in the core library.
   - *Action:* Remove from `Cargo.toml`.
5. **`colored`** (`2.1`)
   - *Status:* Unused. Colorized console printing is not utilized.
   - *Action:* Remove from `Cargo.toml`.
6. **`tokio-stream`** (`0.1`)
   - *Status:* Unused. No asynchronous streaming wrappers are utilized.
   - *Action:* Remove from `Cargo.toml`.
7. **`tower`** (`0.4`)
   - *Status:* Unused. Rate-limiting is handled directly by `governor`.
   - *Action:* Remove from `Cargo.toml`.

---

## 2. Heavy Dependencies & Feature Isolation

1. **`arrow` (`51`) & `parquet` (`51`):**
   - *Impact:* Extremely heavy crates that significantly increase compile time, dependency download overhead, and binary sizes.
   - *Usage:* Only utilized in `src/dataset/export.rs` for Parquet exporting.
   - *Recommendation:* Place these dependencies under an optional `parquet` feature flag:
     ```toml
     [features]
     default = []
     python = ["pyo3"]
     parquet = ["dep:arrow", "dep:parquet"]
     ```
     This allows lightweight uses of Crawlingo to bypass compiling Apache Arrow.

2. **`pyo3` (`0.23`):**
   - *Impact:* Binds to Python development libraries.
   - *Usage:* Correctly isolated under the optional `python` feature flag.

---

## 3. Dependency Summary & Cleanups

Removing the 7 unused dependencies will:
- Reduce target check/build times.
- Decrease compiler memory requirements.
- Shrink the final compiled library binary.
