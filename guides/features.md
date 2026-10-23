# Feature status

What is implemented, where it is available, and which claims have not been established.

<a id="matrix"></a>

## Current development capabilities

| Capability | Scope |
| --- | --- |
| DatasetJob | Rust, Python and Node wrappers; Rust CLI. Development source. |
| Bulk input | A UTF-8 URL file, read in bounded batches. |
| Quality | Scoped fields, strict validation, defaults, error shards. |
| Deduplication | Exact records and URL identities; not semantic similarity. |
| Recovery | Single-writer local checkpoints and replay. |
| Job exports | JSONL and CSV shards. |
| Other exports | Collected JSON/CSV/Parquet and Rust streaming Parquet. |
| Selector recovery | Heuristic CSS recovery in the existing Dataset builder. |
| TLS profiles | wreq transport configuration; no access guarantee. |
| Browser rendering | Optional Rust feature requiring Chromium. |
| Monitoring | Polling callbacks and heuristic change classification. |
| Scale evidence | One million local HTTP pages and verified records. |

<a id="boundaries"></a>

## Release boundaries

Cross-platform packages, browser workloads, power-loss recovery and representative public-site throughput need their own acceptance checks. A feature being implemented does not mean every workload or platform has been verified.

- [Roadmap and release gates](roadmap.md)
- [Measured workload](benchmarks.md)
