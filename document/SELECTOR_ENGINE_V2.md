# docs/SELECTOR_ENGINE_V2.md

This document details the architecture and query router of Crawlingo's Selector Engine.

---

## 1. Single Responsibility Boundary

The Selector Engine is strictly a matching engine. It parses query syntax, crawls the parsed `DomTree` vector layout, and returns matching node index offsets.

- **The Selector Engine MUST:**
  - Standardize queries across CSS selectors, XPath expressions, Regex patterns, and Text anchors.
  - Return lists of matching `usize` node indices inside the target `DomTree`.
  - Compile and cache query selector rules (like regex expressions or parsed CSS syntax) to avoid compile overhead.
- **The Selector Engine MUST NOT:**
  - Extract text strings or transform data attributes.
  - Know about dataset schemas, fields, or exporting formats.
  - Interact with files, caches, or network pipelines.

---

## 2. Standardized Query Router

```rust
use crate::parser::document::{DomTree, PyElementCollection};
use crate::error::Result;

pub enum SelectorQuery<'a> {
    Css(&'a str),
    XPath(&'a str),
    Regex(&'a str),
    TextAnchor(&'a str),
}

pub struct SelectorEngine;

impl SelectorEngine {
    /// Evaluates a selector query on a DOM tree and returns matching node indices.
    pub fn query(tree: &DomTree, query: SelectorQuery<'_>) -> Result<Vec<usize>> {
        match query {
            SelectorQuery::Css(sel) => {
                Ok(crate::selector::css::query(tree, sel))
            }
            SelectorQuery::XPath(expr) => {
                Ok(crate::selector::xpath::query(tree, expr))
            }
            SelectorQuery::Regex(pattern) => {
                crate::selector::regex_selector::query(tree, pattern)
            }
            SelectorQuery::TextAnchor(anchor) => {
                Ok(crate::selector::text_anchor::find(tree, anchor))
            }
        }
    }
}
```

---

## 3. Selector Strategies

### A. CSS Selector Engine
- **Underlying Engine:** Built using parsed selectors matching the `DomTree` depth tags.
- **Optimization:** Matches tag sequences, class names, IDs, and attributes against DOM nodes using linear vector scans.

### B. XPath Selector Engine
- **Underlying Engine:** Evaluates standard path routes (e.g. `/html/body/div//h1`).
- **Optimization:** Walks parent-child indexes without building tree-pointer structures, avoiding reference cycles.

### C. Regex Selector Engine
- **Underlying Engine:** Matches text contents or raw HTML snippets against a compiled regular expression.
- **Optimization:** Pre-compiles the Regex pattern and scans DOM text nodes.

### D. Text-Anchor Selector Engine
- **Underlying Engine:** Locates nodes matching or surrounding literal search keys.
- **Support:**
  - `find_text`: Literal string matching.
  - `after_text`: Returns the sibling node directly following the anchor.
  - `before_text`: Returns the sibling node directly preceding the anchor.
