<h1 align="center">
  <a href="https://crawlingo.dev">
    <img alt="Crawlingo" src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/assets/Logo%20and%20name.png" width="560">
  </a>
  <br>
  <small>Python SDK — Self-Healing Web Scraping for Python</small>
</h1>

<p align="center">
  <a href="https://github.com/Vamshavardhan50/crawlingo/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/Vamshavardhan50/crawlingo/ci.yml?branch=main&style=flat-square&logo=github&label=CI" /></a>
  <a href="https://pypi.org/project/crawlingo/"><img src="https://img.shields.io/pypi/v/crawlingo?style=flat-square&logo=python&color=blue&label=PyPI" alt="PyPI version" /></a>
  <a href="https://pypi.org/project/crawlingo/"><img src="https://img.shields.io/pypi/pyversions/crawlingo?style=flat-square&logo=python" alt="Python Versions" /></a>
  <a href="https://github.com/Vamshavardhan50/crawlingo/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Vamshavardhan50/crawlingo?style=flat-square&label=License" alt="License" /></a>
  <a href="https://crawlingo.dev/docs"><img src="https://img.shields.io/badge/docs-crawlingo.dev-6366F1?style=flat-square" alt="Docs" /></a>
</p>

<p align="center">
  <a href="#installation"><strong>Installation</strong></a> ·
  <a href="#why-crawlingo"><strong>Why Crawlingo</strong></a> ·
  <a href="#features"><strong>Features</strong></a> ·
  <a href="#quick-start"><strong>Quick Start</strong></a> ·
  <a href="#cli-interface"><strong>CLI</strong></a>
</p>

---

**Crawlingo Python SDK** is a next-generation web data extraction, crawling, and website monitoring library. It wraps a high-performance Rust core in an elegant Python API — scraping workflows that survive page design shifts automatically.

📚 **Full API reference and guides at [crawlingo.dev/docs](https://crawlingo.dev/docs)**

---

## 🎥 Demo

<p align="center">
  <img src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/assets/crawlingo_demo.webp" alt="Crawlingo Self-Healing Demo" width="600">
</p>

### How Self-Healing Works:
1. **Drift Detection** — When `button#submit.btn-primary` renames to `button#send-btn.btn-primary-new`, traditional scrapers return empty.
2. **DOM Parsing** — Crawlingo's Rust engine intercepts the mismatch and isolates candidates within the parent node.
3. **Jaro-Winkler Matching** — Candidates are ranked by tag, attributes, text content, and structural fingerprints.
4. **Auto-Match Recovery** — The highest-scoring candidate (e.g. **94% confidence**) is auto-bound and cached — zero pipeline downtime.

---

## 📦 Installation

<a id="installation"></a>

```bash
pip install crawlingo
```

Or build from source:

```bash
git clone https://github.com/Vamshavardhan50/crawlingo.git
cd crawlingo/sdk/python
pip install -e .
```

---

## 🚀 Why Crawlingo?

<a id="why-crawlingo"></a>

| Feature | Crawlingo | Scrapy | Crawl4AI |
|----------|-----------|--------|----------|
| Rust Core | ✅ | ❌ | ❌ |
| Python SDK | ✅ | ✅ | ✅ |
| Node SDK | ✅ | ❌ | ❌ |
| Self-Healing Selectors | ✅ | ❌ | ❌ |
| Change Monitoring | ✅ | ❌ | ❌ |
| Dataset Export | ✅ | ⚠️ | ⚠️ |
| Stealth TLS | ✅ | ❌ | ❌ |
| AI / MCP Ready | ✅ | ❌ | ✅ |

---

## 🛠️ Core Features

<a id="features"></a>

- **🧠 Self-Healing DOM Fingerprinting** — Tracks layout changes via Jaro-Winkler. [Learn more](https://crawlingo.dev/docs/features#auto-match-self-healing)
- **🛡️ Stealth Browser Impersonation** — Bypasses Cloudflare, Akamai via HTTP/2 TLS fingerprint rotation. [Learn more](https://crawlingo.dev/docs/features#stealthy-browser-impersonation)
- **⚡ SIMD-Accelerated Text Anchors** — Faster than CSS/XPath via vector math. [Learn more](https://crawlingo.dev/docs/features#text-anchor-simd-accelerated)
- **🔄 High-Speed Proxy Rotation** — Automatic round-robin proxy cycling. [Learn more](https://crawlingo.dev/docs/spiders#proxy-rotation)
- **⏰ Reactive Watch Monitors** — Background polling with webhook notifications on changes. [Learn more](https://crawlingo.dev/docs/features#change-monitoring-watches)
- **🤖 Built-in MCP Server** — Native Claude/Cursor integration. [Learn more](https://crawlingo.dev/docs/ai/mcp-server)
- **📦 Schema-Driven Datasets** — Export to JSON, CSV, Arrow, or Pandas DataFrames. [Learn more](https://crawlingo.dev/docs/features#multi-format-exports)

---

## ⚡ Quick Start

<a id="quick-start"></a>

### 1. Basic Extraction

```python
from crawlingo import Page

page = Page("https://example.com")
print(page.title())
print(page.css("p").text())
```

### 2. Self-Healing Datasets

```python
from crawlingo import Dataset

dataset = (
    Dataset("https://example.com/products")
    .auto_match(True)
    .field("title", "h1.product-title")
    .field("price", "span.price")
    .build()
)

print(dataset.to_dict())
dataset.to_csv("products.csv")
dataset.to_parquet("products.parquet")
```

### 3. Watch Monitor for Changes

```python
import asyncio
from crawlingo import Watch

def on_price_update(event):
    print(f"Price changed: {event.old_value} → {event.new_value}")

async def main():
    watch = (
        Watch("https://example.com/item")
        .field("price", "span.item-price")
        .interval(60)
        .on_price_change(on_price_update)
    )
    await watch.run_async()

asyncio.run(main())
```

### 4. Stealth Session with Proxies

```python
from crawlingo import Session, Page

session = (
    Session()
    .fetcher_tier("stealthy")
    .proxy_pool(["http://proxy1:8080", "http://proxy2:8080"])
    .rate_limit(3)
    .auto_match(True)
)

page = Page("https://protected-site.com", session=session)
print(page.css("h1").text())
```

---

## 🛠️ CLI Interface

<a id="cli-interface"></a>

```bash
# Interactive REPL preloaded with crawlingo
crawlingo shell https://example.com

# Extract elements directly
crawlingo extract https://example.com --css "h1"

# Start MCP server for AI agents
crawlingo mcp --host 127.0.0.1 --port 8000
```

---

## 💖 Sponsors

<p align="center">
  <a href="https://genzgrowth.com" title="Gen-Z Growth">
    <img src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/assets/genZgrowth.png" alt="Gen-Z Growth" width="180">
  </a>
</p>

---

## 📝 License

MIT License — see [LICENSE](https://github.com/Vamshavardhan50/crawlingo/blob/main/LICENSE).
