# Migrating to the dataset preview

The next source version is **1.0.0-alpha.2** (`1.0.0a2` for Python), currently
unpublished. Build the matching wrappers and native engine together. Do not mix
an old `.node`/Python extension with the new wrappers.

## From the published SDKs

The inspected npm alpha.1 package does not contain `DatasetJob`. Use the new bulk
API when the input is a URL file and the result should be a dataset on disk. Existing
Page/Dataset interfaces remain useful for inspecting pages and collecting small results.

```python
from crawlingo import DatasetJob

report = DatasetJob({
    "urls_file": "urls.txt",
    "output_dir": "datasets/new-job",
    "fields": [{"name": "title", "selector": "h1", "required": True}],
}).run()
```

The call returns counters; records are in committed output shards. Inspect
`pages_failed`, `records_rejected`, `complete` and `cancelled`. A complete job can
contain failures. The [API reference](guides/api-dataset-job.md) describes handling.

## Runtime requirements

- Node.js: use 22 or newer for this SDK version.
- Python: this release targets 3.8–3.13. Use 3.12 for the documented local build.
  The old PyPI 3.14 wheel is not evidence that the current source supports 3.14.
- Native source builds require Rust, C/C++ build tools, CMake and libclang.

## Dataset checkpoints

Keep the complete output directory, including its `state/`, data and error shards.
Resume requires the same configuration and unchanged URL-file contents. Use a new
directory after changing either; copying only JSONL files does not transfer a checkpoint.
The final alpha.2 source uses checkpoint format **2**, with SHA-256 hashes of the
input and committed shards. Earlier development checkpoints have no integrity
metadata and cannot resume in this version. Preserve their output and start a new
job directory; do not delete or rewrite the old state. Engine version changes also
require a new directory. Resume verifies all committed shards before fetching.
`manifest.jsonl` lists the schema, input digest, final counters and per-file hashes.

## Persistent crawler frontier

The current frontier indexes pending URL keys and replays interrupted in-flight
items. It can add membership indexes to compatible `pending:` entries. This does
not establish compatibility with every older serialized-queue format.

Back up an old frontier directory before migration. For an unsupported legacy
format, export its remaining URLs using the matching old version, then feed them
to a new crawl/job directory. Retain the original until the new output is verified.
Do not delete a queue as a generic upgrade step.

## Rust configuration changes

- `FetchRequest` no longer has `rate_limit_rps`. In Rust, set
  `*session.rate_limit_rps.write().unwrap() = 2.0` before starting work, or pass
  `rate_limit_rps` through `CrawlingoConfig` to `Session::from_config`.
- `ExtractionRule` is a type alias for `DatasetField`; direct conversions between
  these names are unnecessary.
- Bulk jobs use `JobConfig`/`JobField`, with JSONL and CSV output. They do not inherit
  automatic selector healing from the existing Dataset builder.
- Configuration files reject unknown keys. Invalid environment values now return
  an error instead of being ignored. `apply_env()` and `apply_env_from()` return
  `Result<()>`; callers must handle the result. CLI configuration failures exit 1.
- Node `Session.fromConfig(path)` uses the same loader as Python `Session.from_config(path)`.
  Unsupported Node fetcher tiers return an error instead of silently selecting HTTP.

See [release status](guides/RELEASE_STATUS.md), [publishing](guides/publishing.md)
and the [local product example](examples/product_dataset/README.md).
