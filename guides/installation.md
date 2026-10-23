# Installation

Choose a language, build the matching native engine, and check your import.

<a id="version"></a>

## Development source and published packages

These guides describe the current development source. DatasetJob is new in this working tree; published alpha packages may not contain it. Build the matching source before using these examples.

This reference describes source version 1.0.0-alpha.2 (Python 1.0.0a2). Registry artifacts may lag behind this source; check the installed version and release notes before using an API. Native build prerequisites include Rust, a C/C++ toolchain, CMake and the tools required by the TLS dependencies.

<a id="rust"></a>

## Rust and CLI

Change the path dependency to your checkout. Enable the browser feature only when you need Chromium rendering.

### From repository root

```shell
cargo build --release --features cli
cargo run --release --features cli -- --help
```

### Local application Cargo.toml

```toml
[dependencies]
crawlingo = { path = "../crawlingo" }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

<a id="python"></a>

## Python

Use a virtual environment. Python 3.12 was used for the local binding check. The current PyO3 dependency does not support building against Python 3.14.

### From repository root

```shell
python -m venv .venv
# Activate .venv using the command for your shell.
python -m pip install maturin
cd sdk/python
maturin develop
python -c "from crawlingo import DatasetJob; print(DatasetJob)"
```

<a id="node"></a>

## Node.js and TypeScript

Build the native library before compiling TypeScript. The SDK has operating-system-specific native assets; rebuild for each target platform.

### From repository root

```shell
cd sdk/nodejs
npm ci
npm run build
npm test
```

- [Use the local Node.js package](sdk-nodejs.md)

<a id="browser"></a>

## Optional Chromium rendering

The Rust browser feature requires a local Chrome or Chromium executable. Set CRAWLINGO_CHROME_PATH to its absolute path. Browser rendering has its own resource costs; the HTTP scale result does not cover browser workloads.

### Rust feature

```shell
cargo build --release --features browser,cli
```
