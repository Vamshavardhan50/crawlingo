# Performance and resource tuning

Crawlingo is optimized around a Rust-owned dataset pipeline. The Python and Node
APIs receive a report while accepted records go directly to files. Avoid collecting
one million `Page` or `Dataset` objects in application memory.

## Start with the correct workload

Use `DatasetJob` for a URL file and explicit extraction schema. Use HTTP transport
when the response contains the data; rendered-browser work has a different cost
and must be measured separately. Reuse a Session so requests share connections,
rate limits and transport settings.

The checked local run produced one million records from one million small HTTP
pages. Its 1,094.569-second job duration is a fixture result, not a universal rate.
See [the complete measurement](../SCALE_RESULTS.md).

## What the current allocation changes do

- Share one immutable job configuration among extraction workers instead of
  cloning every field and selector for every page.
- Share canonical deduplication bytes between the pending lookup and Sled batch
  using `IVec`; do not copy an existing database record just to compare it.
- Skip canonical deduplication serialization when deduplication is disabled.
- Serialize JSONL directly into buffered writers instead of allocating a temporary
  JSON byte vector for every output record and error.
- Keep string values in place during validation and avoid character counting when
  `min_length` is zero. Nonzero minimum checks stop once enough characters are found.

These reduce identifiable copies and allocations. A measured throughput or memory
improvement has **not** yet been established. Record ordering, exact collision
checks, checkpoint ordering and durability calls are retained.

## Tune in this order

| Setting | Starting point | Tradeoff |
| --- | --- | --- |
| Session rate limit | Follow the source's permitted request rate | A source limit caps throughput regardless of concurrency |
| `concurrency` | Default 16; compare 8, 16, 32 and 50 on your fixture | Higher values increase in-flight bodies, parsing work and buffered results |
| `checkpoint_pages` | Default 256 | Larger batches reduce sync/commit overhead but replay more work after interruption |
| `checkpoint_bytes` | Default 32 MiB | Caps canonical dedup payload accounted for per batch; not a total-process memory cap |
| `max_response_bytes` | Default 2 MiB job acceptance limit | Currently checked after transport fetch; transport has its own separate cap |
| `max_records_per_page` | Default 1,000 | Limits the row results held for a single page |
| `max_record_bytes` | Default 256 KiB | Rejects unexpectedly large extracted records |
| `dedup_fields` | Stable identity fields when appropriate | Changes record identity; choose for correctness before performance |

Change one setting at a time and use a **new output directory** for changed input
or configuration. Do not disable robots handling or rate limits to improve public
website benchmark numbers. The controlled fixture explicitly disables robots checks.

Results currently preserve input order. A slow early page can delay consumption of
later completed pages. Removing that constraint needs an explicit ordering contract
and recovery checks; simply switching to unordered futures would change output.

## Reproduce the existing scale workload

From the repository root on Windows:

```powershell
cargo build --locked --release --example dataset_scale
./scripts/measure-dataset-scale.ps1 -Pages 10000 -OutputDirectory D:/Scraper/target/scale-new -Mode http
```

The harness already checks every record and resume behavior. Use a fresh path for
each repetition. Compare at least three runs of the same source/configuration and
report the median, range and peak working set. Keep compiler, lockfiles, hardware,
response fixture, concurrency and checkpoint size constant. The measurement process
also contains the local HTTP server and output verification.

Run `generated` mode separately to isolate most network overhead. It is not HTTP
throughput. Increase to the million-page workload only after smaller runs are correct.

## Next engineering priorities

1. Enforce the job response limit while streaming the response body, including
   decoded-size handling, rather than only after fetch.
2. Measure fetch, parse/extract, dedup, output and checkpoint costs separately.
3. Add an explicitly documented completion-order mode only if varied-latency
   workloads demonstrate a useful gain and recovery behavior remains correct.
4. Profile selector traversal and CPU utilization on realistic large pages before
   replacing the parser or adding further parallel workers.
5. Consider independent worker partitions only after fault recovery and identity
   rules have been established for one process.
