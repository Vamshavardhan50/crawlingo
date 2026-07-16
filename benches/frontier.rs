use crawlingo::crawl::frontier::{Frontier, PersistentFrontier};
use criterion::{criterion_group, criterion_main, Criterion};
use tempfile::tempdir;

fn bench_frontier(c: &mut Criterion) {
    c.bench_function("frontier_enqueue_dequeue_1k", |b| {
        b.iter_with_setup(
            || {
                let dir = tempdir().unwrap();
                let frontier = PersistentFrontier::open(dir.path()).unwrap();
                (dir, frontier)
            },
            |(_dir, frontier)| {
                for i in 0..1000 {
                    frontier.enqueue(format!("https://example.com/page/{}", i), 0);
                }
                for _ in 0..1000 {
                    let _ = frontier.dequeue();
                }
            },
        );
    });

    c.bench_function("frontier_enqueue_dequeue_10k", |b| {
        b.iter_with_setup(
            || {
                let dir = tempdir().unwrap();
                let frontier = PersistentFrontier::open(dir.path()).unwrap();
                (dir, frontier)
            },
            |(_dir, frontier)| {
                for i in 0..10000 {
                    frontier.enqueue(format!("https://example.com/page/{}", i), 0);
                }
                for _ in 0..10000 {
                    let _ = frontier.dequeue();
                }
            },
        );
    });
}

criterion_group!(benches, bench_frontier);
criterion_main!(benches);
