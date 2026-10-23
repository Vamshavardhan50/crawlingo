# Node.js / TypeScript API

Browse every package export and its declared methods, fields, parameters, returns, source links and known limitations.

<a id="scope"></a>

## What this reference covers

34 package exports; 34 documented symbols including reachable types and helpers; 193 explicitly declared members. Inherited standard-library members are not included in this count.

These are source inventory counts, not a percentage of working features. Classes with broken or limited wrapper paths are marked in their member entries.

- [Implementation audit](implementation-status.md)

<a id="class"></a>

## Classes and result objects

| Symbol | Members | Purpose |
| --- | --- | --- |
| Page | 13 | Fetch one page and inspect its parsed HTML before defining a dataset schema. |
| Element | 4 | Read one selected DOM element and its attributes. |
| ElementCollection | 8 | Read, iterate and select from multiple DOM matches. |
| Session | 23 | Own shared request defaults, connection pools, per-host pacing and retry settings. |
| CrawlingoError | 9 | Base class for failures reported by the Crawlingo engine. |
| FetchError | 1 |  |
| ParseError | 1 |  |
| SelectorError | 1 |  |
| AutoMatchFailed | 1 |  |
| TimeoutError | 1 |  |
| RateLimitError | 1 |  |
| ChangeDetectionError | 1 |  |
| ExportError | 1 |  |
| DnsError | 1 |  |
| FingerprintStoreError | 1 |  |
| Dataset | 11 | Extract named fields from one page, with optional validation and small-result exports. |
| DatasetResult | 5 | Read and export a single extracted field map. |
| DatasetJob | 3 | Process a URL file into bounded, validated, resumable dataset shards. |
| Crawl | 13 | Discover linked pages and extract a small in-memory result set. |
| Watch | 5 | Poll selected fields and deliver detected changes to application callbacks. |
| Sitemap | 11 | Discover URLs from a sitemap or use them to seed a bounded crawl. |
| Downloader | 5 | Save a response to a local file or receive its bytes in memory. |
| PaginationConfig | 4 | Choose how a crawler discovers the next page. |
| DatasetSchema | 4 | Validate required fields and canonicalize string values in small Dataset results. |
| FieldConstraint | 4 | Describe a field name, its type and whether it is required. |

- [Page](node/page.md)
- [Element](node/element.md)
- [ElementCollection](node/element-collection.md)
- [Session](node/session.md)
- [CrawlingoError](node/crawlingo-error.md)
- [FetchError](node/fetch-error.md)
- [ParseError](node/parse-error.md)
- [SelectorError](node/selector-error.md)
- [AutoMatchFailed](node/auto-match-failed.md)
- [TimeoutError](node/timeout-error.md)
- [RateLimitError](node/rate-limit-error.md)
- [ChangeDetectionError](node/change-detection-error.md)
- [ExportError](node/export-error.md)
- [DnsError](node/dns-error.md)
- [FingerprintStoreError](node/fingerprint-store-error.md)
- [Dataset](node/dataset.md)
- [DatasetResult](node/dataset-result.md)
- [DatasetJob](node/dataset-job.md)
- [Crawl](node/crawl.md)
- [Watch](node/watch.md)
- [Sitemap](node/sitemap.md)
- [Downloader](node/downloader.md)
- [PaginationConfig](node/pagination-config.md)
- [DatasetSchema](node/dataset-schema.md)
- [FieldConstraint](node/field-constraint.md)

<a id="interface"></a>

## Interface declarations

| Symbol | Members | Purpose |
| --- | --- | --- |
| CrawlingoErrorOptions | 7 | Optional Node error context, including inherited ErrorOptions.cause. Native context is optional. |
| JobConfig | 14 | Shared input, output, schema and resource-limit configuration. Unknown keys are rejected. |
| JobField | 9 | One extraction rule and its validation requirements. Exported values remain strings. |
| JobReport | 11 | Job progress and quality counters. Completion is distinct from successful extraction. |
| WatchChangeEvent | 5 | Node callback event with camelCase names and change types such as ContentChange and PriceChange. |
| SitemapEntry | 4 | Sitemap URL metadata. lastmod and priority are source strings, not validated dates or numbers. |
| DownloadResult | 6 | Response metadata and bytes actually retained/written. Check status and resumed explicitly. |

- [CrawlingoErrorOptions](node/crawlingo-error-options.md)
- [JobConfig](node/job-config.md)
- [JobField](node/job-field.md)
- [JobReport](node/job-report.md)
- [WatchChangeEvent](node/watch-change-event.md)
- [SitemapEntry](node/sitemap-entry.md)
- [DownloadResult](node/download-result.md)

<a id="enum"></a>

## Enum declarations

| Symbol | Members | Purpose |
| --- | --- | --- |
| FieldType | 4 | Select string, integer, float or boolean validation. |

- [FieldType](node/field-type.md)

<a id="function"></a>

## Function declarations

| Symbol | Members | Purpose |
| --- | --- | --- |
| sitemapUrlForOrigin | 1 | Return the canonical `/sitemap.xml` URL for an origin. |

- [sitemapUrlForOrigin](node/sitemap-url-for-origin.md)
