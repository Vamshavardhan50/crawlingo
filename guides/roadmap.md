# Roadmap

The next work is driven by correctness checks and measured workloads.

<a id="current"></a>

## Current foundation

- Bounded Rust dataset jobs with Python, Node.js and CLI entry points.
- Scoped extraction, validation and exact deduplication.
- Local shard checkpoints, cancellation and resume.
- A verified million-page local HTTP acceptance run.

<a id="next"></a>

## Next release gates

- Cross-platform SDK packages and reproducible installation checks.
- Process-kill, disk-full and state-corruption recovery fixtures.
- Larger pages, many origins, many rows per page and failure-heavy workloads.
- Independent browser, selector recovery and monitoring acceptance.

<a id="later"></a>

## Planned extensions

These are planned directions, not currently available APIs or delivery-date commitments.

- Checkpointed Parquet jobs and typed output contracts.
- Dataset manifests and richer provenance.
- Durable discovery feeding the job pipeline.
- Worker partitions and remote shard storage after single-node recovery is established.
