<p align="center">
  <img src="crawlingo.png" width="200" alt="Crawlingo Eagle Logo" />
</p>

<h1 align="center">Crawlingo</h1>

<p align="center">
  <strong>Build Scrapers That Survive Change.</strong>
</p>

<p align="center">
  <a href="https://github.com/Vamshavardhan50/crawlingo/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Vamshavardhan50/crawlingo" alt="License" /></a>
  <a href="https://pypi.org/project/crawlingo/"><img src="https://img.shields.io/pypi/v/crawlingo" alt="PyPI Version" /></a>
  <a href="https://www.npmjs.com/package/crawlingo"><img src="https://img.shields.io/npm/v/crawlingo" alt="NPM Version" /></a>
</p>

---

**Crawlingo** is a next-generation, high-performance web data extraction and monitoring framework. It combines a blazing-fast **Rust core engine** with a React-like developer experience exposed through native **Python** and **Node.js** SDK bindings.

Crawlingo is engineered to solve **selector drift**—when websites update their layouts, traditional scrapers fail. Crawlingo utilizes cached DOM tree fingerprints and dynamic Jaro-Winkler similarity matching to self-heal and locate drifted elements automatically.

---

## 🚀 Key Features

* 🛡️ **Stealth Browser Impersonation**: Impersonate TLS/JA3 handshakes and HTTP/2 settings (Chrome, Firefox, Safari) natively inside Rust without the memory overhead of headless browsers.
* 🧠 **Self-Healing Selectors (Auto-Match)**: Compares current page layouts against cached element profiles to restore broken query paths automatically.
* 🔄 **Built-in Proxy Pools**: Rotate requests round-robin across a list of static proxies, or load active proxy lists dynamically from remote provider API endpoints.
* ⏰ **Background Scheduled Crawling**: Spin up non-blocking interval tasks that execute recurring crawls in a dedicated background thread.
* 🎛️ **Customizable Similarity Weights**: Tweak the auto-match heuristics by prioritizing tag, text content, class, or attribute matches to suit specific target sites.
* 📡 **Real-time Webhook Streaming**: Post scraped JSON dataset results instantly to webhook servers, minimizing memory usage during massive spider crawl sessions.
* 📊 **Multi-Format Native Arrow Exports**: Save datasets directly to **CSV**, **JSON**, or column-oriented **Apache Parquet** files, or call `.df()` to return a standard **Pandas DataFrame**.

---

## 📦 Installation

### Python SDK
```bash
pip install crawlingo
```

### Node.js SDK
```bash
npm install crawlingo
```

### Rust Crate
```toml
[dependencies]
crawlingo = "0.1.0"
```

---

## ⚡ Quick Start

### 1. Basic Self-Healing Scraping
Create a session, enable auto-matching, and inspect page contents.

```python
from crawlingo import Session

# Create a session (maintains cookies, connections, and fingerprints)
session = Session()
session.auto_match(True)          # Enable auto-match self-healing
session.fetcher_tier("stealthy")  # Impersonate browser fingerprint
session.browser_profile("chrome")

# Fetch and query page
page = session.page("https://example.com/products")
title = page.title()
print(f"Page Title: {title}")

# Extracts even if the card class path has drifted/changed!
products = page.css("div.product-card")
for item in products:
    name = item.css("h2.title").text()
    price = item.css("span.price-tag").text()
    print(f"{name}: {price}")
```

### 2. Advanced: Rotating Proxies, Webhooks & Scheduling
Scale up to multi-page crawls with advanced controls.

```python
from crawlingo import Session

# Initialize session with proxy rotation pool
session = Session()
session.proxy_pool([
    "http://proxy1.example.com:8080",
    "http://proxy2.example.com:8080"
])

# Configure custom auto-match similarity weights
session.auto_match_weights({
    "text": 3.0,       # Strongly prioritize text matching
    "class": 1.0,      # De-emphasize class name matching
    "ancestor": 2.0,   # Prioritize parent path tags
})

# Setup crawling configurations
crawl = session.crawl("https://example.com/products")
crawl.follow("a.next-page")
crawl.field("title", "h1")

# Option A: Deliver crawled items to an external webhook in real-time
crawl.webhook("https://my-receiver-api.com/v1/crawl-ingest")
crawl.build()

# Option B: Run a background crawl loop every 1 hour (3600 seconds)
crawl.schedule(3600)
```

---

## 🛠️ Architecture

Crawlingo shares a zero-copy memory footprint across programming languages:
1. **Fetcher (Rust/wreq)**: Manages concurrent HTTP connection pooling, hickory-resolver DNS, and rate-limiting.
2. **Parser (Rust/lol_html)**: Binds to Cloudflare’s streaming parser, building vector-indexed DOM mappings on the fly.
3. **Matcher (Rust/strsim)**: Measures Jaro-Winkler distance matrices to match drifted CSS/XPath selectors.
4. **Export (Rust/Arrow)**: Streams records directly to disk file blocks (JSON, Parquet) bypassing language runtime memory copies.

---

## 📝 License

Released under the [MIT License](LICENSE).
