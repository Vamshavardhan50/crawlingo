# Python API

Browse every package export and its declared methods, fields, parameters, returns, source links and known limitations.

<a id="scope"></a>

## What this reference covers

30 package exports; 39 documented symbols including reachable types and helpers; 198 explicitly declared members. Inherited standard-library members are not included in this count.

These are source inventory counts, not a percentage of working features. Classes with broken or limited wrapper paths are marked in their member entries.

- [Implementation audit](implementation-status.md)

<a id="class"></a>

## Classes and result objects

| Symbol | Members | Purpose |
| --- | --- | --- |
| CrawlResults | 9 | Python collection of per-page extraction results. |
| Crawl | 16 | Discover linked pages and extract a small in-memory result set. |
| Dataset | 12 | Extract named fields from one page, with optional validation and small-result exports. |
| DatasetResult | 8 | Read and export a single extracted field map. |
| DatasetJob | 4 | Process a URL file into bounded, validated, resumable dataset shards. |
| DownloadResult | 6 | Response metadata and bytes actually retained/written. Check status and resumed explicitly. |
| Downloader | 6 | Save a response to a local file or receive its bytes in memory. |
| Element | 11 | Read one selected DOM element and its attributes. |
| ElementCollection | 14 | Read, iterate and select from multiple DOM matches. |
| CrawlingoError | 0 | Base exception for Crawlingo. |
| FetchError | 0 | Raised when request fetching fails. |
| ParseError | 0 | Raised when parsing fails. |
| SelectorError | 0 | Raised when selection query syntax or evaluation fails. |
| AutoMatchFailed | 0 | Raised when DOM fingerprint auto matching fails. |
| TimeoutError | 0 | Raised when an operation times out. |
| RateLimitError | 0 | Raised when host request rate limit is hit. |
| ChangeDetectionError | 0 | Raised when change detection fails. |
| ExportError | 0 | Raised when exporting dataset to CSV/Parquet/JSON fails. |
| DnsError | 0 | Raised when DNS resolution or caching fails. |
| FingerprintStoreError | 0 | Raised when reading or writing to the fingerprint store fails. |
| Page | 16 | Fetch one page and inspect its parsed HTML before defining a dataset schema. |
| RequestContext | 3 |  |
| ResponseContext | 2 |  |
| PaginationConfig | 4 | Choose how a crawler discovers the next page. |
| FieldType | 4 | Select string, integer, float or boolean validation. |
| FieldConstraint | 4 | Describe a field name, its type and whether it is required. |
| DatasetSchema | 3 | Validate required fields and canonicalize string values in small Dataset results. |
| Session | 28 | Own shared request defaults, connection pools, per-host pacing and retry settings. |
| SitemapEntry | 6 | Sitemap URL metadata. lastmod and priority are source strings, not validated dates or numbers. |
| Sitemap | 11 | Discover URLs from a sitemap or use them to seed a bounded crawl. |
| Watch | 12 | Poll selected fields and deliver detected changes to application callbacks. |
| ChangeEvent | 8 | Python callback event with snake_case fields. change_type is content, price, stock, added or removed. |

- [CrawlResults](python/crawl-results.md)
- [Crawl](python/crawl.md)
- [Dataset](python/dataset.md)
- [DatasetResult](python/dataset-result.md)
- [DatasetJob](python/dataset-job.md)
- [DownloadResult](python/download-result.md)
- [Downloader](python/downloader.md)
- [Element](python/element.md)
- [ElementCollection](python/element-collection.md)
- [CrawlingoError](python/crawlingo-error.md)
- [FetchError](python/fetch-error.md)
- [ParseError](python/parse-error.md)
- [SelectorError](python/selector-error.md)
- [AutoMatchFailed](python/auto-match-failed.md)
- [TimeoutError](python/timeout-error.md)
- [RateLimitError](python/rate-limit-error.md)
- [ChangeDetectionError](python/change-detection-error.md)
- [ExportError](python/export-error.md)
- [DnsError](python/dns-error.md)
- [FingerprintStoreError](python/fingerprint-store-error.md)
- [Page](python/page.md)
- [RequestContext](python/request-context.md)
- [ResponseContext](python/response-context.md)
- [PaginationConfig](python/pagination-config.md)
- [FieldType](python/field-type.md)
- [FieldConstraint](python/field-constraint.md)
- [DatasetSchema](python/dataset-schema.md)
- [Session](python/session.md)
- [SitemapEntry](python/sitemap-entry.md)
- [Sitemap](python/sitemap.md)
- [Watch](python/watch.md)
- [ChangeEvent](python/change-event.md)

<a id="function"></a>

## Function declarations

| Symbol | Members | Purpose |
| --- | --- | --- |
| hooks.strip_whitespace | 1 | Strips leading and trailing whitespace from extracted content. |
| hooks.uppercase | 1 | Converts extracted content to uppercase. |
| hooks.lowercase | 1 | Converts extracted content to lowercase. |
| hooks.log_request | 1 | Simple logger hook that prints the target URL before fetching. |
| hooks.log_response | 1 | Simple logger hook that prints the HTTP status code after fetching. |
| sitemap_url_for_origin | 1 | Return the canonical ``/sitemap.xml`` URL for a given origin. |

- [hooks.strip_whitespace](python/hooks-strip-whitespace.md)
- [hooks.uppercase](python/hooks-uppercase.md)
- [hooks.lowercase](python/hooks-lowercase.md)
- [hooks.log_request](python/hooks-log-request.md)
- [hooks.log_response](python/hooks-log-response.md)
- [sitemap_url_for_origin](python/sitemap-url-for-origin.md)

<a id="module"></a>

## Module declarations

| Symbol | Members | Purpose |
| --- | --- | --- |
| hooks | 5 | Value-cleaning and logging helpers for Python Page hooks. |

- [hooks](python/hooks.md)
