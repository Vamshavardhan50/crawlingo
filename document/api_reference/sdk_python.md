# Python SDK API (crawlingo)

## Installation

```bash
pip install crawlingo
```

Requires Python 3.9+. Pre-compiled wheels available for Windows, macOS, and Linux (x86_64, aarch64).

## Classes

| Class | Description |
|-------|-------------|
| [`Session`](session.md) | Shared request configuration (cookies, headers, proxy, rate limits, auto-match) |
| [`Page`](page.md) | Fetched and parsed web page with CSS/XPath/Text/Regex selectors |
| [`Element`](sdk_python.md#element) | Single DOM element with text, HTML, attribute access, and parent/child/sibling navigation |
| [`ElementCollection`](sdk_python.md#elementcollection) | Collection of elements with filter, map, first, last, nth operations |
| [`Dataset`](dataset.md) | Fluent builder for schema-driven structured data extraction and export |
| [`DatasetResult`](dataset.md#datasetresult) | Extracted fields with export methods (JSON, CSV, Parquet, Pandas) |
| [`Crawl`](crawler.md) | Multi-page concurrent crawl orchestrator with webhooks and scheduling |
| [`CrawlResults`](crawler.md#crawlresults) | Collection of crawl results with export methods |
| [`Watch`](watcher.md) | Periodic page-change detection with callback events |
| [`ChangeEvent`](watcher.md#changeevent) | Change detection event with old/new values, change type, and metadata |

## Element

```python
class Element:
    def text(self) -> str: ...
    def html(self) -> str: ...
    def attr(self, name: str) -> str: ...
    def attrs(self) -> dict: ...
    def parent(self) -> Element | None: ...
    def children(self) -> ElementCollection: ...
    def next(self) -> Element | None: ...
    def prev(self) -> Element | None: ...
    def siblings(self) -> ElementCollection: ...
```

## ElementCollection

```python
class ElementCollection:
    def text(self) -> str: ...
    def texts(self) -> list[str]: ...
    def attr(self, name: str) -> str: ...
    def attrs(self) -> dict: ...
    def first(self) -> Element | None: ...
    def last(self) -> Element | None: ...
    def nth(self, n: int) -> Element | None: ...
    def filter(self, fn: Callable) -> ElementCollection: ...
    def map(self, fn: Callable) -> list: ...
    def __getitem__(self, index: int) -> Element | None: ...
    def __len__(self) -> int: ...
    def __iter__(self) -> Iterator[Element]: ...
```

## Exceptions

| Exception | Description |
|-----------|-------------|
| `CrawlingoError` | Base exception |
| `FetchError` | Network timeouts, proxy issues, connection failures |
| `ParseError` | HTML parsing failures |
| `SelectorError` | Invalid selector syntax or evaluation failures |
| `AutoMatchFailed` | Auto-match could not find a matching element |
| `TimeoutError` | Request timed out |
| `RateLimitError` | Host rate limit reached |
| `ChangeDetectionError` | Change detection processing failed |
| `ExportError` | CSV/Parquet/JSON export failed |
| `DnsError` | DNS resolution failed |
| `FingerprintStoreError` | Fingerprint database read/write failed |

## CLI

```
crawlingo shell [url]                         Start interactive Python shell
crawlingo extract <url> [--css] [--xpath]     One-shot extraction from URL
crawlingo mcp [--host] [--port]               Start MCP SSE server (default 127.0.0.1:8000)
```

## See Also

- [Node.js SDK](sdk_nodejs.md): Equivalent API for JavaScript/TypeScript
- [CLI reference](cli.md): Detailed CLI command documentation
