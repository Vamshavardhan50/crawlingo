<h1 align="center">
    <a href="https://crawlingo.dev">
        <img alt="Crawlingo Cover" src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/crawlingo.jpg" width="600">
    </a>
    <br>
    <small>Crawlingo Node.js SDK - Self-Healing Web Scraping for JS/TS</small>
</h1>

<p align="center">
    <a href="https://github.com/Vamshavardhan50/crawlingo/actions/workflows/ci.yml"><img alt="Build Status" src="https://img.shields.io/github/actions/workflow/status/Vamshavardhan50/crawlingo/ci.yml?branch=main&style=flat-square&logo=github&label=Build" /></a>
    <a href="https://www.npmjs.com/package/crawlingo"><img src="https://img.shields.io/npm/v/crawlingo?style=flat-square&logo=nodedotjs&color=red&label=NPM" alt="NPM Version" /></a>
    <a href="https://www.npmjs.com/package/crawlingo"><img src="https://img.shields.io/npm/dm/crawlingo?style=flat-square" alt="NPM Downloads" /></a>
    <a href="https://github.com/Vamshavardhan50/crawlingo/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Vamshavardhan50/crawlingo?style=flat-square&label=License" alt="License" /></a>
</p>

<p align="center">
    <a href="#installation"><strong>Installation</strong></a>
    &middot;
    <a href="#why-crawlingo"><strong>Why Crawlingo</strong></a>
    &middot;
    <a href="#features"><strong>Core Features</strong></a>
    &middot;
    <a href="#quick-start"><strong>Quick Start</strong></a>
    &middot;
    <a href="#ai-benchmarks"><strong>LLM Benchmarks</strong></a>
</p>

---

**Crawlingo Node.js SDK** is a next-generation web data extraction, crawling, and website monitoring library. It wraps a high-performance Rust core in an elegant React-inspired developer-first Node.js API with complete TypeScript support, allowing you to build scraping workflows that survive page design shifts.

📚 **Read the full guide and API references at [crawlingo.dev/docs](https://crawlingo.dev/docs)**

---

## 🎥 30-Second Demo

Watch Crawlingo's self-healing DOM selector engine dynamically recover element references when a website's layout/DOM structure drifts:

![Crawlingo Self-Healing Demo](../../crawlingo_demo.webp)

### How Self-Healing Works Under the Hood:
1. **Drift Detection**: When the target element (e.g., `button#submit.btn-primary`) undergoes styling or structure updates (e.g., renamed to `button#send-btn.btn-primary-new`), traditional scrapers fail and return empty results.
2. **Dynamic DOM Parsing**: Crawlingo's Rust engine intercepts the mismatch, loads the active DOM, and isolates candidates within the parent node coordinates.
3. **Jaro-Winkler Similarity Comparison**: The engine ranks candidates by checking tag names, surrounding attributes, text contents, and deep structural fingerprints.
4. **Auto-Match Recovery**: The candidate with the highest similarity score exceeding the threshold (e.g., **94% confidence**) is automatically bound, updating the cache without breaking your production data pipeline.

---

## 📦 Installation

<a id="installation"></a>

Install the package directly from npm:

```bash
npm install crawlingo
```

---

## 🚀 Why Crawlingo?

<a id="why-crawlingo"></a>

Traditional scrapers break when websites change their class names, IDs, or HTML structures (**selector drift**). Crawlingo solves this by caching element layout fingerprints and using similarity matching heuristics to self-heal and find drifted elements on the fly.

### Comparison Matrix

| Feature | Crawlingo | Scrapy | Crawl4AI |
|----------|------------|---------|---------|
| Rust Core | ✅ | ❌ | ❌ |
| Python SDK | ✅ | ✅ | ✅ |
| Node SDK | ✅ | ❌ | ❌ |
| AI Agent Ready | ✅ | ⚠️ | ✅ |
| Change Monitoring | ✅ | ❌ | ❌ |
| Dataset Extraction | ✅ | ⚠️ | ⚠️ |
| Cross Language | ✅ | ❌ | ❌ |

---

## 🛠️ Core Features

<a id="features"></a>

Crawlingo packs all components required to scrape, watch, and pipe modern web pages under JavaScript/TypeScript:

*   **🧠 Self-Healing DOM Fingerprinting**: Tracks layout changes and leverages Jaro-Winkler calculations dynamically. [Learn more](https://crawlingo.dev/docs/features#auto-match-self-healing).
*   **🛡️ Stealth Browser Impersonation**: Bypasses bot verification systems (Cloudflare, etc.) using high-performance HTTP/2 TLS fingerprint rotation. [Learn more](https://crawlingo.dev/docs/features#stealthy-browser-impersonation).
*   **⚡ SIMD-Accelerated Text Anchors**: CSS/XPath is great, but anchoring relative to text values using vector calculations is faster. [Learn more](https://crawlingo.dev/docs/features#text-anchor-simd-accelerated).
*   **🔄 High-Speed Proxy Rotation**: Automatically rotates proxy configurations inside background crawling loops. [Learn more](https://crawlingo.dev/docs/spiders#proxy-rotation).
*   **⏰ Reactive Watch Monitors**: Run background threads that poll websites and notify handlers upon layout shifts or price changes. [Learn more](https://crawlingo.dev/docs/features#change-monitoring-watches).
*   **🤖 Built-in MCP Server**: Native server that connects scraping tool functions straight to Claude Code or Cursor. [Learn more](https://crawlingo.dev/docs/ai/mcp-server).
*   **📦 Schema-Driven Datasets**: Map results and export them straight to JSON, CSV, Apache Arrow, or Pandas DataFrames. [Learn more](https://crawlingo.dev/docs/features#multi-format-exports).

---

## ⚡ Quick Start

<a id="quick-start"></a>

### 1. Basic Web Scrape
```typescript
import { Session } from 'crawlingo';

const session = new Session();
session.autoMatch(true);

const page = await session.page("https://example.com");
console.log("Page Title:", page.title());

const headings = page.css("h1");
console.log("Header text:", headings.text());
```

### 2. Multi-Page Crawling & Webhooks
```typescript
import { Session, Crawl } from 'crawlingo';

const session = new Session();
session.proxyPool([
  "http://proxy1.example.com:8080",
  "http://proxy2.example.com:8080"
]);

const crawl = new Crawl("https://example.com/products", session);
crawl.follow("a.next-page");
crawl.field("title", "h1");

// Deliver items to a webhook endpoint in real-time
crawl.webhook("https://my-api.com/webhooks/crawl");

// Run background crawl loops every hour
crawl.schedule(3600);
```

---

## 🤖 AI LLM Ingestion & Benchmarking

<a id="ai-benchmarks"></a>

For web parsing pipelines feeding LLM context or RAG indices, Crawlingo provides structured inputs. The table below outlines how different AI models compare on processing raw scraped web pages for automated RAG/extraction tasks:

| Model | Context Window | Speed (tok/s) | Avg. Cost / 1M Tok | Markdown Parsing Accuracy | Native MCP Support |
|-----------|----------------|---------------|--------------------|---------------------------|---------------------|
| **Claude 3.5 Sonnet** | 200k | ~80 | $3.00 / $15.00 | 👑 **98%** (Best for tables/JSON) | ✅ Native |
| **GPT-4o** | 128k | ~90 | $2.50 / $10.00 | **95%** (Excellent formatting) | ✅ Via Gateway |
| **Gemini 1.5 Pro** | 2M | ~60 | $1.25 / $5.00 | **92%** (Huge content ingestion) | ⚠️ Experimental |
| **Llama 3.1 70B** | 128k | ~45 | $0.60 / $0.60 | **88%** (Great open-source alternative) | ❌ Needs wrapper |

---

## 📝 License

MIT License. See [LICENSE](../../LICENSE) file.
