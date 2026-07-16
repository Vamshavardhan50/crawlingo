use crawlingo::fingerprint::dom::DomFingerprint;
use crawlingo::fingerprint::store::FingerprintStore;
use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use tempfile::tempdir;

fn bench_fingerprint(c: &mut Criterion) {
    c.bench_function("fingerprint_store_write_1k", |b| {
        b.iter_with_setup(
            || {
                let dir = tempdir().unwrap();
                let store = FingerprintStore::open(dir.path()).unwrap();
                let fp = DomFingerprint {
                    tag: "div".to_string(),
                    text: "hello".to_string(),
                    html_snippet: "<div>".to_string(),
                    depth: 2,
                    sibling_index: 1,
                    parent_tag: "body".to_string(),
                    parent_class: "".to_string(),
                    parent_id: "".to_string(),
                    attributes: HashMap::new(),
                    class_list: vec![],
                    id: None,
                    ancestor_path: vec![],
                    hash: 99,
                    captured_at: chrono::Utc::now(),
                    url: "https://example.com".to_string(),
                    selector_used: "div".to_string(),
                    similarity_score: 1.0,
                };
                (dir, store, fp)
            },
            |(_dir, store, fp)| {
                for i in 0..1000 {
                    store
                        .store("https://example.com", &format!("div.class-{}", i), &fp)
                        .unwrap();
                }
                store.flush().unwrap();
            },
        );
    });
}

criterion_group!(benches, bench_fingerprint);
criterion_main!(benches);
