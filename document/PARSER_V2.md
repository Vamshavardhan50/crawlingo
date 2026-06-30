# docs/PARSER_V2.md

This document defines the single-responsibility boundary of the HTML Parser in Crawlingo V2.

---

## 1. Single Responsibility Boundary

In Crawlingo V1, parsing was scattered across network request cycles and dataset compilation. In V2, the Parser is strictly decoupled.

- **The Parser MUST:**
  - Consume raw HTML byte streams (`NormalizedResponse`).
  - Safe-decode text encodings into UTF-8.
  - Construct a flat, vector-indexed DOM tree (`DomTree`).
  - Capture basic document structures (e.g. tracking document metadata, title tags, resource tags, and links).
  - Return an immutable, fully formed `Page` object.
- **The Parser MUST NOT:**
  - Execute HTTP calls or interact with the network.
  - Write files or execute database writes.
  - Perform structured dataset schema validations or write exporter outputs.

```
+--------------------------+
|    NormalizedResponse    |
+--------------------------+
             |
             v
+-------------------------------------------------------+
|                       PARSER                          |
|  - Decode Charset Encodings                           |
|  - Tokenize Elements (lol-html / streaming parser)    |
|  - Populate Flat DomTree Vector                       |
+-------------------------------------------------------+
             |
             v
+--------------------------+
|       Page Object        |
+--------------------------+
```

---

## 2. Parser Interface Design

```rust
use crate::error::Result;
use crate::engine::fetcher::NormalizedResponse;
use crate::page::Page;

pub struct HtmlParser;

impl HtmlParser {
    /// Entry point for document compilation. Parses the response body, 
    /// builds the DomTree, and initializes the Page struct.
    pub fn parse(response: NormalizedResponse) -> Result<Page> {
        // 1. Decode bytes into a String using target encoding
        let html_string = Self::decode_body(&response.body, &response.encoding)?;

        // 2. Tokenize and construct the flat DomTree
        let dom_tree = Self::build_dom_tree(&response.body)?;

        // 3. Assemble and return the Page object
        Ok(Page::new(
            response.url,
            response.status,
            response.headers,
            response.cookies,
            html_string,
            dom_tree,
        ))
    }

    fn decode_body(body: &[u8], encoding: &str) -> Result<String> {
        // Safe encoding resolution (falling back to lossy UTF-8 if invalid)
        let decoded = match encoding.to_lowercase().as_str() {
            "latin1" | "iso-8859-1" => {
                body.iter().map(|&b| b as char).collect::<String>()
            }
            _ => String::from_utf8_lossy(body).into_owned(),
        };
        Ok(decoded)
    }

    fn build_dom_tree(body: &[u8]) -> Result<crate::parser::document::DomTree> {
        // Stream tokenization and index assignment using lol_html
        crate::parser::streaming::parse_html(body)
    }
}
```

---

## 3. Advantages of Decoupled Parsing
1. **Offline Testability:** The parsing module can be unit-tested completely offline. We can load local HTML files from a test fixture directly into `HtmlParser::parse`, completely bypassing network, proxy, or server configurations.
2. **Predictable CPU Allocations:** Because parsing is completely decoupled from async network I/O, heavy parsing workloads can be scheduled on worker threads cleanly using `rayon` or `tokio::task::spawn_blocking`.
3. **Immutability Guarantee:** The parser constructs the page state exactly once. Once parsed, the `Page` is fully immutable, preventing race conditions.
