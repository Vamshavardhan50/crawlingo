//! Compare the production text-anchor selector with exhaustive byte-window search.
use crawlingo::parser::{document::DomTree, streaming::parse_html};
use crawlingo::selector::text_anchor;
use std::{hint::black_box, time::Instant};

fn reference(tree: &DomTree, needle: &str) -> Vec<usize> {
    tree.nodes
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            if matches!(node.tag.as_str(), "script" | "style") {
                return None;
            }
            node.text
                .as_bytes()
                .windows(needle.len())
                .any(|w| w == needle.as_bytes())
                .then_some(index)
        })
        .collect()
}

fn time_scan(scan: fn(&DomTree, &str) -> Vec<usize>, tree: &DomTree, needle: &str) -> f64 {
    let started = Instant::now();
    for _ in 0..64 {
        black_box(scan(black_box(tree), black_box(needle)));
    }
    started.elapsed().as_secs_f64()
}

fn main() {
    let needle = "catalogue-é";
    let mut html = String::from("<main>");
    let prefix = "research record ".repeat(32);
    for i in 0..2048 {
        html.push_str("<p>");
        html.push_str(&prefix);
        if i % 8 == 0 {
            html.push_str(needle);
        }
        html.push_str("</p>");
    }
    html.push_str("</main>");
    let tree = parse_html(html.as_bytes()).unwrap();
    let expected = reference(&tree, needle);
    assert_eq!(expected.len(), 256);
    assert_eq!(text_anchor::find(&tree, needle), expected);
    let mut production = Vec::new();
    let mut baseline = Vec::new();
    for trial in 0..6 {
        if trial % 2 == 0 {
            production.push(time_scan(text_anchor::find, &tree, needle));
        }
        baseline.push(time_scan(reference, &tree, needle));
        if trial % 2 == 1 {
            production.push(time_scan(text_anchor::find, &tree, needle));
        }
    }
    println!(
        "{}",
        serde_json::json!({"fixture_bytes":html.len(),"expected_matches":expected.len(),"scans_per_trial":64,
        "production_seconds":production,"reference_seconds":baseline,"scope":"Production memmem text selector versus exhaustive byte windows over identical parsed nodes. This comparison includes algorithm choice and compiler optimizations."})
    );
}
