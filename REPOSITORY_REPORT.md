# Crawlingo repository capability report

**Report date:** 19 October 2026  
**Engine and SDK revision:** `b258d6236c2deacd6ed5f8ed463000548e7e9358`  
**Documentation revision:** `dd8dc3c551c6a59644d804d69ca4c295795e1532`  
**Prepared source version:** Rust/npm `1.0.0-alpha.2`; Python `1.0.0a2`

This report reviews source code, existing measurement artifacts, documentation
and the GitHub checks for these revisions. No new scraping run, benchmark or test
suite was executed to prepare this report. Older measurements are dated below.

## 1. Executive assessment

**Crawlingo is a dataset-generation engine for developers.** Its central workflow
takes a file of web-page URLs and a field schema, extracts records, applies
validation and exact deduplication, and writes resumable JSONL or CSV files.
Rust runs the pipeline; Python, Node.js/TypeScript and a CLI provide entry points.

Its strongest demonstrated use case is **structured research and directory
collection from known HTML pages**. It can also collect compatible product
catalogs, document metadata and other repeated records when a developer supplies
the correct selectors.

**Assessment:** suitable for controlled development and collection pilots using
the current source. A general production release remains premature: the main CI
gate is failing, the prepared dataset SDK release is not published, and important
scale, parser and failure-recovery cases remain unproven.

The implementation provides useful collection infrastructure. Applications still
define the dataset, discover sources, assess extraction quality, schedule runs,
manage storage and perform analysis.

## 2. What the main dataset workflow can do

“Demonstrated” below means supported by the recorded focused checks or sample
runs; it does not mean certified for every website or deployment.

| Capability | What it does | Status and boundary |
| --- | --- | --- |
| URL-file input | Reads one HTTP(S) URL per line without collecting the entire input into an SDK array | Implemented; a full streaming input hash is computed for resume validation |
| Concurrent fetching | Uses the shared Rust HTTP/session machinery to process pages concurrently | Demonstrated; concurrency is not a website request-rate allowance |
| One record per page | Extracts named fields from a document | Demonstrated on 20 public document pages |
| Multiple records per page | Uses `row_selector` to scope independent records within a page | Demonstrated on a 744-row public directory |
| Field extraction | Extracts text or attributes, with additional price/date/URL normalization modes | Implemented; normalization is limited and source-dependent |
| Explicit selectors | Supports a subset of CSS, XPath, regex and text-anchor selectors | Implemented; not complete browser CSS or full XPath |
| Table columns | Supports positive-integer `:nth-child(N)` positions | Demonstrated; `odd`, `even` and `an+b` are unsupported |
| Validation | Required values, defaults, whitespace normalization, minimum lengths, integer/float/boolean checks | Demonstrated; successful validation does not establish semantic correctness |
| Exact deduplication | Deduplicates URLs and records using selected fields or all extracted values | Implemented with persistent local state; no fuzzy or semantic deduplication |
| JSONL/CSV export | Writes numbered data shards and separate error shards | Demonstrated; field values remain strings after type validation |
| Reports | Returns counts for pages, records, failures, rejections, duplicates and completion | Demonstrated; a completed job may still contain failures |
| Cancellation and resume | Reuses committed checkpoints and replays unfinished work | Focused cancellation checks and completed-job restarts passed; hard-crash recovery coverage is incomplete |
| Resource controls | Limits concurrent work, checkpoint size, accepted record size and records per page | Implemented; these are not a guaranteed total-memory ceiling |

The relevant implementation is [DatasetJob](src/dataset/job.rs), with
[Python](sdk/python/crawlingo/dataset_job.py) and
[Node.js](sdk/nodejs/src/dataset-job.ts) wrappers. See the
[API reference](guides/api-dataset-job.md) for signatures and return values.

### What a run produces

```text
URL file + field schema + Session settings
                    |
                    v
fetch -> extract -> validate -> exact deduplication
                    |
                    v
output/
  part-00000000.jsonl         accepted records; CSV is also supported
  part-00000000.errors.jsonl  failed-page and rejected-row information
  report.json                outcome counters
  state/                     checkpoint and deduplication state
```

Accepted records include `_source_url` and `_row`. The row position is not a
stable entity ID. A canonical URL, DOI or source-issued identifier must be
explicitly extracted if the dataset needs it. Collection dates, source licenses,
engine revision and transformation versions need a caller-managed manifest.

## 3. Other capabilities in the repository

These are supporting APIs with different availability and evidence from the bulk
dataset job. Their presence in source does not establish universal SDK parity.

| Component | Available functionality | Practical limitation |
| --- | --- | --- |
| Page/element APIs | Fetch pages; inspect text, attributes, links, tables, images and other document information | Custom parser has HTML, entity, encoding and Markdown-fidelity limits |
| Crawler/frontier | Follow links, manage crawl depth/page limits and persist frontier state | Returned URLs, records and errors can accumulate in memory; use bounded discovery workloads |
| Sitemaps/pagination | Discover URLs from supported sitemap or pagination patterns | Not a complete durable discovery service feeding millions of URLs automatically |
| Session/transport | Headers, cookies, proxies, connection reuse, timeouts, retries and per-host pacing | Separate sessions/processes do not share one global rate budget |
| Robots handling | Fetch and apply robots rules, including supported crawl delays | Keep enabled for public dataset jobs; source access/reuse policy is a separate decision |
| TLS/browser profiles | Configures browser-like HTTP/TLS emulation through the transport dependency | No demonstrated guarantee of access to protected sites or challenge avoidance |
| Native browser tier | Optional Chromium rendering through the `browser` Cargo feature | Requires an appropriate source build and Chrome/Chromium; different resource profile from plain HTTP |
| Selector recovery | Heuristically matches saved DOM fingerprints in the older Dataset builder | Can select the wrong element; not automatically applied to DatasetJob |
| Change detection/watch | Compares extracted values and supports polling/callback workflows in relevant APIs | Separate from immutable dataset snapshots; not a hosted monitoring service |
| Rust DatasetStream | Bounded producer/consumer records and CSV/Parquet export | Lower-level stream API; checkpointed Parquet DatasetJob is not implemented |
| Auth/cache/metrics layers | Rust authentication middleware, optional HTTP caching and request metrics | Advanced integration surfaces; configuration and binding support vary |
| Download API | Downloads files with supported size/resume controls | Downloading a PDF does not extract its text into dataset records |
| Python MCP module | Contains a local HTTP/SSE tool-server implementation | Treat as experimental; this review does not establish protocol interoperability or service hardening |

Source areas: [engine](src/engine), [crawl](src/crawl),
[parser](src/parser), [matcher](src/matcher), [watch](src/watch),
[stream exports](src/dataset/stream.rs), and
[Python MCP module](sdk/python/crawlingo/mcp.py).

## 4. What users can build with it

| Intended outcome | Crawlingo's role | Work supplied by the application |
| --- | --- | --- |
| Research directory | Collect IDs, titles, categories and links from repeated rows | Choose sources, resolve identity and measure coverage |
| Document collection | Extract selected page text and metadata with source URLs | Clean text, review fidelity, handle PDFs and preserve provenance |
| Organization/project catalog | Extract records from compatible directory templates | Normalize names/categories and reconcile entities across sources |
| Product dataset | Collect identifiers, names, prices and other declared fields | Account for variants, currencies, missing values and update policy |
| Scheduled snapshots | Repeat independently named collection jobs | Scheduler, fresh output directories, retention and comparison logic |
| Search/RAG input | Produce source-linked records for downstream ingestion | Chunking, embeddings, indexing, retrieval and quality evaluation |
| Training-data preparation | Supply a raw structured collection stage | Dataset eligibility, cleaning, near-duplicate removal, splits and final validation |

The included [research example](examples/research_dataset/README.md) is the most
relevant starting point for the requested direction. A
[local product fixture](examples/product_dataset/README.md) provides a smaller
deterministic example with known accepted, duplicate and rejected rows.

## 5. Integrations and developer experience

### Entry points

| Interface | Dataset invocation | Packaging position |
| --- | --- | --- |
| Rust | `DatasetJob::new(config, session)?.run().await?` | Current source crate |
| Python | `DatasetJob(config).run()` or `await job.run_async()` | Native source extension; prepared version `1.0.0a2` |
| Node.js/TypeScript | `await new DatasetJob(config).run()` | Native addon and TypeScript wrapper; Node 22+ source target |
| CLI | `crawlingo dataset job.json` | Build with `--features cli` |

The SDK returns a report while Rust writes records to disk. Callers do not need
to receive a million-record Python list or JavaScript array.

### Tool composition

- **Playwright/Puppeteer:** documented link-discovery workflows can write the URL
  file. Browser state and rendered HTML are not automatically transferred into
  Python/Node DatasetJob.
- **pandas/DuckDB:** consume JSONL/CSV shards, convert strings to desired types,
  analyze data and produce derived Parquet datasets.
- **FastAPI/Express:** application recipes put SDK jobs behind a worker queue;
  Crawlingo does not ship a managed REST dataset service.
- **Containers/schedulers:** run the CLI or SDK with persistent job storage and
  one owner per output directory.

See [integration guides](guides/integrations.md). These are composition recipes,
not a catalog of bundled third-party connectors.

The separate documentation repository includes a landing page, API reference,
research guides, code-copy controls, search and collapsible navigation. Its last
local check covered **50 guides and 121 code examples**; a production build
generated 55 pages. Content/build checks do not establish that every displayed
example has run on every platform.

## 6. Measured performance and correctness

### Public website: 19 October 2026

| Workload | Pages | Records | Job duration | Peak CLI working set |
| --- | ---: | ---: | ---: | ---: |
| Python PEP numerical directory | 1 | 744 | 4.620 seconds | 42.0 MiB |
| First 20 linked PEP documents | 20 | 20 | 20.539 seconds | 38.9 MiB |

Conditions: Windows 11, eight logical CPUs, release-mode Rust CLI, concurrency
two, one request per second per host, robots checks enabled. Both jobs reported
zero page failures and rejected records. Repeating completed jobs preserved all
data/error shard hashes.

All 744 directory IDs, titles and classifications matched an independent source
comparison after correcting table-column selection. PEP 1's text character order
matched an independent parser after entity decoding and ignoring whitespace.
The other 19 document bodies were not semantically certified.

The directory and document outputs overlap in entity identity; this is not a
claim of 764 distinct research entities. These deliberately paced samples do not
measure maximum throughput. See [the report](PUBLIC_WEB_RESULTS.md) and
[machine-readable evidence](measurements/2026-10-19-peps.json).

### Local scale fixture: 14 October 2026

- 1,000,000 distinct local HTTP pages produced 1,000,000 records.
- Concurrency 50; 1,000 output shards; zero failed pages or rejected records.
- All one million records were checked against expected values.
- Job duration: 1,094.569 seconds, approximately 18.24 minutes.
- Peak working set: 181.5 MiB **including the fixture server and verification**.
- Completed-job resume passed; robots checks were disabled for that local fixture.

This older run predates subsequent parser and other fixes. It has not been rerun
on the current source and does not demonstrate one million public-web pages.
Its memory scope differs from the public CLI samples, so those memory numbers
are not directly comparable. See [SCALE_RESULTS.md](SCALE_RESULTS.md).

### Review of earlier feature claims

| Earlier claim | Evidence-supported statement |
| --- | --- |
| Approximately 3,500 requests/second | Not established by the recorded workloads |
| 2.4 MB idle memory | No supporting current idle-memory measurement |
| Self-healing selectors | Heuristic recovery exists in a separate builder path; no universal guarantee |
| Stealth TLS fingerprinting | Transport emulation profiles exist; success across protected sites is unproven |
| Built-in change monitoring | Supporting watch/change APIs exist; independent of bulk dataset refresh |
| SIMD acceleration | Text search uses `memchr`; some matching/change work uses Rayon parallelism. No isolated end-to-end speedup measurement |
| Faster than Scrapy/Playwright | No controlled equivalent-workload comparison supports this ranking |

## 7. Release, CI and hosting readiness

### Latest observed GitHub checks for the reviewed commits

| Area | Result |
| --- | --- |
| Version consistency | Passed |
| Python source checks | Passed: Python 3.12 and 3.13 on Linux and Windows |
| Node source checks | Passed: Node 22/24 on Linux and Node 22 on Windows |
| Rust jobs | Failed at `cargo clippy --all-targets -- -D warnings` on Linux, Windows and macOS |
| Dependency audit | `cargo audit` step failed; advisory details/root cause were not evaluated in this report |
| Overall engine CI | Failed |
| Documentation CI | Passed |
| Documentation deployment workflow | Failed at “Pull Vercel environment”; lint/type check and build passed |
| Engine repository GitHub Pages | Failed; previous inspection identified the Jekyll build step |

Sources: [engine CI](https://github.com/Vamshavardhan50/crawlingo/actions/runs/39234036480),
[docs CI](https://github.com/Vamshavardhan50/crawlingo-docs/actions/runs/39234061774),
[docs deployment](https://github.com/Vamshavardhan50/crawlingo-docs/actions/runs/39234061725),
and [Pages run](https://github.com/Vamshavardhan50/crawlingo/actions/runs/39234036215).

The existing focused local evidence contains 19 passing parser, selector,
dataset-job and edge-case tests. These results and the successful SDK jobs do
not override the failing overall release gate.

### Package availability

The repository identifies alpha.2 as **prepared but unpublished**. Use a matching
source build for the documented DatasetJob workflow. The separate package audit
dated 18 October recorded older registry versions and incomplete native-platform
coverage; those registry inventories were not refreshed for this report.

Successful source CI jobs are useful evidence, but they do not prove that released
wheels/addons exist, install cleanly or contain the same APIs. See
[release status](guides/RELEASE_STATUS.md).

The repository also contains contribution/support/security documents, issue/PR
templates, CODEOWNERS, dependency-update configuration and release workflows.
Their presence does not prove branch protection, release secrets or every GitHub
setting has been configured. See [repository setup](.github/REPOSITORY_SETUP.md).

## 8. Important limits for a large research dataset

1. **Extraction needs explicit rules.** Crawlingo does not automatically infer a
   trustworthy schema, distinguish an abstract from navigation, or validate facts.
2. **The job is single-writer and local.** It is not a distributed scheduler or
   globally coordinated deduplication system.
3. **Some APIs collect results in memory.** Use DatasetJob for the bulk export
   path; do not assume Crawler or the older Dataset builder has the same bounds.
4. **Memory limits have different scopes.** Standard HTTP transport defaults to
   a 16 MiB streaming body cap; the job's default 2 MiB acceptance check happens
   after fetch. Neither setting is a total process-memory limit.
5. **Output is string-valued JSONL/CSV.** Typed JSON and checkpointed Parquet jobs
   remain unimplemented. A separate Rust stream exporter supports Parquet.
6. **Resume is not refresh.** A finished job will not refetch current website data.
   New snapshots need new directories. Pin the engine and Session externally:
   checkpoints validate input/JobConfig, not the full execution environment.
7. **Text fidelity needs review.** Mixed inline order was fixed, but HTML entities,
   non-UTF-8 inputs, malformed markup and Markdown conversion need broader work.
8. **Recovery evidence is incomplete.** Controlled cancellation and finished-job
   restart do not prove process-kill, disk-full or corrupted-state recovery.
9. **No automatic PDF/API ingestion.** Download and HTTP primitives do not amount
   to arbitrary PDF parsing or JSON research-API-to-record mapping.
10. **No finished AI dataset service.** Cleaning, chunking, embeddings, semantic
    deduplication, dataset splits and downstream storage remain application work.

For one million records, estimate pages from **accepted unique records per page**.
At one allowed page/second, a million single-record pages require at least 11.6
days of request time. At 20 accepted records/page, 50,000 pages require at least
13.9 hours. These are arithmetic planning examples before overhead, not measured
throughput guarantees. See [dataset design](guides/dataset-design.md).

## 9. Recommended priorities

These priorities are an assessment from the reviewed implementation and checks.

| Priority | Work | Completion evidence |
| --- | --- | --- |
| 1 | Resolve Clippy, dependency-audit and deployment failures | Required CI and hosting jobs pass on the same reviewed revision |
| 2 | Complete SDK artifact/release validation | Fresh consumer installs run the dataset example on each advertised platform |
| 3 | Make configuration errors fatal; record source/schema/engine/session provenance | Invalid configuration cannot silently change collection behavior; reproducible manifest accompanies output |
| 4 | Expand parser and extraction fidelity | Representative research HTML fixtures with independently checked fields/text |
| 5 | Strengthen interrupted-job recovery | Process-kill, disk-full and state-damage scenarios have explicit tested outcomes |
| 6 | Revalidate scale on the current engine | Current-revision local million-record run plus modest representative public-source pilots |
| 7 | Build durable discovery and worker coordination | Bounded URL production, partition ownership, shared pacing and cross-job identity rules |

## 10. How to use the repository now

1. Build the source after following the [installation guide](https://crawlingo-docs.vercel.app/installation).
2. Run the [local fixture](examples/product_dataset/README.md) to understand the output contract.
3. Follow the [research example](examples/research_dataset/README.md) for directory-to-document collection.
4. Review counts, error shards, identity and manually sampled fields before enlarging the job.
5. Consume records incrementally through the [data-tool integrations](guides/integrations-data.md).

**Product description to use:**

> Crawlingo is a Rust dataset-generation engine with Python and Node.js SDKs.
> It converts known web pages into validated, exactly deduplicated JSONL or CSV
> records with local checkpoints and resumable processing.

This description matches the repository's central implemented workflow without
claiming universal website support or an unmeasured speed advantage.
