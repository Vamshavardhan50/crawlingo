# Code Style

## Rust

| Tool | Standard |
|------|----------|
| Formatter | `cargo fmt` |
| Linter | `cargo clippy` |
| Convention | `snake_case`, `CamelCase`, `SCREAMING_CASE` |

Rules:
- No bare `unwrap()` — use `?`, `expect()`, or `match`.
- Use `thiserror` for error types.
- Use `tracing` for structured logging.
- Document public items with docstrings.

## Python

| Tool | Standard |
|------|----------|
| Formatter | `black` |
| Linter | `ruff` |
| Convention | PEP 8, `snake_case` |

Rules:
- Type hints for all public function signatures.
- Public wrappers expose clean APIs; core Rust logic is not duplicated.

## Node.js / TypeScript

| Tool | Standard |
|------|----------|
| Formatter | `prettier` |
| Linter | `eslint` |
| Convention | `camelCase`, interfaces prefixed with `I` |

Rules:
- TypeScript strict mode enabled.
- Async functions use `Promise` or `async/await`.
