# 17_CODE_STYLE.md

This document defines the coding standards, style guidelines, and linter configurations for Crawlingo.

---

## 1. Rust Styling & Lints

- **Format Tool:** `rustfmt`. Files must be formatted using the rules in the Rust 2021 edition.
- **Linter Tool:** `clippy`.
- **Command:**
  ```bash
  cargo fmt --all -- --check
  cargo clippy --all-targets --all-features -- -D warnings
  ```
- **Rules:**
  - Avoid using `unwrap()` in production-ready files. Always propagate errors or provide explicit context with `expect()`.
  - Use `tracing::instrument` to trace complex function boundaries.
  - Document all public struct fields and functions using docstrings.

---

## 2. Python Styling & Lints

- **Format Tool:** `black` or `ruff`.
- **Linter Tool:** `flake8` or `ruff check`.
- **Command:**
  ```bash
  ruff check .
  ruff format --check .
  ```
- **Rules:**
  - Follow PEP 8 guidelines.
  - Apply type assertions for all function parameters and return signatures.
  - Expose clean public classes that hide FFI `PyPage`/`PySession` classes behind pythonic wrappers.

---

## 3. Node.js & Javascript Styling

- **Format Tool:** `prettier`.
- **Linter Tool:** `eslint`.
- **Command:**
  ```bash
  npm run lint
  ```
- **Rules:**
  - Standard JavaScript naming style (camelCase for variables, PascalCase for classes).
  - Explicit TypeScript interface definitions for native binding wrappers.
