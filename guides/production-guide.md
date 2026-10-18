# Operate larger jobs

Plan capacity, observe progress, handle shutdown and understand the limits of the current single-node engine.

> Current development source. New dataset APIs require a matching source build; published alpha packages may differ.

## Start with a representative baseline

A small sample should represent the hard cases in your real input, not only the shortest successful pages. Include large responses, many rows, missing fields, redirects and fetch failures. Record accepted/rejected counts and inspect the extracted content.

Measure the workload with the same build profile and session policy you intend to use later. Keep the source revision, machine, concurrency, checkpoint settings and payload characteristics with the result. Comparing one release build to another debug build is not a useful throughput comparison.

- [Recorded scale method](https://crawlingo-docs.vercel.app/benchmarks)

## Budget memory and disk separately

The input URL file is read in batches, but concurrent responses, DOMs, extracted rows and pending identity values still consume memory. The default maximum response is 2 MiB per job page; at concurrency 16, response bodies alone could account for about 32 MiB before parsing and other overhead. This is a budgeting illustration, not an RSS prediction.

Disk grows with data shards, error shards, exact identity values and checkpoint storage. Exact deduplication retains comparison data so a hash collision does not silently discard a distinct record. Large identity values can therefore increase storage cost.

Measure output and state growth on your sample, then extrapolate with headroom. Watch available disk throughout a long run. Disk-full and power-loss behavior still need broader fault acceptance; do not describe the current checkpoint design as a guarantee against every storage failure.

## Tune one limit at a time

These settings are part of the job configuration signature. Change them in a new job directory. Do not edit limits midway and expect the same output directory to resume. Rate limits and source policies may constrain useful throughput before CPU or memory does.

| Setting | When to reconsider it | Tradeoff |
| --- | --- | --- |
| concurrency | Requests spend time waiting and the source permits more activity. | More active responses and DOMs can increase memory pressure. |
| checkpoint_pages | Checkpoint overhead or replay size matters. | Larger batches can retain more pending work and repeat more requests after interruption. |
| checkpoint_bytes | Identity payloads exceed the pending-key budget. | Increasing the budget consumes more memory; inspect large identity fields first. |
| max_response_bytes | Legitimate source pages are larger than the default. | Larger documents can also create substantially larger DOMs. |
| max_records_per_page | A valid page contains more rows than the configured bound. | More extracted records may be retained per page. |
| max_record_bytes | A legitimate record contains large text fields. | Larger accepted records affect buffers, output size and deduplication storage. |

- [Limits and defaults](https://crawlingo-docs.vercel.app/dataset-jobs#limits)

## Make job ownership explicit

One process owns each output directory. Give every logical dataset a distinct directory and keep its input/configuration immutable. The local database lock prevents concurrent writers from opening the same state, but it is not a distributed lease service.

If your application runs several jobs, it must decide how many workers may run at once and how their combined resource use is limited. A per-job concurrency of 16 becomes up to 160 active page operations across ten jobs. Do not size the machine using one job’s setting alone.

Shared network filesystems, multi-host state writers and remote shard commits have not been validated. Partitioning a URL file across separate jobs is possible at the application layer, but deduplication remains local to each job unless you implement a later global reconciliation step.

## Handle shutdown and restart

Request cancellation, await the active run and then release its storage. The Rust CLI responds to Ctrl+C and Unix SIGTERM. SDK applications should connect their own lifecycle handlers to job.cancel(). A cancelled run discards its uncommitted batch, so some GET requests can be repeated on resume.

Keep the URL file, job configuration, state directory and output shards together. Resume with a new instance after the old writer is finished. After a hard interruption, resume before downstream ingestion so orphan output is reconciled with committed state.

Use a stopped job directory for a coherent backup. Copying selected shards while the writer is active does not capture a complete checkpoint snapshot.

- [Recovery contract](https://crawlingo-docs.vercel.app/recovery)
- [Deployment template](https://crawlingo-docs.vercel.app/deployment)

## Observe outcomes without inventing live metrics

The current job API returns a final report and writes report.json for the latest invocation. It is not a live progress event stream. Your service may observe process health and resource usage separately, then persist the returned counters when the invocation ends.

Track page failures, rejected rows, duplicates, accepted records, elapsed time and storage growth. Preserve error shards and the exact configuration for investigation. A higher request rate is useful only when it produces the correct records at an acceptable resource cost.

- [Counter meanings and outcomes](https://crawlingo-docs.vercel.app/report-reference)

## Review changes in the source

Selectors can continue matching after a site redesign while extracting the wrong value. Check representative records against source pages and use field-specific expectations in your application. Required-field rules catch missing data, but they do not replace semantic review.

DatasetJob keeps explicit selectors stable. Heuristic selector recovery is available in the separate Dataset builder, not silently enabled in bulk jobs. If a selector needs to change, review it and create a new job configuration/output directory.

- [Selector recovery scope](https://crawlingo-docs.vercel.app/auto-match)

## Separate local evidence from a production promise

The million-page acceptance result is a real local HTTP workload with verified records and idempotent restart. It is not evidence for every live website, native package target or browser workload.

Before a release, install packaged SDK artifacts in clean target environments and run the representative workload there. The repository includes a revised release workflow, but successful local builds do not mean its full hosted matrix or registry publication has run. Docker and cloud deployment templates also need validation on the intended platform.

- [SDK publishing steps](https://crawlingo-docs.vercel.app/publishing)
- [Remaining implementation gates](https://crawlingo-docs.vercel.app/roadmap)
