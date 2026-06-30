# docs/EXTRACTION_ENGINE_V2.md

This document defines the single-responsibility boundary of the Extraction Engine in Crawlingo V2.

---

## 1. Single Responsibility Boundary

The Extraction Engine sits between the DOM/Page representation and the Dataset Engine. Its sole responsibility is converting raw DOM elements into typed, clean values.

- **The Extraction Engine MUST:**
  - Retrieve text or attributes from specified DOM node indices.
  - Parse and normalize formats (e.g. converting `"$1,299.99"` into a float value, parsing date formats, resolving relative URL paths).
  - Support fallback default values.
- **The Extraction Engine MUST NOT:**
  - Perform HTTP requests or connect to the network.
  - Parse HTML byte streams.
  - Manage database tables or file write streams.

---

## 2. Extraction Pipeline Flow

```
+-------------+
|    Page     | (Provides DomTree & URL context)
+-------------+
       |
       | 1. Query indices via SelectorEngine
       v
+-------------+
| Node List   | (Slices of matching usize elements)
+-------------+
       |
       | 2. Extract raw attributes or inner text
       v
+-------------+
| Raw String  | (e.g. "$ 49.99 USD")
+-------------+
       |
       | 3. Apply cleaning transform strategies
       v
+-------------+
| Typed Value | (e.g. Float: 49.99)
+-------------+
```

---

## 3. Extraction Rule Configuration

```rust
use crate::page::Page;
use crate::error::Result;

pub enum ExtractionType {
    Text,
    Attribute(String),
    Price,
    DateTime,
    NormalizedUrl,
}

pub struct ExtractionRule {
    pub name: String,
    pub selector: String,
    pub selector_type: String,
    pub extract_type: ExtractionType,
    pub default_value: Option<String>,
}

pub struct ExtractionEngine;

impl ExtractionEngine {
    /// Extracts structured field values from a Page using specified rules.
    pub fn extract(page: &Page, rules: &[ExtractionRule]) -> Result<std::collections::HashMap<String, String>> {
        let mut fields = std::collections::HashMap::new();

        for rule in rules {
            // 1. Resolve Selector matching indices
            let query = match rule.selector_type.as_str() {
                "xpath" => crate::docs::SELECTOR_ENGINE_V2::SelectorQuery::XPath(&rule.selector),
                "regex" => crate::docs::SELECTOR_ENGINE_V2::SelectorQuery::Regex(&rule.selector),
                _ => crate::docs::SELECTOR_ENGINE_V2::SelectorQuery::Css(&rule.selector),
            };
            
            let matched_indices = page.query(query)?;
            
            // 2. Perform Extraction & Normalization
            let extracted_opt = if matched_indices.is_empty() {
                None
            } else {
                let raw_text = page.get_nodes_combined_text(&matched_indices);
                Some(Self::normalize_value(&raw_text, &rule.extract_type, page.url()))
            };

            // 3. Fallback to default
            let final_value = extracted_opt
                .or_else(|| rule.default_value.clone())
                .unwrap_or_default();

            fields.insert(rule.name.clone(), final_value);
        }

        Ok(fields)
    }

    fn normalize_value(raw: &str, extract_type: &ExtractionType, base_url: &str) -> String {
        match extract_type {
            ExtractionType::Price => {
                // Strip currency markers and format decimal
                let digits: String = raw.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
                digits
            }
            ExtractionType::NormalizedUrl => {
                // Resolve relative URLs to absolute
                if let Ok(base) = url::Url::parse(base_url) {
                    if let Ok(resolved) = base.join(raw) {
                        return resolved.to_string();
                    }
                }
                raw.to_string()
            }
            _ => raw.trim().to_string(),
        }
    }
}
```
