# Dataset jobs

The bulk API for fetching, validating, deduplicating and exporting records without collecting the entire dataset in memory.

<a id="configuration"></a>

## A shared configuration

Rust, Python, Node.js and the CLI use the same JSON keys. Unknown configuration keys are rejected. Define at least one field and use a job-owned output directory.

### job.json

```json
{
  "urls_file": "urls.txt",
  "output_dir": "datasets/pages",
  "fields": [
    {
      "name": "title",
      "selector": "h1",
      "required": true
    }
  ],
  "concurrency": 16,
  "format": "jsonl"
}
```

<a id="row-containers"></a>

## Multiple rows per page

Set row_selector to the CSS container for one record. Field matches are assigned to that container, so a missing price cannot shift the price of the next product into the wrong row. Without a row selector, the page produces one record.

### Product rows

```json
{
  "urls_file": "urls.txt",
  "output_dir": "datasets/products",
  "row_selector": "article.product",
  "fields": [
    {
      "name": "name",
      "selector": "h2",
      "required": true
    },
    {
      "name": "sku",
      "selector": "article.product",
      "extract_type": "attr:data-sku",
      "required": true
    },
    {
      "name": "price",
      "selector": ".price",
      "extract_type": "price",
      "data_type": "float"
    }
  ],
  "dedup_fields": [
    "sku"
  ],
  "format": "jsonl"
}
```

<a id="limits"></a>

## Configuration reference

The default HTTP transport caps bodies while reading at 16 MiB; the smaller job limit is checked before parsing. Custom transports must enforce their own streaming cap. Limits bound specific buffers, not total process memory.

| Setting | Default | Meaning |
| --- | --- | --- |
| urls_file / output_dir | Required | Input file and exclusive job directory. |
| fields | Required | Named field extraction definitions. |
| row_selector | None | CSS container for one row. |
| concurrency | 16 | In-flight page limit; range 1–1024. |
| checkpoint_pages | 256 | Input lines in one checkpoint batch; range 1–10000. |
| max_response_bytes | 2097152 | Body limit before parsing. |
| max_records_per_page | 1000 | Maximum row containers per page. |
| max_record_bytes | 262144 | Maximum serialized extracted field bytes. |
| checkpoint_bytes | 33554432 | Pending exact-dedup key/value byte budget. |
| deduplicate | true | Exact record deduplication. |
| dedup_fields | [] | Identity fields; empty means all extracted fields. |
| respect_robots | true | Check robots rules and crawl delay. |
| format | jsonl | jsonl or csv. |

<a id="run"></a>

## Run a job

### Python

```python
import json
from crawlingo import DatasetJob

with open("job.json", encoding="utf-8") as source:
    config = json.load(source)

report = DatasetJob(config).run()
print(report["records_written"])
print(report["pages_failed"], report["records_rejected"])
```

### Node.js

```typescript
import { readFileSync } from 'node:fs';
import { DatasetJob, type JobConfig } from 'crawlingo';

const config: JobConfig = JSON.parse(
  readFileSync('job.json', 'utf8')
);
const report = await new DatasetJob(config).run();
console.log(report.records_written);
console.log(report.pages_failed, report.records_rejected);
```

### Rust

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
    let report = job.run().await?;
    println!("{report:?}");
    Ok(())
}
```

### CLI

```shell
cargo run --release --features cli -- dataset job.json
```

- [Field schemas and validation](schema.md)
- [Output and resume](recovery.md)
