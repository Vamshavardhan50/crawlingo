# JobReport · Node.js

Job progress and quality counters. Completion is distinct from successful extraction.

<a id="overview"></a>

## Purpose and lifecycle

Job progress and quality counters. Completion is distinct from successful extraction.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](../../report-reference.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export interface JobReport {
  source_lines: number;
  pages_fetched: number;
  pages_failed: number;
  duplicate_urls: number;
  records_written: number;
  records_rejected: number;
  duplicate_records: number;
  shards: number;
  complete: boolean;
  cancelled: boolean;
  elapsed_ms: number;
}
```

- [sdk/nodejs/src/dataset-job.ts:33](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L33)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| source_lines | property | number | Present in source |
| pages_fetched | property | number | Present in source |
| pages_failed | property | number | Present in source |
| duplicate_urls | property | number | Present in source |
| records_written | property | number | Present in source |
| records_rejected | property | number | Present in source |
| duplicate_records | property | number | Present in source |
| shards | property | number | Present in source |
| complete | property | boolean | Present in source |
| cancelled | property | boolean | Present in source |
| elapsed_ms | property | number | Present in source |

- [source_lines](job-report.md#source-lines)
- [pages_fetched](job-report.md#pages-fetched)
- [pages_failed](job-report.md#pages-failed)
- [duplicate_urls](job-report.md#duplicate-urls)
- [records_written](job-report.md#records-written)
- [records_rejected](job-report.md#records-rejected)
- [duplicate_records](job-report.md#duplicate-records)
- [shards](job-report.md#shards)
- [complete](job-report.md#complete)
- [cancelled](job-report.md#cancelled)
- [elapsed_ms](job-report.md#elapsed-ms)

<a id="source-lines"></a>

## source_lines

Cumulative processed input lines.

Returns / value: number.

### Signature

```typescript
source_lines: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:34](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L34)

<a id="pages-fetched"></a>

## pages_fetched

Cumulative successfully processed pages.

Returns / value: number.

### Signature

```typescript
pages_fetched: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:35](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L35)

<a id="pages-failed"></a>

## pages_failed

Cumulative failed pages; inspect error shards.

Returns / value: number.

### Signature

```typescript
pages_failed: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:36](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L36)

<a id="duplicate-urls"></a>

## duplicate_urls

Input URLs skipped because their identity was already seen.

Returns / value: number.

### Signature

```typescript
duplicate_urls: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:37](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L37)

<a id="records-written"></a>

## records_written

Accepted unique records committed to data shards.

Returns / value: number.

### Signature

```typescript
records_written: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:38](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L38)

<a id="records-rejected"></a>

## records_rejected

Rows rejected by validation/extraction checks.

Returns / value: number.

### Signature

```typescript
records_rejected: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:39](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L39)

<a id="duplicate-records"></a>

## duplicate_records

Rows suppressed by exact record identity.

Returns / value: number.

### Signature

```typescript
duplicate_records: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:40](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L40)

<a id="shards"></a>

## shards

Committed shard count used to enumerate output files.

Returns / value: number.

### Signature

```typescript
shards: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:41](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L41)

<a id="complete"></a>

## complete

Input processing completed; failures may still be nonzero.

Returns / value: boolean.

### Signature

```typescript
complete: boolean;
```

- [Source · sdk/nodejs/src/dataset-job.ts:42](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L42)

<a id="cancelled"></a>

## cancelled

This invocation observed cancellation.

Returns / value: boolean.

### Signature

```typescript
cancelled: boolean;
```

- [Source · sdk/nodejs/src/dataset-job.ts:43](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L43)

<a id="elapsed-ms"></a>

## elapsed_ms

Elapsed time for this invocation in milliseconds; not a cumulative historical duration.

Returns / value: number.

### Signature

```typescript
elapsed_ms: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:44](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L44)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
