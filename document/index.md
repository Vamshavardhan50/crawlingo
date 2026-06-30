# Crawlingo Documentation

**Version:** 0.1.0 | **License:** MIT | **Language:** Rust core + Python/Node.js SDKs

## User Guide

| Section | Description |
|---------|-------------|
| [Getting Started](user_guide/01_getting_started.md) | Installation and first extraction |
| [Fetching and Selecting](user_guide/02_fetch_and_select.md) | Page fetching, CSS/XPath/Regex/Text selectors |
| [Dataset Extraction](user_guide/03_dataset_extraction.md) | Schema-driven structured data extraction |
| [Self-Healing Selectors](user_guide/04_auto_healing.md) | Automatic recovery from broken selectors |
| [Crawling Multiple Pages](user_guide/05_crawling.md) | Multi-threaded site crawling |
| [Change Detection](user_guide/06_change_detection.md) | Periodic page monitoring |

## API Reference

| Module | Description |
|--------|-------------|
| [Session](api_reference/session.md) | Shared request configuration |
| [Page](api_reference/page.md) | Fetched web page with parsed DOM |
| [DOM Tree](api_reference/dom_tree.md) | Flat-vector DOM representation |
| [Selector Engine](api_reference/selector_engine.md) | CSS/XPath/Regex/Text query engine |
| [Auto Matcher](api_reference/auto_matcher.md) | Self-healing selector recovery |
| [Fingerprint](api_reference/fingerprint.md) | Fingerprint database |
| [Dataset](api_reference/dataset.md) | Schema-driven extraction and export |
| [Fetch](api_reference/fetch.md) | HTTP fetch orchestration |
| [Crawl](api_reference/crawler.md) | Multi-page crawling |
| [Watch](api_reference/watcher.md) | Change detection |
| [Python SDK](api_reference/sdk_python.md) | Python binding API |
| [Node.js SDK](api_reference/sdk_nodejs.md) | Node.js binding API |
| [CLI](api_reference/cli.md) | Command-line interface |
| [MCP Server](api_reference/mcp.md) | LLM agent integration |

## Developer Guide

| Section | Description |
|---------|-------------|
| [Architecture Overview](developer_guide/01_architecture_overview.md) | High-level system architecture |
| [Development Setup](developer_guide/02_development_setup.md) | Building from source |
| [Codebase Map](developer_guide/03_codebase_map.md) | File-by-file source guide |
| [Testing](developer_guide/04_testing.md) | Running and writing tests |
| [Release Process](developer_guide/05_release.md) | Versioning and publishing |
| [Contributing](developer_guide/06_contributing.md) | PR workflow and conventions |
| [Code Style](developer_guide/07_code_style.md) | Language-specific standards |
| [Design Decisions](developer_guide/08_design_decisions.md) | Architectural rationale |
