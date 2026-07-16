use crawlingo::dataset::export::write_parquet_stream;
use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use tempfile::tempdir;
use tokio::runtime::Runtime;

fn bench_parquet(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("parquet_stream_1k", |b| {
        b.iter_with_setup(
            || {
                let dir = tempdir().unwrap();
                let path = dir.path().join("bench_1k.parquet");
                let (tx, rx) = tokio::sync::mpsc::channel(1000);
                for i in 0..1000 {
                    let mut record = HashMap::new();
                    record.insert("id".to_string(), i.to_string());
                    record.insert("name".to_string(), format!("Product {}", i));
                    record.insert("price".to_string(), format!("{}.99", i));
                    tx.blocking_send(record).unwrap();
                }
                drop(tx);
                (dir, path, rx)
            },
            |(_dir, path, rx)| {
                rt.block_on(async {
                    write_parquet_stream(path.to_str().unwrap(), rx, Some(1000))
                        .await
                        .unwrap();
                });
            },
        );
    });

    c.bench_function("parquet_stream_10k", |b| {
        b.iter_with_setup(
            || {
                let dir = tempdir().unwrap();
                let path = dir.path().join("bench_10k.parquet");
                let (tx, rx) = tokio::sync::mpsc::channel(10000);
                for i in 0..10000 {
                    let mut record = HashMap::new();
                    record.insert("id".to_string(), i.to_string());
                    record.insert("name".to_string(), format!("Product {}", i));
                    record.insert("price".to_string(), format!("{}.99", i));
                    tx.blocking_send(record).unwrap();
                }
                drop(tx);
                (dir, path, rx)
            },
            |(_dir, path, rx)| {
                rt.block_on(async {
                    write_parquet_stream(path.to_str().unwrap(), rx, Some(1000))
                        .await
                        .unwrap();
                });
            },
        );
    });
}

criterion_group!(benches, bench_parquet);
criterion_main!(benches);
