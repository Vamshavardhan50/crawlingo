use crate::parser::document::DomTree;
use memchr::memmem;

/// Finds all nodes containing the target text (using SIMD search via `memchr`).
pub fn find(tree: &DomTree, text: &str) -> Vec<usize> {
    if text.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();
    let finder = memmem::Finder::new(text);

    for (idx, node) in tree.nodes.iter().enumerate() {
        // Exclude script and style blocks from text search
        if node.tag == "script" || node.tag == "style" {
            continue;
        }

        // Match against local text bytes using SIMD
        if finder.find(node.text.as_bytes()).is_some() {
            results.push(idx);
        }
    }

    results
}

/// Finds the element immediately after the element containing the anchor text.
pub fn after(tree: &DomTree, text: &str) -> Vec<usize> {
    let matched = find(tree, text);
    let mut results = Vec::new();

    for idx in matched {
        let node = &tree.nodes[idx];

        // 1. Try next sibling first
        let mut found_sibling = false;
        if let Some(parent_idx) = node.parent {
            let parent_node = &tree.nodes[parent_idx];
            if let Some(pos) = parent_node.children.iter().position(|&child| child == idx) {
                if let Some(&next_sibling_idx) = parent_node.children.get(pos + 1) {
                    results.push(next_sibling_idx);
                    found_sibling = true;
                }
            }
        }

        // 2. Fallback to next node in document order (flattened index + 1)
        if !found_sibling && idx + 1 < tree.nodes.len() {
            results.push(idx + 1);
        }
    }

    results
}

/// Finds the element immediately before the element containing the anchor text.
pub fn before(tree: &DomTree, text: &str) -> Vec<usize> {
    let matched = find(tree, text);
    let mut results = Vec::new();

    for idx in matched {
        let node = &tree.nodes[idx];

        // 1. Try previous sibling first
        let mut found_sibling = false;
        if let Some(parent_idx) = node.parent {
            let parent_node = &tree.nodes[parent_idx];
            if let Some(pos) = parent_node.children.iter().position(|&child| child == idx) {
                if pos > 0 {
                    if let Some(&prev_sibling_idx) = parent_node.children.get(pos - 1) {
                        results.push(prev_sibling_idx);
                        found_sibling = true;
                    }
                }
            }
        }

        // 2. Fallback to previous node in document order (flattened index - 1)
        if !found_sibling && idx > 0 {
            results.push(idx - 1);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::streaming::parse_html;

    #[test]
    fn test_text_anchors() {
        let html = b"<html><body><div><p>Price:</p><span>$150</span></div></body></html>";
        let tree = parse_html(html).unwrap();

        let found_nodes = find(&tree, "Price:");
        assert_eq!(found_nodes.len(), 1);

        let after_nodes = after(&tree, "Price:");
        assert_eq!(after_nodes.len(), 1);
        assert_eq!(tree.get_text(after_nodes[0]), "$150");

        let before_nodes = before(&tree, "$150");
        assert_eq!(before_nodes.len(), 1);
        assert_eq!(tree.get_text(before_nodes[0]), "Price:");
    }
}
