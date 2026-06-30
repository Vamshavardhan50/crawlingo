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
