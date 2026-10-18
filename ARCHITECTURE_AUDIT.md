# Dataset architecture audit — 2026-10-14

Scope: current working tree, including existing uncommitted phase 1–3 changes.

## Verified findings before implementation

- `dataset/stream.rs` uses an unbounded channel despite its constant-memory claim.
- Its Parquet writer sends all rows into a bounded channel before starting the reader: more than 100 rows can deadlock.
- Its CSV writer enumerates each HashMap independently, so values can appear under the wrong header.
- `dataset/builder.rs` structured extraction zips global field matches; a missing field shifts subsequent rows. It ignores extraction types, defaults and schemas. Single-record attribute extraction reads text instead of attributes. Auto-match fingerprints are not captured on successful extraction.
- `crawl/crawler.rs` collects every record, discovered URL and error. Its page limit uses successful output length and can overshoot under concurrency. Worker panics and parse failures can be lost.
- `crawl/frontier.rs` scans all pending entries to deduplicate each URL and all entries to count them. Persistent dequeue and counter updates are not atomic across workers; storage errors are discarded.
- `engine/fetcher.rs` buffers entire response bodies without a size cap. Browser initialization blocks a running async runtime. Retries bypass per-host pacing after the first request.
- Public throughput and memory comparisons have no reproducible supporting suite. The existing crawler performance test visits a single seed without generating links to its purported 100 pages.

## Baseline

This section records the state before implementation. For the implemented design
and completed million-page acceptance, see [SCALE_RESULTS.md](SCALE_RESULTS.md).

Previous `cargo test --all-targets`: 121 of 124 library tests passed. Two frontier reopen tests failed with Windows access denied; one loopback fetch failed to connect. Remaining targets did not run. Environment effects versus product defects need reproduction. No million-page result has yet been established.

## Implementation decision

Keep the Rust engine and SDKs. Introduce a dataset job API over existing fetching and parsing, with lazy URL-file input, bounded fetch concurrency and checkpoint batches, explicit extraction/quality failures, disk-backed exact deduplication, durable output shards, cancellation and restart. Keep collecting APIs for small interactive use. Scale acceptance must exercise the production job pipeline, verify all output rows, report execution mode and memory, and distinguish generated transport fixtures from real HTTP.

## Compatibility and boundaries

Existing APIs remain, except corrupt/invalid inputs must return errors. A bounded stream's synchronous push returns a capacity error; async producers await capacity. New job configuration is versioned. Checkpoint/output ownership is local and single-writer. Schema/identity changes require a new job directory. Near-duplicate classification and universal browser/anti-bot success are not implied by exact deduplication or TLS emulation.
