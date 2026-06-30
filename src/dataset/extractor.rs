use crate::error::Result;
use crate::parser::document::Page;
use crate::selector::SelectorQuery;
use std::collections::HashMap;

/// Denotes the type of data or format to extract and normalize from matching DOM nodes.
pub enum ExtractionType {
    Text,
    Attribute(String),
    Price,
    NormalizedUrl,
}

/// A structured extraction rule for a single target field.
pub struct ExtractionRule {
    pub name: String,
    pub selector: String,
    pub selector_type: String,
    pub extract_type: ExtractionType,
    pub default_value: Option<String>,
}

/// Standalone extraction engine that parses and normalizes structural page DOM elements.
pub struct ExtractionEngine;

impl ExtractionEngine {
    /// Extracts structured fields from a compiled Page using a list of rules.
    pub fn extract(page: &Page, rules: &[ExtractionRule]) -> Result<HashMap<String, String>> {
        let mut fields = HashMap::new();

        for rule in rules {
            let query = match rule.selector_type.as_str() {
                "xpath" => SelectorQuery::XPath(&rule.selector),
                "regex" => SelectorQuery::Regex(&rule.selector),
                "text" => SelectorQuery::TextAnchor(&rule.selector),
                "after_text" => SelectorQuery::AfterText(&rule.selector),
                "before_text" => SelectorQuery::BeforeText(&rule.selector),
                _ => SelectorQuery::Css(&rule.selector),
            };

            let matched_indices = page.query(query)?;

            let extracted_opt = if matched_indices.is_empty() {
                None
            } else {
                let raw_text = page.get_nodes_combined_text(&matched_indices);
                Some(Self::normalize_value(
                    &raw_text,
                    &rule.extract_type,
                    page.url(),
                ))
            };

            let final_val = extracted_opt
                .or_else(|| rule.default_value.clone())
                .unwrap_or_default();

            fields.insert(rule.name.clone(), final_val);
        }

        Ok(fields)
    }

    fn normalize_value(raw: &str, extract_type: &ExtractionType, base_url: &str) -> String {
        match extract_type {
            ExtractionType::Price => {
                let digits: String = raw
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                    .collect();
                digits
            }
            ExtractionType::NormalizedUrl => {
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
