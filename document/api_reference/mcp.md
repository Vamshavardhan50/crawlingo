# crawlingo MCP Server

Model Context Protocol (MCP) server exposing Crawlingo capabilities as JSON-RPC 2.0 tools over SSE for LLM agents.

## Tools

### `fetch_page`

Fetch a URL and return normalized content.

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `str` | Target URL |
| `auto_match` | `bool` | Enable self-healing selectors |
| `timeout` | `int` | Request timeout in seconds |

**Returns:** Object with `text`, `html`, `title`, `url`, `status`.

### `extract_data`

Extract structured data using defined fields.

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `str` | Target URL |
| `fields` | `array` | Array of `{name, selector, selector_type}` objects |
| `auto_match` | `bool` | Enable self-healing selectors |

**Returns:** Object with extracted key-value pairs.

### `crawl_site`

Crawl a site starting from a URL.

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `str` | Seed URL |
| `follow_selector` | `str` | CSS selector for links to follow |
| `fields` | `array` | Array of field definitions |
| `max_pages` | `int` | Maximum pages to fetch |
| `max_depth` | `int` | Maximum link depth |

**Returns:** List of `{url, title}` objects.

## Server

```bash
crawlingo mcp --host 127.0.0.1 --port 8000
```

Protocol: SSE at `/sse`, POST to `/messages/?session_id=<id>`.

## See Also

- [CLI reference](cli.md): MCP server startup flags
