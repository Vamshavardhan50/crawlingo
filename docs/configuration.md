# Configuration

Crawlingo supports three configuration methods (in order of precedence): **TOML files**, **environment variables**, and **programmatic API**.

## TOML Configuration

Create a `crawlingo.toml` in your working directory:

```toml
[default]
rate_limit = 5.0            # Per-host requests/second (0 = unlimited)
fetcher_tier = "stealthy"   # "standard" or "stealthy"
browser_profile = "chrome"  # "chrome", "firefox", "safari"
timeout = 30                # Request timeout in seconds
auto_match = true           # Enable self-healing selectors

[default.headers]
User-Agent = "Crawlingo/1.0"
Accept = "text/html,application/xhtml+xml"

[default.retry]
base_delay = 500            # Initial retry delay (ms)
max_delay = 30000           # Maximum retry delay (ms)
multiplier = 2.0            # Exponential backoff factor

[default.cache]
enabled = true
max_entries = 500
ttl_secs = 300

[default.auto_match.weights]
tag = 1.0
class_name = 0.8
id = 0.6
attributes = 0.4
parent_tag = 0.5
depth = 0.1
```

## Environment Variables

Prefix all variables with `CRAWLINGO_`:

| Variable | Maps To | Example |
|----------|---------|---------|
| `CRAWLINGO_RATE_LIMIT` | `rate_limit` | `5.0` |
| `CRAWLINGO_PROXY_URL` | `proxy_url` | `http://proxy:8080` |
| `CRAWLINGO_FETCHER_TIER` | `fetcher_tier` | `stealthy` |
| `CRAWLINGO_BROWSER_PROFILE` | `browser_profile` | `chrome` |
| `CRAWLINGO_TIMEOUT` | `timeout` | `30` |
| `CRAWLINGO_AUTO_MATCH` | `auto_match` | `true` |
| `CRAWLINGO_CACHE_ENABLED` | `cache_enabled` | `true` |
| `CRAWLINGO_CACHE_TTL` | `cache_ttl` | `300` |
| `CRAWLINGO_RETRY_BASE_DELAY` | `retry_base_delay` | `500` |
| `CRAWLINGO_RETRY_MAX_DELAY` | `retry_max_delay` | `30000` |
| `CRAWLINGO_VERBOSE` | log level | `1` |
| `RUST_LOG` | Rust log level | `crawlingo=debug` |

## Programmatic Configuration

```python
from crawlingo import Session

session = Session()
session.headers({"User-Agent": "MyBot/1.0"})
session.proxy("http://proxy:8080")
session.rate_limit(5.0)
session.auto_match(True)
session.fetcher_tier("stealthy")
session.browser_profile("chrome")
```

## Configuration Resolution Order

Settings are resolved top-down (later values override earlier):

1. `crawlingo.toml` defaults
2. Environment variables (`CRAWLINGO_*`)
3. `Session()` constructor parameters
4. `Session.xxx()` method calls
5. Per-call parameters (e.g. `Page(url, session=None)`)

## Fetcher Tiers

| Tier | Description | When to Use |
|------|-------------|-------------|
| `standard` | Basic HTTP client with standard TLS | Public APIs, simple sites |
| `stealthy` | TLS fingerprint emulation, browser headers, request pattern randomization | Sites with bot detection (Cloudflare, Akamai) |

## Browser Profiles

| Profile | TLS Fingerprint | HTTP/2 Settings |
|---------|----------------|-----------------|
| `chrome` | Chrome 120+ | HPACK, PRIORITY, SETTINGS |
| `firefox` | Firefox 121+ | HPACK, PRIORITY_UPDATE |
| `safari` | Safari 17+ | HPACK, limited SETTINGS |
