# Session API

The `Session` object manages shared configuration across multiple Page, Dataset, Crawl, and Watch operations. It centralizes headers, cookies, proxies, rate limits, and other settings.

## Python

```python
from crawlingo import Session, Page, Dataset, Crawl, Watch

session = Session()
session.headers({
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ...",
    "Accept-Language": "en-US,en;q=0.9"
})
session.cookies({"session_id": "abc123"})
session.proxy("http://proxy.example.com:8080")
session.rate_limit(5.0)       # 5 requests/second per host
session.timeout(30)           # 30 second timeout
session.auto_match(True)      # Enable self-healing selectors
session.fetcher_tier("stealthy")  # "standard" or "stealthy"
session.browser_profile("chrome") # "chrome", "firefox", "safari"
session.fingerprint_path("/tmp/fingerprints")

# Use with Page
page = session.page("https://example.com")

# Use with Dataset
dataset = session.dataset("https://example.com")

# Use with Crawl
crawl = session.crawl("https://example.com")

# Use with Watch
watch = session.watch("https://example.com")

# Context manager
with Session() as session:
    session.headers({"User-Agent": "MyBot/1.0"})
    page = Page("https://example.com", session=session)
```

## Node.js

```javascript
const { Session, Page, Dataset, Crawl, Watch } = require('crawlingo');

const session = new Session();
session.headers({'User-Agent': 'MyBot/1.0'});
session.rateLimit(5.0);
session.timeout(30);
session.autoMatch(true);

const page = await Page.create('https://example.com', session);
```

## Session Methods

| Method | Parameters | Description |
|--------|------------|-------------|
| `headers(dict)` | Header key-value pairs | Set default request headers |
| `cookies(dict)` | Cookie key-value pairs | Set default cookies |
| `proxy(url)` | Proxy URL | Set HTTP/HTTPS proxy |
| `proxy_pool(list)` | List of proxy URLs | Rotating proxy pool |
| `proxy_provider(url)` | Provider URL | Dynamic proxy provider endpoint |
| `rate_limit(rps)` | Requests per second | Per-host rate limiting |
| `timeout(secs)` | Timeout in seconds | Request timeout |
| `auto_match(bool)` | Enable/disable | Self-healing selector recovery |
| `fetcher_tier(tier)` | `"standard"` / `"stealthy"` | TLS fingerprint emulation level |
| `browser_profile(profile)` | `"chrome"` / `"firefox"` / `"safari"` | Browser TLS profile |
| `fingerprint_path(path)` | File system path | Fingerprint database location |
| `auto_match_weights(dict)` | Weight configuration | Custom auto-match scoring weights |

## Configuration Precedence

Settings are resolved in this order (later overrides earlier):

1. TOML config file (`crawlingo.toml`)
2. Environment variables (`CRAWLINGO_*`)
3. Session constructor
4. Session method calls
5. Per-call parameters

## Thread Safety

`Session` is thread-safe. A single session can be shared across multiple concurrent Page/Dataset/Crawl calls. Rate limiting and proxy rotation are synchronized internally.
