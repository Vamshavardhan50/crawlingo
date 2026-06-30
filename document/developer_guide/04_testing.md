# Testing Guide

## Test Layers

| Layer | Location | Command |
|-------|----------|---------|
| Rust unit tests | Inline `#[test]` in each module | `cargo test --lib` |
| Rust integration tests | `tests/` directory | `cargo test` |
| SDK wrapper tests | Python tests, Node.js tests | Language-specific runners |

## Running Tests

```bash
# All tests
cargo test

# Core library only
cargo test --lib

# Specific test
cargo test test_css_selector

# Integration tests
cargo test --test integration_test
```

## Writing Tests

### Rust Unit Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_class_selector() {
        let html = r#"<div class="product"><h1>Title</h1></div>"#;
        let page = Page::from_html(html).unwrap();
        let result = page.css(".product");
        assert_eq!(result.len(), 1);
    }
}
```

### Integration Test

```rust
use crawlingo::Page;

#[tokio::test]
async fn test_fetch_real_page() {
    let page = Page::new("https://httpbin.org/html")
        .fetch()
        .await
        .unwrap();
    assert_eq!(page.status(), 200);
    assert!(page.html().contains("Herman Melville"));
}
```

## See Also

- [Contributing Guide](03_contributing.md): Coding standards and PR workflow.
- [Release Guide](05_release.md): Testing before release.
