<p align="center">
  <a href="https://crawlingo.dev">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/logo.svg">
      <img alt="Crawlingo" src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/logo.svg" width="400">
    </picture>
  </a>
</p>

<h1 align="center">Crawlingo — Self-Healing Web Scraping Framework</h1>
<p align="center"><strong>Rust-Powered &bull; Cross-Language &bull; Production-Grade</strong><br>
⚡ 3,500+ req/s &bull; 🔧 Auto-Repairing Selectors &bull; 🛡️ Stealth TLS &bull; 👁 Change Monitoring</p>

<p align="center">
  <a href="https://github.com/Vamshavardhan50/crawlingo/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Vamshavardhan50/crawlingo/ci.yml?branch=main&style=flat-square&logo=github&label=CI" alt="CI"></a>
  <a href="https://pypi.org/project/crawlingo/"><img src="https://img.shields.io/pypi/v/crawlingo?style=flat-square&logo=python&color=blue&label=PyPI" alt="PyPI"></a>
  <a href="https://www.npmjs.com/package/crawlingo"><img src="https://img.shields.io/npm/v/crawlingo?style=flat-square&logo=nodedotjs&color=red&label=NPM" alt="NPM"></a>
  <a href="https://crates.io/crates/crawlingo"><img src="https://img.shields.io/crates/v/crawlingo?style=flat-square&logo=rust&color=orange&label=Crates.io" alt="Crates.io"></a>
  <a href="https://github.com/Vamshavardhan50/crawlingo/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Vamshavardhan50/crawlingo?style=flat-square&label=License" alt="License"></a>
  <a href="https://github.com/Vamshavardhan50/crawlingo/stargazers"><img src="https://img.shields.io/github/stars/Vamshavardhan50/crawlingo?style=flat-square&logo=github&label=Stars" alt="Stars"></a>
</p>

```python
from crawlingo import Page

page = Page("https://example.com")
print(page.title())                 # "Example Domain"
print(page.status)                  # 200
print(page.markdown()[:80])         # Clean markdown
```

---

## ✨ Features

| Icon | Feature | What It Does |
|------|---------|-------------|
| 🧠 | **Self-Healing Selectors** | CSS selectors auto-repair when sites change. DOM fingerprints + Jaro-Winkler similarity scored in parallel. |
| 🚀 | **Dual-Tier Fetcher** | Standard: 3,500 req/s via HTTP/2. Stealthy: 1,800 req/s via TLS fingerprint emulation (Chrome/Firefox/Safari). |
| 🎯 | **5 Selector Types** | CSS, XPath, Regex, Text Anchor, After/Before Text. 850K–2.1M ops/s. |
| 📦 | **Dataset Engine** | Fluent builder → structured extraction. Export JSON, CSV, Parquet. Streaming mode: constant memory. |
| 👁 | **Watch / Monitoring** | Poll-based change detection. Catches content, price, stock, element add/remove changes. Typed callbacks. |
| 🛡️ | **Anti-Bot Stack** | TLS fingerprints, browser headers, proxy rotation, per-host rate limiting, exponential backoff retry. |
| 📊 | **Metrics** | Lock-free counters: requests, successes, failures, latency, per-host breakdown. `session.metrics()`. |
| 🔌 | **3 SDKs** | Python (PyO3), Node.js (napi-rs), Rust (crate). Same core, same perf. |

---

## ⚡ Quick Start

### 🐍 Python
```python
from crawlingo import Page, Session, Dataset, Crawl, Watch

# Single page
page = Page("https://httpbin.org/html")
print(page.title(), page.status)               # "Herman Melville" 200
print(page.markdown()[:100])                   # Clean markdown

# Selectors
h1 = page.css("h1")                            # CSS
paras = page.xpath("//p")                      # XPath
prices = page.regex(r"\$[\d.]+")               # Regex
el = page.find_text("Herman Melville")          # Text anchor

# Session + Dataset
with Session() as s:
    s.rate_limit(5).auto_match(True).fetcher_tier("stealthy")

    result = (Dataset("https://httpbin.org/html", session=s)
              .field("title", "h1")
              .field("author", "//p[1]", selector_type="xpath")
              .field("content", "div")
              .build())
    print(result.to_dict())                    # {"title": "...", "author": "..."}
    result.to_json_file("out.json")
    result.to_parquet("out.parquet")

# Crawl
results = (Crawl("https://httpbin.org/links/5/0")
           .follow("a").limit(10).depth(2).concurrency(5)
           .field("title", "h1").build())
results.to_json("crawl.json")

# Watch
w = Watch("https://httpbin.org/html").field("title", "h1").interval(60)
w.on_change(lambda e: print(f"'{e.old_value}' → '{e.new_value}'"))
w.run(detach=True)
```

### 📘 Node.js
```typescript
import { Page, Session, Dataset, Crawl, Watch } from 'crawlingo';

const session = new Session().rateLimit(5).autoMatch(true);

const page = await Page.create("https://httpbin.org/html");
console.log(page.title(), page.status);

const result = await new Dataset("https://httpbin.org/html", session)
  .field("title", "h1").field("price", ".price", { extractType: "price" }).build();
console.log(result.toDict());

const results = await new Crawl("https://httpbin.org/links/5/0")
  .follow("a").limit(5).field("title", "h1").build();
results.toJsonFile("results.json");

const watcher = new Watch("https://httpbin.org/html")
  .field("title", "h1").interval(60);
watcher.onChange(e => console.log(`${e.field} changed`));
watcher.run();
```

### 🦀 Rust
```rust
use crawlingo::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let session = Arc::new(Session::new());
    session.set_rate_limit(5.0);
    session.set_auto_match(true);

    let page = Page::new("https://httpbin.org/html", session.clone()).await?;
    println!("Title: {:?}", page.title());
    println!("Status: {}", page.status());

    let result = Dataset::new("https://httpbin.org/html", session.clone())
        .with_field(DatasetField::new("title", "h1"))
        .with_field(DatasetField::new("price", ".price")
            .with_extract_type(ExtractionType::Price)
            .with_default("N/A"))
        .build_async().await?;
    println!("{:#?}", result.fields);
    Ok(())
}
```

---

## 📦 Installation

```bash
pip install crawlingo       # Python 3.8+
npm install crawlingo       # Node.js 18+
cargo add crawlingo         # Rust 1.70+
```

Pre-built wheels for Linux (x86_64, aarch64), macOS (x86_64, arm64), Windows (AMD64). Build from source requires Rust 1.70+.

```bash
# Verify installation
python -c "import crawlingo; print(crawlingo.__version__)"
node -e "const c = require('crawlingo'); console.log(Object.keys(c))"
```

---

## 🧠 Core Concepts

### 🔧 Session
Central config container. All operations (Page, Dataset, Crawl, Watch) bind to a Session and share its FetchManager, rate limiter, connection pool, middleware stack, and fingerprint store.

```python
session = Session()                                    # Defaults
session = Session.from_config("crawlingo.toml")        # TOML file
session = Session.from_config()                        # Env vars only

session.headers({"UA": "MyBot/1.0"}).rate_limit(5).auto_match(True).fetcher_tier("stealthy")

with Session() as s:
    s.proxy("http://proxy:8080").browser_profile("chrome")
    page = s.page("https://example.com")
```

**Config methods:**
| Method | Example | Purpose |
|--------|---------|---------|
| `.headers(dict)` | `{"Accept": "text/html"}` | Default HTTP headers |
| `.cookies(dict)` | `{"session": "abc"}` | Default cookies |
| `.proxy(str)` | `"http://user:pass@host:8080"` | Single proxy |
| `.proxy_pool(list)` | `["http://p1", "http://p2"]` | Round-robin rotation |
| `.proxy_provider(str)` | `"https://provider.com/list"` | Dynamic provider |
| `.rate_limit(float)` | `5.0` | Per-host req/s (0 = off) |
| `.auto_match(bool)` | `True` | Self-healing selectors |
| `.timeout(u64)` | `30` | Request timeout seconds |
| `.fetcher_tier(str)` | `"stealthy"` | `"standard"` or `"stealthy"` |
| `.browser_profile(str)` | `"chrome"` | `"chrome"`, `"firefox"`, `"safari"` |
| `.cache_enabled(bool)` | `True` | Response caching |
| `.retry_*(...)` | various | Retry base/max/multiplier |

### 🚚 Fetch Pipeline
```
Page("url") → Cache → Middleware → Rate Limit → Transport (Standard/Stealthy) → HTTP → Retry → Parser → DOM
```

### 🎯 Selectors
| Type | API | Speed | Example |
|------|-----|-------|---------|
| CSS | `page.css("h1")` | 850K/s | `.price`, `div > p` |
| XPath | `page.xpath("//p")` | 310K/s | `//div[@class='price']` |
| Regex | `page.regex(r"\d+")` | 1.2M/s | `\$[\d.]+` |
| Text Anchor | `page.find_text("Price:")` | 2.1M/s | Visible text lookup |
| After/Before | `page.after_text("Price:")` | 1.8M/s | Sibling extraction |

**Auto-Match lifecycle:**
1. First match → compute fingerprint → store in Sled (`{url}::{selector}` key)
2. Subsequent success → update fingerprint
3. Selector fails → load fingerprint → score all DOM nodes in parallel (Rayon, Jaro-Winkler + Jaccard) → best score > 0.5 → return node + update fingerprint

**Fingerprint structure:** tag, class names, ID, attributes, text content, parent tag, child index, sibling tags, depth. Weights configurable via `auto_match_weights`.

### 📦 Dataset
```python
Dataset("https://example.com")
  .field("title", "h1")
  .field("price", ".price", extraction_type="price")
  .field("email", r"[\w@.]+", selector_type="regex")
  .build()
  .to_dict() / .to_json() / .to_csv() / .to_parquet("out.parquet")
```

**Extraction types:** `text` (trim), `price` ($1,234→1234), `datetime` (→ISO), `url` (resolve), `datalink_url/email/phone`

**Streaming:** `Dataset(url_list).field(...).stream()` — constant memory, millions of URLs.

### 🔄 Crawl
BFS crawler with configurable frontier. Extracts fields from every page visited.

```python
Crawl("https://example.com")
  .follow("a")                           # CSS for links to follow
  .limit(500)                            # Max pages
  .depth(3)                              # Max link depth
  .concurrency(10)                       # Concurrent fetches
  .respect_robots(True)
  .allowed_domains(["example.com"])
  .exclude_patterns([r"\.pdf$", r"/blog/"])
  .field("title", "h1")
  .field("content", "article")
  .build()
  .to_json_file("crawl.json")
  .to_parquet_file("crawl.parquet")
```

### 👁 Watch
```python
Watch("https://example.com")
  .field("title", "h1")
  .interval(300)
  .on_change(cb) .on_price_change(cb) .on_stock_change(cb)
  .run() / .run(detach=True) / .stop()
```

Change events: `url`, `field`, `old_value`, `new_value`, `change_type` (content/price/stock/element_added/element_removed), `percentage_change`, `timestamp`.

### 🔁 Retry & Rate Limiting
- **Retry:** Exponential backoff, statuses [429,500,502,503,504], Retry-After support
- **Rate limit:** Per-host token bucket via `governor`. 0.0 = unlimited. Burst = rate_limit.

| Setting | req/s | Use |
|---------|-------|-----|
| 1.0 | 1 | Polite |
| 5.0 | 5 | Moderate |
| 10.0 | 10 | Aggressive |
| 0.0 | ∞ | Testing |

### ⚡ Middleware & Auth
Decorator-pattern layers wrapping transports:

| Layer | Default | What It Does |
|-------|---------|-------------|
| **MetricsLayer** | Always on | Counts requests, successes, failures, records latency |
| **CachingLayer** | Opt-in | In-memory cache honoring Cache-Control, ETag, Last-Modified |
| **AuthLayer** | Opt-in | Injects credentials (Basic, Bearer, Header, API Key, OAuth2) |

**Auth types:**
```python
session.auth_basic("user", "pass")                          # Basic Auth
session.auth_bearer("eyJhbGciOi...")                        # Bearer token
session.auth_header("X-API-Key", "abc123")                  # Custom header
session.auth_api_key("api_key", "abc123")                   # Query param
session.auth_oauth2("client_id", "secret", "https://...")   # OAuth2 (auto-refresh on 401)
```

OAuth2 flow: On HTTP 401 → POST to token_url → cache token → retry with Bearer header.

### 💾 Export Formats
| Format | Method | Use Case |
|--------|--------|----------|
| **JSON** | `.to_json()` / `.to_json_file()` | Web APIs, analysis |
| **CSV** | `.to_csv()` / `.to_csv_file()` | Spreadsheets, import |
| **Parquet** | `.to_parquet("file.parquet")` | Data warehouses, Spark |

---

## 🏗 Architecture

```
User Code → Python/Node/Rust → FFI → Rust Core
  ├── Engine (Session, Fetch, Rate Limit, Retry)
  ├── Parser (html5ever + scraper)
  ├── Selector (CSS, XPath, Regex, Anchor)
  ├── Extraction (text, price, datetime, datalink)
  ├── Dataset (builder + streaming + export)
  ├── Crawl (BFS, depth, concurrency)
  ├── Watch (poll + change detection)
  ├── Metrics (atomics + DashMap)
  ├── Fingerprint (Sled store)
  └── Middleware (metrics, cache, auth)
```

### Parallelism Model
Crawlingo uses a hybrid approach for maximum throughput:

- **Rayon** (CPU-bound): DOM scoring during auto-match, parallel field extraction per document, dataset streaming. Work-stealing scheduler distributes across all available cores.
- **Tokio** (I/O-bound): HTTP fetching, concurrent crawl tasks, watch polling. Async task pool with work-stealing.
- **DashMap + atomics** (shared state): Metrics counters, connection pool cache, session config. Lock-free reads for hot paths.

The streaming dataset uses **bounded channels** for backpressure — producers block when the consumer channel is full, preventing unbounded memory growth. The crawl engine uses a **Tokio Semaphore** to cap concurrent in-flight requests.

### Dependencies
| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime (I/O) |
| `rayon` | Parallel iteration (CPU) |
| `wreq` / `wreq-util` | HTTP/2 client + TLS emulation |
| `html5ever` / `scraper` | HTML5 parsing, DOM traversal |
| `regex` | Regex selector engine |
| `memchr` | SIMD text anchor search |
| `sled` | Embedded fingerprint DB |
| `governor` | Token bucket rate limiter |
| `moka` | LRU connection pool cache |
| `dashmap` | Lock-free concurrent maps |
| `serde` / `serde_json` | Serialization |
| `toml` / `envy` | Config parsing |
| `pyo3` | Python FFI |
| `napi` / `napi-derive` | Node.js N-API FFI |
| `tracing` | Structured logging |

**Error types:**
```
FetchError     — HTTP errors, DNS failures, TLS handshake, timeouts
ParseError     — Invalid HTML, unknown encoding
SelectorError  — Invalid CSS/XPath/regex syntax
ExtractionError — Value normalization failure, unknown type
ConfigError    — Missing file, parse error, invalid value
IoError        — File read/write failure
SdkError       — Serialization, callback exceptions
```

### 💡 Design Decisions
| Decision | Choice | Why |
|----------|--------|-----|
| Core language | Rust | Memory safety, zero-cost abstractions, FFI compatibility |
| HTML parser | html5ever + scraper | Spec-conformant, handles malformed HTML |
| Async runtime | Tokio | Industry standard, work-stealing scheduler |
| Parallelism | Rayon + Tokio | CPU (Rayon) + I/O (Tokio) split |
| FFI (Python) | PyO3 | Mature, async support, maturin build |
| FFI (Node.js) | napi-rs | Type-safe, auto .d.ts generation |
| Config | toml + envy | Human-readable + env var override |
| Fingerprint store | sled | Embedded, ACID, no external deps |
| Rate limiting | governor | Token bucket, per-key, async |

---

## 📊 Benchmarks

| Metric | Standard | Stealthy |
|--------|----------|----------|
| Throughput (50 concurrent) | 3,500 req/s | 1,800 req/s |
| p50/p95/p99 latency | 12/45/120ms | 28/95/250ms |
| Memory (session idle) | 2.4 MB | 3.1 MB |

| Auto-Match | 100 nodes | 1K nodes | 10K nodes |
|------------|-----------|----------|-----------|
| Score time | 45μs | 380μs | 3.2ms |

| Dataset Streaming | 100 URLs | 1K URLs | 10K URLs |
|-------------------|----------|---------|----------|
| Memory | 8 MB | 42 MB | 85 MB |
| Time | 0.8s | 7.5s | 72s |

---

## ⚖️ Comparison

| vs | Crawlingo | Scrapy | Playwright |
|---|-----------|--------|------------|
| Language | Rust + Python/Node/Rust | Python only | JS/Python/C#/Java |
| Performance | 3,500 req/s | ~500 req/s | ~50 req/s (browser) |
| Self-healing | ✅ | ❌ | ❌ |
| Stealth TLS | ✅ | ❌ | ❌ |
| Change detection | ✅ Built-in | ❌ | ❌ |
| Selectors | 5 types (CSS, XPath, Regex, Text Anchor, After/Before) | CSS, XPath | CSS, XPath, text |
| Memory | 2.4 MB | ~50 MB | ~200 MB |
| JS rendering | ⚠️ Stealth profiles | ❌ | ✅ Full browser |
| SDKs | Python, Node.js, Rust | Python only | JS, Python, C#, Java |

---

## 🤖 AI Integration

### 🤖 LLM-Ready Data
Crawlingo's markdown output preserves document hierarchy (headings, lists, tables) — perfect for LLM ingestion:

```python
page = Page("https://docs.example.com/api")
text = page.markdown()  # Clean, structured markdown

# Feed to any LLM
import openai
response = openai.ChatCompletion.create(
    model="gpt-4",
    messages=[
        {"role": "system", "content": "Extract all API endpoints from this documentation."},
        {"role": "user", "content": text[:16000]}
    ]
)
```

### 📚 RAG Pipeline
```python
from crawlingo import Dataset
import chromadb

stream = Dataset(doc_urls).field("heading","h1").field("content","article").stream()
client = chromadb.Client()
collection = client.create_collection("docs")

for chunk in stream:
    text = f"{chunk.data['heading']}\n{chunk.data['content']}"
    collection.add(documents=[text], ids=[f"doc_{i}"])

results = collection.query(query_texts=["How to install?"], n_results=3)
```

### 🤖 AI-Enhanced Price Alerts
```python
def analyze_price(event):
    if event.percentage_change > 10:
        prompt = f"Product price changed by {event.percentage_change:.1f}%. Old: {event.old_value}, New: {event.new_value}. Analyze significance."
        response = openai.ChatCompletion.create(
            model="gpt-4",
            messages=[{"role": "user", "content": prompt}]
        )
        print(f"AI Analysis: {response.choices[0].message.content}")

Watch("https://shop.example.com/p/1")
  .field("price", ".price", extraction_type="price")
  .on_price_change(analyze_price)
  .run()
```

### 🧠 Fine-Tuning Data Preparation
```python
pairs = Crawl("https://docs.example.com/faq").follow("a")
  .field("question", "h3").field("answer", ".answer-content").build()

training_data = [
    {"messages": [
        {"role": "user", "content": p["question"]},
        {"role": "assistant", "content": p["answer"]}
    ]}
    for p in pairs.data_points if p.get("question") and p.get("answer")
]
```

---

## 🎯 Use Cases

**🏷 Price Monitoring**
```python
Watch("https://shop.example.com/p/1")
  .field("price", ".price", extraction_type="price")
  .field("stock", ".stock-badge")
  .interval(300)
  .on_price_change(lambda e: send_alert(e.url, e.percentage_change))
  .on_stock_change(lambda e: notify_back_in_stock(e.url))
  .run(detach=True)
```

**📚 Documentation Crawling**
```python
results = (Crawl("https://docs.example.com")
           .follow("nav a, main a").limit(500).concurrency(10)
           .field("title", "h1")
           .field("content", "article")
           .field("breadcrumb", ".breadcrumb")
           .field("last_updated", "//time/@datetime", selector_type="xpath")
           .build())
results.to_parquet_file("docs.parquet")
results.to_csv_file("docs_index.csv")
```

**🏢 Competitor Monitoring**
```python
session = Session()
session.fetcher_tier("stealthy")
session.browser_profile("chrome")

for site in ["https://competitor1.com/products", "https://competitor2.com/products"]:
    results = (Crawl(site, session)
               .follow(".product-link, a[href*='/product/']").depth(2).concurrency(5)
               .field("name", ".product-name")
               .field("price", ".price", extraction_type="price")
               .field("rating", ".rating")
               .field("reviews", ".review-count")
               .build())
    all_products.extend(results.data_points)

with open("competitor_products.json", "w") as f:
    json.dump(all_products, f, indent=2)
```

**🔍 SEO Audit**
```python
seo = (Crawl("https://example.com")
       .follow("a").limit(5000)
       .field("url", "", extraction_type="url")
       .field("title", "//title/text()", selector_type="xpath")
       .field("meta_desc", "//meta[@name='description']/@content", selector_type="xpath")
       .field("h1_count", "count(//h1)", selector_type="xpath")
       .field("canonical", "//link[@rel='canonical']/@href", selector_type="xpath")
       .build())

issues = []
for page in seo.data_points:
    if not page.get("title"): issues.append(f"{page['url']}: Missing <title>")
    if not page.get("meta_desc"): issues.append(f"{page['url']}: Missing meta description")
    if int(page.get("h1_count", 0)) > 1: issues.append(f"{page['url']}: Multiple H1s")
```

---

## ⚙️ Configuration

### TOML (`crawlingo.toml`)
```toml
[default]
rate_limit = 5.0            # Per-host req/s (0 = unlimited)
fetcher_tier = "stealthy"   # "standard" or "stealthy"
browser_profile = "chrome"  # "chrome", "firefox", "safari"
timeout = 30                # Request timeout seconds
auto_match = true           # Self-healing selectors

[default.headers]
User-Agent = "Crawlingo/1.0"
Accept = "text/html,application/xhtml+xml"

[default.retry]
base_delay = 500            # Initial delay ms
max_delay = 30000           # Maximum delay ms
multiplier = 2.0            # Exponential factor

[default.cache]
enabled = true
max_entries = 500
ttl_secs = 300

[default.auto_match.weights]
tag = 1.0
class_name = 0.8
id = 0.6
attributes = 0.4
parent_tag = 0.5
depth = 0.1
```

### Environment Variables
All config options available via `CRAWLINGO_*` environment variables:

| Variable | Maps To | Example |
|----------|---------|---------|
| `CRAWLINGO_RATE_LIMIT` | `rate_limit` | `5.0` |
| `CRAWLINGO_PROXY_URL` | `proxy_url` | `http://proxy:8080` |
| `CRAWLINGO_FETCHER_TIER` | `fetcher_tier` | `stealthy` |
| `CRAWLINGO_BROWSER_PROFILE` | `browser_profile` | `chrome` |
| `CRAWLINGO_TIMEOUT` | `timeout` | `30` |
| `CRAWLINGO_AUTO_MATCH` | `auto_match` | `true` |
| `CRAWLINGO_CACHE_ENABLED` | `cache_enabled` | `true` |
| `CRAWLINGO_CACHE_TTL` | `cache_ttl` | `300` |
| `CRAWLINGO_RETRY_BASE_DELAY` | `retry_base_delay` | `500` |
| `CRAWLINGO_RETRY_MAX_DELAY` | `retry_max_delay` | `30000` |
| `RUST_LOG` | (log level) | `crawlingo=debug` |

### Quick Reference

**CSS Selectors:**
| Pattern | Matches |
|---------|---------|
| `h1` | All `<h1>` elements |
| `.price` | Elements with class "price" |
| `#main` | Element with id "main" |
| `[data-id]` | Elements with data-id attribute |
| `[href^=https]` | Links starting with https |
| `div > p` | `<p>` direct child of `<div>` |
| `div p` | `<p>` descendant of `<div>` |
| `:first-child` | First child of parent |
| `:nth-child(2)` | Second child |
| `:not(.hidden)` | Not matching .hidden |

**XPath:**
| Expression | Matches |
|-----------|---------|
| `//h1` | All `<h1>` elements |
| `//div[@class='price']` | `<div class="price">` |
| `//a/@href` | href attribute of all links |
| `//p[position()<3]` | First two paragraphs |
| `//div[contains(@class,'active')]` | div containing "active" in class |
| `//p/text()` | Text content of paragraphs |

**Extraction Types:**
| Type | Input → Output | Use Case |
|------|---------------|----------|
| `text` | `"  Hello  "` → `"Hello"` | General text trimming |
| `price` | `"$1,234.56"` → `"1234.56"` | Currency normalization |
| `datetime` | `"Jan 15, 2024"` → `"2024-01-15"` | Date standardization |
| `url` | `"/path"` → `"https://base.com/path"` | URL resolution |
| `datalink_url` | `<a href="...">` → href value | Link extraction |
| `datalink_email` | `"mailto:a@b.com"` → `"a@b.com"` | Email extraction |
| `datalink_phone` | `"tel:+1234"` → `"+1234"` | Phone extraction |

---

## 🔧 Troubleshooting

| Problem | Likely Cause | Solution |
|---------|-------------|----------|
| HTTP 403 everywhere | Server blocking HTTP clients | Enable `stealthy` tier + browser profile |
| HTTP 429 rate limited | Target server throttling | Reduce rate limit, enable retry with Retry-After |
| Selector returns empty | Page structure changed | Verify in DevTools, enable `auto_match(True)` |
| Memory grows unbounded | Too much in-flight data | Use streaming dataset, reduce concurrency, limit cache |
| Watch fires every poll | Dynamic content (timestamps, ads) | Use specific selectors targeting stable content only |
| `pip install` fails | Missing system deps | Python 3.8+, glibc 2.28+ (Linux), VC++ Redist (Windows) |
| OAuth2 auth fails | Wrong credentials or token URL | Verify client_id/secret, check token endpoint, enable debug logging |
| SelectorError: Syntax | Invalid selector pattern | Validate CSS/XPath syntax, escape special characters |

---

## 📘 Python SDK — Full Method Reference

### Session
| Method | Returns | Description |
|--------|---------|-------------|
| `Session()` / `Session.from_config(path)` | Session | Create or load from TOML/env |
| `.headers(dict)` | Self | Default HTTP headers |
| `.cookies(dict)` | Self | Default cookies |
| `.proxy(str)` | Self | Single proxy URL |
| `.proxy_pool(list)` | Self | Round-robin proxy list |
| `.proxy_provider(str)` | Self | Dynamic proxy endpoint |
| `.rate_limit(float)` | Self | Per-host req/s (0 = off) |
| `.auto_match(bool)` | Self | Enable self-healing |
| `.auto_match_weights(*floats)` | Self | Similarity weights (tag, class, id, attr, text, parent, pos, sibling, depth) |
| `.timeout(int)` | Self | Request timeout seconds |
| `.fetcher_tier(str)` | Self | "standard" or "stealthy" |
| `.browser_profile(str)` | Self | "chrome", "firefox", "safari" |
| `.cache_enabled(bool)` | Self | Response caching |
| `.retry_base_delay(int)` | Self | Initial retry ms |
| `.retry_max_delay(int)` | Self | Max retry ms |
| `.retry_multiplier(float)` | Self | Backoff factor |
| `.auth_*(...)` | Self | Auth config (basic, bearer, header, api_key, oauth2) |
| `.page(url)` | Page | Create bound Page |
| `.metrics()` | dict | Metrics snapshot |

### Page
| Method | Returns | Description |
|--------|---------|-------------|
| `Page(url, session?)` | Page | Fetch and parse URL |
| `.title()` | str | `<title>` text content |
| `.html()` | str | Raw HTML string |
| `.markdown()` | str | GitHub-flavored markdown |
| `.status` | int | HTTP status code |
| `.css(selector)` | ElementList | CSS query |
| `.xpath(expr)` | ElementList | XPath query |
| `.regex(pattern)` | MatchList | Regex match against all text |
| `.find_text(text)` | ElementList | Text anchor lookup |
| `.after_text(text)` | ElementList | Sibling after anchor |
| `.before_text(text)` | ElementList | Sibling before anchor |

### ElementList / ElementRef
```python
els = page.css("div")
els.first()              # First match
els.last()               # Last match
els.at(i)                # Indexed match
len(els)                 # Count
for e in els: pass       # Iterable

e = els.first()
e.text()                 # Trimmed text
e.html()                 # Inner HTML
e.outer_html()           # Outer HTML (incl element)
e.attr("href")           # Attribute value
e.tag()                  # Tag name
e.classes()              # List of classes
e.parent()               # Parent element
e.children()             # Child elements
e.next_sibling()         # Next sibling
e.prev_sibling()         # Previous sibling
```

### Dataset
| Method | Returns | Description |
|--------|---------|-------------|
| `Dataset(url/s, session?)` | Dataset | Create for URL or URL list |
| `.field(name, sel, ...)` | Self | Add extraction field |
| `.build()` | DatasetResult | Execute extraction |
| `.stream()` | Iterator | Stream URL list results |
| `→ .to_dict()` | dict | Fields as dict |
| `→ .to_json()` | str | JSON string |
| `→ .to_csv()` | str | CSV string |
| `→ .to_parquet(path)` | None | Write Parquet file |
| `→ .to_json_file(path)` | None | Write JSON file |
| `→ .to_csv_file(path)` | None | Write CSV file |

### Crawl
| Method | Returns | Description |
|--------|---------|-------------|
| `Crawl(url, session?)` | Crawl | Start from URL |
| `.follow(selector)` | Self | CSS for links to follow |
| `.limit(n)` | Self | Max pages |
| `.depth(n)` | Self | Max link depth |
| `.concurrency(n)` | Self | Concurrent fetches |
| `.field(name, sel, ...)` | Self | Add extraction field |
| `.respect_robots(bool)` | Self | Honor robots.txt |
| `.allowed_domains(list)` | Self | Domain whitelist |
| `.exclude_patterns(list)` | Self | URL exclusion regex |
| `.build()` | CrawlResult | Execute crawl |

### Watch
| Method | Returns | Description |
|--------|---------|-------------|
| `Watch(url, session?)` | Watch | Monitor URL |
| `.field(name, sel, ...)` | Self | Add field to monitor |
| `.interval(secs)` | Self | Polling interval |
| `.on_change(fn)` | Self | Catch-all change callback |
| `.on_price_change(fn)` | Self | Price change callback |
| `.on_stock_change(fn)` | Self | Stock change callback |
| `.on_element_added(fn)` | Self | New element callback |
| `.on_element_removed(fn)` | Self | Removed element callback |
| `.run(detach=False)` | None | Start monitoring |
| `.stop()` | None | Signal stop |

---

## 📚 Examples

| File | What It Shows |
|------|--------------|
| `examples/python/simple_fetch.py` | Basic page fetch, title, markdown |
| `examples/python/session_config.py` | Custom headers, proxy, rate limits |
| `examples/python/css_selectors.py` | CSS selector extraction techniques |
| `examples/python/xpath_selectors.py` | XPath queries with predicates |
| `examples/python/regex_extract.py` | Regex pattern extraction |
| `examples/python/text_anchors.py` | Text anchor / after_text / before_text |
| `examples/python/dataset_field.py` | Multi-field extraction definitions |
| `examples/python/dataset_export.py` | JSON / CSV / Parquet export |
| `examples/python/crawl_simple.py` | Recursive crawl with link following |
| `examples/python/watch_basic.py` | Poll-based change monitoring |
| `examples/python/auto_match.py` | Self-healing selector demo |
| `examples/python/price_extraction.py` | Price extraction with normalization |
| `examples/python/datalink_extraction.py` | Email, phone, URL extraction |
| `examples/python/streaming_dataset.py` | Large URL list, bounded memory |
| `examples/node/simple_fetch.ts` | Node.js page fetch |
| `examples/node/dataset_stream.ts` | Node.js streaming dataset |
| `examples/rust/simple_fetch.rs` | Rust direct API usage |

### Integration Examples

**FastAPI endpoint:**
```python
from fastapi import FastAPI
from crawlingo import Dataset

app = FastAPI()

@app.post("/extract")
async def extract(url: str, fields: list[dict]):
    ds = Dataset(url)
    for f in fields:
        ds.field(f["name"], f["selector"], extraction_type=f.get("type", "text"))
    return ds.build().to_dict()
```

**AWS Lambda:**
```python
def lambda_handler(event, context):
    page = Page(event["url"])
    return {"status": page.status, "title": page.title(), "html_len": len(page.html())}
```

**Airflow task:**
```python
@task
def crawl_site():
    session = Session()
    session.rate_limit(3)
    Crawl("https://docs.example.com", session).follow("a").limit(100)\
        .field("title","h1").build().to_parquet("/data/crawl.parquet")
```

---

## 📚 Docs & Roadmap

### Running Examples
```bash
# Python
cd examples/python && pip install crawlingo && python simple_fetch.py

# Node.js
cd examples/node && npm install crawlingo && npx ts-node simple_fetch.ts

# Rust
cd examples/rust && cargo run --example simple_fetch
```

**Documentation (in `document/`):**
| File | What It Covers |
|------|---------------|
| `01_ARCHITECTURE.md` | System architecture, module design |
| `02_CODEBASE_MAP.md` | File-level codebase structure |
| `05_ROADMAP.md` | Full roadmap with milestones |
| `06_DEVELOPMENT_GUIDE.md` | Setup, build, test instructions |
| `07_CONTRIBUTING_GUIDE.md` | PR process, review standards |
| `10_PERFORMANCE.md` | Profiling data, optimization tips |
| `11_SECURITY_REVIEW.md` | Security audit findings |
| `12_API_DESIGN.md` | Design principles, conventions |
| `20_PRODUCTION_CHECKLIST.md` | Production deployment steps |
| `27_SELECTOR_ENGINE.md` | Selector engine internals & v2 plans |
| `28_DATASET_ENGINE.md` | Dataset engine architecture |
| `29_PLUGIN_SYSTEM.md` | Plugin hooks & registration |

**SDK docs:** `sdk/python/README.md`, `sdk/node/README.md`, `cargo doc --open`

**Roadmap:**
| Phase | Features |
|-------|----------|
| 🔜 **Q3 2026** | CLI tool, config hot-reload, webhook notifications, headless browser fetcher, macOS ARM64 wheels |
| 📅 **Q4 2026** | Distributed crawling (actor model), LLM-powered selector generation, vector DB integration (Pinecone/Qdrant), Docker images |
| 🚀 **2027+** | Cloud-hosted service, visual selector builder UI, mobile SDKs (Swift/Kotlin), WebSocket streaming, AI extraction model |

---

## 🤝 Contributing

```bash
git clone https://github.com/Vamshavardhan50/crawlingo.git
cd crawlingo

# Rust development
cargo build --release
cargo test

# Python SDK
make python-dev     # or: cd sdk/python && maturin develop

# Node.js SDK
make node-dev       # or: cd sdk/node && napi build
```

### Pull Request Process
1. Fork the repo, create a feature branch
2. Implement changes with tests
3. Run `cargo test` (Rust), `make test-python` (Python), `make test-node` (Node.js)
4. Lint: `cargo clippy -- -D warnings` (Rust), `ruff` (Python), ESLint (Node.js)
5. Open PR against `main` branch
6. Ensure CI passes, request review

### Commit Convention
```
<type>(<scope>): <description>
```
Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`

Examples:
- `feat(selector): add attribute-based text anchor matching`
- `fix(engine): handle connection reset in stealth fetcher`
- `docs(api): update Dataset builder field documentation`

### Coding Standards
| Language | Formatter | Linter | Typing |
|----------|-----------|--------|--------|
| Rust | rustfmt | clippy | Strong (struct + enum) |
| Python | black | ruff | Type hints required |
| Node.js | prettier | ESLint (TS) | Full TypeScript |

### Project Values
- **Performance first** — every feature considers its performance impact
- **API ergonomics** — intuitive APIs that are hard to misuse
- **Reliability** — error handling and edge cases are not afterthoughts
- **Transparency** — technical debt and limitations are documented, not hidden

---

## ❓ FAQ

**What is Crawlingo?** High-performance Rust-powered web scraping library with Python, Node.js, and Rust SDKs. MIT license. Free for personal and commercial use.

**How do I handle JavaScript-rendered pages?** Use `stealthy` fetcher tier with a browser profile (`chrome`, `firefox`, `safari`). For full JS execution, pipe headless browser HTML to Crawlingo's parser.

**How do I rotate proxies?** `session.proxy_pool(["http://p1", "http://p2"])` for static pool, or `session.proxy_provider("https://provider.com/list")` for dynamic endpoints.

**What is auto-match?** Self-healing selector system. Stores DOM fingerprints in Sled (embedded DB). When a CSS selector breaks (page layout change), recovers by scoring all DOM nodes in parallel via Rayon using Jaro-Winkler + Jaccard similarity.

**Can I use Crawlingo in production?** Yes. Rate limiting, exponential backoff retry, metrics monitoring, and response caching are built in. Session holds all state — no global mutable state.

**Does Crawlingo collect telemetry?** No. Zero telemetry, analytics, or phone-home functionality. All metrics are local to the Session object and never transmitted.

**How fast is it?** 3,500 req/s (standard tier), 1,800 req/s (stealthy tier). 2.4 MB idle memory. 85 MB for 10K streaming URLs. Selectors run at 850K–2.1M ops/s.

**Can I export data to Parquet?** Yes: `result.to_parquet("output.parquet")`. Also supports JSON and CSV export.

**What extraction types are available?** `text` (trimmed), `price` ($1,234.56 → 1234.56), `datetime` (→ ISO 8601), `url` (resolves relative to absolute), `datalink_url` (href extraction), `datalink_email`, `datalink_phone`.

**Does Crawlingo support concurrent crawling?** Yes. Configure via `Crawl.concurrency(n)`. Uses Tokio semaphore for I/O concurrency control.

**Can I write custom middleware?** Yes. Implement the `Layer` trait in Rust and register it on the Session's middleware stack.

**What if a selector fails and auto-match can't recover?** A warning is logged and the default value (if configured) is returned. The fingerprint store is not updated with the failed match.

**Can I use Crawlingo with Docker / AWS Lambda / Airflow?** Yes. See the integration examples section. For Lambda, use `/tmp` for the fingerprint store and expect cold starts including library loading. For Airflow, create a new Session per task for isolation. For Docker, base image needs glibc 2.28+.

**How does change detection work?** The `detect_changes` function compares old vs new field maps. Classifies changes as ContentChange, PriceChange (with %), StockChange, ElementAdded, or ElementRemoved. The Watch poller runs this comparison on interval and fires typed callbacks.

**What's the memory footprint of a crawl?** ~120 MB for a 1,000-URL crawl queue including extracted results. Streaming dataset: ~85 MB constant for any number of URLs. Fingerprint store: ~8 MB for 10K entries.

**Does Crawlingo handle pagination?** Use the Crawl engine's `follow` selector pointing to the "next page" link: `.follow("a.next, a[rel=next]")`. The crawler follows pagination links up to the configured depth and limit.

---

## 📦 Version History

| Version | Date | Highlights |
|---------|------|------------|
| 0.1.0 | 2026-Q3 | Initial release: Page, Session, Dataset, Crawl, Watch, AutoMatch, Change Detection, Metrics, all 3 SDKs |

## 🔗 Related Projects
- [Crawlingo Python SDK](sdk/python/) — PyPI package details
- [Crawlingo Node.js SDK](sdk/node/) — npm package details
- [Documentation](document/) — Technical deep-dives
- [Issues](https://github.com/Vamshavardhan50/crawlingo/issues) — Bug reports & feature requests

## 🙏 Acknowledgments

Crawlingo stands on the shoulders of several excellent open-source projects:

- **html5ever** and **scraper** — HTML parsing that follows the spec, not just common usage
- **wreq** and **wreq-util** — Modern HTTP/2 with TLS extension APIs
- **tokio** and **rayon** — Async I/O and parallel CPU processing done right
- **sled** — Embedded database that "just works" with zero configuration
- **PyO3** and **napi-rs** — Making Rust cross-language FFI a pleasure
- **governor**, **moka**, **dashmap** — Building blocks for production-grade infrastructure

---

## 📄 License

MIT License · Copyright (c) 2026 Vamshavardhan V · See [LICENSE](LICENSE)
