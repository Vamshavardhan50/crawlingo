# Node.js SDK

A typed job configuration with asynchronous execution through the Rust native module.

<a id="setup"></a>

## Build and link the local package

These guides describe the current development source. DatasetJob is new in this working tree; published alpha packages may not contain it. Build the matching source before using these examples.

Replace the absolute path with your checkout. Build creates dist/ and copies the native assets needed by the package.

### SDK checkout

```shell
cd sdk/nodejs
npm ci
npm run build
```

### Your application

```shell
npm install /absolute/path/to/crawlingo/sdk/nodejs
```

<a id="job"></a>

## Generate a dataset

Use an ES module or an async function for top-level await. DatasetJob.run() returns Promise<JobReport>; cancel() requests cancellation. Fields and report counters remain in Rust until the final summary crosses the binding.

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

<a id="small"></a>

## Extract one page

### TypeScript

```typescript
import { Dataset } from "crawlingo";

const result = await new Dataset("https://example.com/")
  .field("title", "h1")
  .build();
console.log(result.toDict());
await result.toJson("page.json");
```

<a id="types"></a>

## Exported job types

- JobConfig: input, extraction and resource controls.
- JobField: selector, required/default values and validation.
- JobReport: page, row, duplicate, shard and completion counters.

- [Dataset job reference](dataset-jobs.md)
