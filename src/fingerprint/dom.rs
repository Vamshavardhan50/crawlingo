use crate::parser::document::DomTree;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use xxhash_rust::xxh64::xxh64;

/// Represents context about an ancestor node in the DOM tree.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AncestorNode {
    pub tag: String,
    pub class: String,
    pub id: String,
    pub depth: usize,
}

/// A DOM Fingerprint representing an element's structural position, attributes, and surrounding context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomFingerprint {
    // Identity
    pub tag: String,
    pub text: String,
    pub html_snippet: String,

    // Position
    pub depth: usize,
    pub sibling_index: usize,

    // Parent context
    pub parent_tag: String,
    pub parent_class: String,
    pub parent_id: String,

    // Self attributes
    pub attributes: HashMap<String, String>,
    pub class_list: Vec<String>,
    pub id: Option<String>,

    // Full ancestor chain
    pub ancestor_path: Vec<AncestorNode>,

    // Computed
    pub hash: u64,

    // Meta
    pub captured_at: DateTime<Utc>,
    pub url: String,
    pub selector_used: String,
    pub similarity_score: f64,
}

impl DomFingerprint {
    /// Generates a `DomFingerprint` for a node in a `DomTree`.
    pub fn generate(
        tree: &DomTree,
        node_idx: usize,
        url: &str,
        selector_used: &str,
    ) -> Option<Self> {
        let node = tree.nodes.get(node_idx)?;
        let tag = node.tag.clone();
        let text = tree.get_text(node_idx);
        let html_snippet = node.html_snippet.clone();
        let depth = node.depth;

        // Compute sibling index
        let mut sibling_index = 0;
        let mut parent_tag = String::new();
        let mut parent_class = String::new();
        let mut parent_id = String::new();

        if let Some(parent_idx) = node.parent {
            if let Some(parent_node) = tree.nodes.get(parent_idx) {
                parent_tag = parent_node.tag.clone();
                parent_class = parent_node.attrs.get("class").cloned().unwrap_or_default();
                parent_id = parent_node.attrs.get("id").cloned().unwrap_or_default();

                if let Some(pos) = parent_node.children.iter().position(|&idx| idx == node_idx) {
                    sibling_index = pos;
                }
            }
        }

        let attributes = node.attrs.clone();
        let class_list: Vec<String> = node
            .attrs
            .get("class")
            .map(|c| c.split_whitespace().map(|s| s.to_string()).collect())
            .unwrap_or_default();
        let id = node.attrs.get("id").cloned();

        // Build ancestor path
        let mut ancestor_path = Vec::new();
        let mut current_parent = node.parent;
        while let Some(parent_idx) = current_parent {
            if let Some(p) = tree.nodes.get(parent_idx) {
                ancestor_path.push(AncestorNode {
                    tag: p.tag.clone(),
                    class: p.attrs.get("class").cloned().unwrap_or_default(),
                    id: p.attrs.get("id").cloned().unwrap_or_default(),
                    depth: p.depth,
                });
                current_parent = p.parent;
            } else {
                break;
            }
        }

        // Compute xxhash of identifying characteristics
        let raw_hash_input = format!(
            "{}:{}:{}:{:?}:{:?}:{:?}:{}",
            tag, text, depth, class_list, id, ancestor_path, sibling_index
        );
        let hash = xxh64(raw_hash_input.as_bytes(), 0);

        Some(Self {
            tag,
            text,
            html_snippet,
            depth,
            sibling_index,
            parent_tag,
            parent_class,
            parent_id,
            attributes,
            class_list,
            id,
            ancestor_path,
            hash,
            captured_at: Utc::now(),
            url: url.to_string(),
            selector_used: selector_used.to_string(),
            similarity_score: 1.0, // default perfect similarity
        })
    }
}
