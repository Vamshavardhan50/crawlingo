use crate::error::Result;
use crate::parser::document::Page;
use std::collections::HashMap;

/// Defines how a raw extracted string should be cleaned and typed.
#[derive(Debug, Clone, Default)]
pub enum ExtractionType {
    /// Plain text extraction (trimmed).
    #[default]
    Text,
    /// Extract a specific HTML attribute value.
    Attribute(String),
    /// Strip currency markers and parse as decimal string.
    Price,
    /// Parse common date/time formats into ISO-like string.
    DateTime,
    /// Resolve relative URLs to absolute against the page URL.
    NormalizedUrl,
}

impl ExtractionType {
    /// Parses a selector-config string (as used by the SDKs' `field(..., extract_type=...)`
    /// parameter) into an [`ExtractionType`]. Unrecognized values fall back to `Text`.
    pub fn from_str_or_text(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "price" => ExtractionType::Price,
            "datetime" | "date" => ExtractionType::DateTime,
            "url" | "normalized_url" => ExtractionType::NormalizedUrl,
            other if other.starts_with("attr:") => {
                ExtractionType::Attribute(other["attr:".len()..].to_string())
            }
            _ => ExtractionType::Text,
        }
    }
}

/// A single extraction rule binding a field name to a selector + extraction type.
pub type ExtractionRule = crate::dataset::builder::DatasetField;

/// The Extraction Engine — converts DOM node queries into typed, clean values.
///
/// This engine sits between the Page/DOM layer and the Dataset layer.
/// It does NOT perform network fetches or manage file output.
pub struct ExtractionEngine;

impl ExtractionEngine {
    /// Applies a set of extraction rules to a Page and returns a field map.
    pub fn extract(page: &Page, rules: &[ExtractionRule]) -> Result<HashMap<String, String>> {
        let mut fields = HashMap::new();

        for rule in rules {
            let query =
                crate::selector::SelectorQuery::from_type_str(&rule.selector_type, &rule.selector);

            let matched_indices = page.query(query)?;
            let extracted_val: Option<String> = if matched_indices.is_empty() {
                None
            } else {
                // For Attribute extraction we read directly from the DOM node's attribute map —
                // NOT from the combined text content. This is the canonical attribute extraction path.
                let value = match &rule.extract_type {
                    ExtractionType::Attribute(attr_name) => {
                        page.get_node_attr(&matched_indices, attr_name)
                    }
                    other => {
                        let raw_text = page.get_nodes_combined_text(&matched_indices);
                        Self::normalize_value(&raw_text, other, page.url())
                    }
                };
                if value.is_empty() {
                    None
                } else {
                    Some(value)
                }
            };

            let final_val = extracted_val
                .or_else(|| rule.default.clone())
                .unwrap_or_default();

            fields.insert(rule.name.clone(), final_val);
        }

        Ok(fields)
    }

    /// Applies type-specific normalization to a raw extracted string.
    pub fn normalize_value(raw: &str, extract_type: &ExtractionType, base_url: &str) -> String {
        match extract_type {
            ExtractionType::Text => raw.trim().to_string(),
            ExtractionType::Price => {
                crate::util::price::parse_price_string(raw).unwrap_or_default()
            }
            ExtractionType::DateTime => {
                // Normalize common date formats to YYYY-MM-DD
                let lower = raw.trim().to_lowercase();
                // Try "Month DD, YYYY" or "DD Month YYYY"
                if let Ok(parsed) = chrono::NaiveDate::parse_from_str(&lower, "%B %d, %Y") {
                    return parsed.format("%Y-%m-%d").to_string();
                }
                if let Ok(parsed) = chrono::NaiveDate::parse_from_str(&lower, "%d %B %Y") {
                    return parsed.format("%Y-%m-%d").to_string();
                }
                // Try ISO-like formats
                if let Ok(parsed) = chrono::NaiveDate::parse_from_str(&lower, "%Y-%m-%d") {
                    return parsed.format("%Y-%m-%d").to_string();
                }
                if let Ok(parsed) = chrono::NaiveDate::parse_from_str(&lower, "%m/%d/%Y") {
                    return parsed.format("%Y-%m-%d").to_string();
                }
                raw.trim().to_string()
            }
            ExtractionType::NormalizedUrl => crate::util::url::resolve_url(base_url, raw.trim())
                .unwrap_or_else(|| raw.trim().to_string()),
            ExtractionType::Attribute(_attr_name) => {
                // Attribute values are always extracted at the call site via `Page::get_node_attr`.
                // `normalize_value` is only called for text-based types; reaching this arm means
                // a caller passed a pre-fetched attribute string — just trim and return it.
                raw.trim().to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_extraction_type_text() {
        assert_eq!(
            ExtractionEngine::normalize_value(
                "  hello world  ",
                &ExtractionType::Text,
                "https://example.com"
            ),
            "hello world"
        );
        assert_eq!(
            ExtractionEngine::normalize_value("", &ExtractionType::Text, "https://example.com"),
            ""
        );
    }

    #[test]
    fn test_extraction_type_price() {
        assert_eq!(
            ExtractionEngine::normalize_value(
                " $ 12.34 ",
                &ExtractionType::Price,
                "https://example.com"
            ),
            "12.34"
        );
        assert_eq!(
            ExtractionEngine::normalize_value(
                "1.234.56",
                &ExtractionType::Price,
                "https://example.com"
            ),
            "1234.56"
        );
        assert_eq!(
            ExtractionEngine::normalize_value(
                "no digits here",
                &ExtractionType::Price,
                "https://example.com"
            ),
            ""
        );
    }

    #[test]
    fn test_extraction_type_datetime() {
        // Test Month DD, YYYY
        assert_eq!(
            ExtractionEngine::normalize_value(
                "July 14, 2026",
                &ExtractionType::DateTime,
                "https://example.com"
            ),
            "2026-07-14"
        );
        // Test DD Month YYYY
        assert_eq!(
            ExtractionEngine::normalize_value(
                "14 July 2026",
                &ExtractionType::DateTime,
                "https://example.com"
            ),
            "2026-07-14"
        );
        // Test YYYY-MM-DD
        assert_eq!(
            ExtractionEngine::normalize_value(
                "2026-07-14",
                &ExtractionType::DateTime,
                "https://example.com"
            ),
            "2026-07-14"
        );
        // Test MM/DD/YYYY
        assert_eq!(
            ExtractionEngine::normalize_value(
                "07/14/2026",
                &ExtractionType::DateTime,
                "https://example.com"
            ),
            "2026-07-14"
        );
    }

    #[test]
    fn test_extraction_type_normalized_url() {
        assert_eq!(
            ExtractionEngine::normalize_value(
                " /path/to/page ",
                &ExtractionType::NormalizedUrl,
                "https://example.com/sub/"
            ),
            "https://example.com/path/to/page"
        );
        assert_eq!(
            ExtractionEngine::normalize_value(
                "https://google.com",
                &ExtractionType::NormalizedUrl,
                "https://example.com"
            ),
            "https://google.com"
        );
    }

    /// Verifies that `ExtractionType::Attribute` pulls the value from the DOM attribute map,
    /// NOT from the node's text content. This is the regression test for the P0 bug where
    /// `extract_type="attr:href"` would return the link text instead of the href URL.
    #[test]
    fn test_extraction_type_attribute_extracts_from_dom_not_text() {
        use std::collections::HashMap;
        let html = r#"
            <div>
              <a class="link" href="https://example.com/page">Click here</a>
              <img id="hero" src="/images/hero.png" alt="Hero image">
              <span data-id="42">some text</span>
              <a class="empty-href" href="">Empty href</a>
            </div>
        "#;
        let page = crate::parser::streaming::HtmlParser::parse(
            crate::engine::fetcher::NormalizedResponse {
                url: "https://example.com".to_string(),
                status: 200,
                headers: HashMap::new(),
                cookies: HashMap::new(),
                body: html.into(),
                content_type: "text/html".to_string(),
                encoding: "utf-8".to_string(),
                timings: Default::default(),
            },
        )
        .unwrap();

        let rules = vec![
            // href — must return URL string, not "Click here"
            ExtractionRule {
                name: "link_href".to_string(),
                selector: "a.link".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::Attribute("href".to_string()),
                default: None,
                #[cfg(feature = "python")]
                transform: None,
            },
            // src — must return image path, not alt text
            ExtractionRule {
                name: "img_src".to_string(),
                selector: "#hero".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::Attribute("src".to_string()),
                default: None,
                #[cfg(feature = "python")]
                transform: None,
            },
            // data-id — custom data attribute
            ExtractionRule {
                name: "data_id".to_string(),
                selector: "span".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::Attribute("data-id".to_string()),
                default: None,
                #[cfg(feature = "python")]
                transform: None,
            },
            // missing attribute — must fall back to default
            ExtractionRule {
                name: "missing_attr".to_string(),
                selector: "a.link".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::Attribute("nonexistent".to_string()),
                default: Some("MISSING".to_string()),
                #[cfg(feature = "python")]
                transform: None,
            },
            // empty href — empty attribute should fall back to default
            ExtractionRule {
                name: "empty_href".to_string(),
                selector: "a.empty-href".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::Attribute("href".to_string()),
                default: Some("DEFAULT".to_string()),
                #[cfg(feature = "python")]
                transform: None,
            },
            // class attribute
            ExtractionRule {
                name: "link_class".to_string(),
                selector: "a.link".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::Attribute("class".to_string()),
                default: None,
                #[cfg(feature = "python")]
                transform: None,
            },
        ];

        let result = ExtractionEngine::extract(&page, &rules).unwrap();

        // P0 regression: href must be the URL, NOT the link text "Click here"
        assert_eq!(
            result.get("link_href").unwrap(),
            "https://example.com/page",
            "attr:href must return the href attribute value, not the node text"
        );
        assert_eq!(
            result.get("img_src").unwrap(),
            "/images/hero.png",
            "attr:src must return the src attribute value"
        );
        assert_eq!(
            result.get("data_id").unwrap(),
            "42",
            "attr:data-id must return the data-id attribute value"
        );
        // missing attribute → falls through to default
        assert_eq!(
            result.get("missing_attr").unwrap(),
            "MISSING",
            "missing attribute must use field default"
        );
        // empty href → empty string → falls through to default
        assert_eq!(
            result.get("empty_href").unwrap(),
            "DEFAULT",
            "empty attribute string must use field default"
        );
        assert_eq!(
            result.get("link_class").unwrap(),
            "link",
            "attr:class must return the class attribute value"
        );
    }

    #[test]
    fn test_extraction_engine_extract() {
        let html = r#"
            <div class="product">
                <h1 class="title">  Awesome Shoes  </h1>
                <span class="price">$99.99</span>
                <a class="link" href="/shoes/1">/shoes/1</a>
            </div>
        "#;
        let page = crate::parser::streaming::HtmlParser::parse(
            crate::engine::fetcher::NormalizedResponse {
                url: "https://example.com/products".to_string(),
                status: 200,
                headers: HashMap::new(),
                cookies: HashMap::new(),
                body: html.into(),
                content_type: "text/html".to_string(),
                encoding: "utf-8".to_string(),
                timings: Default::default(),
            },
        )
        .unwrap();

        let rules = vec![
            ExtractionRule {
                name: "title".to_string(),
                selector: ".title".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::Text,
                default: None,
                #[cfg(feature = "python")]
                transform: None,
            },
            ExtractionRule {
                name: "price".to_string(),
                selector: ".price".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::Price,
                default: None,
                #[cfg(feature = "python")]
                transform: None,
            },
            ExtractionRule {
                name: "link".to_string(),
                selector: ".link".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::NormalizedUrl,
                default: None,
                #[cfg(feature = "python")]
                transform: None,
            },
            ExtractionRule {
                name: "missing".to_string(),
                selector: ".missing".to_string(),
                selector_type: "css".to_string(),
                extract_type: ExtractionType::Text,
                default: Some("default-value".to_string()),
                #[cfg(feature = "python")]
                transform: None,
            },
        ];

        let extracted = ExtractionEngine::extract(&page, &rules).unwrap();
        assert_eq!(extracted.get("title").unwrap(), "Awesome Shoes");
        assert_eq!(extracted.get("price").unwrap(), "99.99");
        assert_eq!(
            extracted.get("link").unwrap(),
            "https://example.com/shoes/1"
        );
        assert_eq!(extracted.get("missing").unwrap(), "default-value");
    }
}
