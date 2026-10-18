pub mod css;
pub mod regex_selector;
pub mod text_anchor;
pub mod xpath;

use crate::error::Result;
use crate::parser::document::DomTree;

/// Unified query selector enum.
pub enum SelectorQuery<'a> {
    Css(&'a str),
    XPath(&'a str),
    Regex(&'a str),
    TextAnchor(&'a str),
    AfterText(&'a str),
    BeforeText(&'a str),
}

impl<'a> SelectorQuery<'a> {
    /// Creates a [`SelectorQuery`] from a string type tag and a selector expression.
    ///
    /// This is the **single canonical dispatch factory** for selector type routing. Every
    /// subsystem (extraction engine, dataset builder, crawler, watch monitor) must call this
    /// instead of duplicating the match arm locally.
    ///
    /// | `selector_type`                | Variant              |
    /// |-------------------------------|----------------------|
    /// | `"xpath"`                      | `XPath`              |
    /// | `"regex"`                      | `Regex`              |
    /// | `"text"` / `"text_anchor"`     | `TextAnchor`         |
    /// | `"after_text"`                 | `AfterText`          |
    /// | `"before_text"`                | `BeforeText`         |
    /// | anything else (incl. `"css"`)  | `Css` (default)      |
    pub fn from_type_str(selector_type: &str, selector: &'a str) -> Self {
        match selector_type {
            "xpath" => SelectorQuery::XPath(selector),
            "regex" => SelectorQuery::Regex(selector),
            "text" | "text_anchor" => SelectorQuery::TextAnchor(selector),
            "after_text" => SelectorQuery::AfterText(selector),
            "before_text" => SelectorQuery::BeforeText(selector),
            // "css" or any unrecognised value defaults to CSS
            _ => SelectorQuery::Css(selector),
        }
    }
}

/// The unified selector engine routing requests to different query engines.
pub struct SelectorEngine;

impl SelectorEngine {
    /// Queries a DomTree using the specified selector query, returning node indices.
    pub fn query(tree: &DomTree, query: SelectorQuery<'_>) -> Result<Vec<usize>> {
        match query {
            SelectorQuery::Css(sel) => Ok(css::query(tree, sel)),
            SelectorQuery::XPath(expr) => Ok(xpath::query(tree, expr)),
            SelectorQuery::Regex(pat) => regex_selector::query(tree, pat),
            SelectorQuery::TextAnchor(txt) => Ok(text_anchor::find(tree, txt)),
            SelectorQuery::AfterText(txt) => Ok(text_anchor::after(tree, txt)),
            SelectorQuery::BeforeText(txt) => Ok(text_anchor::before(tree, txt)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_type_str_routes_xpath() {
        let q = SelectorQuery::from_type_str("xpath", "//h1");
        assert!(matches!(q, SelectorQuery::XPath("//h1")));
    }

    #[test]
    fn from_type_str_routes_regex() {
        let q = SelectorQuery::from_type_str("regex", r"\d+");
        assert!(matches!(q, SelectorQuery::Regex(_)));
    }

    #[test]
    fn from_type_str_routes_text_anchor() {
        let q = SelectorQuery::from_type_str("text", "Price");
        assert!(matches!(q, SelectorQuery::TextAnchor("Price")));
        let q2 = SelectorQuery::from_type_str("text_anchor", "Price");
        assert!(matches!(q2, SelectorQuery::TextAnchor("Price")));
    }

    #[test]
    fn from_type_str_routes_after_text() {
        let q = SelectorQuery::from_type_str("after_text", "Label");
        assert!(matches!(q, SelectorQuery::AfterText("Label")));
    }

    #[test]
    fn from_type_str_routes_before_text() {
        let q = SelectorQuery::from_type_str("before_text", "Label");
        assert!(matches!(q, SelectorQuery::BeforeText("Label")));
    }

    #[test]
    fn from_type_str_defaults_to_css_for_unknown() {
        // "css" explicitly
        let q = SelectorQuery::from_type_str("css", "h1");
        assert!(matches!(q, SelectorQuery::Css("h1")));
        // empty string
        let q2 = SelectorQuery::from_type_str("", ".item");
        assert!(matches!(q2, SelectorQuery::Css(".item")));
        // invalid / garbage
        let q3 = SelectorQuery::from_type_str("NOTATYPE", "#id");
        assert!(matches!(q3, SelectorQuery::Css("#id")));
    }
}
