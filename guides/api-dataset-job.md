# DatasetJob

Stream a URL file into validated JSONL or CSV shards. Construct a job, run it, inspect its report, and resume committed progress.

> Current development source. New dataset APIs require a matching source build; published alpha packages may differ.

## Use this API to build a dataset

Start by defining the unit of observation, source identity and required fields. The research walkthrough runs this API through the CLI against a public directory, then collects a fixed sample of documents.

- [Research dataset walkthrough](https://crawlingo-docs.vercel.app/research-datasets)
- [Dataset design and quality](https://crawlingo-docs.vercel.app/dataset-design)
- [Public-site measurements](https://crawlingo-docs.vercel.app/real-world-results)

## Import

### Python

```python
from crawlingo import DatasetJob, Session
```

### Node.js

```typescript
import { DatasetJob, Session, type JobConfig, type JobReport } from 'crawlingo';
```

### Rust

```rust
use crawlingo::dataset::{DatasetJob, JobConfig, JobReport};
use crawlingo::engine::session::Session;
use std::sync::Arc;
```

- [Build and install the SDK](https://crawlingo-docs.vercel.app/installation)

## Constructor

Validates the extraction configuration and creates a job handle. Fetching starts when run is called. Python and Node.js create a default session when one is omitted; Rust requires an Arc<Session>. These are signatures, followed by a complete usage example below.

### Python signature

```python
DatasetJob(config: dict, session: Optional[Session] = None)
```

### Node.js signature

```typescript
new DatasetJob(config: JobConfig, session?: Session)
```

### Rust signature

```rust
pub fn new(config: JobConfig, session: Arc<Session>) -> Result<Self>
```

| Parameter | Required | Contract |
| --- | --- | --- |
| config | Yes | JobConfig: URL file, output directory, at least one field, and optional execution limits. |
| session | Rust: yes; Python / Node: optional | Request headers, timeout, per-host pacing and transport configuration. Configure it before starting the job. |

- [All JobConfig fields](https://crawlingo-docs.vercel.app/reference/job-config)
- [Session methods](https://crawlingo-docs.vercel.app/reference/session)

## run()

Runs a new job or resumes the same input and configuration. Records stay in the Rust pipeline and are written to disk; the method returns counters rather than a list of every record. The output directory permits one writer at a time.

### Python signature

```python
def run(self) -> dict: ...
async def run_async(self) -> dict: ...
```

### Node.js signature

```typescript
run(): Promise<JobReport>
```

### Rust signature

```rust
pub async fn run(&self) -> Result<JobReport>
```

| Binding | Execution | Return value |
| --- | --- | --- |
| Python run() | Blocking; native work releases the GIL. | Dictionary with JobReport keys. |
| Python run_async() | Runs the blocking job on an executor. | Awaitable yielding the same report dictionary. |
| Node.js run() | Asynchronous native job. | Promise<JobReport>. |
| Rust run().await | Async handle; filesystem work uses a blocking worker. | Result<JobReport, CrawlingoError>. |

- [Every JobReport field](https://crawlingo-docs.vercel.app/report-reference)
- [Committed output and resume](https://crawlingo-docs.vercel.app/recovery)

## Complete usage example

Build the matching SDK, then run one example in a writable working directory. This example writes its one-line URL input and extracts the heading from example.com. Repeating it with the same configuration resumes the existing output. Use a new directory if you change the input or schema.

### Python · main.py

```python
from pathlib import Path
from crawlingo import DatasetJob

Path("urls.txt").write_text("https://example.com/\n", encoding="utf-8")
job = DatasetJob({
    "urls_file": "urls.txt",
    "output_dir": "datasets/example",
    "fields": [{"name": "title", "selector": "h1", "required": True}],
    "format": "jsonl",
})
report = job.run()
print(report)
if not report["complete"] or report["pages_failed"] or report["records_rejected"]:
    raise RuntimeError("Review the report and error shards before using this dataset")
```

### Node.js · main.mjs

```javascript
import { writeFileSync } from 'node:fs';
import { DatasetJob } from 'crawlingo';

writeFileSync('urls.txt', 'https://example.com/\n', 'utf8');
const job = new DatasetJob({
  urls_file: 'urls.txt',
  output_dir: 'datasets/example',
  fields: [{ name: 'title', selector: 'h1', required: true }],
  format: 'jsonl',
});
const report = await job.run();
console.log(report);
if (!report.complete || report.pages_failed || report.records_rejected) {
  throw new Error('Review the report and error shards before using this dataset');
}
```

### Rust · src/main.rs

```rust
use crawlingo::dataset::{DatasetJob, JobConfig};
use crawlingo::engine::session::Session;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write("urls.txt", "https://example.com/\n")?;
    let config: JobConfig = serde_json::from_value(serde_json::json!({
        "urls_file": "urls.txt",
        "output_dir": "datasets/example",
        "fields": [{"name": "title", "selector": "h1", "required": true}],
        "format": "jsonl"
    }))?;
    let report = DatasetJob::new(config, Arc::new(Session::new()))?.run().await?;
    println!("{report:?}");
    if !report.complete || report.pages_failed > 0 || report.records_rejected > 0 {
        return Err("Review the report and error shards".into());
    }
    Ok(())
}
```

## Output record

After a successful fetch of the current example page, a data shard contains a record with this shape. Values are strings; _source_url and _row are provenance fields. Read report.shards to find the committed shard boundary. This is an illustrative result, not a live request from the documentation.

### part-00000000.jsonl

```json
{"title":"Example Domain","_source_url":"https://example.com/","_row":"0"}
```

## cancel()

Requests cancellation and returns immediately: None in Python, void in Node.js and () in Rust. It does not wait for the worker to release the directory. The uncommitted batch is discarded and can be replayed by a new instance. Keep the same URL file, configuration and output directory when resuming.

When a Python run_async task is cancelled, the wrapper requests native cancellation and re-raises CancelledError. The executor worker may still be shutting down. Wait for directory ownership to be released before launching a replacement. A cancelled handle stays cancelled; construct a new one to resume.

### Node.js · cancellation pattern

```javascript
const stop = () => job.cancel();
process.once('SIGINT', stop);
process.once('SIGTERM', stop);
try {
  const report = await job.run();
  console.log({ cancelled: report.cancelled, shards: report.shards });
} finally {
  process.removeListener('SIGINT', stop);
  process.removeListener('SIGTERM', stop);
}
```

### Rust · cancellation pattern

```rust
let running_job = job.clone();
let task = tokio::spawn(async move { running_job.run().await });
// Call this from your application's shutdown path:
job.cancel();
let report = task.await??;
```

- [Production shutdown and recovery](https://crawlingo-docs.vercel.app/production-guide)

## Errors and failure handling

Constructor and run failures throw in Python / Node.js and return Err in Rust. They include invalid configuration, inaccessible files, locked state and input changes. DatasetJob does not currently expose a portable structured error-code enum through the bindings; avoid treating message text as a stable machine API.

| Condition | Where to look | Next action |
| --- | --- | --- |
| Invalid config or unsupported format | Constructor exception / Err | Check required fields, accepted values and positive resource limits. |
| Input file missing or output not writable | run exception / rejected promise / Err | Resolve paths against the worker’s current directory and check access. |
| Another writer owns output_dir | run failure while opening state | Wait for the previous worker; use one owner per directory. |
| Input or config changed on resume | run failure | Start a new job directory for the changed dataset. |
| HTTP failure, robots denial, missing row container | pages_failed and error shards | Review failed sources. A completed job will not retry them automatically. |
| Required field missing or invalid value | records_rejected and error shards | Inspect the affected row and correct the extraction rules. |
