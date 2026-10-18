# Product dataset example

A deterministic catalog with two accepted products, one duplicate SKU and one
invalid row. It demonstrates extraction, validation, exact deduplication and resume.

## Run

Build the [CLI or SDK](../../README.md) from the current source. From the repository
root, start the fixture server and keep it running:

```sh
python -m http.server 8765 --bind 127.0.0.1 --directory examples/product_dataset/site
```

In a second terminal, choose one entry point:

```sh
# CLI: paths in job.json are relative to this directory.
cd examples/product_dataset
cargo run --release --features cli -- dataset job.json
```

Or, from the repository root:

```sh
python examples/product_dataset/run.py
# Or use the Node SDK built in this checkout:
node examples/product_dataset/run.cjs
```

The SDK runners resolve paths from this example directory. The Python runner
requires the installed current-source Python extension.

## Expected result

| Counter | Value |
| --- | ---: |
| pages_fetched | 2 |
| pages_failed | 0 |
| records_written | 2 |
| records_rejected | 1 |
| duplicate_records | 1 |
| duplicate_urls | 1 |

The command exits **2** because the third product intentionally has no name.
This is an expected partial-success result. Inspect `output/report.json`,
`output/part-00000000.jsonl` and `output/part-00000000.errors.jsonl`.
Prices are validated strings, such as `"12.5"`, not JSON numbers.

Run the same command again. Counts remain cumulative and completed pages are not
fetched again. Change `output_dir` to a fresh directory before modifying the
input, fields or format. The example output directory is ignored by Git.

See [the output and recovery contract](../../DATASET_JOBS.md).
