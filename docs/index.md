# Crawlingo Documentation

**Version 0.1.0** — Rust-powered, cross-language web scraping framework with self-healing selectors, stealth TLS, and change monitoring.

---

## Quick Links

| Guide | Description |
|-------|-------------|
| [Getting Started](getting-started.md) | Install and run your first extraction in 5 minutes |
| [Installation](installation.md) | Detailed install guides for all platforms and SDKs |
| [Configuration](configuration.md) | TOML files, environment variables, programmatic config |

## Core API Reference

| Module | Description |
|--------|-------------|
| [Page](page.md) | Fetch a single page, extract with CSS/XPath/Regex/Text selectors |
| [Session](session.md) | Shared configuration — headers, proxies, rate limits, auto-match |
| [Dataset](dataset.md) | Schema-driven structured data extraction and export |
| [Crawl](crawl.md) | Multi-page recursive crawling with concurrency control |
| [Watch](watch.md) | Periodic page monitoring with change detection callbacks |
| [Authentication](authentication.md) | Basic, Bearer, Header, API Key, Dynamic auth helpers |

## Feature Guides

| Guide | Description |
|-------|-------------|
| [Selectors](selectors.md) | CSS, XPath, Regex, and Text anchor selectors |
| [Auto-Match](auto-match.md) | Self-healing selectors that survive page layout changes |
| [Change Detection](change-detection.md) | Detect content, price, stock, and structural changes |
| [Advanced](advanced.md) | Hooks, middleware, streaming datasets, custom transports |

## SDK References

| SDK | Details |
|-----|---------|
| [Python SDK](sdk-python.md) | `pip install crawlingo` — full API, CLI, MCP server |
| [Node.js SDK](sdk-nodejs.md) | `npm install crawlingo` — native addon for Node.js |

## Support

- [FAQ & Troubleshooting](faq.md) — Common issues and solutions
- [GitHub Issues](https://github.com/Vamshavardhan50/crawlingo/issues) — Bug reports and feature requests
