# Production Deployment

## Network and Resource Limits

- Configure connection pool size and per-host rate limits via `Session`.
- Enable DNS caching to reduce resolution latency.
- Use proxy rotation for large-scale deployments.

## Rate Limiting and Stealth

- Set reasonable RPS limits per domain (default: 10 req/s).
- JA3 fingerprinting is built into the `wreq` HTTP client.
- Rotate User-Agent strings via session `headers`.

## Database and Caching

- The Sled fingerprint database is embedded — ensure write permissions on the target directory.
- Open the database once per `Session` lifetime (avoid open/close per extraction — see BUG-002).
- For ephemeral environments (serverless), the CSV fallback logs to `/tmp`.

## Observability

- Structured logging via `tracing` (Rust) — configure log levels with `RUST_LOG`.
- FFI error boundaries: all Rust panics are caught and returned as SDK exceptions.
- Monitor extraction success rates and selector health with the `Watcher`.

## CI/CD

The release pipeline is fully automated:
1. Tag push triggers CI.
2. Tests run on Linux, macOS, and Windows.
3. Python wheels and Node.js addons are compiled for all targets.
4. Assets are published to PyPI and npm.
5. GitHub Release is created with binaries attached.

## See Also

- [Architecture Overview](01_architecture_overview.md): Understanding the full system.
- [Release Process](05_release.md): Release workflow details.
- [Security Review](../developer_guide/08_design_decisions.md): Security considerations.
