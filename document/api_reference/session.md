# crawlingo.Session

Shared request configuration manager that coordinates cookies, headers, proxy settings, rate limits, and connection lifecycle across multiple requests.

## Constructor

```python
session = Session()
```

No constructor parameters. All configuration is done via fluent setter methods.

## Methods

All methods return `self` for fluent chaining.

### Configuration

| Method | Parameters | Description |
|--------|-----------|-------------|
| `.headers(headers: dict)` | `headers` — Custom HTTP headers applied to every request | Set default request headers |
| `.cookies(cookies: dict)` | `cookies` — Persistent cookie jar shared across requests | Set default cookies |
| `.proxy(proxy_url: str)` | `proxy_url` — Proxy URL (e.g. `http://user:pass@host:8080`) | Configure session proxy server |
| `.rate_limit(requests_per_second: float)` | `requests_per_second` — Max requests per second per host | Configure per-host rate limiter |
| `.auto_match(enabled: bool)` | `enabled` — Enable self-healing selector recovery globally | Enable auto-matching |
| `.timeout(seconds: int)` | `seconds` — Request timeout in seconds | Set request timeout |
| `.fingerprint_path(path: str)` | `path` — Path for auto-match fingerprint store directory | Set fingerprint storage path |
| `.fetcher_tier(tier: str)` | `tier` — `"standard"` or `"stealthy"` | Set fetcher mode |
| `.browser_profile(profile: str)` | `profile` — `"chrome"`, `"firefox"`, or `"safari"` | Set browser impersonation profile |
| `.auto_match_weights(weights: dict)` | `weights` — Dict of similarity scoring weights e.g. `{"text": 2.0, "class": 1.0}` | Customize auto-match similarity weights |
| `.proxy_pool(proxies: list)` | `proxies` — List of proxy URLs to rotate round-robin | Set rotating proxy pool |
| `.proxy_provider(url: str)` | `url` — Proxy list provider API endpoint URL | Set dynamic proxy provider |

### Factory Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.page(url: str)` | `Page` | Create a new lazy Page attached to this session |
| `.dataset(url: str)` | `Dataset` | Create a new Dataset builder attached to this session |
| `.crawl(url: str)` | `Crawl` | Create a new multi-page crawler attached to this session |
| `.watch(url: str)` | `Watch` | Create a new change monitor attached to this session |

### Context Manager

```python
with Session() as session:
    page = session.page("https://example.com")
```

## Examples

```python
from crawlingo import Session

session = Session()
session.headers({"User-Agent": "CrawlingoBot/1.0"})
session.timeout(15)
session.auto_match(True)

page = session.page("https://example.com")
print(page.title())
```

## See Also

- [`Page`](page.md): The page object returned by `Session.page()`
- [`Dataset`](dataset.md): Schema-driven structured extraction
- [`Crawl`](crawler.md): Multi-page concurrent crawling
- [`Watch`](watcher.md): Periodic change detection
