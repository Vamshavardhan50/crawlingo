# Dataset builder

Extract and export a small result, or move to DatasetJob for bulk generation.

<a id="single"></a>

## One page, one result

### Python

```python
from crawlingo import Dataset

result = Dataset("https://example.com/").field("title", "h1").build()
result.to_json("page.json")
result.to_csv("page.csv")
result.to_parquet("page.parquet")
```

### Node.js

```typescript
import { Dataset } from "crawlingo";

const result = await new Dataset("https://example.com/")
  .field("title", "h1").build();
await result.toJson("page.json");
await result.toCsv("page.csv");
await result.toParquet("page.parquet");
```

<a id="rows"></a>

## Structured extraction

Legacy structured extraction collects columns and rejects unequal match counts. For repeated row containers with optional fields, prefer DatasetJob.row_selector; it keeps fields attached to their own record.

<a id="exports"></a>

## Choose an export API

| API | Use | Formats |
| --- | --- | --- |
| DatasetResult | Small collected result | JSON, CSV, Parquet |
| DatasetStream | Rust record producers | CSV, Parquet |
| DatasetJob | Resumable URL-file pipeline | JSONL shards, CSV shards |

- [Dataset job reference](dataset-jobs.md)
