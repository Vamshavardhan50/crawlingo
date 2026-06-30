# 06_DEVELOPMENT_GUIDE.md

This document serves as the guide for setting up your local environment, building, and developing Crawlingo.

---

## 1. Prerequisites

Before starting, ensure the following components are installed on your system:
- **Rust Toolchain:** Stable version (`1.75.0` or higher recommended). Install via [rustup.rs](https://rustup.rs).
- **Python:** Version `3.9` to `3.13`.
- **Node.js:** LTS version (e.g., `18.x` or `20.x`) with `npm`.
- **Compilers:**
  - Windows: Visual Studio Build Tools with MSVC compiler.
  - Linux/macOS: GCC, Clang, or Xcode Command Line Tools.

---

## 2. Building the Rust Core

From the root directory:
```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

---

## 3. Developing the Python SDK

Python bindings are built using `maturin`.

### Setup Virtual Environment
```bash
# Move to the Python SDK directory
cd sdk/python

# Create a virtual environment
python -m venv .venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate

# Install development dependencies
pip install -r requirements.txt
```

### Build & Link Python Bindings
Maturin compiles the Rust code and installs the resulting module directly into the active Python environment.
```bash
# Compile and install in development mode
maturin develop --features python
```

---

## 4. Developing the Node.js SDK

Node.js bindings are built using `napi-rs`.

```bash
# Move to the Node.js SDK directory
cd sdk/nodejs

# Install dependencies and build native bindings
npm install
npm run build
```

This compiles the native Rust library (`sdk/nodejs/native`) and outputs a `.node` binary in the build directory.

---

## 5. Running Tests

### Rust Unit & Integration Tests
```bash
# Run all core tests
cargo test

# Run a specific test file
cargo test --test integration_test
```

### Python SDK Tests
Ensure `maturin develop` has been run first.
```bash
cd sdk/python
pytest
```

### Node.js SDK Tests
```bash
cd sdk/nodejs
npm test
```

---

## 6. Running Benchmarks

Crawlingo uses `criterion` for benchmarking parser and matching bottlenecks.

```bash
# Run all benchmarks
cargo bench

# Run a specific benchmark
cargo bench --bench selector
cargo bench --bench auto_matcher
```
Benchmark reports are generated as interactive HTML files under `target/criterion/report/index.html`.
