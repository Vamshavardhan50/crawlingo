# Dataset generation for large inputs

`DatasetJob` is the primary bulk API. Rust performs fetching, scoped extraction,
validation, exact deduplication and file writing. Python and Node.js receive a
small report rather than a list containing all records.

```text
URL file → bounded fetches → parsed pages → container-scoped records
         → normalization/validation → disk dedup index → committed output shards
```

Create `urls.txt` with one HTTP(S) URL per line. Create `job.json`:

```json
{
  "urls_file": "urls.txt",
  "output_dir": "datasets/products",
  "row_selector": "article.product",
  "concurrency": 16,
  "checkpoint_pages": 256,
  "fields": [
    {"name": "name", "selector": "h2", "required": true},
    {"name": "price", "selector": ".price", "extract_type": "price", "data_type": "float"},
    {"name": "sku", "selector": "article.product", "extract_type": "attr:data-sku"}
  ],
  "format": "jsonl"
}
```

`row_selector` is optional: omit it for one record per page. Fields are resolved
inside their own container; absent fields cannot shift values between records.
Values use the existing string-based SDK data model. Numeric types validate and
canonicalize strings; they do not change JSON strings into JSON numbers.

```bash
cargo run --release --features cli -- dataset job.json
```

```python
import json
from crawlingo import DatasetJob
with open("job.json", encoding="utf-8") as source:
    job = DatasetJob(json.load(source))
report = job.run()  # or await job.run_async()
print(report)
```

```typescript
import { DatasetJob, JobConfig } from 'crawlingo';
import { readFileSync } from 'node:fs';
const config: JobConfig = JSON.parse(readFileSync('job.json', 'utf8'));
const report = await new DatasetJob(config).run();
console.log(report);
```

All three interfaces use the same JSON configuration. Rust exposes
`dataset::{DatasetJob, JobConfig, JobField, JobReport}`. Existing collecting
`Dataset`/`Crawler` APIs are intended for smaller interactive workloads.

## Outputs and restart

- `part-00000000.jsonl` (or `.csv`) contains valid records, `_source_url`, and `_row`.
- `part-00000000.errors.jsonl` records invalid URLs, fetch failures, empty records,
  and field validation failures. Errors are never counted as exported records.
- `state/` owns the exclusive job lock, input/config signature, source byte offset,
  URL index, exact-record index and committed shard count.
- `report.json` is a summary of the last invocation. Read only shards below its
  `shards` count. A crash can leave files beyond that boundary; they are replayed.

Rerun the same config to resume. The input and extraction configuration must
match. Output files are flushed before checkpoint state advances; an interrupted
batch is replayed from its prior source offset. Pending request side effects are
limited to GET requests, and requests can be repeated after interruption.
This is process-interruption recovery; power-loss durability also depends on
the filesystem and hardware. Do not modify, move or selectively delete job files.

`cancel()` cancels in-flight requests and discards the current uncommitted batch.
A new job instance resumes from the last committed batch. CLI Ctrl+C requests
cancellation. CLI exit code 2 means cancelled or completed with rejected/failed
items; code 1 means the job itself could not finish. A complete report can still
contain failures: inspect the counters and error shards.

## Resource and quality controls

`concurrency`, `checkpoint_pages`, `checkpoint_bytes`, `max_response_bytes`,
`max_records_per_page`, and `max_record_bytes` bound work and buffers independently
of total URL count. Default HTTP transport limits bodies to 16 MiB while reading;
the job's default 2 MiB limit applies before parsing. Custom transports must enforce
their own streaming body cap. Storage grows with source/record count; the Sled
cache is capped at 32 MiB, which is not a cap on total process RSS.

Whitespace normalization is configurable per field; disable it for preformatted
content. Unicode symbols and punctuation are preserved. Required fields, minimum
lengths, defaults, and strict integer/finite-float/boolean validation are supported.
JSONL and CSV are committed as independent shards. Existing `DatasetStream`
supports bounded asynchronous production and streaming Parquet export.

Exact dedup compares canonical field values on disk, with collision checking.
`dedup_fields` chooses an entity key; changing it changes dataset semantics. Use
all fields to retain changed observations. Exact dedup does not detect semantic
or near duplicates. URLs are deduplicated separately. Robots rules are checked by
default; configure request pacing on the shared Session. Use inputs you are
authorized to access. The current URL validator is not an SSRF boundary for an
untrusted public URL submission service.

## Scale acceptance

```bash
cargo run --release --example dataset_scale -- 10000 target/scale-10k http
cargo run --release --example dataset_scale -- 1000000 target/scale-1m http
```

Each run requires a new output directory. `http` starts a controlled loopback HTTP
server with keep-alive. `generated` replaces only the transport and is explicitly
labelled in its report. Both exercise the production job, verify every exported
record, and verify an idempotent restart. These are small UTF-8 fixtures, not a
prediction for one million arbitrary websites or rendered browser pages. Record
hardware, OS, build profile, payload, concurrency, elapsed time, and process peak
memory alongside `acceptance.json`. No comparison with other tools is implied.
