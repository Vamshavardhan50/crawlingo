# 08_TESTING_GUIDE.md

This document outlines the testing strategy, structural setup, and execution guidelines for Crawlingo.

---

## 1. Test Architecture

Crawlingo's test suite is divided into three layers:
1. **Rust Unit Tests:** Located inline in source modules (e.g. within `src/selector/css.rs` or `src/engine/rate_limiter.rs`). These verify localized logic such as caching or SIMD parsing functions.
2. **Rust Integration Tests:** Located in the [tests](file:///d:/Scraper/tests) directory. These perform full system tests—mocking inputs, loading HTML fixtures, generating DOM trees, executing selectors, running auto-healers, and exporting datasets.
3. **SDK Wrapper Tests:** Located under Python and Node.js folders. These test language-specific exception handling, FFI memory boundaries, context managers, and JS/Python callbacks.

---

## 2. Running Test Commands

### Run All Test Suites
```bash
cargo test
```

### Run Specific Test Target
```bash
# Run only integration tests
cargo test --test integration_test

# Run only edge case integration tests
cargo test --test edge_cases_test
```

### Run with Stdout Capture Disabled
To view output and debug printing during test execution:
```bash
cargo test -- --nocapture
```

---

## 3. Writing New Rust Integration Tests

When adding a test under `tests/`:
- Create a file or update [integration_test.rs](file:///d:/Scraper/tests/integration_test.rs).
- Load an HTML string dynamically (or read a local fixture file).
- Build the `DomTree` using `parse_html()`.
- Validate selectors, scoring metrics, or dataset construction.

### Example Test Template
```rust
use crawlingo::parser::streaming::parse_html;
use crawlingo::selector::css;

#[test]
fn test_custom_element_parsing() {
    let html = b"<div><span class='target'>Data</span></div>";
    let tree = parse_html(html).expect("Failed to parse HTML");
    
    let indices = css::query(&tree, ".target");
    assert_eq!(indices.len(), 1);
    
    let text = tree.get_text(indices[0]);
    assert_eq!(text, "Data");
}
```

---

## 4. Edge Cases Tested

The project maintains comprehensive regression tests under [edge_cases_test.rs](file:///d:/Scraper/tests/edge_cases_test.rs) covering:
- **Empty and Malformed HTML:** Ensures the parser gracefully yields an empty structure rather than crashing.
- **Deep DOM Trees:** Validates that stack limits are not exceeded on recursively nested tables/divs.
- **Character Encodings:** Checks correct parsing of Shift-JIS, UTF-8, UTF-16, and special symbols.
- **SVG / MathML Elements:** Verifies namespace prefix support in CSS and XPath selectors.
