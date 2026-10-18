.PHONY: all develop test clippy fmt clean python node release-check docs docs-dev

all: develop

develop: python

test:
	cargo test --workspace

test-verbose:
	cargo test --workspace --verbose

clippy:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all -- --check

clean:
	cargo clean

python:
	cd sdk/python && maturin develop --features python --release

python-build:
	cd sdk/python && maturin build --features python --release

node:
	cd sdk/nodejs && npm run build

node-test:
	cd sdk/nodejs && npm test

release-check:
	node scripts/check-release.cjs

# Quick smoke test for all SDKs
smoke-test:
	cargo test --lib --release --features python
	cd sdk/python && python -c "import crawlingo; p = crawlingo.Page('https://httpbin.org/html'); print('Python SDK OK:', p.title())"
	cd sdk/nodejs && node -e "const { Page } = require('./dist'); console.log('Node.js SDK builds OK')"

# Build all release artifacts
build-all:
	cargo build --release --workspace --features python
	cd sdk/python && maturin build --features python --release --out dist
	cd sdk/nodejs && npm run build

# Documentation
docs:
	cd crawlingo-docs && npm ci && npm run build

docs-dev:
	cd crawlingo-docs && npm run dev