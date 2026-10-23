# Export and resume

Understand which files are committed and how interrupted work is replayed.

<a id="files"></a>

## Output directory

CSV jobs use .csv data shards with a header in each shard. Every accepted row includes _source_url and _row. New jobs support JSONL and CSV; the existing DatasetStream API separately supports Parquet.

### datasets/pages/

```text
state/                         # checkpoint and identity indexes
part-00000000.jsonl             # accepted records
part-00000000.errors.jsonl      # rejected rows and failed pages
part-00000001.jsonl
part-00000001.errors.jsonl
report.json                    # last invocation summary
```

<a id="checkpoint"></a>

## What is committed

Data and error files are flushed before a single database batch commits progress, URL identities and record identities. A process interruption replays the uncommitted input batch. GET requests in that batch can be repeated.

Use report.shards as the output boundary after a successful or cancelled run. A hard crash can leave orphan files or an old report: resume the job before consuming the final output. This recovery design does not by itself guarantee power-loss durability.

<a id="resume"></a>

## Resume the same job

### CLI

```shell
cargo run --release --features cli -- dataset job.json
```

- Keep the URL file and job configuration unchanged.
- Keep state and shards together; do not selectively delete or edit them.
- Only one writer may own an output directory.
- Use a fresh directory when changing input, schema, identity rules or limits.

<a id="cancellation"></a>

## Cancellation and completion

Call job.cancel() or press Ctrl+C in the Rust CLI. The current uncommitted batch is discarded. Resume with a new job instance. Python run_async() requests cancellation when its task is cancelled.

CLI status 0 means completion without failures or rejected records; 2 means cancellation or reported page/row failures; 1 means the job could not finish. A complete report can still contain errors. elapsed_ms measures only the latest invocation.

- [Troubleshoot a job](faq.md)
