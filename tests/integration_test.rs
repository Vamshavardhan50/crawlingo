use crawlingo::change::detector::detect_changes;
use crawlingo::fingerprint::store::FingerprintStore;
use crawlingo::matcher::auto_matcher::auto_match;
use crawlingo::parser::streaming::parse_html;
use crawlingo::selector::css;
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_integration_flow() {
    // 1. Load HTML fixtures via compile-time include_bytes!
    let simple_html = include_bytes!("fixtures/simple.html");
    let changed_html = include_bytes!("fixtures/changed.html");

    // 2. Parse initial HTML DOM tree
    let simple_tree = parse_html(simple_html).expect("Failed to parse simple.html");
    let changed_tree = parse_html(changed_html).expect("Failed to parse changed.html");

    // 3. Test basic CSS query selection
    let title_indices = css::query(&simple_tree, "h1.product-title");
    assert_eq!(
        title_indices.len(),
        1,
        "Should match exactly 1 product title"
    );
    assert_eq!(
        simple_tree.get_text(title_indices[0]),
        "Premium Wireless Headphone"
    );

    let price_indices = css::query(&simple_tree, "span.price-value");
    assert_eq!(price_indices.len(), 1, "Should match exactly 1 price");
    assert_eq!(simple_tree.get_text(price_indices[0]), "$299.99");

    // 4. Test fingerprint-based auto-matcher self-healing
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let store = FingerprintStore::open(temp_dir.path()).expect("Failed to open fingerprint store");
    let url = "https://example.com/product/998";
    let selector = "span.price-value";

    // First, run auto_match on simple_tree. This will locate the element via standard CSS
    // selector, generate the DOM fingerprint, and cache it in the Sled persistent store.
    let matched_idx = auto_match(&simple_tree, url, selector, &store, None)
        .expect("First auto_match should locate the element via CSS");
    assert_eq!(simple_tree.get_text(matched_idx), "$299.99");

    // Next, run auto_match on changed_tree. In changed.html, the selector "span.price-value"
    // is broken because the class was mutated to "amount". The auto-matcher must trigger
    // fingerprint self-healing, scan all elements, score similarity, and recover the node.
    let recovered_idx = auto_match(&changed_tree, url, selector, &store, None)
        .expect("Auto-matcher should recover the broken selector via DOM fingerprint similarity");

    // Check that we successfully recovered the price element
    assert_eq!(changed_tree.get_text(recovered_idx), "$299.99");

    // 5. Test parallel change detection
    let mut old_data = HashMap::new();
    old_data.insert(
        "title".to_string(),
        "Premium Wireless Headphone".to_string(),
    );
    old_data.insert("price".to_string(), "$299.99".to_string());
    old_data.insert(
        "description".to_string(),
        "High quality sound with active noise cancellation features.".to_string(),
    );

    let mut new_data = HashMap::new();
    new_data.insert(
        "title".to_string(),
        "Premium Wireless Headphone".to_string(),
    );
    new_data.insert("price".to_string(), "$249.99".to_string()); // Simulated price reduction change
    new_data.insert(
        "description".to_string(),
        "High quality sound with active noise cancellation features.".to_string(),
    );

    let changes = detect_changes(url, &old_data, &new_data);
    assert_eq!(changes.len(), 1, "Should detect exactly 1 changed field");
    assert_eq!(changes[0].field, "price");

    // Verify it detected a PriceChange variant
    match &changes[0].change_type {
        crawlingo::change::detector::ChangeType::PriceChange {
            old_price,
            new_price,
            ..
        } => {
            assert_eq!(*old_price, 299.99);
            assert_eq!(*new_price, 249.99);
        }
        _ => panic!("Expected a PriceChange variant"),
    }
}
