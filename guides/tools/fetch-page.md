# CLI and integrations

Use the documented CLI and SDK entry points in your own tools.

<a id="cli"></a>

## Inspect the CLI

### Terminal

```shell
cargo run --release --features cli -- --help
cargo run --release --features cli -- fetch https://example.com/
cargo run --release --features cli -- dataset job.json
```

<a id="mcp"></a>

## Agent and MCP integrations

The dataset job can be called by an application or tool server through its SDK. This guide does not claim that a production MCP server is shipped or that a configuration snippet alone creates one. Define input validation, job ownership and output access in the hosting application.

- [Advanced integration boundaries](../advanced.md)
