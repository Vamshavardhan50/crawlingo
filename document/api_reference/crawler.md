# crawlingo.Crawl

Multi-page concurrent crawler that discovers and fetches pages from a start URL with configurable depth, concurrency, and field extraction.

## Constructor

```python
crawl = Crawl(start_url, session=None)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `start_url` | `str` | — | Starting URL for the crawl |
| `session` | `Session` | `None` | Optional session for shared configuration |

## Methods

All configuration methods return `self` for fluent chaining.

| Method | Parameters | Description |
|--------|-----------|-------------|
| `.follow(selector: str)` | CSS selector for links to follow | Set CSS selector for links to queue recursively |
| `.limit(pages: int)` | Maximum page count | Limit the crawler to a maximum number of pages |
| `.depth(max_depth: int)` | Maximum link hop depth | Set maximum crawl depth level |
| `.field(name, selector, selector_type="css", default=None)` | Field name, selector, type, default | Define an extraction field for every page |
| `.auto_match(enabled: bool)` | Enable/disable self-healing | Enable auto-match recovery |
| `.concurrency(n: int)` | Worker thread count | Set max concurrent fetching workers |
| `.delay(seconds: float)` | Pacing delay in seconds | Set politeness delay between requests |
| `.webhook(url: str)` | Webhook endpoint URL | Set webhook for real-time JSON delivery |
| `.schedule(interval_seconds: int)` | Interval in seconds | Schedule recurring crawl in background |
| `.build()` | — | Execute crawl synchronously, returns `CrawlResults` |

## CrawlResults

| Method | Description |
|--------|-------------|
| `.to_json(path: str)` | Export all results to a JSON file |
| `.to_csv(path: str)` | Export all results to a CSV file |
| `.to_parquet(path: str)` | Export all results to a Parquet file |
| `.df()` | Convert results to a Pandas DataFrame |
| `__iter__`, `__len__`, `__getitem__` | Iterate, count, and index results |

## Examples

```python
from crawlingo import Session

session = Session()
session.rate_limit(2.0)
session.auto_match(True)

crawl = session.crawl("https://example-blog.com/posts")
crawl.follow("a.next-page")
crawl.limit(50)
crawl.depth(3)
crawl.field("title", "h1.entry-title")
crawl.field("author", "span.author-name", default="Anonymous")
crawl.concurrency(5)
crawl.delay(0.5)

results = crawl.build()
results.to_csv("output_posts.csv")

# Or schedule recurring crawls
crawl.schedule(3600)  # Every hour
```

## See Also

- [`Session`](session.md): Shared configuration for all crawl requests
- [`Dataset`](dataset.md): Single-page structured extraction
- [`Watch`](watcher.md): Periodic change detection
