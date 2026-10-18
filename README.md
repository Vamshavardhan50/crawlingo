<img src="assets/brand-mark.svg" alt="Crawlingo" width="64" height="64">

# Crawlingo

**Turn web pages into validated datasets.** A Rust engine with Python and Node.js
bindings for extraction, validation, exact deduplication and resumable export.

[![CI](https://github.com/Vamshavardhan50/crawlingo/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Vamshavardhan50/crawlingo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-007f78)](LICENSE)
[![Status: alpha](https://img.shields.io/badge/status-alpha-1a2d42)](CHANGELOG.md)

[Documentation](guides/README.md) ·
[API reference](guides/api-dataset-job.md) ·
[Dataset jobs](DATASET_JOBS.md) · [Measured scale](SCALE_RESULTS.md) ·
[Contributing](CONTRIBUTING.md)

[Runnable product example](examples/product_dataset/README.md) ·
[Deploy a worker](guides/deployment.md) · [Publish the SDKs](guides/publishing.md) ·
[Workspace map](guides/WORKSPACE.md)

**New to Crawlingo?** Read [what it does](guides/overview.md) and follow
[the guided first run](guides/first-run.md).
**Building an integration?** Use the [developer tutorial](guides/developer-tutorial.md)
and [operating guide](guides/production-guide.md).

> **Next release: `1.0.0-alpha.2` (unpublished).** `DatasetJob` requires this source
> build. The published npm alpha does not export it. Registry installation alone
> does not provide the dataset workflow shown below.

## Release and SDK status

Checked on 2026-10-18. This table describes published artifacts, not build targets.

| Channel | Published version | Available artifacts |
| --- | --- | --- |
| [GitHub Releases](https://github.com/Vamshavardhan50/crawlingo/releases) | `v1.0.0-alpha.1` | Tag/source archives; no uploaded SDK or CLI assets |
| [npm](https://www.npmjs.com/package/crawlingo) | `0.1.2` latest; `1.0.0-alpha.1` alpha | Inspected tarballs contain a Windows x64 native addon only |
| [PyPI](https://pypi.org/project/crawlingo/) | `0.1.2` | Python 3.14 Windows x64 wheel and a source archive |
| [crates.io](https://crates.io/crates/crawlingo) | `0.1.2` | Rust source crate |

The next source version supports Python 3.8–3.13 and Node.js 22+. Use Python 3.12
for the documented local build. The old Python 3.14 wheel does not establish
Python 3.14 support for the current source. Read the [release audit](guides/RELEASE_STATUS.md)
for verified gaps, intended platforms and the remaining publication steps.

## What is Crawlingo for?

Crawlingo helps developers turn a known set of web pages into a dataset they can
use in an application, analysis pipeline or downstream search system. You supply
URLs and extraction rules. Crawlingo fetches the pages, checks each record, removes
exact duplicates and writes data with source information to disk.

| Your task | What you define | What you receive |
| --- | --- | --- |
| Product catalog | Product container, SKU, name and price fields | Deduplicated records and a report of invalid rows |
| Article or documentation dataset | Title, text and source selectors | JSONL records to clean, index or analyze downstream |
| Research and analysis | A reproducible URL list and a field schema | CSV/JSONL shards with source URLs and validation results |
| Backend data collection | SDK configuration, output storage and a schedule | A resumable job with completion and error counters |

Start with the [local product example](examples/product_dataset/README.md): it uses
two pages, includes a duplicate and an invalid row, and shows the expected result.
The [product guide](guides/PRODUCT.md) explains the intended workflow and boundaries.

### Choose an entry point

| You want to… | Use |
| --- | --- |
| Generate a large dataset from a URL file | `DatasetJob` — the primary bulk API |
| Inspect one page and develop selectors | `Page` |
| Build a small in-memory result | `Dataset` |
| Discover links from a starting page | `Crawl`; review discovered URLs before a bulk job |
| Monitor selected fields for changes | `Watch` |

The current product runs locally or in a backend worker. It needs explicit
selectors and technical setup. A hosted REST service, no-code dashboard, semantic
deduplication, distributed workers and checkpointed Parquet jobs are not shipped.
Go is not a supported SDK in this source tree.

## Generate your first dataset

Clone the repository and build the CLI. Native dependencies require Rust, C/C++
build tools, CMake and libclang. On Windows, configure `LIBCLANG_PATH` if LLVM is
installed with Visual Studio but is not discovered automatically.

```sh
git clone https://github.com/Vamshavardhan50/crawlingo.git
cd crawlingo
cargo build --locked --release --features cli
```

Create `urls.txt`:

```text
https://example.com/
```

Create `job.json` in the same working directory:

```json
{
  "urls_file": "urls.txt",
  "output_dir": "datasets/pages",
  "fields": [
    { "name": "title", "selector": "h1", "required": true }
  ],
  "concurrency": 16,
  "format": "jsonl"
}
```

Run the job:

```sh
cargo run --locked --release --features cli -- dataset job.json
```

Accepted records are written to `datasets/pages/part-00000000.jsonl`. Failed pages
and rejected rows are written to matching `.errors.jsonl` shards. Inspect the
report counters as well as the data: `complete` does not mean every page succeeded.
Run the unchanged configuration again to resume. Use a new directory for changed
input or configuration.

## Use your language

Build the matching SDK from source first: [Python](sdk/python/README.md) or
[Node.js](sdk/nodejs/README.md). All interfaces use the same `job.json`.

### Python

```python
import json
from crawlingo import DatasetJob

with open("job.json", encoding="utf-8") as source:
    config = json.load(source)

report = DatasetJob(config).run()  # or await job.run_async()
print(report)
```

### Node.js / TypeScript

```typescript
import { readFileSync } from 'node:fs';
import { DatasetJob, type JobConfig } from 'crawlingo';

const config: JobConfig = JSON.parse(readFileSync('job.json', 'utf8'));
const report = await new DatasetJob(config).run();
console.log(report);
```

### Rust

Use the current checkout as a path dependency, plus `tokio` with the `full` feature
and `serde_json`.

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
    println!("{:?}", job.run().await?);
    Ok(())
}
```

## What the pipeline does

```text
URL file → bounded fetching → scoped extraction → validation
         → disk-backed exact deduplication → output shards → checkpoint
```

| Capability | Current scope |
| --- | --- |
| Bulk input | UTF-8 file of HTTP(S) URLs, read in bounded batches |
| Extraction | One record per page or multiple records using `row_selector` |
| Quality | Required fields, defaults, strict validation and rejection reports |
| Deduplication | Exact record comparison and separate URL identities |
| Recovery | Local single-writer checkpoints, cancellation and replay |
| Job exports | JSONL and CSV shards with source URL and row metadata |
| Other exports | Collected JSON/CSV/Parquet; bounded Rust streaming Parquet |
| Selector recovery | Heuristic CSS recovery in the existing Dataset builder |
| TLS profiles | Optional browser-like transport profiles; no access guarantee |
| Browser rendering | Optional Rust `browser` feature and a local Chromium binary |
| Monitoring | Polling callbacks and heuristic change classification |

Job values are currently strings, including validated numeric and boolean values.
Exact deduplication does not detect semantic duplicates. `DatasetJob` uses explicit
selectors; it does not silently heal them. The collecting Page/Dataset/Crawler APIs
remain available for smaller tasks.

See [the job reference](DATASET_JOBS.md) for limits, schema options, output ownership,
CLI exit codes and recovery behavior.

## Scale evidence

The local HTTP acceptance run fetched **1,000,000 pages and exported 1,000,000
records**. Every record was checked and restart did not refetch completed pages.

| Measurement | Observed result |
| --- | ---: |
| Concurrent requests | 50 |
| Failed pages / rejected rows | 0 / 0 |
| Initial job time | 1,094.569 seconds |
| Peak process working set | 181.5 MiB |

This was a release build on Windows with small generated UTF-8 pages. Memory
includes the local fixture server and verification. It is not a throughput promise
for arbitrary websites or a comparison with other tools. The previous 3,500 req/s
and 2.4 MB claims were unsupported and have been withdrawn.

See [the complete method, results and remaining release gates](SCALE_RESULTS.md).
For tuning concurrency, checkpoint sizes and memory limits, see the
[performance guide](guides/PERFORMANCE.md). Recent allocation reductions have not
yet been assigned a measured speedup.

## Development

```sh
cargo test --tests --lib
cargo check --features browser,cli
```

Tests need a writable temporary directory and localhost networking. Browser runtime
tests also require Chromium. Source builds may need `LIBCLANG_PATH` configured.
The documentation website lives in `crawlingo-docs/`, which is a separate Git
repository in this workspace. See its README for local development.

## Community

[Issues](https://github.com/Vamshavardhan50/crawlingo/issues) ·
[Changelog](CHANGELOG.md) · [Security policy](SECURITY.md) ·
[Code of conduct](CODE_OF_CONDUCT.md) · [Support](SUPPORT.md) ·
[GitHub Sponsors](https://github.com/sponsors/Vamshavardhan50)

Maintainers: see the [GitHub repository setup](.github/REPOSITORY_SETUP.md) for
repository metadata, review rules, release environments and the publishing checklist.

Supported by [Gen-Z Growth](https://genzgrowth.com).

Licensed under [MIT](LICENSE).
