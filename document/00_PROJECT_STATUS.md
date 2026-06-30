# 00_PROJECT_STATUS.md

## Project Metadata
- **Project Name:** Crawlingo
- **Current Version:** `0.1.0`
- **License:** MIT
- **Primary Language:** Rust (Core Engine)
- **Secondary Languages:** Pyt hon (maturin/PyO3 bindings), JavaScript/TypeScript (NAPI-RS bindings)
- **Target Architecture:** Modular, extensible, high-performance web crawling & parsing engine.

---

## Current Status Summary
Crawlingo is a multi-language library wrapper built around a core Rust parsing and network engine. The engine provides low-level streaming HTML parsing, auto-healing elements via CSS selector fingerprinting, and structured extraction. FFI bindings exist for Python and Node.js.

### Active Components & CI Status
1. **Core Rust Engine (`crawlingo`)**: High-performance core.
   - Status: Active / Passing.
   - Build Tool: `cargo` (Rust Edition 2021).
   - Features: Parallel DOM matching via Rayon, SIMD anchor matching via memchr, streaming parsing using `lol_html`.
2. **Python SDK (`sdk/python`)**: Python wrappers around the core.
   - Status: Active / Passing.
   - Build Tool: `maturin` and `pip`.
   - Integration: Implements hooks, CLI, and MCP servers.
3. **Node.js SDK (`sdk/nodejs`)**: Node wrappers around the core.
   - Status: Active / Passing.
   - Build Tool: `napi-rs` and `npm`.
   - Integration: Native addons (`.node`) built for multiple targets.

### Supported Platforms (CI Matrix)
- **Windows**: x86_64-pc-windows-msvc
- **macOS**: x86_64-apple-darwin, aarch64-apple-darwin
- **Linux**: x86_64-unknown-linux-gnu

---

## Active Key Dependencies
From `Cargo.toml`:
- **Network Engine:** `wreq` (6.0.0-rc.29) - high-performance stealth client.
- **HTML Parser:** `lol_html` (2.0) - fast streaming HTML rewriter/parser.
- **Embedded Database:** `sled` (0.34) - lock-free, embedded transactional database for DOM fingerprints.
- **DNS caching:** `hickory-resolver` & `moka` cache.
- **Rate limiting:** `governor` & `tower`.
- **Parallel processing:** `rayon` (1.10) for parallel similarity scoring.
- **Serialization:** `serde_json`, `arrow` (51) & `parquet` (51) for data output.

---

## Known Blocker / Limitations
1. **FFI Complexity**: Node.js and Python bindings duplicate API logic. Changes in the core engine API require manually editing `sdk/nodejs/native/src/lib.rs` and Python wrapper files.
2. **No Real HTTP Integration Tests**: Tests run against offline HTML fixtures.
3. **Fetcher and RateLimiter Lifecycle**: Crawler and Dataset modules construct new fetcher instances per crawler work loop rather than reusing a shared instance from a Session, degrading connection reuse and rate limit enforcement.
