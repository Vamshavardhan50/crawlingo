# Python SDK

## Installation

```bash
pip install crawlingo
```

Requires Python 3.8+. Pre-built wheels for Linux, macOS, and Windows.

## Quick Reference

```python
from crawlingo import (
    Page,      # Single page fetch and extraction
    Session,   # Shared configuration
    Dataset,   # Structured data extraction
    Crawl,     # Multi-page crawling
    Watch,     # Change monitoring
)
```

## API Summary

### Page
| Method | Description |
|--------|-------------|
| `Page(url, session=None)` | Fetch and parse a web page |
| `page.css(selector)` | CSS selector query |
| `page.xpath(expr)` | XPath query |
| `page.regex(pattern)` | Regex pattern search |
| `page.find_text(text)` | Text anchor search |
| `page.after_text(text)` | Element after text |
| `page.before_text(text)` | Element before text |
| `page.between_texts(a, b)` | Elements between texts |
| `page.html()` | Raw HTML |
| `page.markdown()` | Markdown conversion |
| `page.title()` | Page title |
| `page.status` | HTTP status code |

### Session
| Method | Description |
|--------|-------------|
| `Session()` | Create session |
| `session.headers(dict)` | Set default headers |
| `session.proxy(url)` | Set proxy |
| `session.proxy_pool(list)` | Rotating proxies |
| `session.rate_limit(rps)` | Per-host rate limit |
| `session.auto_match(bool)` | Self-healing selectors |
| `session.fetcher_tier(tier)` | TLS fingerprint level |
| `session.browser_profile(profile)` | Browser profile |
| `session.timeout(secs)` | Request timeout |
| `session.fingerprint_path(path)` | Fingerprint DB path |
| `session.cache(bool)` | Enable response cache |

### Dataset
| Method | Description |
|--------|-------------|
| `Dataset(url, session=None)` | Create dataset |
| `.field(name, selector, ...)` | Add extraction field |
| `.auto_match(bool)` | Enable auto-healing |
| `.build()` | Run extraction |
| `.to_json(path)` | Export to JSON |
| `.to_csv(path)` | Export to CSV |
| `.to_parquet(path)` | Export to Parquet |
| `.to_dict()` | Return as dict |
| `.build_many_streamed(urls, concurrency)` | Batch extract |

### Crawl
| Method | Description |
|--------|-------------|
| `Crawl(url, session=None)` | Create crawler |
| `.follow(selector)` | Links to follow |
| `.limit(n)` | Max pages |
| `.depth(n)` | Max link depth |
| `.concurrency(n)` | Parallel requests |
| `.delay(secs)` | Request delay |
| `.field(name, selector)` | Extract field |
| `.build()` | Run crawl |

### Watch
| Method | Description |
|--------|-------------|
| `Watch(url, session=None)` | Create monitor |
| `.field(name, selector)` | Field to monitor |
| `.interval(secs)` | Polling interval |
| `.on_change(callback)` | Change handler |
| `.stop()` | Stop monitoring |
| `.tolerance(pct)` | Price change tolerance |

## Exceptions

```python
from crawlingo import CrawlingoError, ConnectionError, TimeoutError, HttpError, SelectorError
```

## CLI Commands

```bash
# Fetch a page
crawlingo fetch <url> [--css SELECTOR] [--xpath EXPR] [--json]

# Extract with dataset
crawlingo extract <url> [--field NAME=SELECTOR] [--schema FILE]

# Crawl a site
crawlingo crawl <start_url> [--follow SELECTOR] [--limit N]

# Start MCP server
crawlingo mcp

# Interactive shell
crawlingo shell
```

## MCP Server

The MCP (Model Context Protocol) server enables LLM agents to use Crawlingo:

```bash
crawlingo mcp --port 3100
```

Connect from Claude Code, Cursor, or any MCP client to:
- Fetch pages and extract content
- Run CSS/XPath queries
- Build datasets
- Crawl websites
- Monitor for changes
