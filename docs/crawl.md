# Crawl API

The `Crawl` class performs multi-page recursive crawling from a starting URL. It follows links, extracts data from each page, and collects results.

## Python

```python
from crawlingo import Crawl

results = (
    Crawl("https://docs.example.com")
    .follow("a[href^='/docs']")    # CSS selector for links to follow
    .limit(100)                      # Max pages to crawl
    .depth(3)                        # Max link depth
    .concurrency(5)                  # Concurrent requests
    .delay(1.0)                      # Delay between requests (seconds)
    .field("title", "h1")
    .field("content", "main p")
    .build()
)

print(f"Crawled {len(results)} pages")
for page in results:
    print(f"  {page['url']}: {page['title']}")

# Export
results.to_json("crawl_output.json")
results.to_csv("crawl_output.csv")
results.to_parquet("crawl_output.parquet")
```

## Node.js

```javascript
const { Crawl } = require('crawlingo');

const results = await new Crawl('https://docs.example.com')
  .follow('a[href^="/docs"]')
  .limit(100)
  .depth(3)
  .concurrency(5)
  .delay(1.0)
  .field('title', 'h1')
  .run();

console.log(`Crawled ${results.length} pages`);
for (const page of results) {
  console.log(`  ${page.toDict().url}: ${page.toDict().title}`);
}
```

## Parameters

| Method | Default | Description |
|--------|---------|-------------|
| `follow(selector)` | — | CSS selector for links to crawl |
| `limit(n)` | `1000` | Maximum pages to crawl |
| `depth(n)` | `5` | Maximum link depth from start URL |
| `concurrency(n)` | `5` | Concurrent request count |
| `delay(secs)` | `0.5` | Delay between requests (seconds) |
| `field(name, sel)` | — | Extract fields from each page |
| `session(session)` | — | Shared Session configuration |
| `same_domain(bool)` | `True` | Restrict to same domain |
| `respect_robots(bool)` | `True` | Respect robots.txt |
| `user_agent(string)` | — | Custom User-Agent |

## Field Extraction

Works identically to [Dataset](dataset.md) fields — supports CSS, XPath, regex, text anchors, and extraction types.

```python
Crawl(url)
    .field("title", "h1")
    .field("price", "span.price", extraction_type="price")
    .field("email", "a[href^=mailto]", extraction_type="datalink_email")
```

## Rate Limiting and Politeness

```python
# Per-host rate limit
session = Session()
session.rate_limit(5.0)
Crawl(url, session=session).delay(1.0)

# Respect robots.txt
Crawl(url).respect_robots(True)

# Custom delay between pages
Crawl(url).delay(2.0)
```

## Session Integration

```python
from crawlingo import Session, Crawl

with Session() as session:
    session.headers({"User-Agent": "MyCrawler/1.0"})
    session.rate_limit(3.0)
    session.proxy("http://proxy:8080")

    results = Crawl("https://example.com", session=session).follow("a").limit(50).build()
```

## Error Handling

Failed pages are collected but don't stop the crawl:

```python
results = Crawl(url).follow("a").limit(100).build()

for page in results:
    if page.error:
        print(f"Failed: {page.url} — {page.error}")
    else:
        print(f"OK: {page.url} — {page.fields}")
```
