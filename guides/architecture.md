# Architecture

A Rust pipeline that keeps total input size out of the in-memory work queue.

<a id="pipeline"></a>

## The dataset pipeline

SDKs pass configuration into Rust. Fetching runs asynchronously; parsing and extraction run on blocking workers. The job coordinator writes output and state incrementally, returning a small final report.

### Data flow

```text
URL file
  → bounded fetch concurrency
  → body limit and HTML parsing
  → records scoped to row containers
  → normalization and validation
  → disk-backed exact deduplication
  → flushed data/error shards
  → atomic checkpoint advancement
```

<a id="memory"></a>

## Memory and storage

Input is read in bounded checkpoint batches. Total memory includes concurrent responses, DOM trees, extracted page records, pending identity keys, transport pools, runtime threads and the database cache. No single configuration value caps total RSS.

Disk usage grows with input and accepted records. Exact identity values are retained so hash collisions cannot silently discard distinct records.

<a id="recovery"></a>

## Local ownership and recovery

Each job directory has one Sled writer. Output is flushed before state advances. Recovery replays the unfinished batch. The design is single-node; distributed leases and remote shard commits are not implemented.

- [Recovery contract](recovery.md)
- [Measured scale](benchmarks.md)
