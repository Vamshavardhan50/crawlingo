# Getting Started

Install Crawlingo and make your first extraction in under five minutes.

## Installation

### Python

```bash
pip install crawlingo
```

Verification:

```python
import crawlingo
print(crawlingo.__version__)
```

### Node.js

```bash
npm install crawlingo
```

```typescript
import { Page } from 'crawlingo';
```

### From Source

See the [Development Setup](../developer_guide/02_development_setup.md) guide.

## Your First Extraction

```python
from crawlingo import Page

page = Page("https://example.com")
print(page.title())
```

## Core Concepts

| Concept | Description |
|---------|-------------|
| **Session** | Shared configuration container (headers, cookies, proxy, rate limits, auto-match). |
| **Page** | A fetched web page with a parsed, immutable DOM tree and selector methods. |
| **Selector** | CSS, XPath, regex, or text-anchor queries against the DOM tree. |
| **AutoMatcher** | Self-healing engine that recovers from broken selectors using DOM fingerprints. |
| **Dataset** | Fluent builder for schema-driven structured extraction with JSON/CSV/Parquet export. |
| **Crawl** | Multi-page parallel crawl orchestrator with webhooks and scheduling. |
| **Watch** | Periodic change-detection engine with callback events. |

## Next Steps

- [Fetching and Selecting Elements](02_fetch_and_select.md): Learn how to navigate the DOM tree.
- [Dataset Extraction](03_dataset_extraction.md): Structure data extraction with schemas.
- [Self-Healing Selectors](04_auto_healing.md): Make your scrapers resilient to DOM changes.
