# Advanced Features

## Hooks and Middleware

Crawlingo supports lifecycle hooks that run at each stage of the page fetch and extraction pipeline.

```python
from crawlingo import Page
from crawlingo.hooks import strip_whitespace, uppercase, log_request

page = (
    Page("https://example.com")
    .before_fetch(log_request)                      # Before HTTP request
    .before_parse(lambda html: html.replace("Old", "New"))  # Before parsing
    .after_extract(strip_whitespace)                 # After extraction
    .after_extract(uppercase)                        # Chain multiple hooks
)

# Built-in hooks
from crawlingo.hooks import (
    strip_whitespace,    # Trim and collapse whitespace
    uppercase,           # Convert to uppercase
    lowercase,           # Convert to lowercase
    log_request,         # Log request details
    add_timestamp,       # Add extraction timestamp
)
```

## Custom Hooks

```python
def my_hook(value: str) -> str:
    """Transform extracted value."""
    return value.strip().replace("\n", " ")

page.after_extract(my_hook)
```

## Custom Transport (Mock for Testing)

Replace the real HTTP transport with a mock for offline testing:

```python
from crawlingo import Session, Dataset
from crawlingo.transport import MockTransport

mock = MockTransport()
mock.with_html("https://example.com", "<h1>Hello</h1>")

session = Session()
session.set_transport(mock)

result = Dataset("https://example.com", session=session)\
    .field("title", "h1")\
    .build()

print(result.to_dict())  # {"title": "Hello"}
```

## Streaming Dataset

Process large URL lists with constant memory:

```python
dataset = Dataset("https://example.com")
dataset.field("title", "h1")
dataset.field("price", "span.price", extraction_type="price")

# Stream results, bounded memory regardless of URL count
stream = dataset.build_many_streamed(
    urls=["https://example.com/a", "https://example.com/b", "https://example.com/c"],
    concurrency=10
)

for record in stream:
    print(record.fields)

# Backpressure: blocks when internal buffer is full
```

## Proxy Rotation

```python
session = Session()

# Static proxy
session.proxy("http://user:pass@proxy:8080")

# Proxy pool (rotates round-robin)
session.proxy_pool([
    "http://proxy1:8080",
    "http://proxy2:8080",
    "http://proxy3:8080",
])

# Dynamic proxy provider
session.proxy_provider("https://proxy-service.example.com/get")
# Fetches a fresh proxy URL before each request
```

## Rate Limiting

```python
# Per-host rate limiting
session.rate_limit(5.0)  # 5 requests/second per unique host

# With retry on 429
from crawlingo import Session
session = Session()
session.rate_limit(3.0)
# Retries with exponential backoff on rate-limit responses
```

## Cache Layer

```python
# In-memory LRU cache
from crawlingo import Session
session = Session()
session.cache_enabled(True)
session.cache_ttl(300)  # 5 minute TTL
session.cache_max_entries(500)

# Cache respects Cache-Control, ETag, and Last-Modified headers
```

## Metrics

Built-in metrics collection for monitoring:

```python
from crawlingo import Session

session = Session()
# Access metrics
metrics = session.metrics()

print(f"Requests: {metrics.request_count}")
print(f"Success: {metrics.success_count}")
print(f"Errors: {metrics.error_count}")
print(f"Cache hits: {metrics.cache_hits}")
print(f"Cache misses: {metrics.cache_misses}")
print(f"Avg response time: {metrics.avg_response_time}ms")

# Metrics are aggregated across all operations using the session
```

## CLI

```bash
# Fetch a page and extract with CSS
crawlingo fetch https://example.com --css h1 --css .price

# Extract with dataset schema
crawlingo extract https://example.com --schema schema.toml

# Crawl a site
crawlingo crawl https://docs.example.com --follow "a" --limit 50

# JSON output
crawlingo fetch https://example.com --css h1 --json
```

## MCP Server

Crawlingo includes a Model Context Protocol (MCP) server for LLM agent integration:

```bash
crawlingo mcp  # Start MCP server
```

Connect from Claude Code, Cursor, or any MCP-compatible client to enable web scraping capabilities for your AI agents.
