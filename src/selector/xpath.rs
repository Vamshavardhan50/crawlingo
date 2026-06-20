use crate::parser::document::{DomNode, DomTree};

#[derive(Debug, Clone)]
pub struct XPathSegment {
    pub tag: String,
    pub is_descendant: bool,                   // true for '//', false for '/'
    pub attr_filter: Option<(String, String)>, // e.g. Some(("class", "price"))
}

/// A parsed basic XPath expression.
#[derive(Debug, Clone)]
pub struct CompiledXPath {
    pub segments: Vec<XPathSegment>,
}

/// Parses common basic XPath queries like `//div[@class='product']` or `//span`.
pub fn parse_xpath(xpath_str: &str) -> Option<CompiledXPath> {
    if xpath_str.is_empty() {
        return None;
    }

    let mut segments = Vec::new();
    let mut rest = xpath_str;

    // Standardize input prefix
    let mut is_descendant = false;
    if rest.starts_with("//") {
        is_descendant = true;
        rest = &rest[2..];
    } else if rest.starts_with('/') {
        rest = &rest[1..];
    }

    // Split segments by '/' (ignoring inner slashes inside filter quotes if any, but basic splits are standard)
    let raw_segs = rest.split('/');
    for (i, seg) in raw_segs.enumerate() {
        if seg.is_empty() {
            // double slash inside expression
            is_descendant = true;
            continue;
        }

        // Determine descendant flag for this segment
        let desc = if i == 0 { is_descendant } else { is_descendant };
        // Reset descendant flag for subsequent direct children
        is_descendant = false;

        // Parse segment parts like tag and filter: "div[@class='product']"
        let tag;
        let mut attr_filter = None;

        if let Some(start_idx) = seg.find('[') {
            tag = seg[..start_idx].to_string();
            if let Some(end_idx) = seg.find(']') {
                let filter = &seg[start_idx + 1..end_idx]; // "@class='product'"
                if filter.starts_with('@') {
                    let parts: Vec<&str> = filter[1..].split('=').collect();
                    if parts.len() == 2 {
                        let attr_name = parts[0].trim().to_string();
                        let attr_val = parts[1]
                            .trim()
                            .trim_matches('\'')
                            .trim_matches('"')
                            .to_string();
                        attr_filter = Some((attr_name, attr_val));
                    }
                }
            }
        } else {
            tag = seg.to_string();
        }

        segments.push(XPathSegment {
            tag,
            is_descendant: desc,
            attr_filter,
        });
    }

    Some(CompiledXPath { segments })
}

/// Evaluates if a node fits a parsed XPath segment.
fn match_segment(node: &DomNode, segment: &XPathSegment) -> bool {
    if segment.tag != "*" && node.tag.to_lowercase() != segment.tag.to_lowercase() {
        return false;
    }

    if let Some((ref name, ref val)) = segment.attr_filter {
        if let Some(node_val) = node.attrs.get(name) {
            // For class name match, let it support standard word matching or exact match
            if name == "class" {
                if !node_val.split_whitespace().any(|c| c == val) && node_val != val {
                    return false;
                }
            } else if node_val != val {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

/// Evaluates the compiled XPath recursively down the tree.
pub fn matches(tree: &DomTree, node_idx: usize, xpath: &CompiledXPath) -> bool {
    if xpath.segments.is_empty() {
        return false;
    }

    let last_seg_idx = xpath.segments.len() - 1;
    let leaf_node = &tree.nodes[node_idx];
    if !match_segment(leaf_node, &xpath.segments[last_seg_idx]) {
        return false;
    }

    if xpath.segments.len() == 1 {
        return true;
    }

    let mut current_node_idx = node_idx;
    let mut seg_idx = last_seg_idx;

    while seg_idx > 0 {
        let current_node = &tree.nodes[current_node_idx];
        let segment = &xpath.segments[seg_idx - 1];

        if segment.is_descendant {
            // Traverse all ancestors to find a match
            let mut matched = false;
            let mut ancestor_idx = current_node.parent;
            while let Some(parent_idx) = ancestor_idx {
                if match_segment(&tree.nodes[parent_idx], segment) {
                    matched = true;
                    current_node_idx = parent_idx;
                    break;
                }
                ancestor_idx = tree.nodes[parent_idx].parent;
            }
            if !matched {
                return false;
            }
            seg_idx -= 1;
        } else {
            // Direct parent match
            if let Some(parent_idx) = current_node.parent {
                if match_segment(&tree.nodes[parent_idx], segment) {
                    current_node_idx = parent_idx;
                    seg_idx -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    true
}

/// Evaluates basic XPath expressions on a `DomTree` and returns matched node indices.
pub fn query(tree: &DomTree, xpath_str: &str) -> Vec<usize> {
    let mut results = Vec::new();
    if let Some(xpath) = parse_xpath(xpath_str) {
        for idx in 0..tree.nodes.len() {
            if matches(tree, idx, &xpath) {
                results.push(idx);
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::streaming::parse_html;

    #[test]
    fn test_xpath_matching() {
        let html = b"<html><body><div class='product'><h1>Title</h1><span class='price'>$200</span></div></body></html>";
        let tree = parse_html(html).unwrap();

        let res = query(&tree, "//div[@class='product']/span[@class='price']");
        assert_eq!(res.len(), 1);
        assert_eq!(tree.get_text(res[0]), "$200");

        let res_desc = query(&tree, "//span");
        assert_eq!(res_desc.len(), 1);
        assert_eq!(tree.get_text(res_desc[0]), "$200");
    }
}
