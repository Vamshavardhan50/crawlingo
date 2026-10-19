# Dataset architecture audit — 2026-10-14

Scope: current working tree, including existing uncommitted phase 1–3 changes.

## Research dataset review — 2026-10-19

The primary product remains a **dataset generator**. Use `DatasetJob` for large
exports; `Crawler` and the collecting builder are appropriate for bounded
discovery or interactive use. The PEP example exercises the production HTTP
transport and job writer, with a separate normalization step.

Correctness work in this review:

- Preserve mixed text/element order in extracted text and reconstructed HTML.
  Text ranges reference each node's buffer rather than copying every subtree.
- Add positive-integer CSS `:nth-child(N)` for table columns. The public-directory
  pilot exposed title contamination from neighboring version links; independent
  field comparison, not row counts alone, detected it.
- Add a reproducible research example, a CLI measurement runner, and an
  independent source-HTML comparison. See [public-site results](PUBLIC_WEB_RESULTS.md).

Remaining engineering limits, relevant to the next architecture work:

1. The job-specific response-size check occurs after fetch. It does not itself
   enforce a streaming download budget; transport and parser allocations remain.
2. Ordered concurrent processing can wait behind a slow earlier page. Any move
   to unordered processing must preserve deterministic deduplication and recovery.
3. Checkpoints validate input and JobConfig, but not the engine revision or full
   Session configuration. Pin these externally when resuming partial jobs.
4. `Crawler` still returns collections; durable discovery needs its own bounded
   interface before it becomes a million-URL producer.
5. Exact deduplication is local to one job. Cross-worker pacing and global dataset
   identity belong in an explicit coordinator, not concurrent writers to one DB.
6. CSS/XPath coverage, HTML entity handling, non-UTF-8 decoding and Markdown
   fidelity remain limited. The text-order fix is not a full parser replacement.
7. CLI configuration-load errors currently warn and fall back to defaults. Use
   explicit pacing flags for the measurement; fail-closed configuration handling
   should precede wider unattended operation.

Next priorities: source/schema manifests, streaming response budgets, representative
text fixtures, process-kill/disk-failure recovery, and portable SDK release gates.
Do not advertise a new maximum throughput from this small public-site sample.

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
