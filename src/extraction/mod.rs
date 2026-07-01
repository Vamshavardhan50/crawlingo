use crate::error::Result;
use crate::parser::document::Page;
use crate::selector::SelectorQuery;
use std::collections::HashMap;

/// Defines how a raw extracted string should be cleaned and typed.
#[derive(Debug, Clone)]
pub enum ExtractionType {
    /// Plain text extraction (trimmed).
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

/// A single extraction rule binding a field name to a selector + extraction type.
#[derive(Debug, Clone)]
pub struct ExtractionRule {
    pub name: String,
    pub selector: String,
    pub selector_type: String,
    pub extract_type: ExtractionType,
    pub default_value: Option<String>,
}

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
                if cleaned.is_empty() { None } else { Some(cleaned) }
            };

            let final_val = extracted_val
                .or_else(|| rule.default_value.clone())
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
                // Strip currency markers, keep digits and decimal point
                let cleaned: String = raw
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                    .collect();
                // Remove duplicate dots
                let mut parts: Vec<&str> = cleaned.split('.').collect();
                if parts.len() > 2 {
                    let decimal = parts.pop().unwrap_or("");
                    let integer = parts.join("");
                    format!("{}.{}", integer, decimal)
                } else {
                    cleaned
                }
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
            ExtractionType::NormalizedUrl => {
                if let Ok(base) = url::Url::parse(base_url) {
                    if let Ok(resolved) = base.join(raw.trim()) {
                        return resolved.to_string();
                    }
                }
                raw.trim().to_string()
            }
            ExtractionType::Attribute(_attr_name) => {
                // This type should be paired with attribute extraction at call site;
                // here we just return the raw value as-is
                raw.trim().to_string()
            }
        }
    }
}
