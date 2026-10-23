# Crawlingo for Node.js

[Complete example](https://github.com/Vamshavardhan50/crawlingo/blob/main/examples/product_dataset/README.md) · [Deployment](https://github.com/Vamshavardhan50/crawlingo/blob/main/guides/deployment.md) · [Publishing](https://github.com/Vamshavardhan50/crawlingo/blob/main/guides/publishing.md)

A TypeScript API for the Rust dataset engine, with bounded processing and
resumable file exports.

[Documentation](https://crawlingo-docs.vercel.app/sdk/nodejs) ·
[Job reference](https://github.com/Vamshavardhan50/crawlingo/blob/main/DATASET_JOBS.md) · [Scale evidence](https://github.com/Vamshavardhan50/crawlingo/blob/main/SCALE_RESULTS.md)

## Build from source

This source targets **1.0.0-alpha.2 (unpublished)** and Node.js 22+. The inspected
npm `1.0.0-alpha.1` tarball does not include `DatasetJob` and contains a Windows x64
addon only. Build the native library and TypeScript wrapper together.
Native compilation requires Rust, C/C++ build tools, CMake and libclang.

The release targets Linux x64 GNU (glibc 2.35+), macOS Intel/Apple Silicon and
Windows x64. The Linux addon is built on Ubuntu 22.04 and checked in Node's
Debian 12 container. Linux ARM64 and Alpine/musl prebuilt addons are not included;
build from source on those targets. Check release status before relying on a
platform's prepared artifacts.

From the repository root:

```sh
cd sdk/nodejs
npm ci
npm run build
npm test
```

Then install the local package from your application:

```sh
npm install /absolute/path/to/crawlingo/sdk/nodejs
```

Change the path to your checkout. `npm run build` creates `dist/` and copies the
platform-specific native assets. If LLVM discovery fails, set `LIBCLANG_PATH` to
the directory containing its shared library. Rebuild for each target platform.

## Generate a dataset

Create `urls.txt` with one HTTP(S) URL per line. In a TypeScript ES module:

```typescript
import { DatasetJob, type JobConfig } from 'crawlingo';

const config: JobConfig = {
  urls_file: 'urls.txt',
  output_dir: 'datasets/pages',
  fields: [{ name: 'title', selector: 'h1', required: true }],
  format: 'jsonl',
};

const job = new DatasetJob(config);
const report = await job.run();
console.log(report);
```

The native engine writes records and error shards. `run()` returns
`Promise<JobReport>`, not an array of all records. Check `pages_failed` and
`records_rejected`, even when `complete` is true.

Call `job.cancel()` to request cancellation. Resume with a new instance and the
same configuration, input and output directory. Use a new directory for changes.

## Configure requests

```typescript
import { DatasetJob, Session } from 'crawlingo';

const session = new Session().rateLimit(2).timeout(30);
session.headers({ 'User-Agent': 'MyDatasetBot/1.0' });
const job = new DatasetJob(config, session);
```

`JobConfig`, `JobField` and `JobReport` are exported types. See the
[job reference](https://github.com/Vamshavardhan50/crawlingo/blob/main/DATASET_JOBS.md) for scoped rows, validation and identity keys.

## Extract a small result

```typescript
import { Dataset } from 'crawlingo';

const result = await new Dataset('https://example.com/')
  .field('title', 'h1')
  .build();
console.log(result.toDict());
await result.toJson('page.json');
```

Collected results support JSON, CSV and Parquet. Bulk jobs support JSONL and CSV.
Job values are strings with optional strict validation. Browser rendering requires
the optional Rust browser feature; the current Node Session type exposes HTTP
`standard` and `stealthy` tiers. TLS profile emulation is not an access guarantee.

## Checks

```sh
npm run lint
npm test
node --test tests/dataset_job.test.cjs
npm run test:all
```

The integration suites use local HTTP fixtures. The broader suite reports planned
or unavailable APIs separately from failed checks; inspect its report before
claiming full API parity.

MIT license. [Report an issue](https://github.com/Vamshavardhan50/crawlingo/issues).
