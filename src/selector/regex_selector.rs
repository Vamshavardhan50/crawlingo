use once_cell::sync::Lazy;
use dashmap::DashMap;
use regex::Regex;
use crate::parser::document::DomTree;
use crate::error::{CrawlingoError, Result};

static REGEX_CACHE: Lazy<DashMap<String, Regex>> = Lazy::new(DashMap::new);

/// Retrieves a precompiled regular expression from the cache or compiles it.
pub fn get_or_compile_regex(pattern: &str) -> Result<Regex> {
    if let Some(re) = REGEX_CACHE.get(pattern) {
        return Ok(re.clone());
    }
    let re = Regex::new(pattern)
        .map_err(|e| CrawlingoError::SelectorError(format!("Invalid regex pattern: {}", e)))?;
    REGEX_CACHE.insert(pattern.to_string(), re.clone());
    Ok(re)
}

/// Finds all nodes in the `DomTree` whose text contents match the regular expression pattern.
pub fn query(tree: &DomTree, pattern: &str) -> Result<Vec<usize>> {
    let re = get_or_compile_regex(pattern)?;
    let mut results = Vec::new();

    for (idx, node) in tree.nodes.iter().enumerate() {
        if node.tag == "script" || node.tag == "style" {
            continue;
        }
        if re.is_match(&node.text) {
            results.push(idx);
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::streaming::parse_html;

    #[test]
    fn test_regex_matching() {
        let html = b"<html><body><div class='product'><p>Stock: In Stock</p></div></body></html>";
        let tree = parse_html(html).unwrap();

        let matched = query(&tree, r"In Stock|Out of Stock").unwrap();
        assert_eq!(matched.len(), 1);
        assert_eq!(tree.get_text(matched[0]), "Stock: In Stock");
    }
}
