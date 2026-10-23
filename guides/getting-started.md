# Your first dataset

Turn a file of URLs into validated, resumable output in four steps.

<a id="build"></a>

## 1. Build the current source

These guides describe the current development source. DatasetJob is new in this working tree; published alpha packages may not contain it. Build the matching source before using these examples.

Start with the Rust CLI for the shortest path. Python and Node.js use the same job configuration.

### Terminal

```shell
git clone https://github.com/Vamshavardhan50/crawlingo.git
cd crawlingo
cargo build --release --features cli
```

- [SDK installation instructions](installation.md)

<a id="input"></a>

## 2. Create your input

Save urls.txt in your working directory. Each line is an HTTP or HTTPS URL. This small example uses a public example page with an h1 heading.

### urls.txt

```text
https://example.com/
```

<a id="configure"></a>

## 3. Define a record

Save job.json beside urls.txt. The title field is required: pages without it are reported instead of exported as valid records. Paths are relative to the process working directory.

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

<a id="run"></a>

## 4. Run and inspect the result

Open datasets/pages/part-00000000.jsonl for accepted records. Inspect the report counters and error shards too: complete means the input was processed, not that every page succeeded.

Run the same configuration again to resume. Keep the input file and configuration unchanged, and use a new output directory for a different dataset.

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

- [Understand output and recovery](recovery.md)
- [Extract multiple rows per page](dataset-jobs.md#row-containers)
