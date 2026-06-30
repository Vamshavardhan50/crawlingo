# docs/PAGE_MODEL_V2.md

This document details the redesigned `Page` model, representing the centralized core representation of fetched web documents in Crawlingo.

---

## 1. Struct Definition & Property Ownership

The `Page` struct owns both the raw document content, the parsed structural DOM representation, and pre-parsed resource indices to maximize parsing speed.

```rust
use crate::parser::document::DomTree;
use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::OnceCell;

pub struct Page {
    // 1. Raw Core Data
    url: String,
    status: u16,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    raw_html: String,
    
    // 2. Parsed Structural DOM representation
    dom_tree: Arc<DomTree>,
    
    // 3. Cached / Lazily Evaluated Fields
    text_content: OnceCell<String>,
    markdown_content: OnceCell<String>,
    
    // Extracted Document Metadata Indices
    links: OnceCell<Vec<String>>,
    images: OnceCell<Vec<ImageResource>>,
    tables: OnceCell<Vec<TableData>>,
    forms: OnceCell<Vec<FormDetails>>,
    scripts: OnceCell<Vec<String>>,
    styles: OnceCell<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ImageResource {
    pub src: String,
    pub alt: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct FormDetails {
    pub action: Option<String>,
    pub method: Option<String>,
    pub inputs: Vec<String>,
}
```

---

## 2. API Methods & Lazy Evaluation

By utilizing `once_cell::sync::OnceCell`, we ensure that secondary resource extraction is completely lazy. The initial parser only constructs the flat `DomTree`. Heavier calculations, such as converting HTML to clean markdown or parsing form inputs, are deferred until requested.

### A. Core Content Methods
- **`html() -> &str`**
  - *Behavior:* Immediate. Returns a borrow of the cached `raw_html`.
- **`text() -> &str`**
  - *Behavior:* Lazy. On the first call, it walks the `DomTree` recursively, filters out script/style elements, joins clean visible text blocks, and caches the result.
- **`markdown() -> &str`**
  - *Behavior:* Lazy. Converts the `DomTree` elements to standard markdown representations (converting headers, links, list tags), caching the string.

### B. Element Query Methods
- **`css(selector: &str) -> ElementCollection`**
  - *Behavior:* Executed on-demand, caching selector matches where appropriate.
- **`xpath(query: &str) -> ElementCollection`**
  - *Behavior:* Executed on-demand.
- **`regex(pattern: &str) -> ElementCollection`**
  - *Behavior:* Executed on-demand.

### C. Resource Scrapers (Lazy & Cached)
- **`links() -> &[String]`**
  - *Behavior:* Lazy. Walks the DOM, collects all `<a href="...">` anchor attributes, resolves relative paths, and caches the vector.
- **`images() -> &[ImageResource]`**
  - *Behavior:* Lazy. Scrapes `<img src="..." alt="...">` tags.
- **`tables() -> &[TableData]`**
  - *Behavior:* Lazy. Parses `<table>` headers, rows, and cell text into tabular objects.
- **`forms() -> &[FormDetails]`**
  - *Behavior:* Lazy. Extracts `<form>` controls, input names, actions, and POST/GET methods.

---

## 3. Benefits of the V2 Design
1. **Thread-Safe Shared State:** Page contains no interior mutability (all caching uses `OnceCell`, which is thread-safe). It can be shared across multiple worker threads (`Arc<Page>`) without locks or mutexes.
2. **Minimal Parsing Overhead:** Web pages with thousands of tags but zero tabular content do not pay the cost of table parsing.
3. **API Consistency:** Both python and Node.js SDK wrappers call the identical underlying methods, ensuring consistent text extraction, relative link resolution, and selector results.
