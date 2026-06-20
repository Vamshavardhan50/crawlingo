use crate::fingerprint::dom::{AncestorNode, DomFingerprint};
use crate::parser::document::{DomNode, DomTree};
use std::collections::{HashMap, HashSet};
use strsim::jaro_winkler;

/// Holds individual similarity metrics and a composite total.
#[derive(Debug, Clone, Copy)]
pub struct SimilarityScore {
    pub total: f64,
    pub text_score: f64,
    pub class_score: f64,
    pub ancestor_score: f64,
    pub attribute_score: f64,
    pub depth_score: f64,
    pub tag_score: f64,
}

/// Calculates the Jaccard similarity index of two sets.
fn jaccard_similarity<T: Eq + std::hash::Hash>(set_a: &HashSet<T>, set_b: &HashSet<T>) -> f64 {
    if set_a.is_empty() && set_b.is_empty() {
        return 1.0;
    }
    let intersection = set_a.intersection(set_b).count();
    let union = set_a.union(set_b).count();
    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Compares text contents using Jaro-Winkler algorithm.
pub fn text_similarity(text_a: &str, text_b: &str) -> f64 {
    if text_a.is_empty() && text_b.is_empty() {
        return 1.0;
    }
    jaro_winkler(text_a, text_b)
}

/// Compares class lists using Jaccard index.
pub fn class_similarity(classes_a: &[String], classes_b: &[String]) -> f64 {
    let set_a: HashSet<&String> = classes_a.iter().collect();
    let set_b: HashSet<&String> = classes_b.iter().collect();
    jaccard_similarity(&set_a, &set_b)
}

/// Compares attribute maps using Jaccard index.
pub fn attribute_similarity(
    attrs_a: &HashMap<String, String>,
    attrs_b: &HashMap<String, String>,
) -> f64 {
    let set_a: HashSet<(&String, &String)> = attrs_a.iter().collect();
    let set_b: HashSet<(&String, &String)> = attrs_b.iter().collect();
    jaccard_similarity(&set_a, &set_b)
}

/// Compares ancestor path chains from leaf upward.
pub fn ancestor_similarity(path_a: &[AncestorNode], path_b: &[AncestorNode]) -> f64 {
    if path_a.is_empty() && path_b.is_empty() {
        return 1.0;
    }
    if path_a.is_empty() || path_b.is_empty() {
        return 0.0;
    }

    let mut total_score = 0.0;
    let mut used_b = vec![false; path_b.len()];

    for node_a in path_a {
        let mut best_node_score = 0.0;
        let mut best_b_idx = None;

        for (i, node_b) in path_b.iter().enumerate() {
            if used_b[i] {
                continue;
            }
            if node_a.tag != node_b.tag {
                continue;
            }

            let mut score = 0.4; // Base score for same tag
            if !node_a.id.is_empty() && node_a.id == node_b.id {
                score += 0.3;
            }
            if !node_a.class.is_empty() && node_a.class == node_b.class {
                score += 0.3;
            }
            if node_a.id.is_empty()
                && node_b.id.is_empty()
                && node_a.class.is_empty()
                && node_b.class.is_empty()
            {
                score += 0.6; // Perfect match for empty tag
            }

            if score > best_node_score {
                best_node_score = score;
                best_b_idx = Some(i);
            }
        }

        if let Some(idx) = best_b_idx {
            total_score += best_node_score;
            used_b[idx] = true;
        }
    }

    total_score / path_a.len().max(path_b.len()) as f64
}

/// Computes a score based on the inverse absolute difference in depth levels.
pub fn depth_score(depth_a: usize, depth_b: usize) -> f64 {
    1.0 / (1.0 + (depth_a as f64 - depth_b as f64).abs())
}

/// Computes the tag score (1.0 if matching, 0.0 if not).
pub fn tag_score(tag_a: &str, tag_b: &str) -> f64 {
    if tag_a.to_lowercase() == tag_b.to_lowercase() {
        1.0
    } else {
        0.0
    }
}

/// Calculates the composite similarity score between a current DOM node and a saved fingerprint.
pub fn composite_score(
    tree: &DomTree,
    node_idx: usize,
    fingerprint: &DomFingerprint,
    custom_weights: Option<&HashMap<String, f64>>,
) -> SimilarityScore {
    let node = &tree.nodes[node_idx];

    let text_val = tree.get_text(node_idx);
    let text_score = text_similarity(&text_val, &fingerprint.text);

    let class_list: Vec<String> = node
        .attrs
        .get("class")
        .map(|c| c.split_whitespace().map(|s| s.to_string()).collect())
        .unwrap_or_default();
    let class_score = class_similarity(&class_list, &fingerprint.class_list);

    let attribute_score = attribute_similarity(&node.attrs, &fingerprint.attributes);

    // Build temporary ancestor path for current node to compare with fingerprint path
    let mut current_ancestor_path = Vec::new();
    let mut current_parent = node.parent;
    while let Some(parent_idx) = current_parent {
        if let Some(p) = tree.nodes.get(parent_idx) {
            current_ancestor_path.push(AncestorNode {
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
    let ancestor_score = ancestor_similarity(&current_ancestor_path, &fingerprint.ancestor_path);

    let depth_score_val = depth_score(node.depth, fingerprint.depth);
    let tag_score_val = tag_score(&node.tag, &fingerprint.tag);

    // Compound weights
    let w_text = custom_weights
        .and_then(|w| w.get("text").cloned())
        .unwrap_or(2.0);
    let w_class = custom_weights
        .and_then(|w| w.get("class").cloned())
        .unwrap_or(1.5);
    let w_ancestor = custom_weights
        .and_then(|w| w.get("ancestor").cloned())
        .unwrap_or(1.5);
    let w_attr = custom_weights
        .and_then(|w| w.get("attribute").cloned())
        .unwrap_or(1.0);
    let w_tag = custom_weights
        .and_then(|w| w.get("tag").cloned())
        .unwrap_or(1.0);
    let w_depth = custom_weights
        .and_then(|w| w.get("depth").cloned())
        .unwrap_or(0.5);
    let total_weights = w_text + w_class + w_ancestor + w_attr + w_tag + w_depth;

    let weighted_sum = (text_score * w_text)
        + (class_score * w_class)
        + (ancestor_score * w_ancestor)
        + (attribute_score * w_attr)
        + (tag_score_val * w_tag)
        + (depth_score_val * w_depth);

    let total = weighted_sum / total_weights;

    SimilarityScore {
        total,
        text_score,
        class_score,
        ancestor_score,
        attribute_score,
        depth_score: depth_score_val,
        tag_score: tag_score_val,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarities() {
        let classes_a = vec!["price".to_string(), "red".to_string()];
        let classes_b = vec!["price".to_string(), "blue".to_string()];
        let class_sim = class_similarity(&classes_a, &classes_b);
        assert!((class_sim - 0.333).abs() < 0.01); // 1 intersection / 3 union

        let text_sim = text_similarity("In Stock", "In Stock - Only 2 left!");
        assert!(text_sim > 0.7);
    }
}
