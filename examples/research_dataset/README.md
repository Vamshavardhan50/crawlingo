# Research directory → document dataset

Collect a small research dataset from the public Python PEP site using the real
Rust HTTP transport. One index page produces directory records; a fixed sample
of detail pages produces document records. The source also offers a
[metadata API](https://peps.python.org/), which is preferable when it already
provides the fields you need.

## Requirements

Build the current source (`1.0.0-alpha.2`, not yet published). Follow the
[native installation guide](https://crawlingo-docs.vercel.app/installation). Python 3 is needed
only for the small preparation and optional measurement scripts.

Run every command below from the repository root. Relative job paths are resolved
against the working directory. The supplied session identifies Crawlingo and
paces requests at **one request per second per host**; robots checks stay enabled.
This sample uses two concurrent workers. It is a modest collection demonstration,
not a load test of the source.

## 1. Build and collect the directory

```sh
cargo build --locked --release --features cli --bin crawlingo
cargo run --locked --release --features cli -- --config examples/research_dataset/session.json --rate-limit-rps 1 dataset examples/research_dataset/index-job.json
```

`index-job.json` scopes a record to each table row. `td:nth-child(2)` selects the
proposal number; column 3 selects the title without neighboring version links.
Only positive integer `:nth-child(N)` is implemented, not `an+b`, `odd` or `even`.

Read `work/research/index/report.json` and the data/error shards. Exit code 0 means
a clean job; 2 means page failures, row rejections or cancellation; 1 means a fatal
error. `complete=true` alone does not certify quality.

## 2. Normalize and freeze 20 URLs

```sh
python examples/research_dataset/prepare_details.py --limit 20
```

This local step produces:

- `work/research/directory.jsonl`: numeric PEP ID, title, absolute URL,
  classification and directory source.
- `work/research/detail-urls.txt`: the first 20 directory URLs in source order.

The script validates the expected destination origin and rejects duplicate IDs.
It refuses to overwrite prepared files. The sample limit is 1–50 documents;
this is a convenience sample, not a statistically representative one.

## 3. Extract document records

```sh
cargo run --locked --release --features cli -- --config examples/research_dataset/session.json --rate-limit-rps 1 dataset examples/research_dataset/detail-job.json
```

Read `work/research/details/part-00000000.jsonl` and the remaining numbered shards.
Each record contains `title`, `canonical_url`, `document_text`, `_source_url` and
`_row`. The document selector includes article metadata and section navigation;
it is not a cleaned abstract. Job values are strings. The current parser still
needs explicit handling/review for HTML entities and non-UTF-8 sources.

Keep raw records immutable. Decode entities with `html.unescape` in a derived
dataset, then apply your own cleaning, chunking or analysis. Preserve the source
URL, identity, transformation version and collection dates alongside the result.

## 4. Resume or refresh

Repeat the same job command to resume it. A finished job does **not** refetch pages
and does not refresh the dataset. Keep input, configuration, engine build and
session settings fixed for a partial resume. Use a new directory and updated
configuration for a new dated snapshot.

## Measure a fresh run

Use this sequence **instead of** the collection commands above when the configured
output directories do not exist. The harness refuses to overwrite an earlier run.
Its default binary path works on Windows and Unix after the release build.

```sh
python scripts/measure-public-dataset.py examples/research_dataset/index-job.json target/research-evidence/index.json
python examples/research_dataset/prepare_details.py --limit 20
python scripts/measure-public-dataset.py examples/research_dataset/detail-job.json target/research-evidence/details.json
```

The harness records job/process times, byte counts, binary/input hashes and report
counters. It checks every exported record's required fields, provenance and exact
identity, then repeats the job and compares shard hashes. Memory is the Rust CLI's
Windows peak working set; it is `null` on other systems. It does not measure a
million records, interrupted recovery, maximum throughput or scientific validity.

For an independent field check, save the numerical index and PEP 1 HTML near the
collection time, then run:

```sh
python scripts/verify-pep-sample.py path/to/index.html path/to/pep1.html target/research-evidence/semantic-check.json
```

That check uses Python's HTMLParser against all directory titles/classifications
and the PEP 1 text's character order after entity decoding, ignoring whitespace.
The other document bodies still require quality review.

## Read next

- [Measured public-site results](../../PUBLIC_WEB_RESULTS.md)
- [Research dataset handbook](../../guides/research-datasets.md)
- [Dataset design and capacity planning](../../guides/dataset-design.md)
- [pandas and DuckDB integration](../../guides/integrations-data.md)
