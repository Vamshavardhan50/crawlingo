# Development Setup

Build Crawlingo from source for local development.

## Prerequisites

| Tool | Version |
|------|---------|
| Rust | 1.75+ |
| Python | 3.9 – 3.13 |
| Node.js | 18+ / 20+ |
| Cargo | stable toolchain |
| Compiler | MSVC (Windows), GCC/Clang (Linux/macOS) |

## Building the Rust Core

```bash
git clone https://github.com/Vamshavardhan50/crawlingo
cd crawlingo

# Build all targets
cargo build --release

# Run core tests
cargo test

# Run benchmarks
cargo bench
```

## Python SDK

```bash
# Create and activate virtual environment
python -m venv .venv
source .venv/bin/activate  # Linux/macOS
.venv\Scripts\Activate.ps1  # Windows

# Build and install the Python package
maturin develop --release

# Verify
python -c "import crawlingo; print(crawlingo.__version__)"
```

## Node.js SDK

```bash
cd sdk/nodejs

# Add Rust target for native addon
rustup target add x86_64-pc-windows-msvc  # Windows
rustup target add x86_64-unknown-linux-gnu  # Linux

# Build
npm install
npm run build

# Verify
node -e "const { Page } = require('crawlingo-native'); console.log('OK')"
```

## Running Tests

See the [Testing Guide](04_testing.md) for the full test suite reference.
