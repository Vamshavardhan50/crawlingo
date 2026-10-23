# Rust SDK

Use the native job engine directly, with explicit session ownership and asynchronous execution.

<a id="dependencies"></a>

## Dependencies

Use a path to the current checkout for DatasetJob. The same package version on a registry may represent an earlier alpha build.

### Cargo.toml

```toml
[dependencies]
crawlingo = { path = "../crawlingo" }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

<a id="job"></a>

## Generate a dataset

DatasetJob::new validates configuration. run().await performs the job. Clone the job handle when another task needs to call cancel(); each output directory permits only one active writer.

### Rust

```rust
use crawlingo::dataset::{DatasetJob, JobConfig};
use crawlingo::engine::session::Session;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: JobConfig = serde_json::from_reader(
        std::fs::File::open("job.json")?
    )?;
    let job = DatasetJob::new(config, Arc::new(Session::new()))?;
    let report = job.run().await?;
    println!("{report:?}");
    Ok(())
}
```

<a id="stream"></a>

## Bounded record streaming

DatasetStream is the lower-level record stream. Create a producer handle, detach the stream’s own sender, and await handle.send(record) from the producer while a consumer drains the stream. Synchronous push returns a capacity error when the queue is full.

write_csv and write_parquet consume the stream. Export errors propagate to the caller. Keep channel capacity and record size bounded; do not enqueue every record before starting the consumer.

<a id="features"></a>

## Cargo features

| Feature | Purpose |
| --- | --- |
| default | Core engine; no Python extension or browser dependency. |
| cli | Rust command-line application. |
| python | PyO3 extension for the Python SDK. |
| browser | Chromium rendering via the browser manager. |

- [Architecture](../architecture.md)
