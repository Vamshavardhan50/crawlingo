# Crawlingo Node.js SDK

<p align="center">
  <img src="https://raw.githubusercontent.com/Vamshavardhan50/crawlingo/main/crawlingo.jpg" width="180" alt="Crawlingo Logo" />
</p>

**Crawlingo** is a next-generation web data extraction, crawling, and website monitoring framework. It wraps a high-performance Rust core in an elegant, React-inspired developer-first Node.js API with complete TypeScript support.

---

## Installation

```bash
npm install crawlingo
```

---

## Features

- 🛡️ **Stealthy Fetcher**: Bypasses bot verification systems by rotating TLS/JA3 profiles and HTTP/2 settings natively (no headless browsers required).
- 🧠 **Auto-Match Selector Healing**: Stores DOM node fingerprints using local embedded DB files to recover broken/drifted tags automatically.
- 🔄 **IP Proxy Pools**: Cycle requests round-robin using static list pools or remote proxy list provider API URLs.
- ⏰ **Scheduled Crawling**: Run background crawlers recurringly on specific time interval loops.
- 📡 **Webhooks**: Deliver scraped JSON records in real-time straight to ingestion endpoints.

---

## Quick Start

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

## License

MIT License. See [LICENSE](../../LICENSE) file.
