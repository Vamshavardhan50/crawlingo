# Design and validate a dataset

Define records, identities, quality gates and capacity before scaling a collection.

> Current development source. New dataset APIs require a matching source build; published alpha packages may differ.

## Write a record contract first

A useful dataset has an explicit unit of observation. “One million data” can mean a million pages, directory rows, documents or text chunks. These require different workloads and cannot share a single requests-per-second claim.

| Decision | Research example | Why it matters |
| --- | --- | --- |
| Record unit | One proposal or organization | Prevents counting both cards and detail pages as new entities |
| Stable identity | Canonical URL or source-issued ID | Supports deduplication and later comparisons |
| Required fields | ID, title, source URL | Rejects unusable rows instead of filling them silently |
| Optional fields | Published date, category | Represents missing data without invented values |
| Coverage | All directory rows; first 20 documents | Makes the sampling boundary visible |
| Update policy | Monthly immutable snapshots | Separates resume from refreshing stale records |

## Use the right collection path

| Code path | Use it for | Current boundary |
| --- | --- | --- |
| DatasetJob | URL file → validated, deduplicated JSONL/CSV shards | Single writer, local state and bounded checkpoint batches |
| Crawler / Dataset builder | Small discovery jobs and interactive extraction | Returned collections can grow in memory; not the million-record export path |
| DatasetStream (Rust) | Your own producer feeding a bounded export queue | Advanced streaming API; not DatasetJob checkpoint semantics |
| Playwright / Puppeteer | Discover links or inspect JavaScript-rendered content | Rendered DOM is not passed automatically into Python/Node DatasetJob |
| Source API or bulk data | Already structured research metadata | Use its own ingestion client; HTML selectors do not parse API JSON |

- [Architecture](https://crawlingo-docs.vercel.app/architecture)
- [Browser integration boundaries](https://crawlingo-docs.vercel.app/integrations/playwright)

## Choose selectors and quality rules

Select a repeated container with row_selector for a directory page; omit it for one record per document. Each field is resolved within the record’s container. Make identifiers required before enabling deduplication on them.

data_type validates and normalizes string values; integer, float and boolean values are still exported as strings. Convert types explicitly in pandas, SQL or your application. Defaults should encode an intentional value, not hide a missing required identifier.

Use normalize_whitespace=true for prose metadata. Disable it for content where whitespace matters, but verify extraction: this is not a lossless HTML or source-code archive. Avoid broad body selectors that collect menus and footer text.

### Example identifier and title fields

```json
{
  "fields": [
    {
      "name": "source_id",
      "selector": ".record",
      "extract_type": "attr:data-id",
      "required": true
    },
    {
      "name": "title",
      "selector": ".record-title",
      "required": true,
      "min_length": 5
    }
  ],
  "dedup_fields": [
    "source_id"
  ]
}
```

- [JobConfig field reference](https://crawlingo-docs.vercel.app/reference/job-config)

## Deduplication is an identity decision

With no dedup_fields, the job compares all extracted field values, excluding _source_url and _row. With dedup_fields, it compares only those fields. This is exact comparison, not fuzzy matching or semantic deduplication.

A title alone is rarely a safe key: unrelated publications can share a title. A URL alone may also represent a document that changes over time. Choose a source ID for a snapshot and add a version or collection identity in your derived datasets when preserving revisions.

Within one job, the first accepted value for an exact key wins. No later row merges or updates it. Across independently partitioned jobs, deduplication is separate; reconcile global identity downstream.

## Establish a measurable pilot

- Select a small set of different page templates, long documents and missing-field cases.
- Start with concurrency 2 and a source-approved host rate. Increase only after looking at latency, errors and resource use.
- Manually label a sample of expected fields and compare actual extraction, not just record counts.
- Keep failure records and rate-limit responses visible. A partial dataset can be useful only when its missing coverage is recorded.
- Repeat the unchanged job and compare shard hashes to check restart idempotence. Separately test interrupted-job recovery in a controlled environment.

## Estimate one million records honestly

Start with pages_needed = target_records / accepted_unique_records_per_page. Then estimate collection time using the slower of the permitted request rate and the measured pipeline rate, with additional time for robots requests, retries, parsing, export and discovery.

At an allowed one page per second, one million single-record pages need at least 11.6 days of request time. At 20 accepted records per page, the same target needs 50,000 pages: at least 13.9 hours. These are arithmetic examples, not measured Crawlingo performance or permission to use those rates.

Output storage is approximately record_count × average serialized record size, plus error shards, exact-dedup state, temporary shards and derived datasets. One million 2 KiB records alone are about 1.91 GiB. Full documents can be far larger.

Measure a representative pilot on the intended source. A local million-record fixture proves a particular workload completes; it does not establish a universal public-web throughput or memory ceiling.

- [Measurements and methodology](https://crawlingo-docs.vercel.app/real-world-results)
- [Local scale acceptance](https://crawlingo-docs.vercel.app/benchmarks)

## Understand what the resource limits bound

Keep checkpoint batches smaller for long documents and large keys. The engine preserves input order when processing responses, so one slow page can delay later completed pages. Parser workers, network buffers, Sled, dedup state and output buffering all contribute to memory.

| Setting | Controls | Does not guarantee |
| --- | --- | --- |
| concurrency | In-flight page processing for one job | An allowed website rate or total process memory |
| Session rate_limit_rps | Request pacing per host in one session | Coordination across workers or machines |
| checkpoint_pages | Pages represented in a checkpoint batch | A fixed number of records when pages have different row counts |
| checkpoint_bytes | Pending exact-dedup key budget | A total RSS cap; exceeding it is a fatal job error |
| max_response_bytes | Job acceptance of a fetched response body | A job-specific streaming download cap; this check is after fetch |
| max_records_per_page / max_record_bytes | Extraction and serialization acceptance limits | Automatic truncation into a valid smaller document |

## Define a publication gate

Your application decides when a dataset is ready to publish. A useful default is: complete=true, cancelled=false, no unreviewed page failures or rejected records, expected coverage, validated identity, a manual quality sample and stable artifact hashes.

Never have two workers write one output directory. Treat local job state as recovery state, and publish immutable data shards with a manifest only after review. If some source failures are acceptable, record an explicit threshold and the missing population instead of silently marking the run successful.

- [Reports and error records](https://crawlingo-docs.vercel.app/report-reference)
- [Production operations](https://crawlingo-docs.vercel.app/production-guide)
