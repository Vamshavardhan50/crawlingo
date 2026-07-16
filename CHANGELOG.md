# Changelog

All notable changes to Crawlingo will be documented in this file.

## [1.0.0-beta.1] - 2026-07-15

### Added (M4)

- **FFI Pagination Bindings**: Added `PaginationConfig` (`PyPaginationConfig` / `JsPaginationConfig`) to configure pagination (next link CSS selectors, numbered page URLs, or regex page-number incrementation) during crawls.
- **Dataset Schema Bindings**: Added `DatasetSchema`, `FieldType`, and `FieldConstraint` wrappers in Python and NodeJS FFI to perform validation (required status, data type validation for String/Integer/Float/Boolean) on extracted dataset fields.
- **Sitemap Gzip Decompression**: Enabled automated detection (magic bytes `[0x1F, 0x8B]` or `.xml.gz`/`.gz` URL suffix) and decompression of gzipped sitemaps.
- **NodeJS Bindings completeness**: Defined and stubbed all corresponding TypeScript classes and definitions.

### Breaking Changes (M3)

- **Field Removal in FetchRequest (M3)**: The `rate_limit_rps` field has been removed from `FetchRequest`. Rate limiting is now configured at the `Session` level and managed dynamically by the `FetchManager` and its shared `HostRateLimiter` instances, rather than being passed on individual requests.
- **Type Alias for ExtractionRule (M3)**: `ExtractionRule` has been redefined as a type alias for `DatasetField` (`pub type ExtractionRule = crate::dataset::builder::DatasetField;`) to eliminate duplication between the extraction rule layer and the dataset field definition layer.
- **New Environment Variables (M3)**: Added 6 new environment variables for configuring client pool parameters and retry policies directly from the environment.

### Added (M2)

- **PersistentFrontier Queue B-Tree Optimization**: Re-engineered the persistent database frontier to use LIFO B-tree structures (`pending:{id:016}`) achieving $O(1)$ enqueue and $O(\log n)$ dequeue times.
- **Non-blocking Fingerprint Writes**: Deferred durability flushes in `FingerprintStore` to background tasks and session cleanup drop hooks, avoiding blocking on every write.
- **Batched Parquet Streams**: Serialized dataset records in batched chunks (default 1000 records) to boost I/O throughput.
- **Pre-compiled Regex in Pagination**: Pre-compiled regex patterns once at config instantiation to eliminate compilation overhead during crawls.

### Added (M1)

- **Utility Module Set**: Added high-performance utility helpers for MD5/SHA256 hashing, base64 encoding/decoding, domain extraction, and URL normalization.
- **Test Coverage Expansion**: Extended unit and integration tests coverage to reach 100% core coverage.

### Added (M0)

- **Self-Healing Auto-Match Selector Recovery**: Added Jaro-Winkler/Levenshtein string metric similarity scoring to automatically recover broken selectors.
- **Transform Hook Pipelines**: Added support for text post-processing transform hooks (uppercase, lowercase, whitespace trimming).
