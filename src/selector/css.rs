use once_cell::sync::Lazy;
use dashmap::DashMap;
use crate::parser::document::{DomNode, DomTree};

/// Represents a single selector atom (e.g., a tag name, class name, or id).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectorPart {
    Tag(String),
    Class(String),
    Id(String),
}

/// Represents a compound selector (e.g. `div.product#title` is Tag("div") + Class("product") + Id("title")).
#[derive(Debug, Clone)]
pub struct SelectorGroup {
    pub parts: Vec<SelectorPart>,
}

/// A parsed CSS selector query containing descendant hierarchy groups.
#[derive(Debug, Clone)]
pub struct CompiledSelector {
    pub groups: Vec<SelectorGroup>,
}

// Global thread-safe selector compilation cache
static SELECTOR_CACHE: Lazy<DashMap<String, CompiledSelector>> = Lazy::new(DashMap::new);

/// Parses a selector string into its structural parts.
pub fn parse_selector(sel_str: &str) -> CompiledSelector {
    let mut groups = Vec::new();
    for word in sel_str.split_whitespace() {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut mode = ' '; // ' ' for tag, '.' for class, '#' for id

        let chars: Vec<char> = word.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if c == '.' || c == '#' {
                if !current.is_empty() {
                    match mode {
                        ' ' => parts.push(SelectorPart::Tag(current.clone())),
                        '.' => parts.push(SelectorPart::Class(current.clone())),
                        '#' => parts.push(SelectorPart::Id(current.clone())),
                        _ => {}
                    }
                    current.clear();
                }
                mode = c;
            } else {
                current.push(c);
            }
            i += 1;
        }
        if !current.is_empty() {
            match mode {
                ' ' => parts.push(SelectorPart::Tag(current)),
                '.' => parts.push(SelectorPart::Class(current)),
                '#' => parts.push(SelectorPart::Id(current)),
                _ => {}
            }
        }
        groups.push(SelectorGroup { parts });
    }
    CompiledSelector { groups }
}

/// Retrieves the precompiled selector from the cache, or parses it on first miss.
pub fn get_or_compile(sel_str: &str) -> CompiledSelector {
    SELECTOR_CACHE
        .entry(sel_str.to_string())
        .or_insert_with(|| parse_selector(sel_str))
        .value()
        .clone()
}

/// Evaluates if a node matches a single compound selector group.
fn match_group(node: &DomNode, group: &SelectorGroup) -> bool {
    for part in &group.parts {
        match part {
            SelectorPart::Tag(t) => {
                if node.tag.to_lowercase() != t.to_lowercase() {
                    return false;
                }
            }
            SelectorPart::Class(c) => {
                if let Some(classes) = node.attrs.get("class") {
                    let has_class = classes.split_whitespace().any(|cls| cls == c);
                    if !has_class {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            SelectorPart::Id(id_val) => {
                if let Some(id) = node.attrs.get("id") {
                    if id != id_val {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    true
}

/// Recursively checks if a node fits the full hierarchal compiled selector.
pub fn matches(tree: &DomTree, node_idx: usize, selector: &CompiledSelector) -> bool {
    if selector.groups.is_empty() {
        return false;
    }

    let last_group_idx = selector.groups.len() - 1;
    let leaf_node = &tree.nodes[node_idx];
    if !match_group(leaf_node, &selector.groups[last_group_idx]) {
        return false;
    }

    if selector.groups.len() == 1 {
        return true;
    }

    let mut current_node_idx = node_idx;
    let mut group_idx = last_group_idx;

    while group_idx > 0 {
        let current_node = &tree.nodes[current_node_idx];
        if let Some(parent_idx) = current_node.parent {
            let parent_node = &tree.nodes[parent_idx];
            if match_group(parent_node, &selector.groups[group_idx - 1]) {
                group_idx -= 1;
            }
            current_node_idx = parent_idx;
        } else {
            return false;
        }
    }

    true
}

/// Scans the entire `DomTree` for elements matching the query selector string.
pub fn query(tree: &DomTree, selector_str: &str) -> Vec<usize> {
    let selector = get_or_compile(selector_str);
    let mut results = Vec::new();
    for idx in 0..tree.nodes.len() {
        if matches(tree, idx, &selector) {
            results.push(idx);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::streaming::parse_html;

    #[test]
    fn test_selector_matching() {
        let html = b"<html><body><div class='product' id='p1'><h1>Title</h1><span class='price'>$100</span></div></body></html>";
        let tree = parse_html(html).unwrap();

        let matched = query(&tree, "div.product span.price");
        assert_eq!(matched.len(), 1);
        assert_eq!(tree.get_text(matched[0]), "$100");

        let matched_id = query(&tree, "div#p1 h1");
        assert_eq!(matched_id.len(), 1);
        assert_eq!(tree.get_text(matched_id[0]), "Title");
    }
}
