# JobConfig · Node.js

Shared input, output, schema and resource-limit configuration. Unknown keys are rejected.

<a id="overview"></a>

## Purpose and lifecycle

Shared input, output, schema and resource-limit configuration. Unknown keys are rejected.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](../../api-job-config.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export interface JobConfig {
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

- [sdk/nodejs/src/dataset-job.ts:17](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L17)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| urls_file | property | string | Present in source |
| output_dir | property | string | Present in source |
| fields | property | JobField[] | Present in source |
| row_selector | property | string \| undefined | Present in source |
| concurrency | property | number \| undefined | Present in source |
| checkpoint_pages | property | number \| undefined | Present in source |
| checkpoint_bytes | property | number \| undefined | Present in source |
| max_response_bytes | property | number \| undefined | Present in source |
| max_records_per_page | property | number \| undefined | Present in source |
| max_record_bytes | property | number \| undefined | Present in source |
| deduplicate | property | boolean \| undefined | Present in source |
| dedup_fields | property | string[] \| undefined | Present in source |
| respect_robots | property | boolean \| undefined | Present in source |
| format | property | "jsonl" \| "csv" \| undefined | Present in source |

- [urls_file](job-config.md#urls-file)
- [output_dir](job-config.md#output-dir)
- [fields](job-config.md#fields)
- [row_selector](job-config.md#row-selector)
- [concurrency](job-config.md#concurrency)
- [checkpoint_pages](job-config.md#checkpoint-pages)
- [checkpoint_bytes](job-config.md#checkpoint-bytes)
- [max_response_bytes](job-config.md#max-response-bytes)
- [max_records_per_page](job-config.md#max-records-per-page)
- [max_record_bytes](job-config.md#max-record-bytes)
- [deduplicate](job-config.md#deduplicate)
- [dedup_fields](job-config.md#dedup-fields)
- [respect_robots](job-config.md#respect-robots)
- [format](job-config.md#format)

<a id="urls-file"></a>

## urls_file

Required path to a UTF-8 URL-per-line input file.

Returns / value: string.

### Signature

```typescript
urls_file: string;
```

- [Source · sdk/nodejs/src/dataset-job.ts:18](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L18)

<a id="output-dir"></a>

## output_dir

Required job-owned directory containing state, manifest and shards; only one active writer.

Returns / value: string.

### Signature

```typescript
output_dir: string;
```

- [Source · sdk/nodejs/src/dataset-job.ts:19](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L19)

<a id="fields"></a>

## fields

At least one named field rule. Names must be unique and not use the reserved underscore prefix.

Returns / value: JobField[].

### Signature

```typescript
fields: JobField[];
```

- [Source · sdk/nodejs/src/dataset-job.ts:20](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L20)

<a id="row-selector"></a>

## row_selector

Optional CSS container for one record. Omit for one row per page.

Returns / value: string | undefined.

### Signature

```typescript
row_selector?: string;
```

- [Source · sdk/nodejs/src/dataset-job.ts:21](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L21)

<a id="concurrency"></a>

## concurrency

Maximum in-flight pages; default 16, accepted range 1–1024.

Returns / value: number | undefined.

### Signature

```typescript
concurrency?: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:22](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L22)

<a id="checkpoint-pages"></a>

## checkpoint_pages

Maximum input lines per commit batch; default 256, accepted range 1–10000.

Returns / value: number | undefined.

### Signature

```typescript
checkpoint_pages?: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:23](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L23)

<a id="checkpoint-bytes"></a>

## checkpoint_bytes

Pending exact-dedup key/value byte budget; default 33554432.

Returns / value: number | undefined.

### Signature

```typescript
checkpoint_bytes?: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:24](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L24)

<a id="max-response-bytes"></a>

## max_response_bytes

Per-page body limit before parsing; default 2097152.

Returns / value: number | undefined.

### Signature

```typescript
max_response_bytes?: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:25](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L25)

<a id="max-records-per-page"></a>

## max_records_per_page

Maximum row containers extracted per page; default 1000.

Returns / value: number | undefined.

### Signature

```typescript
max_records_per_page?: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:26](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L26)

<a id="max-record-bytes"></a>

## max_record_bytes

Maximum serialized extracted fields per record; default 262144.

Returns / value: number | undefined.

### Signature

```typescript
max_record_bytes?: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:27](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L27)

<a id="deduplicate"></a>

## deduplicate

Exact record deduplication; default true.

Returns / value: boolean | undefined.

### Signature

```typescript
deduplicate?: boolean;
```

- [Source · sdk/nodejs/src/dataset-job.ts:28](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L28)

<a id="dedup-fields"></a>

## dedup_fields

Declared field names defining identity; an empty list uses all extracted fields.

Returns / value: string[] | undefined.

### Signature

```typescript
dedup_fields?: string[];
```

- [Source · sdk/nodejs/src/dataset-job.ts:29](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L29)

<a id="respect-robots"></a>

## respect_robots

Respect robots rules and crawl delay; default true.

Returns / value: boolean | undefined.

### Signature

```typescript
respect_robots?: boolean;
```

- [Source · sdk/nodejs/src/dataset-job.ts:30](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L30)

<a id="format"></a>

## format

Bulk export format: jsonl (default) or csv.

Returns / value: "jsonl" | "csv" | undefined.

### Signature

```typescript
format?: 'jsonl' | 'csv';
```

- [Source · sdk/nodejs/src/dataset-job.ts:31](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L31)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
