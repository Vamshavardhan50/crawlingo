# Node.js SDK

## Installation

```bash
npm install crawlingo
```

Requires Node.js 18+. Pre-built native addons for Linux (x64), macOS (x64, arm64), and Windows (x64).

## Quick Reference

```javascript
const { Page, Session, Dataset, Crawl, Watch } = require('crawlingo');
```

## API Summary

### Page
```javascript
const page = await Page.create('https://example.com', session?);

page.status;                       // HTTP status code
page.title();                      // Page title
page.html();                       // Raw HTML
page.markdown();                   // Markdown content

const elements = page.css('h1');   // CSS selector
elements.text;                     // Array of texts
elements.at(0).html;               // Inner HTML
elements.at(0).attr('href');       // Attribute value

page.xpath('//h1');                // XPath query
page.regex(/pattern/);             // Regex search
page.findText('Buy Now');          // Text anchor
page.afterText('Price:');          // Text boundary
page.beforeText(' - Details');     // Text boundary
```

### Session
```javascript
const session = new Session();
session.headers({'User-Agent': 'MyBot/1.0'});
session.proxy('http://proxy:8080');
session.rateLimit(5.0);
session.autoMatch(true);
session.fetcherTier('stealthy');
session.browserProfile('chrome');
session.timeout(30);
session.fingerprintPath('./fingerprints');
```

### Dataset
```javascript
const result = await new Dataset('https://example.com')
  .autoMatch(true)
  .field('title', 'h1')
  .field('price', 'span.price', { extractionType: 'price' })
  .build();

console.log(result.toDict());
await result.toJson('output.json');
await result.toCsv('output.csv');
```

### Crawl
```javascript
const results = await new Crawl('https://docs.example.com')
  .follow('a[href^="/docs"]')
  .limit(100)
  .depth(3)
  .concurrency(5)
  .delay(1.0)
  .field('title', 'h1')
  .run();

console.log(`Crawled ${results.length} pages`);
```

### Watch
```javascript
const watcher = new Watch('https://example.com')
  .field('price', 'span.price', { extractionType: 'price' })
  .interval(60);

watcher.run((err, event) => {
  if (err) return console.error(err);
  console.log(`${event.field} changed: ${event.oldValue} → ${event.newValue}`);
});

// Later: watcher.stop();
```

## Async Usage

All I/O methods are async:

```javascript
const { Page, Dataset, Crawl } = require('crawlingo');

async function main() {
  const page = await Page.create('https://example.com');
  console.log(page.title());

  const result = await new Dataset('https://example.com')
    .field('title', 'h1')
    .build();

  const pages = await new Crawl('https://example.com')
    .follow('a')
    .limit(10)
    .run();
}

main().catch(console.error);
```

## TypeScript Support

The npm package includes TypeScript declarations:

```typescript
import { Page, Session, Dataset, Crawl, Watch } from 'crawlingo';

const page = await Page.create('https://example.com');
const title: string = page.title();
```

## Native Addon

The Node.js SDK is implemented as a native N-API addon built with `napi-rs`. The addon is pre-compiled for:

| OS | Architecture | File |
|----|-------------|------|
| Linux | x86_64 | `crawlingo-native.linux-x64-gnu.node` |
| macOS | x86_64 | `crawlingo-native.darwin-x64.node` |
| macOS | arm64 | `crawlingo-native.darwin-arm64.node` |
| Windows | x64 | `crawlingo-native.win32-x64-msvc.node` |

## Error Handling

```javascript
try {
  const page = await Page.create('https://example.com');
  const title = page.css('h1').text;
} catch (err) {
  console.error('Scraping failed:', err.message);
}
```
