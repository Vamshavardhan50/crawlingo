# crawlingo CLI

Command-line interface for interactive shells, one-shot extraction, and MCP server startup.

## Commands

### `shell`

Start an interactive Python shell with preloaded Crawlingo classes.

```bash
crawlingo shell [url]
```

If a URL is provided, it is pre-fetched into a `page` variable.

Pre-imported: `Session`, `Page`, `Element`, `ElementCollection`, `Dataset`, `Crawl`, `Watch`, `hooks`.

### `extract`

One-shot data extraction from a URL using selectors.

```bash
crawlingo extract <url> [--css SELECTOR] [--xpath QUERY] [--text TEXT] [--regex PATTERN] [--auto-match] [--json]
```

| Flag | Type | Description |
|------|------|-------------|
| `--css` | `str` | CSS selector to query |
| `--xpath` | `str` | XPath expression to query |
| `--text` | `str` | Text anchor to find |
| `--regex` | `str` | Regex pattern to match |
| `--auto-match` | `flag` | Enable self-healing selectors |
| `--json` | `flag` | Output as JSON |

### `mcp`

Start the Model Context Protocol SSE server for AI agent integration.

```bash
crawlingo mcp --host 127.0.0.1 --port 8000
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--host` | `str` | `127.0.0.1` | Bind address |
| `--port` | `int` | `8000` | TCP port |

Protocol: SSE at `/sse`, POST to `/messages/?session_id=<id>`.

## Examples

```bash
# Extract product info
crawlingo extract https://example.com --css "h1.title" --json

# Start MCP server for AI agent
crawlingo mcp --host 0.0.0.0 --port 8000

# Interactive exploration
crawlingo shell
>>> page.css("h1").text()
```

## See Also

- [MCP Server](mcp.md): JSON-RPC 2.0 tools exposed over SSE
- [Python SDK](sdk_python.md): Python package API
