# Crawlingo for Python

Generate validated datasets using a Rust engine and a small Python API.

[Documentation](https://crawlingo-docs.vercel.app/sdk/python) ·
[Job reference](https://github.com/Vamshavardhan50/crawlingo/blob/main/DATASET_JOBS.md) · [Scale evidence](https://github.com/Vamshavardhan50/crawlingo/blob/main/SCALE_RESULTS.md)

[Complete product example](https://github.com/Vamshavardhan50/crawlingo/blob/main/examples/product_dataset/README.md) ·
[Deploy a worker](https://github.com/Vamshavardhan50/crawlingo/blob/main/guides/deployment.md) · [Publish to PyPI](https://github.com/Vamshavardhan50/crawlingo/blob/main/guides/publishing.md)

## Build from source

This source targets **1.0.0a2 (unpublished)**. PyPI currently has `0.1.2`, with a
Windows Python 3.14 wheel and source archive. That artifact is not this dataset
API. Build the matching extension before running these examples.

The source declares Python 3.8–3.13; use Python 3.12 for the verified local build.
Python 3.14 is outside this release's artifact matrix and package metadata.
Install Rust, C/C++ build tools, CMake and libclang first.

The planned Linux wheels target x64 and ARM64 with glibc 2.28 or newer
(`manylinux_2_28`). Alpine/musl wheels are not included. macOS Intel/Apple Silicon
and Windows x64 are separate build targets; see the release status for completed
platform checks.

From the repository root:

```sh
python -m venv .venv
# Activate .venv using the command for your shell.
python -m pip install maturin
cd sdk/python
maturin develop --locked --release
python -c "from crawlingo import DatasetJob; print(DatasetJob)"
```

If native compilation cannot find LLVM, set `LIBCLANG_PATH` to the directory
containing `libclang.dll` or the equivalent library on your platform.

## Generate a dataset

Create `urls.txt` with one HTTP(S) URL per line, then run:

```python
from crawlingo import DatasetJob

job = DatasetJob({
    "urls_file": "urls.txt",
    "output_dir": "datasets/pages",
    "fields": [{"name": "title", "selector": "h1", "required": True}],
    "format": "jsonl",
})
report = job.run()
print(report)
```

The Rust engine writes accepted records and error shards to disk. Python receives
a summary instead of a million-element list. Check `pages_failed` and
`records_rejected`, even when `complete` is true.

In an async application, use `await job.run_async()`. Call `job.cancel()` to request
cancellation. Resume with a new instance using the same input, configuration and
output directory. Use a fresh directory if any of them change.

## Configure requests

```python
from crawlingo import DatasetJob, Session

session = Session().rate_limit(2).timeout(30)
session.headers({"User-Agent": "MyDatasetBot/1.0"})
job = DatasetJob(config, session=session)  # config is your job dictionary
```

Robots checks are enabled by default. `row_selector`, field validation and exact
deduplication are described in the [job reference](https://github.com/Vamshavardhan50/crawlingo/blob/main/DATASET_JOBS.md).

## Extract a small result

```python
from crawlingo import Dataset

result = Dataset("https://example.com/").field("title", "h1").build()
print(result.to_dict())
result.to_json("page.json")
```

The Python `Dataset.field` wrapper accepts `selector_type`, `transform` and
`default`; use job field configuration for bulk `extract_type` options. Collected
results support JSON, CSV and Parquet. Bulk jobs currently support JSONL and CSV.

## Limits

Job values are strings with optional strict validation. Selector recovery is a
heuristic feature of the existing Dataset builder, not DatasetJob. Browser-like
TLS profiles do not guarantee access. The million-record result uses local HTTP
fixtures and does not establish arbitrary-site performance.

MIT license. [Report an issue](https://github.com/Vamshavardhan50/crawlingo/issues).
