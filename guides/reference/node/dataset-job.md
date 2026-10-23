# DatasetJob · Node.js

Process a URL file into bounded, validated, resumable dataset shards.

<a id="overview"></a>

## Purpose and lifecycle

The recommended API for large research and directory datasets. Define one row’s meaning, required fields and identity before scaling.

Records remain in Rust and on disk. run returns a report, not all records. Use one active writer per output directory and keep input, configuration, state and committed shards consistent.

A completed resume does not retry already committed failed URLs. Make a new job with corrected input for those failures. Cancellation requires a new instance for resume.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class DatasetJob
```

- [sdk/nodejs/src/dataset-job.ts:48](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L48)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { DatasetJob } from 'crawlingo';
const job = new DatasetJob({
  urls_file: 'urls.txt', output_dir: 'datasets/research',
  fields: [{ name: 'title', selector: 'h1', required: true }],
  format: 'jsonl', concurrency: 8,
});
const report = await job.run();
console.log(report.records_written, report.pages_failed, report.records_rejected);
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | DatasetJob | Present in source |
| run | method | Promise<JobReport> | Present in source |
| cancel | method | void | Present in source |

- [constructor](dataset-job.md#constructor)
- [run](dataset-job.md#run)
- [cancel](dataset-job.md#cancel)

<a id="constructor"></a>

## constructor()

Validate configuration and allocate the job handle. Network requests begin at run, not construction.

Returns / value: DatasetJob.

### Signature

```typescript
constructor(config: JobConfig, session?: Session): DatasetJob
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| config | JobConfig | Required | Configuration object; see the linked job or pagination reference. |
| session | Session | Default: new Session() | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/nodejs/src/dataset-job.ts:50](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L50)

<a id="run"></a>

## run()

Run or resume the job and return JobReport. Rust/Node are asynchronous; Python run blocks and releases the GIL for native work.

Returns / value: Promise<JobReport>.

### Signature

```typescript
run(): Promise<JobReport>
```

- [Source · sdk/nodejs/src/dataset-job.ts:53](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L53)

<a id="cancel"></a>

## cancel()

Request cooperative cancellation. Await the active run before releasing its volume or starting a replacement job. The uncommitted batch may be replayed.

Returns / value: void.

### Signature

```typescript
cancel(): void
```

- [Source · sdk/nodejs/src/dataset-job.ts:54](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L54)

<a id="errors"></a>

## Errors and operational limits

Construction rejects invalid configuration. Job-level storage, checkpoint and configuration failures reject/raise; individual failed pages and rows are counted and written to error shards. complete=true does not mean every page succeeded.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
