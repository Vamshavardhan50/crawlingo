# Contributing to Crawlingo

Thank you for your interest in contributing to Crawlingo! 🎉

We welcome contributions from everyone — bug reports, feature requests, documentation improvements, code contributions, and more.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Pull Request Process](#pull-request-process)
- [Commit Message Convention](#commit-message-convention)
- [Code Style](#code-style)
- [Testing](#testing)
- [Release Process](#release-process)

---

## Code of Conduct

This project follows our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

---

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/crawlingo.git
   cd crawlingo
   ```
3. **Set up** the development environment (see below)
4. **Create** a branch for your changes
5. **Make** your changes
6. **Test** your changes
7. **Submit** a Pull Request

---

## Development Setup

### Prerequisites

- **Rust** 1.70+ (`rustup update stable`)
- **Python** 3.8+ and `maturin` (`pip install maturin`)
- **Node.js** 18+ and `npm`
- **Git** 2.x+

### Rust Core

```bash
# Build and test the Rust core
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

### Python SDK

```bash
# Install development dependencies
pip install maturin pytest ruff

# Build the Python extension in-place
maturin develop

# Run Python tests
pytest python/tests/ -v
```

### Node.js SDK

```bash
cd node
npm install
npm run build
npm test
```

### Documentation

```bash
cd crawlingo-docs
npm install
npm run dev       # Start dev server at http://localhost:3000
```

---

## Making Changes

### Branching

Use descriptive branch names:

| Type | Format | Example |
|------|--------|---------|
| Feature | `feat/description` | `feat/go-sdk` |
| Bug fix | `fix/description` | `fix/proxy-rotation-leak` |
| Documentation | `docs/description` | `docs/benchmarks-update` |
| Refactor | `refactor/description` | `refactor/selector-engine` |
| Performance | `perf/description` | `perf/simd-text-anchors` |

### File Locations

| What | Where |
|------|-------|
| Rust core | `src/` |
| Python bindings | `python/` |
| Node.js bindings | `node/` |
| Rust crate API | `examples/` |
| Documentation | `crawlingo-docs/src/pages/` |
| Tests (Rust) | `src/tests/` |
| Tests (Python) | `python/tests/` |
| Tests (Node.js) | `node/tests/` |

---

## Pull Request Process

1. **Update documentation** — if your change affects the public API, update `crawlingo-docs/src/pages/`
2. **Write tests** — add tests that cover your change
3. **Pass CI** — all checks must pass: `cargo test`, `pytest`, `npm test`
4. **Update CHANGELOG.md** — add a line under `[Unreleased]`
5. **Request review** — tag a maintainer in your PR description

### PR Title Format

```
feat: add Go SDK via CGo bindings
fix: resolve proxy rotation memory leak
docs: add Node.js streaming examples
perf: improve auto-match scoring by 40%
```

---

## Commit Message Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`

Examples:
```
feat(python): add async Page.create() support
fix(watch): prevent duplicate callbacks on rapid DOM changes
perf(selectors): speed up text anchor SIMD by 20%
docs(sdk): add Rust streaming example
```

---

## Code Style

### Rust

```bash
cargo fmt          # Format code
cargo clippy       # Lint code
```

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

### Python

```bash
ruff format .      # Format
ruff check .       # Lint
```

Type annotations required for all public functions.

### TypeScript

```bash
npx prettier --write .
npx eslint .
```

Strict TypeScript — no `any` in public APIs.

---

## Testing

We require tests for all new features and bug fixes.

```bash
# Rust
cargo test
cargo test --features parquet

# Python
pytest python/tests/ -v --tb=short

# Node.js
npm test

# Integration tests (requires network)
cargo test --test integration -- --ignored
```

---

## Release Process

Releases are managed by maintainers. The process:

1. Update `CHANGELOG.md` (move `[Unreleased]` to a version)
2. Bump versions in `Cargo.toml`, `python/pyproject.toml`, `node/package.json`
3. Tag: `git tag v0.X.Y && git push --tags`
4. GitHub Actions publishes wheels to PyPI, binaries to npm, and the crate to crates.io automatically

---

## Questions?

- Open a [GitHub Discussion](https://github.com/Vamshavardhan50/crawlingo/discussions)
- Open a [GitHub Issue](https://github.com/Vamshavardhan50/crawlingo/issues)

Thank you for contributing! 🦀
