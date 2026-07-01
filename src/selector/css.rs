use crate::parser::document::{DomNode, DomTree};
use dashmap::DashMap;
use once_cell::sync::Lazy;

/// How an attribute value is matched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeMatch {
    Exists,
    Equals(String),
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Substring(String),
    Hyphen(String),
}

/// Represents a single selector atom (e.g., a tag name, class name, id, or attribute).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectorPart {
    Tag(String),
    Class(String),
    Id(String),
    Attribute {
        name: String,
        match_type: AttributeMatch,
    },
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

/// Parse an attribute expression from inside `[...]`.
/// Expects the string starting after the opening `[` and returns the parsed
/// `Attribute` SelectorPart, or `None` if the expression is malformed.
fn parse_attribute_expr(inner: &str) -> Option<SelectorPart> {
    let inner = inner.trim();
    if inner.is_empty() {
        return None;
    }

    // Find the operator position (first occurrence of =, ~=, |=, ^=, $=, *=)
    let mut op_pos = None;
    let mut op_char: Option<char> = None;
    let mut chars = inner.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            '~' | '|' | '^' | '$' | '*' => {
                if let Some(&(_, '=')) = chars.peek() {
                    op_pos = Some(i);
                    op_char = Some(c);
                    break;
                }
            }
            '=' => {
                // Check if preceding char indicates a compound operator
                if i > 0 {
                    let prev = inner.as_bytes()[i - 1] as char;
                    match prev {
                        '~' | '|' | '^' | '$' | '*' => {
                            // Handled in the previous iteration
                            continue;
                        }
                        _ => {
                            op_pos = Some(i);
                            op_char = Some('=');
                            break;
                        }
                    }
                } else {
                    op_pos = Some(i);
                    op_char = Some('=');
                    break;
                }
            }
            _ => {}
        }
    }

    let name: String;
    let match_type: AttributeMatch;

    if let Some(pos) = op_pos {
        name = inner[..pos].trim().to_string();
        let val_start = if op_char == Some('=') {
            pos + 1
        } else {
            pos + 2
        };
        let raw_val = inner[val_start..].trim();
        let val = if (raw_val.starts_with('"') && raw_val.ends_with('"'))
            || (raw_val.starts_with('\'') && raw_val.ends_with('\''))
        {
            &raw_val[1..raw_val.len() - 1]
        } else {
            raw_val
        }
        .to_string();

        match_type = match op_char {
            Some('=') => AttributeMatch::Equals(val),
            Some('~') => AttributeMatch::Contains(val),
            Some('|') => AttributeMatch::Hyphen(val),
            Some('^') => AttributeMatch::StartsWith(val),
            Some('$') => AttributeMatch::EndsWith(val),
            Some('*') => AttributeMatch::Substring(val),
            _ => return None,
        };
    } else {
        name = inner.trim().to_string();
        match_type = AttributeMatch::Exists;
    }

    if name.is_empty() {
        return None;
    }

    Some(SelectorPart::Attribute { name, match_type })
}

/// Parses a selector string into its structural parts.
pub fn parse_selector(sel_str: &str) -> CompiledSelector {
    let mut groups = Vec::new();
    for word in sel_str.split_whitespace() {
        let mut parts = Vec::new();
        let mut current = String::new();

        let chars: Vec<char> = word.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            match c {
                '.' | '#' => {
                    if !current.is_empty() {
                        parts.push(SelectorPart::Tag(current.clone()));
                        current.clear();
                    }
                    // Start a class or id selector
                    let mut val = String::new();
                    i += 1;
                    while i < chars.len() && chars[i] != '.' && chars[i] != '#' && chars[i] != '[' {
                        val.push(chars[i]);
                        i += 1;
                    }
                    if c == '.' {
                        parts.push(SelectorPart::Class(val));
                    } else {
                        parts.push(SelectorPart::Id(val));
                    }
                    // Continue without incrementing i again (we stopped at a delimiter)
                    continue;
                }
                '[' => {
                    if !current.is_empty() {
                        parts.push(SelectorPart::Tag(current.clone()));
                        current.clear();
                    }
                    // Extract the contents until closing ]
                    i += 1;
                    let mut attr_inner = String::new();
                    let mut depth = 1;
                    while i < chars.len() && depth > 0 {
                        if chars[i] == ']' {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        attr_inner.push(chars[i]);
                        i += 1;
                    }
                    if let Some(attr_part) = parse_attribute_expr(&attr_inner) {
                        parts.push(attr_part);
                    }
                }
                _ => {
                    current.push(c);
                }
            }
            i += 1;
        }
        if !current.is_empty() {
            parts.push(SelectorPart::Tag(current));
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
            SelectorPart::Attribute { name, match_type } => {
                let attr_val = match node.attrs.get(name.as_str()) {
                    Some(v) => v,
                    None => return false,
                };
                match match_type {
                    AttributeMatch::Exists => {}
                    AttributeMatch::Equals(val) => {
                        if attr_val != val {
                            return false;
                        }
                    }
                    AttributeMatch::Contains(val) => {
                        if !attr_val
                            .split_whitespace()
                            .any(|word| word == val)
                        {
                            return false;
                        }
                    }
                    AttributeMatch::Hyphen(val) => {
                        if attr_val != val
                            && !attr_val.starts_with(&format!("{val}-"))
                        {
                            return false;
                        }
                    }
                    AttributeMatch::StartsWith(val) => {
                        if !attr_val.starts_with(val) {
                            return false;
                        }
                    }
                    AttributeMatch::EndsWith(val) => {
                        if !attr_val.ends_with(val) {
                            return false;
                        }
                    }
                    AttributeMatch::Substring(val) => {
                        if !attr_val.contains(val) {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

/// Recursively checks if a node fits the full hierarchal compiled selector.
pub fn selector_matches(tree: &DomTree, node_idx: usize, selector: &CompiledSelector) -> bool {
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
        if selector_matches(tree, idx, &selector) {
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

    #[test]
    fn test_attribute_selectors() {
        let html = b"<html><body>
            <a href='/page1'>Link 1</a>
            <a href='/page2'>Link 2</a>
            <img src='/img.png'>
            <link rel='stylesheet' href='/style.css'>
            <script src='/app.js'></script>
            <input type='text' disabled>
        </body></html>";
        let tree = parse_html(html).unwrap();

        // Attribute exists
        let res = query(&tree, "a[href]");
        assert_eq!(res.len(), 2, "a[href] should match 2 <a> elements");

        let res = query(&tree, "a[href='/page1']");
        assert_eq!(res.len(), 1, "a[href='/page1'] should match 1");

        let res = query(&tree, "img[src]");
        assert_eq!(res.len(), 1, "img[src] should match 1");

        let res = query(&tree, "script[src]");
        assert_eq!(res.len(), 1, "script[src] should match 1");

        let res = query(&tree, "link[rel]");
        assert_eq!(res.len(), 1, "link[rel] should match 1");

        let res = query(&tree, "link[rel='stylesheet']");
        assert_eq!(res.len(), 1, "link[rel='stylesheet'] should match 1");

        // Attribute exists (no value)
        let res = query(&tree, "input[disabled]");
        assert_eq!(res.len(), 1, "input[disabled] should match 1");

        // Attribute not present
        let res = query(&tree, "a[data-x]");
        assert_eq!(res.len(), 0, "a[data-x] should match 0");

        // Combined: tag + attribute + class
        let html2 = b"<html><body><a class='external' href='http://example.com'>Example</a></body></html>";
        let tree2 = parse_html(html2).unwrap();
        let res = query(&tree2, "a.external[href]");
        assert_eq!(res.len(), 1, "a.external[href] should match 1");
    }
}
