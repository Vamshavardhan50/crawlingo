# Crawlingo

Crawlingo is a next-generation web data extraction and website monitoring framework.
It combines a high-performance Rust core engine with a React-like developer experience in Python via PyO3 bindings.

## Features

- **Standard & Stealth Fetching:** Impersonate browsers and custom TLS profiles (via `rquest`) to bypass bot detection.
- **Pre-compiled Fast Selectors:** CSS, XPath, Regex, and relative text-anchors.
- **Adaptive Extraction:** Auto-matcher selector recovery based on element DOM tree fingerprints.
- **Structured Dataset Generation:** Fluent builder API with JSON, CSV, and Parquet exports.
- **Change Detection & Watcher:** Monitor webpages for layout, stock, price, and content changes with asynchronous poller loops.
- **MCP Server Integration:** Built-in Model Context Protocol server exposing scraper tools directly to AI models.

## Quick Start (Python)

```python
from crawlingo import Page

page = Page("https://example.com")
print(page.css("h1").text())
```
