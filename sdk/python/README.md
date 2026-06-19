# Crawlingo Python SDK

Crawlingo is a next-generation web data extraction, crawling, and website monitoring framework. It wraps a high-performance Rust core in an elegant, React-inspired developer-first Python API.

## Installation

Crawlingo is built using Maturin. You can install it locally by compiling from source:

```bash
cd sdk/python
pip install .
```

To compile in development mode:

```bash
cd sdk/python
pip install -e .
```

## Features

- **Stealth Fetching:** Impersonates browser TLS fingerprints (Chrome, Firefox, Safari) using `wreq` to bypass advanced bot protection systems.
- **Fast Selector Engines:** Supports compiled CSS selectors, XPath queries, relative text anchors, and Regex scanners.
- **Self-Healing Selectors:** Learns element DOM fingerprints and heals broken selectors automatically if the page layout shifts.
- **Structured Datasets:** Define mapping schemas and export results to Parquet, CSV, or JSON.
- **Reactive Watch Loops:** Asynchronous monitor loops that trigger callback hooks on price updates, stock updates, layout changes, or element additions/removals.
- **Model Context Protocol (MCP):** Connect your scraper directly to LLM agents using the built-in MCP server.

## Quick Start

### Basic Extraction

```python
from crawlingo import Page

page = Page("https://example.com")
print(page.title())
print(page.css("p").text())
```

### Self-Healing Datasets

```python
from crawlingo import Dataset

dataset = (
    Dataset("https://example.com/products")
    .auto_match(True) # Learn & heal selectors automatically
    .field("title", "h1.product-title")
    .field("price", "span.price")
    .build()
)

print(dataset.to_dict())
dataset.to_csv("products.csv")
```

### Watch Monitor for Changes

```python
import asyncio
from crawlingo import Watch

def on_price_update(event):
    print(f"Price updated from {event.old_value} to {event.new_value}!")

async def main():
    watch = (
        Watch("https://example.com/item")
        .field("price", "span.item-price")
        .interval(60)
        .on_price_change(on_price_update)
    )
    await watch.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## CLI Subcommands

Crawlingo provides a command-line interface:

### 1. Interactive Shell
Launch a Python REPL preloaded with crawlingo:
```bash
crawlingo shell https://example.com
```

### 2. Direct Extraction
Extract matching elements directly from the command line:
```bash
crawlingo extract https://example.com --css "h1"
```

### 3. Start MCP Server
Expose scraping tools to LLMs:
```bash
crawlingo mcp --host 127.0.0.1 --port 8000
```
