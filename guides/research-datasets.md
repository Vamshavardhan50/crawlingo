# Research and directory datasets

Collect a directory, choose a reproducible sample, extract documents, and produce records that can be traced back to their sources.

<a id="purpose"></a>

## Start with the dataset you need

Crawlingo turns known web pages into local records. For research, those records might describe technical proposals, public institutions, publications, projects or reference documents. Define one record before choosing how many pages to fetch.

This walkthrough uses the public Python Enhancement Proposal directory: one index page produces many directory rows, then a small selection of document pages produces one record per document. This is technical-document research data; it is not a representative sample of journals, authenticated databases or JavaScript applications.

The PEP site also publishes a metadata API. Prefer a maintained API or bulk download when it already supplies the data you need. This HTML example evaluates the extraction path; Crawlingo DatasetJob does not currently map arbitrary JSON API responses or PDF files into rows.

- [Python PEP directory and metadata API](https://peps.python.org/)
- [Complete runnable example](https://github.com/Vamshavardhan50/crawlingo/tree/main/examples/research_dataset)

<a id="model"></a>

## Two related datasets

Keep these datasets separate. A directory may be complete while only 20 of its documents have been collected. Directory coverage and document coverage need different denominators.

| Dataset | One record means | Identity | Useful fields |
| --- | --- | --- | --- |
| Directory | One listed proposal | PEP number or canonical URL | Title, classification, source URL |
| Documents | One sampled proposal page | Canonical document URL | Title, extracted document text, source URL |
| Derived analysis | A cleaned record, section or text chunk | Document ID + transformation version + chunk index | Normalized text, parent ID, source citation |

<a id="prepare"></a>

## 1. Build the CLI and inspect the source

### From the Crawlingo repository root

```shell
cargo build --locked --release --features cli --bin crawlingo
# Review examples/research_dataset/index-job.json and session.json first.
```

- Use a matching source build for 1.0.0-alpha.2; older registry packages may not contain DatasetJob.
- The example session identifies the collector and uses one request per second per host. Concurrency is two.
- Robots checks stay enabled. A missing robots.txt is not a content license; record the source’s applicable reuse terms separately.
- Run commands from the repository root: configuration paths are relative to the working directory, not the JSON file.

- [Native build prerequisites](installation.md)

<a id="directory"></a>

## 2. Extract the directory

The JSON above explains the extraction fields; use the complete index-job.json from the example for execution. A row is scoped to a table row, so fields from neighboring proposals cannot be combined accidentally.

Numbered table columns keep proposal IDs, titles and version links separate. :nth-child(N) accepts positive integer element positions; odd/even and an+b expressions are not supported. Attribute extraction returns the first matching attribute. The preparation script converts the ID to an integer and resolves the relative URL against _source_url.

Inspect work/research/index/report.json. Exit code 0 indicates a clean run; 2 indicates failed pages, rejected rows or cancellation; 1 indicates a fatal error. complete=true alone does not mean the dataset has no errors.

### Run the supplied directory job

```shell
cargo run --locked --release --features cli -- --config examples/research_dataset/session.json --rate-limit-rps 1 dataset examples/research_dataset/index-job.json
```

### How a row is defined

```json
{
  "row_selector": ".pep-zero-table tbody tr",
  "fields": [
    {
      "name": "pep_number",
      "selector": "td:nth-child(2)",
      "data_type": "integer",
      "required": true
    },
    {
      "name": "title",
      "selector": "td:nth-child(3)",
      "required": true
    },
    {
      "name": "relative_url",
      "selector": "td:nth-child(2) a.pep",
      "extract_type": "attr:href",
      "required": true
    },
    {
      "name": "classification",
      "selector": "td:nth-child(1) abbr",
      "extract_type": "attr:title",
      "required": true
    }
  ],
  "dedup_fields": [
    "relative_url"
  ]
}
```

<a id="sample"></a>

## 3. Freeze a small document sample

The script reads only committed data shards, checks the directory report, converts proposal numbers to integers, resolves links to the expected HTTPS origin, and writes work/research/directory.jsonl and work/research/detail-urls.txt. It makes no network calls.

Selection follows directory order. This is a reproducible convenience sample, not a random or representative research sample. Use your own documented sampling method for statistical research. The script caps its demonstration sample at 50 documents.

The script refuses to replace already prepared files. Keep the URL file unchanged when resuming. A later collection date, different sample or selector change needs a fresh output directory and a separate run identity.

### Normalize directory rows and select 20 documents

```shell
python examples/research_dataset/prepare_details.py --limit 20
```

<a id="documents"></a>

## 4. Collect the document records

The detail schema extracts .page-title, the canonical link attribute and #pep-content. It requires a title, URL and at least 100 characters of content. Deduplication uses canonical_url. Required values and length checks catch empty shells; they do not establish scientific accuracy or completeness.

The document selector includes headings, navigation within the article, metadata and body content. It is extracted text, not an automatically cleaned abstract. The text-order fix preserves mixed inline content; the custom parser is still not a browser-complete HTML implementation. HTML entities and non-UTF-8 inputs need explicit review.

Use a known article section selector when you need an abstract or main body. The current selector implementation supports a limited subset; do not assume browser pseudo-classes, full XPath, PDF text or structured author arrays work.

### Run the bounded detail job

```shell
cargo run --locked --release --features cli -- --config examples/research_dataset/session.json --rate-limit-rps 1 dataset examples/research_dataset/detail-job.json
```

- [Selectors and supported syntax](selectors.md)
- [Dataset schema](schema.md)

<a id="quality"></a>

## 5. Check quality before analysis

- Reconcile exported row counts with records_written. Read part-XXXXXXXX.errors.jsonl separately from data.
- Check stable IDs, required values, expected languages and unexpected redirects or login pages.
- Review text around links, emphasis, punctuation, lists and tables; a nonempty field can still be wrong.
- Measure accepted records divided by attempted candidate records, plus failed-page rate and duplicate rate. Document the denominator.
- Record source coverage and sample selection. Twenty correct documents do not prove completeness of the directory or all linked documents.
- Keep raw shards unchanged. Decode entities and apply cleaning in a derived dataset with a versioned transformation.

- [Design and validate a dataset](dataset-design.md)
- [Measured public-site results](real-world-results.md)

<a id="provenance"></a>

## 6. Keep an evidence manifest

DatasetJob writes _source_url and _row. _row is an extraction position, not a permanent document ID. A canonical URL is an explicitly extracted field; it is not automatically supplied by the job.

Your runner should also store collection start/end times, engine revision, URL-file hash, schema hash, session settings without secrets, source/reuse notes, and data-shard hashes. Current checkpoints do not include the engine revision or complete Session settings. Pin both when resuming a partially completed job.

Store the original and derived dataset versions together. For retrieval or AI workflows, cite the source document and keep each chunk connected to its parent record. Embedding, train/test splitting, near-duplicate detection and factual validation belong in downstream processing.

- [pandas, DuckDB and derived datasets](integrations-data.md)

<a id="repeat"></a>

## 7. Resume, then plan the next collection

Repeat the same command to resume the same job. Completed batches are not refreshed. A finished job is idempotent; running it again does not create a new snapshot of the website.

For a new collection date, keep the old output and create new input/configuration files pointing to a fresh output directory. Compare records using stable IDs. The generic watch API is separate from this snapshot workflow.

For large research runs, discover and freeze URLs first, pilot the schema, then partition approved sources into independent jobs with distinct directories. Per-session rate limits do not coordinate multiple processes. A scheduler must enforce each source’s combined request budget.

- [Recovery semantics](recovery.md)
- [Worker orchestration](integrations-workers.md)
