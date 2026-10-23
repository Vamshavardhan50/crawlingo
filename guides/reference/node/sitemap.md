# Sitemap · Node.js

Discover URLs from a sitemap or use them to seed a bounded crawl.

<a id="overview"></a>

## Purpose and lifecycle

listUrls/list_urls returns an in-memory list of sitemap entries. It does not fetch each listed page.

maxDepth/max_depth limits sitemap-index recursion. depth limits subsequent crawling from each seed; these limits have different meanings.

For a large dataset, persist discovered URLs to a file and use DatasetJob. Inspect gzip and child-sitemap handling for your binding before relying on compressed sitemap support.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class Sitemap
```

- [sdk/nodejs/src/sitemap.ts:28](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L28)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { Sitemap } from 'crawlingo';
import { writeFileSync } from 'node:fs';
const entries = await new Sitemap('https://example.com/sitemap.xml').maxDepth(3).listUrls();
writeFileSync('urls.txt', entries.map(entry => entry.loc).join('\n') + '\n');
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | Sitemap | Present in source |
| maxDepth | method | this | Present in source |
| follow | method | this | Present in source |
| limit | method | this | Present in source |
| depth | method | this | Present in source |
| concurrency | method | this | Present in source |
| delay | method | this | Present in source |
| field | method | this | Present in source |
| webhook | method | this | Present in source |
| listUrls | method | Promise<SitemapEntry[]> | Present in source |
| build | method | Promise<DatasetResult[]> | Present in source |

- [constructor](sitemap.md#constructor)
- [maxDepth](sitemap.md#max-depth)
- [follow](sitemap.md#follow)
- [limit](sitemap.md#limit)
- [depth](sitemap.md#depth)
- [concurrency](sitemap.md#concurrency)
- [delay](sitemap.md#delay)
- [field](sitemap.md#field)
- [webhook](sitemap.md#webhook)
- [listUrls](sitemap.md#list-urls)
- [build](sitemap.md#build)

<a id="constructor"></a>

## constructor()

Create a sitemap discovery builder. Fetching begins with listUrls/list_urls or build; the constructor does not materialize discovered URLs.

Returns / value: Sitemap.

### Signature

```typescript
constructor(sitemapUrl: string, opts?: { session?: Session; }): Sitemap
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| sitemapUrl | string | Required | URL of a sitemap XML document or index. |
| opts | { session?: Session; } | Default: {} | Optional extraction settings; use only the keys declared by this method. |

- [Source · sdk/nodejs/src/sitemap.ts:32](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L32)

<a id="max-depth"></a>

## maxDepth()

Maximum sitemap-index nesting depth.

Returns / value: this.

### Signature

```typescript
maxDepth(depth: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| depth | number | Required | Nesting or crawl-hop limit, as specified by this method. |

- [Source · sdk/nodejs/src/sitemap.ts:37](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L37)

<a id="follow"></a>

## follow()

Configure links followed after a seed page is fetched.

Returns / value: this.

### Signature

```typescript
follow(selector: string): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| selector | string | Required | Selector evaluated against the fetched DOM. |

- [Source · sdk/nodejs/src/sitemap.ts:42](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L42)

<a id="limit"></a>

## limit()

Configure the downstream crawl page limit.

Returns / value: this.

### Signature

```typescript
limit(pages: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| pages | number | Required | Maximum downstream crawl pages. |

- [Source · sdk/nodejs/src/sitemap.ts:47](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L47)

<a id="depth"></a>

## depth()

Configure the downstream crawl hop limit.

Returns / value: this.

### Signature

```typescript
depth(maxDepth: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| maxDepth | number | Required | Maximum sitemap nesting or crawl depth, according to the API. |

- [Source · sdk/nodejs/src/sitemap.ts:52](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L52)

<a id="concurrency"></a>

## concurrency()

Configure concurrent downstream crawl workers.

Returns / value: this.

### Signature

```typescript
concurrency(n: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| n | number | Required | Count or index; see the method description. |

- [Source · sdk/nodejs/src/sitemap.ts:57](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L57)

<a id="delay"></a>

## delay()

Configure downstream crawl delay in seconds.

Returns / value: this.

### Signature

```typescript
delay(seconds: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | number | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/nodejs/src/sitemap.ts:62](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L62)

<a id="field"></a>

## field()

Add a downstream extraction field.

Returns / value: this.

### Signature

```typescript
field(name: string, selector: string, selectorType?: string, defaultValue?: string): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | string | Required | Field, header or attribute name for this operation. |
| selector | string | Required | Selector evaluated against the fetched DOM. |
| selectorType | string | Default: 'css' | Selector engine for this field; supported values differ between builders. |
| defaultValue | string \| undefined | Optional | Fallback string when extraction produces no value. |

- [Source · sdk/nodejs/src/sitemap.ts:68](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L68)

<a id="webhook"></a>

## webhook()

Set a downstream result endpoint; delivery is not durable.

Returns / value: this.

### Signature

```typescript
webhook(url: string): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | string | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/nodejs/src/sitemap.ts:73](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L73)

<a id="list-urls"></a>

## listUrls()

Fetch sitemap documents and return discovered URL entries; results are collected in memory.

Returns / value: Promise<SitemapEntry[]>.

### Signature

```typescript
listUrls(): Promise<SitemapEntry[]>
```

- [Source · sdk/nodejs/src/sitemap.ts:79](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L79)

<a id="build"></a>

## build()

Seed a crawl with sitemap URLs and return its in-memory results.

Returns / value: Promise<DatasetResult[]>.

### Signature

```typescript
build(): Promise<DatasetResult[]>
```

- [Source · sdk/nodejs/src/sitemap.ts:92](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L92)

<a id="errors"></a>

## Errors and operational limits

Node listUrls now propagates fetch/parse errors. Python list_urls can skip failing sitemap fetches in its native binding. Empty results can therefore have different meanings.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
