# Release Process

## Version Alignment

Before releasing, align versions across all three packaging files:

| File | Field |
|------|-------|
| `Cargo.toml` | `version` |
| `sdk/python/pyproject.toml` | `version` |
| `sdk/nodejs/package.json` | `version` |

## Release Workflow

```
Developer pushes tag vX.Y.Z
        │
        ▼
CI Pipeline triggered
        │
        ├── 1. Run all tests (cargo test, SDK tests)
        ├── 2. Build Rust core (release)
        ├── 3. Build Python wheels (maturin build)
        ├── 4. Build Node.js addons (napi build)
        ├── 5. Publish to PyPI (twine upload)
        ├── 6. Publish to npm (npm publish)
        └── 7. Create GitHub Release (assets, changelog)
```

## Commands

```bash
# 1. Update version in all three files
# 2. Commit and tag
git tag v0.1.0
git push origin v0.1.0

# CI handles the rest automatically
```

## Pre-Release Checklist

- [ ] All tests pass on Linux, macOS, and Windows
- [ ] Changelog updated
- [ ] Version aligned across all three packages
- [ ] Wheels built and tested for all target platforms
- [ ] npm package tested locally

## See Also

- [Contributing Guide](03_contributing.md): Branch naming and commit messages.
