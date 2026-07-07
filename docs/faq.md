# FAQ & Troubleshooting

## General

**Q: What is Crawlingo?**

A: A Rust-powered web scraping framework with self-healing selectors, stealth TLS, and cross-language SDKs (Python, Node.js, Rust). It handles single-page extraction, multi-page crawling, and change monitoring.

**Q: How does Crawlingo compare to Scrapy / BeautifulSoup / Playwright?**

| Feature | Crawlingo | Scrapy | BS4 | Playwright |
|---------|-----------|--------|-----|------------|
| Performance | 3,500+ req/s | ~1,000 req/s | N/A | ~100 req/s |
| Self-healing selectors | Yes | No | No | No |
| Stealth TLS | Yes | No | No | Yes (browser) |
| Cross-language | Python/JS/Rust | Python | Python | Python/JS |
| Change monitoring | Yes | No | No | No |
| Dataset export | JSON/CSV/Parquet | JSON/CSV/XML | Manual | Manual |

**Q: Is Crawlingo free?**

A: Yes, MIT licensed.

## Installation

**Q: `pip install` fails on Linux**

A: Ensure glibc 2.28+ (`ldd --version`). Try `pip install --no-binary crawlingo crawlingo` to build from source (requires Rust 1.70+).

**Q: `npm install` fails**

A: Ensure Node.js 18+. Check that platform-specific pre-built addon exists for your OS/architecture. If not, build from source: `npm run build` in the `crawlingo-docs/sdk/nodejs/` directory.

**Q: Build from source fails**

A: Install Rust 1.70+ via rustup.rs. On Windows, install Visual Studio Build Tools with MSVC. On Linux, install `build-essential`, `pkg-config`, and `libssl-dev`.

## Usage

**Q: HTTP 403 on every request**

A: The target site blocks basic HTTP clients. Enable stealth mode:

```python
from crawlingo import Session
session = Session()
session.fetcher_tier("stealthy")
session.browser_profile("chrome")
```

**Q: HTTP 429 (rate limited)**

A: Reduce rate limit and enable retry:

```python
session.rate_limit(2.0)
# Retry-After headers are respected automatically
```

**Q: Selector returns empty results**

A: Verify the selector in browser DevTools. Enable auto-match for resilience:

```python
page = Page(url).auto_match(True)
```

**Q: Memory grows during crawl**

A: Use streaming dataset for large URL lists:

```python
stream = dataset.build_many_streamed(urls, concurrency=10)
```

**Q: Watch fires every poll cycle**

A: The page contains dynamic content (timestamps, ads, counters). Use more specific selectors that target only the stable content you care about.

## Performance

**Q: How many requests per second can Crawlingo handle?**

A: ~3,500 req/s for simple fetches on a single machine. Actual throughput depends on network, target server, and extraction complexity.

**Q: What's the memory footprint?**

A: ~120 MB for a 1,000-URL crawl queue. Streaming dataset: ~85 MB constant. Fingerprint DB: ~8 MB for 10K entries.

**Q: Can I use proxies?**

A: Yes — static proxy, rotating proxy pool, or dynamic proxy provider. See [Configuration](configuration.md).

## Integration

**Q: Can I use Crawlingo with Docker?**

A: Yes. Base image needs glibc 2.28+. Use `python:3.12-slim` as base.

**Q: Can I use Crawlingo with AWS Lambda?**

A: Yes. Use `/tmp` for the fingerprint store. Expect cold starts including library loading (1-2 seconds).

**Q: Can I use Crawlingo with Airflow?**

A: Yes. Create a new Session per task for isolation.

**Q: Does Crawlingo handle pagination?**

A: Use the Crawl engine's `follow` selector pointing to the "next page" link:

```python
Crawl("https://example.com").follow("a.next, a[rel=next]")
```

**Q: Can I extract JavaScript-rendered content?**

A: Not yet — this is planned for v0.2. For now, use Crawlingo for server-rendered HTML pages.

## Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `ConnectionError` | DNS/TCP failure | Check URL, network, proxy |
| `TimeoutError` | Request exceeded timeout | Increase timeout, check server |
| `HttpError` | 4xx/5xx response | Check URL, auth, rate limits |
| `SelectorError` | Invalid selector | Validate syntax, escape characters |
| `SelectorError: ambiguous` | Multiple elements match | Use more specific selector |
