# SitemapEntry · Node.js

Sitemap URL metadata. lastmod and priority are source strings, not validated dates or numbers.

<a id="overview"></a>

## Purpose and lifecycle

Sitemap URL metadata. lastmod and priority are source strings, not validated dates or numbers.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](sitemap.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export interface SitemapEntry {
  loc: string;
  lastmod?: string;
  changefreq?: string;
  priority?: string;
}
```

- [sdk/nodejs/src/sitemap.ts:13](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L13)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| loc | property | string | Present in source |
| lastmod | property | string \| undefined | Present in source |
| changefreq | property | string \| undefined | Present in source |
| priority | property | string \| undefined | Present in source |

- [loc](sitemap-entry.md#loc)
- [lastmod](sitemap-entry.md#lastmod)
- [changefreq](sitemap-entry.md#changefreq)
- [priority](sitemap-entry.md#priority)

<a id="loc"></a>

## loc

URL supplied by the sitemap.

Returns / value: string.

### Signature

```typescript
loc: string;
```

- [Source · sdk/nodejs/src/sitemap.ts:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L14)

<a id="lastmod"></a>

## lastmod

Optional last-modified string from the sitemap.

Returns / value: string | undefined.

### Signature

```typescript
lastmod?: string;
```

- [Source · sdk/nodejs/src/sitemap.ts:15](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L15)

<a id="changefreq"></a>

## changefreq

Optional advisory update frequency.

Returns / value: string | undefined.

### Signature

```typescript
changefreq?: string;
```

- [Source · sdk/nodejs/src/sitemap.ts:16](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L16)

<a id="priority"></a>

## priority

Optional advisory priority string.

Returns / value: string | undefined.

### Signature

```typescript
priority?: string;
```

- [Source · sdk/nodejs/src/sitemap.ts:17](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L17)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
