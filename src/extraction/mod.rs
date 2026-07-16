use crate::error::Result;
use crate::parser::document::Page;
use crate::selector::SelectorQuery;
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
            let query = match rule.selector_type.as_str() {
                "xpath" => SelectorQuery::XPath(&rule.selector),
                "regex" => SelectorQuery::Regex(&rule.selector),
                "text" | "text_anchor" => SelectorQuery::TextAnchor(&rule.selector),
                "after_text" => SelectorQuery::AfterText(&rule.selector),
                "before_text" => SelectorQuery::BeforeText(&rule.selector),
                _ => SelectorQuery::Css(&rule.selector),
            };

            let matched_indices = page.query(query)?;
            let extracted_val: Option<String> = if matched_indices.is_empty() {
                None
            } else {
                let raw_text = page.get_nodes_combined_text(&matched_indices);
                let cleaned = Self::normalize_value(&raw_text, &rule.extract_type, page.url());
                if cleaned.is_empty() {
                    None
                } else {
                    Some(cleaned)
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
                // This type should be paired with attribute extraction at call site;
                // here we just return the raw value as-is
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

    #[test]
    fn test_extraction_type_attribute() {
        assert_eq!(
            ExtractionEngine::normalize_value(
                " raw value ",
                &ExtractionType::Attribute("src".to_string()),
                "https://example.com"
            ),
            "raw value"
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
