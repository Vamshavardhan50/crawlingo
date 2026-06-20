use crawlingo::fingerprint::store::FingerprintStore;
use crawlingo::matcher::auto_matcher::auto_match;
use crawlingo::parser::streaming::parse_html;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::tempdir;

fn bench_auto_matcher(c: &mut Criterion) {
    let temp_dir = tempdir().unwrap();
    let store = FingerprintStore::open(temp_dir.path()).unwrap();
    let url = "https://example.com/product";
    let selector = "span.price";

    // Setup initial tree and cache the fingerprint in store
    let html_initial = b"<html><body><div><span class='price'>$250</span></div></body></html>";
    let tree_initial = parse_html(html_initial).unwrap();
    let _ = auto_match(&tree_initial, url, selector, &store).unwrap();

    // Redesigned tree where class is changed to price-tag
    let html_redesigned =
        b"<html><body><div><span class='price-tag'>$250</span></div></body></html>";
    let tree_redesigned = parse_html(html_redesigned).unwrap();

    c.bench_function("auto_match_recovery", |b| {
        b.iter(|| {
            auto_match(
                black_box(&tree_redesigned),
                black_box(url),
                black_box(selector),
                black_box(&store),
            )
            .unwrap()
        })
    });
}

criterion_group!(benches, bench_auto_matcher);
criterion_main!(benches);
