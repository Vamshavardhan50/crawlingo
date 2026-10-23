# PaginationConfig · Node.js

Choose how a crawler discovers the next page.

<a id="overview"></a>

## Purpose and lifecycle

Use the named factories rather than calling the constructor. Attach the returned configuration with Crawl.withPagination/with_pagination.

Set crawl limits even when pagination has its own maximum. A URL pattern is not evidence that every generated page exists.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class PaginationConfig
```

- [sdk/nodejs/src/pagination.ts:3](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/pagination.ts#L3)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { Crawl, PaginationConfig } from 'crawlingo';
const crawl = new Crawl('https://example.com/catalog?page=1')
  .withPagination(PaginationConfig.pageNumber('https://example.com/catalog?page={page}', 1, 5))
  .limit(5).field('title', 'h1');
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| inner | property | JsPaginationConfig | Present in source |
| nextLink | static method | PaginationConfig | Present in source |
| pageNumber | static method | PaginationConfig | Present in source |
| urlPattern | static method | PaginationConfig | Present in source |

- [inner](pagination-config.md#inner)
- [nextLink](pagination-config.md#next-link)
- [pageNumber](pagination-config.md#page-number)
- [urlPattern](pagination-config.md#url-pattern)

<a id="inner"></a>

## inner

Native interop handle. Prefer the documented wrapper methods; bypassing them can bypass validation and typed error conversion.

Returns / value: JsPaginationConfig.

### Signature

```typescript
public readonly inner: JsPaginationConfig;
```

- [Source · sdk/nodejs/src/pagination.ts:4](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/pagination.ts#L4)

<a id="next-link"></a>

## nextLink()

Use the href of the next-link selector.

Returns / value: PaginationConfig.

### Signature

```typescript
nextLink(selector: string): PaginationConfig
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| selector | string | Required | Selector evaluated against the fetched DOM. |

- [Source · sdk/nodejs/src/pagination.ts:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/pagination.ts#L10)

<a id="page-number"></a>

## pageNumber()

Construct numbered URLs from a template containing {page}, starting at startPage/start_page and bounded by maxPages/max_pages.

Returns / value: PaginationConfig.

### Signature

```typescript
pageNumber(urlTemplate: string, startPage: number, maxPages: number): PaginationConfig
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| urlTemplate | string | Required | URL template containing the literal {page} placeholder. |
| startPage | number | Required | First page number used by numbered pagination. |
| maxPages | number | Required | Numbered-pagination bound passed to the native configuration. |

- [Source · sdk/nodejs/src/pagination.ts:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/pagination.ts#L14)

<a id="url-pattern"></a>

## urlPattern()

Increment the page number captured by a regular expression in the current URL; stop at the configured maximum.

Returns / value: PaginationConfig.

### Signature

```typescript
urlPattern(pageRegex: string, maxPage: number): PaginationConfig
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| pageRegex | string | Required | Regular expression capturing the current page number in a URL. |
| maxPage | number | Required | Upper page-number bound for pattern pagination. |

- [Source · sdk/nodejs/src/pagination.ts:18](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/pagination.ts#L18)

<a id="errors"></a>

## Errors and operational limits

Invalid patterns or templates can fail when evaluated by the crawler. Validate a small URL sequence before a larger crawl.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
