# Contributing

## Workflow

1. Fork the repository.
2. Create a feature branch:

   | Prefix | Purpose |
   |--------|---------|
   | `feature/` | New features |
   | `bugfix/` | Bug fixes |
   | `docs/` | Documentation |
   | `refactor/` | Code restructuring |

3. Make changes following the [Code Style](07_code_style.md).
4. Run tests: `cargo test`
5. Open a pull request.

## Commit Messages

```
<type>(<scope>): <description>

feat(parser): add encoding detection
fix(selector): handle empty class attribute
docs(api): document Page.dataset method
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `chore`.

## Code Review

All PRs must pass:
- CI tests (Linux, macOS, Windows)
- `cargo clippy` (Rust lint)
- Formatting: `cargo fmt`, `black` (Python), `prettier` (Node.js)

## See Also

- [Code Style](07_code_style.md): Language-specific formatting standards.
- [Testing Guide](04_testing.md): How to run and write tests.
