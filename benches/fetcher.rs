use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crawlingo::engine::rate_limiter::HostRateLimiter;
use tokio::runtime::Runtime;

fn bench_rate_limiter(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let rl = HostRateLimiter::new();
    let host = "test-host.com";
    let rps = 1000000.0; // High rate to prevent significant pacing delays in benchmarks

    c.bench_function("rate_limiter_wait_concurrent", |b| {
        b.iter(|| {
            rt.block_on(async {
                rl.wait(black_box(host), black_box(rps)).await;
            });
        })
    });
}

criterion_group!(benches, bench_rate_limiter);
criterion_main!(benches);
