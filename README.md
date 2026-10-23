<img src="assets/brand-mark.svg" alt="Crawlingo" width="48" height="48">

# Crawlingo

**Web pages → validated datasets.**

A Rust engine with Python and Node.js SDKs for extracting structured records,
checking their quality, removing exact duplicates, and writing resumable JSONL
or CSV datasets.

[![CI](https://github.com/Vamshavardhan50/crawlingo/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Vamshavardhan50/crawlingo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-007f78)](LICENSE)

[Read the guide](https://crawlingo-docs.vercel.app/guides) ·
[API reference](https://crawlingo-docs.vercel.app/api-reference) ·
[Integrations](guides/integrations.md) ·
[Examples](examples/product_dataset) ·
[Changelog](CHANGELOG.md)

> **Development release: 1.0.0-alpha.2.** Build from source for `DatasetJob`.
> This version is not yet published to npm, PyPI or crates.io.
> See [release and platform availability](guides/RELEASE_STATUS.md).

## What is it for?

Use Crawlingo to collect research directories, document collections, product catalogs and recurring
imports for your application. You define the URLs and field selectors; the engine
handles collection, validation, deduplication and local export.

```text
URL file + field schema
          ↓
fetch → extract → validate → deduplicate → JSONL / CSV + report
                                      ↳ checkpoint and resume
```

- **Explicit extraction:** one record per page or multiple records from a row selector.
- **Quality rules:** required fields, defaults, type validation and rejected-row reports.
- **Large jobs:** bounded batches, disk-backed exact deduplication and output shards.
- **Recovery:** restart an unchanged job using its existing state and output directory.
- **Your stack:** call the SDK in a worker, or run the CLI from a scheduler.

Crawlingo needs explicit rules and technical setup. It does not provide a hosted
API, a no-code interface or automatic semantic cleaning. See the
[capabilities and boundaries](guides/PRODUCT.md).

## Quick start

Clone and build the CLI. Native compilation needs Rust, C/C++ build tools, CMake
and libclang; use the [installation guide](https://crawlingo-docs.vercel.app/installation)
for platform-specific setup.

```sh
git clone https://github.com/Vamshavardhan50/crawlingo.git
cd crawlingo
cargo build --locked --release --features cli
```

Save one URL per line in `urls.txt`:

```text
https://example.com/
```

Save `job.json` beside it:

```json
{
  "urls_file": "urls.txt",
  "output_dir": "datasets/pages",
  "fields": [
    { "name": "title", "selector": "h1", "required": true }
  ],
  "concurrency": 8,
  "format": "jsonl"
}
```

```sh
cargo run --locked --release --features cli -- dataset job.json
```

Read `datasets/pages/report.json` and `part-00000000.jsonl`. A completed job may
contain failed pages or rejected rows; inspect those counters before using its
data. Repeating the unchanged job resumes it. Use a fresh directory for new inputs.

For a reproducible first run with known output, use the
[local product catalog](examples/product_dataset/README.md).
For research data from a public website, follow the
[directory → document dataset walkthrough](examples/research_dataset/README.md).
Read [dataset design](guides/dataset-design.md) before increasing volume.

## Use an SDK

Build the SDK from this checkout, then use the same `job.json`.

| Language | Setup | API |
| --- | --- | --- |
| Python | [Source installation](sdk/python/README.md) | `DatasetJob(config).run()` or `await job.run_async()` |
| Node.js / TypeScript | [Native addon build](sdk/nodejs/README.md) | `await new DatasetJob(config).run()` |
| Rust | [Crate integration](https://crawlingo-docs.vercel.app/sdk/rust) | `DatasetJob::new(config, session)?.run().await?` |

The [DatasetJob reference](guides/api-dataset-job.md) covers complete examples,
configuration, return values, cancellation and errors. Field values are currently
strings, including validated numbers and booleans.

### Complete API documentation

The documentation source includes a generated reference for every Node package
export and Python public export, reachable result types and hook helpers. Each
entry includes signatures, parameters, return types, source locations and known
binding limitations. See the [reference maintenance guide](guides/API_REFERENCE.md)
for inventory generation, Markdown exports and release requirements.

Build the Rust module, type and trait reference directly from this checkout:

```sh
cargo doc --locked --no-deps --lib --features browser --open
```

The SDK release workflow also prepares a versioned rustdoc archive. API presence
in source is separate from runtime validation and registry availability.

## Connect your tools

| Workflow | Integration |
| --- | --- |
| Discover links with Playwright or Puppeteer | [Browser discovery → URL file → dataset job](guides/integrations-playwright.md) |
| Analyze records or create Parquet | [pandas and DuckDB](guides/integrations-data.md) |
| Run behind FastAPI / Express or a scheduler | [Workers, retries and publication](guides/integrations-workers.md) |
| Deploy a container with persistent storage | [Deployment guide](guides/deployment.md) |

These are composition recipes, not bundled third-party plugins. Browser state is
not shared automatically, and the Python/Node SDKs do not accept rendered HTML as
dataset-job input.

## Scale and current scope

A public research-site run produced **744 directory records and 20 document
records**, with no failed pages or rejected rows. The document sample took
20.539 seconds at one request per second. See the [quality checks and measured
limits](PUBLIC_WEB_RESULTS.md); this is not a maximum-throughput benchmark.

A local HTTP fixture run fetched **1 million pages and exported 1 million records**
with 50 concurrent requests, zero page failures and zero rejected rows. Initial
job time was 1,094.569 seconds; peak process working set was 181.5 MiB, including
the fixture server and verification. These measurements describe that workload,
not arbitrary websites. See [method and results](SCALE_RESULTS.md) and
[performance tuning](guides/PERFORMANCE.md).
That local fixture was measured on October 14 and has not been rerun after the
latest parser changes.

Current dataset jobs use local single-writer storage and JSONL/CSV checkpoints.
Distributed scheduling, typed JSON output and checkpointed Parquet jobs are not
implemented. The [architecture](ARCHITECTURE_AUDIT.md) records remaining work.

## Project

[Contributing](CONTRIBUTING.md) · [Support](SUPPORT.md) ·
[Security](SECURITY.md) · [Code of conduct](CODE_OF_CONDUCT.md) ·
[Issues](https://github.com/Vamshavardhan50/crawlingo/issues) ·
[Documentation source](https://github.com/Vamshavardhan50/crawlingo-docs)

Licensed under [MIT](LICENSE).
