use crate::error::{CrawlingoError, Result};
use crate::fingerprint::dom::DomFingerprint;
use crate::fingerprint::store::FingerprintStore;
use crate::matcher::scorer::composite_score;
use crate::parser::document::DomTree;
use crate::selector::css;
use rayon::prelude::*;

/// Resolves a CSS selector against a parsed `DomTree`.
/// If the selector fails to match any elements, it attempts to load a saved fingerprint
/// from the store, search all nodes in parallel using Rayon, and recover the element.
pub fn auto_match(
    tree: &DomTree,
    url: &str,
    selector: &str,
    store: &FingerprintStore,
    weights: Option<&std::collections::HashMap<String, f64>>,
) -> Result<usize> {
    // 1. Try standard CSS query first
    let matched_indices = css::query(tree, selector);
    if !matched_indices.is_empty() {
        let matched_idx = matched_indices[0];
        // Regenerate and update fingerprint cache dynamically
        if let Some(new_fp) = DomFingerprint::generate(tree, matched_idx, url, selector) {
            let _ = store.store(url, selector, &new_fp);
        }
        return Ok(matched_idx);
    }

    // 2. Selector failed: load fingerprint
    let fingerprint = store
        .load(url, selector)?
        .ok_or(CrawlingoError::AutoMatchFailed)?;

    // 3. Parallel Jaro-Winkler + Jaccard similarity scoring using Rayon
    let best_match = (0..tree.nodes.len())
        .into_par_iter()
        .map(|idx| {
            let score = composite_score(tree, idx, &fingerprint, weights);
            (idx, score)
        })
        .max_by(|a, b| {
            a.1.total
                .partial_cmp(&b.1.total)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

    if let Some((best_idx, score)) = best_match {
        // Recover if similarity exceeds 0.5 threshold
        if score.total > 0.5 {
            tracing::warn!(
                "Auto-matcher recovered broken selector '{}' on URL '{}' with similarity: {:.4}",
                selector,
                url,
                score.total
            );

            // Save recovered node fingerprint to adapt to the new site layout
            if let Some(mut updated_fp) = DomFingerprint::generate(tree, best_idx, url, selector) {
                updated_fp.similarity_score = score.total;
                let _ = store.store(url, selector, &updated_fp);
            }
            return Ok(best_idx);
        }
    }

    Err(CrawlingoError::AutoMatchFailed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::streaming::parse_html;

    #[test]
    fn test_auto_matcher_recovery() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = FingerprintStore::open(temp_dir.path()).unwrap();
        let url = "http://example.com/item";
        let selector = "span.price";

        // Initial fetch: generate and cache fingerprint
        let initial_html = b"<html><body><div><span class='price'>$50</span></div></body></html>";
        let initial_tree = parse_html(initial_html).unwrap();
        let matched_idx = auto_match(&initial_tree, url, selector, &store, None).unwrap();
        assert_eq!(initial_tree.get_text(matched_idx), "$50");

        // Layout redesign: selector span.price breaks, class changes to span.price-tag
        let redesigned_html =
            b"<html><body><div><span class='price-tag'>$50</span></div></body></html>";
        let redesigned_tree = parse_html(redesigned_html).unwrap();

        // Recover using auto_match
        let recovered_idx = auto_match(&redesigned_tree, url, selector, &store, None).unwrap();
        assert_eq!(redesigned_tree.get_text(recovered_idx), "$50");
    }
}
