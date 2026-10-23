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

### Retries

```typescript
const session = new Session()
  .retries(3)
  .retryDelay(1)
  .retryBackoff(2);
```

| Method | Meaning |
| --- | --- |
| `retries(count)` | Additional attempts after the first request; `0` disables retries. Accepts integers from `0` through `4294967295`. |
| `retryDelay(seconds)` | Initial delay in whole seconds, from `0` through `86400`. Default: 0.5 seconds unless loaded from config. |
| `retryBackoff(factor)` | Delay multiplier; a finite number at least `1`. Default: `2`. |

These setters return the session for chaining. Invalid numeric values throw
`RangeError` before reaching Rust. With the example above, retry delays are 1, 2,
and 4 seconds. Computed delays are capped at the configured maximum (30 seconds
by default), raised to the initial delay if necessary. A numeric `Retry-After`
header takes precedence unless disabled in the configuration file.

Transport failures and HTTP 429, 500, 502, 503 and 504 are retried by default.
The settings apply to requests made through the session, including `Page`,
`Dataset`, `DatasetJob`, `Crawl`, `Sitemap` and `Downloader`. Changes affect
subsequent requests; an in-flight request keeps its settings. Connection pools
and rate limits stay shared. `clone()` copies the retry settings into an
independent session. `Session.fromConfig(path)` supports millisecond delays and
custom retryable status codes.

### Cookies

```typescript
session.cookies({ session_id: 'your-session-value', locale: 'en' });
const snapshot = session.getCookies();
session.deleteCookie('locale');
session.clearCookies();
```

`cookies()` replaces the default cookie map. `getCookies()` returns a copy;
editing it does not change the session. `deleteCookie()` and `clearCookies()`
return the session, and deleting an absent name is harmless. Cloned sessions
have independent cookie maps. These are explicit request defaults: they have
no domain/path/expiry metadata and do not automatically save `Set-Cookie`
responses. Use a separate session per site for authentication cookies.

`Page.create(url, { session, cookies, headers, timeout })` merges per-request
cookies and headers over session defaults without changing the session. An
explicit request timeout overrides the session timeout.

### Handle errors

```typescript
import { Page, CrawlingoError, TimeoutError, DnsError } from 'crawlingo';

try {
  const page = await Page.create('https://example.com/', { session });
  console.log(page.status, page.title());
} catch (error) {
  if (error instanceof TimeoutError) {
    console.error('Request timed out:', error.seconds);
  } else if (error instanceof DnsError) {
    console.error('Could not resolve the host');
  } else if (error instanceof CrawlingoError) {
    console.error(error.code, error.message);
  } else {
    throw error;
  }
}
```

All the classes below extend `CrawlingoError`, which extends JavaScript `Error`.
Native failures retain their original error as `cause`. Where supplied by the
engine, errors include `url`, `stage`, `recoverable`, `suggestion`, `seconds`, and
`host`. `recoverable` is advice, not a guarantee that another attempt will work.

| Export | Default code |
| --- | --- |
| `CrawlingoError` | `GENERIC_FAILURE` |
| `FetchError` | `FETCH_FAILED` |
| `ParseError` | `INVALID_HTML` |
| `SelectorError` | `SELECTOR_FAILED` |
| `AutoMatchFailed` | `AUTO_MATCH_FAILED` |
| `TimeoutError` | `TIMEOUT` |
| `RateLimitError` | `RATE_LIMIT` |
| `ChangeDetectionError` | `CHANGE_DETECTION_FAILED` |
| `ExportError` | `EXPORT_FAILED` |
| `DnsError` | `DNS_FAILED` |
| `FingerprintStoreError` | `FINGERPRINT_STORE_FAILED` |

`Page.create()` continues to return HTTP error responses, including 429 and 500
after retries are exhausted; inspect `page.status`. Exporting `RateLimitError`
does not turn HTTP 429 responses into exceptions. Dataset jobs record individual
page failures in their error shards and report; they do not reject the entire job
for each failed page. A selector with no matches can return an empty collection;
an invalid regular expression throws `SelectorError`.

This API is in the alpha.2 source. Rebuild both the native addon and TypeScript
wrapper to use it; it is absent from the published alpha.1 package.

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
