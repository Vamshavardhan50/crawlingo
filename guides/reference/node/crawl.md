# Crawl · Node.js

Discover linked pages and extract a small in-memory result set.

<a id="overview"></a>

## Purpose and lifecycle

Use follow, limit, depth and concurrency to bound discovery. For a large output dataset, persist discovered URLs and pass them to DatasetJob.

Node run returns DatasetResult[]. Python build returns a CrawlResults wrapper, but the native result conversion currently has a documented mismatch; review known issues.

Persistent crawl frontier state and DatasetJob checkpoint state are different formats and APIs.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. 1 member(s) have a known limitation; see the marked entries below.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class Crawl
```

- [sdk/nodejs/src/crawl.ts:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L7)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { Crawl, Session } from 'crawlingo';
const results = await new Crawl('https://example.com/', new Session().rateLimit(1))
  .follow('a').limit(10).depth(1).concurrency(2)
  .field('title', 'h1').run();
console.log(results.map(result => result.toDict()));
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | Crawl | Present in source |
| resumable | static method | Crawl | Present in source |
| autoMatch | method | this | Present in source |
| follow | method | this | Present in source |
| withPagination | method | this | Present in source |
| limit | method | this | Present in source |
| depth | method | this | Present in source |
| concurrency | method | this | Present in source |
| delay | method | this | Present in source |
| field | method | this | Present in source |
| webhook | method | this | Present in source |
| schedule | method | void | Known limitation — read entry |
| run | method | Promise<DatasetResult[]> | Present in source |

- [constructor](crawl.md#constructor)
- [resumable](crawl.md#resumable)
- [autoMatch](crawl.md#auto-match)
- [follow](crawl.md#follow)
- [withPagination](crawl.md#with-pagination)
- [limit](crawl.md#limit)
- [depth](crawl.md#depth)
- [concurrency](crawl.md#concurrency)
- [delay](crawl.md#delay)
- [field](crawl.md#field)
- [webhook](crawl.md#webhook)
- [schedule](crawl.md#schedule)
- [run](crawl.md#run)

<a id="constructor"></a>

## constructor()

Create a bounded link-discovery builder. Configure fields, page/depth limits and pacing before running it. Python can also wrap a supplied native crawl handle.

Returns / value: Crawl.

### Signature

```typescript
constructor(startUrl: string, session?: Session, inner?: JsCrawl): Crawl
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| startUrl | string | Required | Initial page used for crawl discovery. |
| session | Session \| undefined | Optional | Shared request state. Omit only where the signature provides a default. |
| inner | JsCrawl \| undefined | Optional | Existing native handle for wrapper interop; not a URL or plain data object. |

- [Source · sdk/nodejs/src/crawl.ts:12](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L12)

<a id="resumable"></a>

## resumable()

Use a persistent frontier directory. Python argument order is start_url, path, session; Node order is startUrl, session, path.

Returns / value: Crawl.

### Signature

```typescript
resumable(startUrl: string, session: Session, path: string): Crawl
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| startUrl | string | Required | Initial page used for crawl discovery. |
| session | Session | Required | Shared request state. Omit only where the signature provides a default. |
| path | string | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/nodejs/src/crawl.ts:17](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L17)

<a id="auto-match"></a>

## autoMatch()

Set heuristic recovery on the associated session.

Returns / value: this.

### Signature

```typescript
autoMatch(enabled: boolean): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | boolean | Required | Enable or disable this behavior. |

- [Source · sdk/nodejs/src/crawl.ts:22](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L22)

<a id="follow"></a>

## follow()

Set the CSS selector for links to enqueue. The link’s href is resolved against the page URL.

Returns / value: this.

### Signature

```typescript
follow(selector: string): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| selector | string | Required | Selector evaluated against the fetched DOM. |

- [Source · sdk/nodejs/src/crawl.ts:27](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L27)

<a id="with-pagination"></a>

## withPagination()

Attach a PaginationConfig built with one of its factories.

Returns / value: this.

### Signature

```typescript
withPagination(config: PaginationConfig): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| config | PaginationConfig | Required | Configuration object; see the linked job or pagination reference. |

- [Source · sdk/nodejs/src/crawl.ts:32](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L32)

<a id="limit"></a>

## limit()

Set a maximum page count for this crawl.

Returns / value: this.

### Signature

```typescript
limit(limitNum: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| limitNum | number | Required | Maximum number of pages to collect. |

- [Source · sdk/nodejs/src/crawl.ts:37](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L37)

<a id="depth"></a>

## depth()

Set the maximum link-hop depth from the starting URL.

Returns / value: this.

### Signature

```typescript
depth(depthNum: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| depthNum | number | Required | Maximum link-following depth from seed pages. |

- [Source · sdk/nodejs/src/crawl.ts:42](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L42)

<a id="concurrency"></a>

## concurrency()

Set concurrent crawl workers. Start small and respect source pacing.

Returns / value: this.

### Signature

```typescript
concurrency(n: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| n | number | Required | Count or index; see the method description. |

- [Source · sdk/nodejs/src/crawl.ts:47](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L47)

<a id="delay"></a>

## delay()

Set request spacing in seconds; this is separate from retry backoff.

Returns / value: this.

### Signature

```typescript
delay(seconds: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | number | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/nodejs/src/crawl.ts:52](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L52)

<a id="field"></a>

## field()

Append an extraction field evaluated on each crawled page.

Returns / value: this.

### Signature

```typescript
field(name: string, selector: string, options?: { selectorType?: "css" | "xpath"; defaultVal?: string; }): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | string | Required | Field, header or attribute name for this operation. |
| selector | string | Required | Selector evaluated against the fetched DOM. |
| options | { selectorType?: "css" \| "xpath"; defaultVal?: string; } \| undefined | Optional | Optional settings object. Its inline signature lists the accepted keys and types. |

- [Source · sdk/nodejs/src/crawl.ts:57](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L57)

<a id="webhook"></a>

## webhook()

Send results to an application-owned endpoint. Delivery is not an exactly-once queue and has no durable retry receipt API.

Returns / value: this.

### Signature

```typescript
webhook(url: string): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | string | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/nodejs/src/crawl.ts:69](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L69)

<a id="schedule"></a>

## schedule()

Known limitation: A background native loop is started without a public cancellation/join handle. Use an external scheduler for controlled worker lifecycle.

Start an unmanaged native background thread that repeats the crawl. Node exposes no stop/join handle for it. Prefer a worker scheduler that owns one run at a time.

Returns / value: void.

### Signature

```typescript
schedule(intervalSeconds: number): void
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| intervalSeconds | number | Required | Repeat interval in seconds. See lifecycle restrictions before scheduling. |

- [Source · sdk/nodejs/src/crawl.ts:74](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L74)

<a id="run"></a>

## run()

Return all successful result wrappers in memory. This is not a disk-sharded bulk job.

Returns / value: Promise<DatasetResult[]>.

### Signature

```typescript
run(): Promise<DatasetResult[]>
```

- [Source · sdk/nodejs/src/crawl.ts:78](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/crawl.ts#L78)

<a id="errors"></a>

## Errors and operational limits

Some page failures are logged while crawling continues; Node run does not expose the complete native CrawlResult diagnostics. A result list alone is not proof that every discovered URL succeeded.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
