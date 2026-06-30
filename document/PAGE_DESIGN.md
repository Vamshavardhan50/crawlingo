# docs/PAGE_DESIGN.md

This document designs the `Page` object, which serves as the central data model for all extraction pipelines in Crawlingo.

---

## 1. Page Struct Definition & Ownership

The `Page` object owns the downloaded response body and metadata, keeping the DOM structure cached via a shared atomic pointer wrapper (`Arc`).

```rust
pub struct Page {
    /// The final, fully resolved URL (after redirects).
    url: String,
    /// HTTP status code.
    status: u16,
    /// Raw HTTP response headers.
    headers: HashMap<String, String>,
    /// Raw page HTML content.
    html: String,
    /// Flat DOM tree representation.
    tree: Arc<DomTree>,
}
```

---

## 2. API Signature & Methods

```rust
impl Page {
    /// Return the raw HTML contents.
    pub fn html(&self) -> String {
        self.html.clone()
    }

    /// Extract all inner text content from the body, stripping HTML tags.
    pub fn text(&self) -> String {
        self.tree.get_inner_text()
    }

    /// Convert the HTML tree to clean Markdown representation.
    pub fn markdown(&self) -> String {
        markdown::from_html(&self.html)
    }

    /// Query elements matching a CSS selector query.
    pub fn css(&self, selector: &str) -> ElementCollection {
        let indices = css::query(&self.tree, selector);
        ElementCollection::new(self.tree.clone(), indices)
    }

    /// Query elements matching an XPath query.
    pub fn xpath(&self, query: &str) -> ElementCollection {
        let indices = xpath::query(&self.tree, query);
        ElementCollection::new(self.tree.clone(), indices)
    }

    /// Retrieve all absolute links on the page.
    pub fn links(&self) -> Vec<String> {
        let indices = css::query(&self.tree, "a[href]");
        indices.iter()
            .filter_map(|&idx| self.tree.get_attribute(idx, "href"))
            .map(|href| self.resolve_url(&href))
            .collect()
    }

    /// Extract all image URLs and alt text parameters.
    pub fn images(&self) -> Vec<ImageItem> {
        let indices = css::query(&self.tree, "img");
        indices.iter().map(|&idx| {
            ImageItem {
                src: self.tree.get_attribute(idx, "src").unwrap_or_default(),
                alt: self.tree.get_attribute(idx, "alt").unwrap_or_default(),
            }
        }).collect()
    }

    /// Extract structured tables as vector maps.
    pub fn tables(&self) -> Vec<TableItem> {
        tables::extract(&self.tree)
    }

    /// Extract common page metadata (e.g. OpenGraph tags, JSON-LD, description).
    pub fn metadata(&self) -> PageMetadata {
        metadata::extract(&self.tree)
    }

    /// Create and extract a structured dataset from this page context.
    pub fn dataset(&self, schema: DatasetSchema) -> Result<DatasetResult, CrawlingoError> {
        let builder = DatasetBuilder::new(schema);
        builder.extract(self)
    }
}
```

---

## 3. Design Rationales

### Why do these methods belong directly on Page?
1. **Developer Ergonomics:** Standardizing `.links()`, `.text()`, `.markdown()`, and `.dataset()` directly on the `Page` object prevents developers from having to write repetitive CSS helper loops or instantiating separate dataset utility modules for common extraction tasks.
2. **Performance Optimization:** Internal helpers can traverse the contiguous `DomTree` vector array directly rather than instantiating high-overhead selector engines.
3. **FFI Cleanliness:** Native JavaScript and Python clients can invoke these methods via FFI to retrieve formatted strings, lists, or structured records directly, avoiding complex custom element traversals inside VM engines.
