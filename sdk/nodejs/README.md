<h1 align="center">
  <a href="https://crawlingo-docs.vercel.app">
    <img alt="Crawlingo" src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/assets/Logo%20and%20name.png" width="560">
  </a>
  <br>
  <small>Node.js SDK — Self-Healing Web Scraping for JavaScript &amp; TypeScript</small>
</h1>

<p align="center">
  <a href="https://github.com/Vamshavardhan50/crawlingo/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/Vamshavardhan50/crawlingo/ci.yml?branch=main&style=flat-square&logo=github&label=CI" /></a>
  <a href="https://www.npmjs.com/package/crawlingo"><img src="https://img.shields.io/npm/v/crawlingo?style=flat-square&logo=nodedotjs&color=red&label=npm" alt="npm version" /></a>
  <a href="https://www.npmjs.com/package/crawlingo"><img src="https://img.shields.io/npm/dm/crawlingo?style=flat-square" alt="npm Downloads" /></a>
  <a href="https://github.com/Vamshavardhan50/crawlingo/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Vamshavardhan50/crawlingo?style=flat-square&label=License" alt="License" /></a>
  <a href="https://crawlingo-docs.vercel.app/docs"><img src="https://img.shields.io/badge/docs-crawlingo.dev-6366F1?style=flat-square" alt="Docs" /></a>
</p>

<p align="center">
  <a href="#installation"><strong>Installation</strong></a> ·
  <a href="#why-crawlingo"><strong>Why Crawlingo</strong></a> ·
  <a href="#features"><strong>Features</strong></a> ·
  <a href="#quick-start"><strong>Quick Start</strong></a> ·
  <a href="#ai-benchmarks"><strong>LLM Benchmarks</strong></a>
</p>

---

**Crawlingo Node.js SDK** is a next-generation web data extraction, crawling, and website monitoring library. It wraps a high-performance Rust core in a TypeScript-first API — scraping workflows that survive page design shifts automatically.

📚 **Full API reference and guides at [crawlingo.dev/docs](https://crawlingo-docs.vercel.app/docs)**

---

## 🎥 Demo

<p align="center">
  <img src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/assets/crawlingo_demo.webp" alt="Crawlingo Self-Healing Demo" width="600">
</p>

### How Self-Healing Works:
1. **Drift Detection** — When `button#submit.btn-primary` renames to `button#send-btn.btn-primary-new`, traditional scrapers return empty.
2. **DOM Parsing** — Crawlingo's Rust engine intercepts the mismatch and isolates candidates within the parent node.
3. **Jaro-Winkler Matching** — Candidates ranked by tag, attributes, text content, and structural fingerprints.
4. **Auto-Match Recovery** — Highest-scoring candidate (e.g. **94% confidence**) auto-bound and cached — zero downtime.

---

## 📦 Installation

<a id="installation"></a>

```bash
npm install crawlingo
```

Or with yarn / pnpm:

```bash
yarn add crawlingo
pnpm add crawlingo
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
| Full TypeScript Support | ✅ | ❌ | ❌ |
| AI / MCP Ready | ✅ | ❌ | ✅ |

---

## 🛠️ Core Features

<a id="features"></a>

- **🧠 Self-Healing DOM Fingerprinting** — Tracks layout changes via Jaro-Winkler. [Learn more](https://crawlingo-docs.vercel.app/docs/features#auto-match-self-healing)
- **🛡️ Stealth Browser Impersonation** — Bypasses Cloudflare, Akamai via HTTP/2 TLS fingerprint rotation. [Learn more](https://crawlingo-docs.vercel.app/docs/features#stealthy-browser-impersonation)
- **⚡ SIMD-Accelerated Text Anchors** — Faster than CSS/XPath via vector math. [Learn more](https://crawlingo-docs.vercel.app/docs/features#text-anchor-simd-accelerated)
- **🔄 High-Speed Proxy Rotation** — Automatic round-robin proxy cycling. [Learn more](https://crawlingo-docs.vercel.app/docs/spiders#proxy-rotation)
- **⏰ Reactive Watch Monitors** — Background polling with webhook notifications on changes. [Learn more](https://crawlingo-docs.vercel.app/docs/features#change-monitoring-watches)
- **🤖 Built-in MCP Server** — Native Claude/Cursor integration. [Learn more](https://crawlingo-docs.vercel.app/docs/ai/mcp-server)
- **📦 Schema-Driven Datasets** — Export to JSON, CSV, or Arrow. [Learn more](https://crawlingo-docs.vercel.app/docs/features#multi-format-exports)

---

## ⚡ Quick Start

<a id="quick-start"></a>

### 1. Basic Extraction

```typescript
import { Session, Page } from 'crawlingo';

const session = new Session();
session.autoMatch(true);

const page = await Page.create("https://example.com", { session });
console.log("Title:", page.title());
console.log("Headings:", page.css("h1").text.join(", "));
```

### 2. Self-Healing Dataset

```typescript
import { Dataset } from 'crawlingo';

const result = await new Dataset("https://example.com/products")
  .autoMatch(true)
  .field("title", "h1.product-title")
  .field("price", "span.price", { extractType: "price" })
  .field("in_stock", ".stock-badge")
  .build();

console.log(result.toDict());
result.toJsonFile("products.json");
```

### 3. Multi-Page Crawl with Webhooks

```typescript
import { Session, Crawl } from 'crawlingo';

const session = new Session().proxyPool([
  "http://proxy1.example.com:8080",
  "http://proxy2.example.com:8080"
]);

const crawl = new Crawl("https://example.com/products", session);
crawl.follow("a.next-page");
crawl.field("title", "h1");
crawl.webhook("https://my-api.com/webhooks/crawl");
crawl.schedule(3600); // run every hour
```

### 4. Watch Monitor for Changes

```typescript
import { Watch } from 'crawlingo';

const watch = new Watch("https://example.com/item")
  .field("price", "span.item-price")
  .interval(60)
  .onPriceChange((event) => {
    console.log(`Price: ${event.oldValue} → ${event.newValue}`);
  });

await watch.runAsync();
```

---

## 🤖 AI / LLM Benchmarks

<a id="ai-benchmarks"></a>

| LLM Model | Context | Speed | Cost / 1M tok | Markdown Accuracy | MCP |
|-----------|---------|-------|---------------|-------------------|-----|
| **Claude 3.5 Sonnet** | 200k | ~80 tok/s | $3/$15 | 👑 98% | ✅ Native |
| **GPT-4o** | 128k | ~90 tok/s | $2.5/$10 | 95% | ✅ Gateway |
| **Gemini 1.5 Pro** | 2M | ~60 tok/s | $1.25/$5 | 92% | ⚠️ Experimental |
| **Llama 3.1 70B** | 128k | ~45 tok/s | $0.60/$0.60 | 88% | ❌ Needs wrapper |

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
