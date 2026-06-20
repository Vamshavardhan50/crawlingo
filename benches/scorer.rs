use crawlingo::fingerprint::dom::DomFingerprint;
use crawlingo::matcher::scorer::composite_score;
use crawlingo::parser::streaming::parse_html;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_similarity_scorer(c: &mut Criterion) {
    let html = b"<html><body><div class='product' id='p1'><h1>Title</h1><span class='price'>$100</span></div></body></html>";
    let tree = parse_html(html).unwrap();
    let node_idx = 4; // index of the price span

    // Generate a fingerprint
    let fp = DomFingerprint::generate(&tree, node_idx, "https://example.com/bench", "span.price")
        .unwrap();

    c.bench_function("composite_score", |b| {
        b.iter(|| composite_score(black_box(&tree), black_box(node_idx), black_box(&fp)))
    });
}

criterion_group!(benches, bench_similarity_scorer);
criterion_main!(benches);
