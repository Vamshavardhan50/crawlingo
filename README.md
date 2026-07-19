<div align="center">
  <a href="https://crawlingo.dev">
    <img src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/assets/Logo%20and%20name.png" alt="Crawlingo" width="640" style="max-width: 100%;">
  </a>
  <p align="center">
    <strong>Effortless, Self-Healing, Stealth Web Scraping for the Modern Web.</strong>
  </p>
  <p align="center">
    <a href="https://github.com/Vamshavardhan50/crawlingo/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Vamshavardhan50/crawlingo/ci.yml?branch=main&label=CI&style=flat-square&logo=github" alt="CI Status"></a>
    <a href="https://crates.io/crates/crawlingo"><img src="https://img.shields.io/crates/v/crawlingo?style=flat-square&logo=rust&label=crates.io" alt="Crates.io"></a>
    <a href="https://pypi.org/project/crawlingo/"><img src="https://img.shields.io/pypi/v/crawlingo?style=flat-square&logo=python&label=PyPI" alt="PyPI"></a>
    <a href="https://www.npmjs.com/package/crawlingo"><img src="https://img.shields.io/npm/v/crawlingo?style=flat-square&logo=nodedotjs&label=npm" alt="npm"></a>
    <a href="https://github.com/Vamshavardhan50/crawlingo/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Vamshavardhan50/crawlingo?style=flat-square&color=orange" alt="License"></a>
    <a href="https://crawlingo.dev/docs"><img src="https://img.shields.io/badge/docs-crawlingo.dev-blue?style=flat-square&color=6366F1" alt="Documentation"></a>
  </p>
</div>

---

## 💖 Sponsors

<p align="center">
  <a href="https://genzgrowth.com" title="Gen-Z Growth — Official Sponsor">
    <img src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/assets/genZgrowth.png" alt="Gen-Z Growth" width="220">
  </a>
</p>

<p align="center">
  <sub>Want your logo here? <a href="https://crawlingo.dev/sponsor">Become a sponsor →</a></sub>
</p>

---

<br>

<p align="center">
  <img src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/assets/Logo.png" alt="Crawlingo Logo" width="160">
</p>

---

## ⚡ What is Crawlingo?

**Crawlingo** is a next-generation, high-performance web scraping and crawler framework powered by a core compiled in Rust. It delivers **self-healing selectors**, **stealth TLS/HTTP/2 fingerprinting**, and **SIMD-accelerated text anchors** to make data extraction resilient, fast, and completely headless-browser-free.

Unlike scraper libraries that break when a website modifies a single CSS class, Crawlingo uses a localized DOM fingerprinting system with Jaro-Winkler similarity algorithms to **automatically heal broken selectors in real-time**.

Crawlingo ships with native, idiomatic SDKs for **Python**, **Node.js (TypeScript)**, and **Rust**.

---

## 🚀 Key Features

*   🛡️ **Self-Healing Selectors (Auto-Match):** Automatically repairs broken CSS or XPath selectors in production using multi-dimensional DOM fingerprints stored in an embedded Sled database.
*   🌐 **Stealth TLS/HTTP2 Profiles:** Mimics genuine browser handshakes (JA3, HTTP/2 frames, user-agent headers) for Chrome, Firefox, and Safari to bypass Cloudflare, Akamai, and Turnstile without heavy headless browsers.
*   ⚡ **High-Throughput Rust Core:** Achieves up to **3,500+ requests/second** using Tokio's async I/O and Rayon's parallel CPU processing on commodity hardware.
*   💾 **Zero-Copy FFI:** Shared memory mappings ensure PyO3 (Python) and napi-rs (Node.js) bindings extract and parse datasets with near-zero overhead.
*   📊 **Flexible Dataset Builder:** Export extracted fields directly to structured JSON, CSV, or Parquet datasets in a single method call.
*   👁️ **Reactive Watch Monitors:** Polls websites on background intervals, automatically computing diffs and firing webhooks on content changes.

---

## 📊 Framework Comparison

| Metric / Feature | Crawlingo | Scrapy | Playwright / Puppeteer |
| :--- | :--- | :--- | :--- |
| **Language Bindings** | Python · Node.js · Rust | Python | JS · Python · C# · Java |
| **Throughput (50 concurrent)** | **~3,500 req/s** | ~500 req/s | ~50 req/s |
| **Memory Footprint (Idle)** | **2.4 MB** | ~50 MB | ~200 MB |
| **Self-Healing Selectors** | **✅ Built-in** | ❌ Manual Fix | ❌ Manual Fix |
| **Stealth TLS Fingerprinting**| **✅ Built-in** | ❌ Third-party | ❌ Heavy plugins |
| **Change Monitoring** | **✅ Built-in** | ❌ | ❌ |
| **CPU Acceleration** | **✅ SIMD** | ❌ | ❌ |

---

## 💻 Multi-Language Quick Start

### 🐍 Python SDK

```bash
pip install crawlingo
```

```python
from crawlingo import Page, Session, Dataset

with Session() as s:
    s.auto_match(True).fetcher_tier("stealthy").rate_limit(5)

    result = (Dataset("https://shop.example.com", session=s)
              .field("title", "h1")
              .field("price", ".price", extraction_type="price")
              .field("in_stock", ".stock-badge")
              .build())

    print(result.to_dict())
    result.to_parquet("shop_data.parquet")
```

### 📘 Node.js (TypeScript) SDK

```bash
npm install crawlingo
```

```typescript
import { Page, Session, Dataset } from 'crawlingo';

const session = new Session().autoMatch(true).fetcherTier("stealthy").rateLimit(5);

const result = await new Dataset("https://shop.example.com", session)
  .field("title", "h1")
  .field("price", ".price", { extractType: "price" })
  .field("in_stock", ".stock-badge")
  .build();

console.log(result.toDict());
result.toJsonFile("shop_data.json");
```

### 🦀 Rust SDK

```toml
[dependencies]
crawlingo = "1.0.0-alpha.1"
tokio = { version = "1", features = ["full"] }
```

```rust
use crawlingo::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let session = Arc::new(Session::new());
    session.set_auto_match(true);
    session.set_fetcher_tier("stealthy");

    let result = Dataset::new("https://shop.example.com", session)
        .with_field(DatasetField::new("title", "h1"))
        .with_field(DatasetField::new("price", ".price").with_extract_type(ExtractionType::Price))
        .build_async()
        .await?;

    println!("{:#?}", result.fields);
    Ok(())
}
```

---

## ⚙️ Advanced Configuration

| Config Key / Env Var | Default | Description |
| :--- | :--- | :--- |
| `fetcher_tier` / `CRAWLINGO_TIER` | `"standard"` | `"standard"` (raw HTTP/2) or `"stealthy"` (browser fingerprint spoofing). |
| `auto_match` / `CRAWLINGO_AUTO_MATCH`| `true` | Enables Jaro-Winkler based self-healing for broken selectors. |
| `rate_limit` / `CRAWLINGO_RATE_LIMIT` | `0` (Disabled) | Restricts request rate to `N` requests per second per host. |
| `proxy_pool` / `CRAWLINGO_PROXIES` | `[]` | List of proxy URLs to rotate in a round-robin pool. |
| `fingerprint_db` / `CRAWLINGO_DB_PATH`| `".crawlingo"` | File path to store DOM selector fingerprints in Sled DB. |

---

## 📚 Resources & Community

| Resource | Link |
| :--- | :--- |
| 🌐 **Official Website** | [crawlingo.dev](https://crawlingo.dev) |
| 📖 **Documentation** | [crawlingo.dev/docs](https://crawlingo.dev/docs) |
| 🐍 **PyPI Package** | [pypi.org/project/crawlingo](https://pypi.org/project/crawlingo/) |
| 📦 **npm Package** | [npmjs.com/package/crawlingo](https://www.npmjs.com/package/crawlingo) |
| 🦀 **crates.io** | [crates.io/crates/crawlingo](https://crates.io/crates/crawlingo) |
| 📋 **Changelog** | [CHANGELOG.md](CHANGELOG.md) |
| 🤝 **Contributing** | [CONTRIBUTING.md](CONTRIBUTING.md) |
| 🔐 **Security Policy** | [SECURITY.md](SECURITY.md) |

---

## 📄 License

Crawlingo is open-source software licensed under the [MIT License](LICENSE).
