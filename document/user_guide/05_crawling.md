# Crawling Multiple Pages

Crawl entire websites with configurable depth, concurrency, and rate limiting.

## Basic Usage

```python
from crawlingo import Session

session = Session()
session.rate_limit(2.0)

crawl = session.crawl("https://example.com")
crawl.follow("a.next-page")
crawl.limit(50)
crawl.depth(3)
crawl.concurrency(5)
crawl.delay(0.5)

results = crawl.build()

for result in results:
    print(result)
```

## With Structured Extraction

Combine crawling with field extraction for large-scale data collection:

```python
from crawlingo import Session

session = Session()
session.auto_match(True)

crawl = session.crawl("https://shop.example.com/categories")
crawl.follow("a.product-link")
crawl.depth(3)
crawl.limit(200)
crawl.concurrency(10)
crawl.field("title", "h1", selector_type="css")
crawl.field("price", "span.price", selector_type="css")
crawl.field("sku", ".sku", default="N/A")

results = crawl.build()
results.to_csv("output/all_products.csv")
```

## Crawl Algorithm

1. Start URL enqueued at depth 0.
2. Workers pop URLs, fetch, extract links, and run field selectors.
3. Links matching `follow` selector from depth `d` are enqueued at depth `d+1`.
4. Duplicate URLs are skipped.
5. All workers share a single `Session` for consistent rate limiting and cookie state.

## Rate Limiting

Per-host rate limiting configured in the shared `Session`:

```python
session.rate_limit(2.0)  # Max 2 requests per second per host
```

## Webhooks & Scheduling

```python
crawl.webhook("https://my-api.com/webhook")
crawl.schedule(3600)  # Re-crawl every hour
```

## Next Steps

- [Change Detection](06_change_detection.md): Monitor pages for updates over time.
- [Crawl API Reference](../api_reference/crawler.md): Full API documentation.
