# JobConfig

The shared configuration contract for Python, Node.js, Rust and the CLI. JSON keys use snake_case in every binding.

<a id="minimal"></a>

## Minimal configuration

Save this as job.json and create urls.txt with one HTTP(S) URL per line. Paths are relative to the process working directory. Unknown keys are rejected.

### job.json

```json
{
  "urls_file": "urls.txt",
  "output_dir": "datasets/pages",
  "fields": [{ "name": "title", "selector": "h1", "required": true }]
}
```

<a id="input"></a>

## Input and extraction

| Property | Type | Default | Behavior |
| --- | --- | --- | --- |
| urls_file | string / Rust PathBuf | Required | UTF-8 input, one URL per line; maximum line size 8192 bytes. |
| output_dir | string / Rust PathBuf | Required | Job-owned state, data, errors and report directory. One active writer. |
| fields | JobField[] / Vec<JobField> | Required | At least one field. Names must be unique, nonempty and cannot begin with _. |
| row_selector | string / Option<String> | Omitted | CSS container for each record. Omit for one record per page. |

- [JobField options and validation](schema.md)
- [Row container example](dataset-jobs.md#row-containers)

<a id="limits"></a>

## Concurrency and resource limits

These limits apply to separate buffers; their sum is not a total memory guarantee. The default HTTP transport also bounds bodies while streaming at 16 MiB. A custom transport needs its own streaming limit.

| Property | Type | Default | Constraint / scope |
| --- | --- | --- | --- |
| concurrency | integer | 16 | 1–1024 concurrent pages; actual active work is also limited by the checkpoint batch size. |
| checkpoint_pages | integer | 256 | 1–10000 input lines per batch. The unfinished batch can replay after interruption. |
| checkpoint_bytes | integer | 33554432 (32 MiB) | Positive byte budget for pending exact dedup keys and values. |
| max_response_bytes | integer | 2097152 (2 MiB) | Positive per-page body limit checked before parsing. |
| max_records_per_page | integer | 1000 | Positive maximum row containers per page. |
| max_record_bytes | integer | 262144 (256 KiB) | Positive maximum serialized extracted field bytes per record. |

- [Capacity planning](production-guide.md)

<a id="export"></a>

## Identity, source policy and output

Changing any serialized configuration or the URL-file content makes it a different job. Use a fresh output directory. JSONL values remain strings after numeric or boolean validation. Checkpointed Parquet output is not part of this API.

| Property | Type | Default | Behavior |
| --- | --- | --- | --- |
| deduplicate | boolean | true | Compare exact record identity against the on-disk index. URL deduplication is separate. |
| dedup_fields | string[] | [] | Empty means all extracted fields; otherwise every name must be declared in fields. Metadata is excluded. |
| respect_robots | boolean | true | Apply robots rules and crawl delay using the session User-Agent. |
| format | "jsonl" \| "csv" | "jsonl" | Fixed-schema shards. CSV includes a header in every data shard. |

<a id="typed"></a>

## TypeScript contract

Import JobConfig from crawlingo in your application. This declaration shows the current exported type; the Rust engine still validates constraints at runtime. Python accepts a dictionary, while Rust can deserialize the same JSON into JobConfig.

### Exported JobConfig type

```typescript
import type { JobField } from 'crawlingo';

interface JobConfig {
  urls_file: string;
  output_dir: string;
  fields: JobField[];
  row_selector?: string;
  concurrency?: number;
  checkpoint_pages?: number;
  checkpoint_bytes?: number;
  max_response_bytes?: number;
  max_records_per_page?: number;
  max_record_bytes?: number;
  deduplicate?: boolean;
  dedup_fields?: string[];
  respect_robots?: boolean;
  format?: 'jsonl' | 'csv';
}
```

- [DatasetJob constructor and methods](api-dataset-job.md)
