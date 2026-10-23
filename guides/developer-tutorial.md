# Build a dataset workflow

Move from the sample catalog to an application-owned job: configuration, selectors, validation, execution and incremental consumption.

<a id="contract"></a>

## 1. Define the record contract first

Write down what one row represents before choosing selectors. For a product catalog, a record might represent one product identified by a SKU. For a price-history dataset, it might represent one observation of a product on a collection date. Those are different identity contracts and should not share a deduplication rule accidentally.

Decide which fields are required, which may be empty, and which defaults are meaningful. Keep source URLs through the pipeline so a reviewer can investigate a surprising value. The job supplies _source_url and _row automatically; application-specific provenance such as collection time must be recorded by your application.

| Field | Rule | Reason |
| --- | --- | --- |
| sku | Required string | Stable product identity. |
| name | Required text | A product without a name is incomplete for this example. |
| price | Required finite float string | Reject nonnumeric text while retaining a parseable value. |
| _source_url / _row | Added by the engine | Trace the record back to its source location. |

<a id="html"></a>

## 2. Match fields within their row

Inspect representative source HTML. Use article.product as the row container and extract the fields relative to it. In this example, the SKU belongs to the row element itself; the matching row selector plus attr:data-sku reads that attribute.

Do not assume every page has the same template. Sample empty prices, optional labels, Unicode text and repeated products. Required values should reject a broken row instead of letting fields from neighboring products shift into it.

### Representative HTML

```html
<article class="product" data-sku="A-101">
  <h2>Café mug</h2>
  <p class="price">$12.50</p>
</article>
<article class="product" data-sku="B-202">
  <h2>Notebook</h2>
  <p class="price">$6.00</p>
</article>
```

- [Selector syntax and extraction options](selectors.md)

<a id="config"></a>

## 3. Write a bounded job configuration

The following configuration is a conservative example for development, not a universal performance preset. Paths are relative to the process working directory. Create the URL file before running the job. Keep output directories unique for different inputs or schemas.

checkpoint_pages bounds how many input lines are handled before advancing the checkpoint. concurrency limits in-flight page operations. Page-body, row-count and record-byte limits protect specific parts of the workload; measure process memory separately.

### job.json

```json
{
  "urls_file": "urls.txt",
  "output_dir": "datasets/catalog-v1",
  "row_selector": "article.product",
  "fields": [
    {
      "name": "sku",
      "selector": "article.product",
      "extract_type": "attr:data-sku",
      "required": true
    },
    {
      "name": "name",
      "selector": "h2",
      "required": true,
      "normalize_whitespace": true
    },
    {
      "name": "price",
      "selector": ".price",
      "extract_type": "price",
      "data_type": "float",
      "required": true
    }
  ],
  "dedup_fields": [
    "sku"
  ],
  "concurrency": 4,
  "checkpoint_pages": 64,
  "max_response_bytes": 2097152,
  "max_records_per_page": 500,
  "max_record_bytes": 65536,
  "checkpoint_bytes": 8388608,
  "respect_robots": true,
  "format": "jsonl"
}
```

- [Complete configuration reference](dataset-jobs.md#limits)

<a id="session"></a>

## 4. Separate request policy from record rules

JobConfig controls extraction, output and resource bounds. A Session controls request behavior such as headers, timeout and request pacing. Reuse a session for the job instead of creating one per URL.

The examples set a request-rate limit and timeout in addition to concurrency. They are example policies, not suitable defaults for every source. Robots checks are enabled in the job configuration. Put private credentials in your runtime configuration and scope their use to the intended sources.

### Python

```python
import json
from crawlingo import DatasetJob, Session

with open("job.json", encoding="utf-8") as source:
    config = json.load(source)
session = Session().rate_limit(2).timeout(30)
report = DatasetJob(config, session=session).run()
print(report)
```

### Node.js

```javascript
import { readFileSync } from "node:fs";
import { DatasetJob, Session } from "crawlingo";

const config = JSON.parse(readFileSync("job.json", "utf8"));
const session = new Session().rateLimit(2).timeout(30);
const job = new DatasetJob(config, session);
process.once("SIGTERM", () => job.cancel());
process.once("SIGINT", () => job.cancel());
const report = await job.run();
console.log(report);
```

- [Session options](session.md)

<a id="quality"></a>

## 5. Treat the report as part of the result

Await the job and inspect complete, cancelled, pages_failed and records_rejected. A returned report does not mean every row passed. Your application should decide whether to accept partial output, request a review, or fail the surrounding workflow.

Read the error shards to distinguish page-level problems from row-level validation failures. Correct the source or configuration, then create a fresh job for corrected or failed inputs. An unchanged resume does not revisit URLs already committed as failures.

### Application outcome policy

```python
if report["cancelled"]:
    outcome = "interrupted: resume the unchanged job"
elif not report["complete"]:
    outcome = "incomplete: investigate before consumption"
elif report["pages_failed"] or report["records_rejected"]:
    outcome = "complete with issues: review error shards"
else:
    outcome = "complete without reported page or row failures"
print(outcome)
```

- [Report and error reference](report-reference.md)

<a id="consume"></a>

## 6. Stream the committed output

After a successful or cancelled run, report.shards identifies the committed output boundary. After a hard crash, resume the job before using the output boundary because the on-disk report may be old and orphan files may remain.

Read one shard and one line at a time. Convert numeric strings where your downstream system requires numbers. Avoid list(...) over the complete generator or reading every file into a single array when processing a large dataset.

### Python JSONL consumer

```python
import json
from pathlib import Path

def records(root, report):
    for shard in range(report["shards"]):
        file = root / f"part-{shard:08d}.jsonl"
        with file.open(encoding="utf-8") as source:
            for line in source:
                yield json.loads(line)

for row in records(Path(config["output_dir"]), report):
    price = float(row["price"])
    # Send this row to your downstream consumer.
    print(row["sku"], price)
```

### Node.js JSONL consumer

```javascript
import { createReadStream } from "node:fs";
import { join } from "node:path";
import { createInterface } from "node:readline";

for (let shard = 0; shard < report.shards; shard++) {
  const name = `part-${String(shard).padStart(8, "0")}.jsonl`;
  const lines = createInterface({
    input: createReadStream(join(config.output_dir, name)),
    crlfDelay: Infinity,
  });
  for await (const line of lines) {
    const row = JSON.parse(line);
    console.log(row.sku, Number(row.price));
  }
}
```

<a id="csv"></a>

## 7. Use CSV when the consumer expects a table

Set format to csv and choose a fresh output directory. Each data shard uses the declared field columns followed by _source_url and _row. Every shard has a header. The error files remain JSONL.

Use a CSV parser even when your fields look simple. Quoted values can contain delimiters and line breaks. When combining shards, write one output header and process each input shard as a separate table.

### Python CSV consumer

```python
import csv
from pathlib import Path

root = Path(config["output_dir"])
for shard in range(report["shards"]):
    with (root / f"part-{shard:08d}.csv").open(
        encoding="utf-8", newline=""
    ) as source:
        for row in csv.DictReader(source):
            print(row["sku"], row["name"])
```

<a id="lifecycle"></a>

## 8. Integrate the job lifecycle

Create one worker owner for each output directory. A service can persist a job ID, input/configuration paths and a process status in its own database. That service database is separate from Crawlingo’s checkpoint store.

In Node, await job.run() and request cancellation from your signal handler. In Python async code, await job.run_async(); cancellation requests stop the native job, but the worker may need time to release its output directory. Start the next instance only after the previous owner has finished. The CLI handles Ctrl+C and Unix SIGTERM.

Do not mark an application job “succeeded” merely because a process exited or complete is true. Use an explicit outcome policy, preserve the report, and keep fatal exceptions separate from per-page errors.

- [Worker deployment](deployment.md)

<a id="larger"></a>

## 9. Grow the workload in stages

- Inspect every row in a tiny fixture before introducing live pages.
- Run a representative sample that includes missing fields and layout variants.
- Measure response sizes, row counts, rejected records, memory and output/state growth.
- Increase input volume while keeping the schema stable; adjust limits only in a fresh job.
- Exercise cancellation and resume before scheduling unattended work.
- Keep the source revision, configuration and measurement method with each benchmark result.

- [Production operating guide](production-guide.md)
- [Million-page measurement](benchmarks.md)
