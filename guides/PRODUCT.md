# Crawlingo: purpose and workflow

## Product definition

**Crawlingo is a Rust-powered dataset generator for developers.** It converts a
URL file and an explicit record schema into validated, deduplicated JSONL or CSV
shards. Python, Node.js and Rust applications control the same native engine.

The primary user is a developer or data engineer who needs a repeatable collection
job. A new user can follow the supplied local example; adapting a website's
selectors and deploying a worker require technical work.

## The first useful outcome

Run the [product catalog fixture](../examples/product_dataset/README.md). It produces
two accepted records, one rejected row, one duplicate record and one duplicate URL.
Rerun it without changing the configuration to check that committed work resumes.

This is the recommended onboarding path because the input and expected output are
available in the repository. Public websites can change independently of Crawlingo.

## The workflow

1. **Choose the dataset.** Define the records you want: products, articles or another
   consistent entity. Decide what makes a record valid and what makes it unique.
2. **Prepare the input.** Save one HTTP(S) URL per line. Discovery and collection
   are separate steps; the bulk job consumes the list you give it.
3. **Define the fields.** Select a row container if a page contains multiple records.
   Add required fields, extraction types and identity fields.
4. **Run a small job.** Start with representative pages, including missing values
   and duplicates. Configure a session for source-specific pacing and headers.
5. **Review quality.** Read accepted records, error shards and report counters.
   Completion alone does not mean all rows were accepted.
6. **Scale the same contract.** Increase the workload after checking output quality.
   Keep one writer per output directory, persistent storage and explicit limits.
7. **Consume incrementally.** Load committed shards into your own analysis, database,
   search index or downstream processing system.

## What belongs in the application

The caller supplies scheduling, job ownership, credentials, storage lifecycle and
downstream ingestion. An HTTP endpoint can enqueue a worker job; the SDK itself
does not create a hosted API. Preserve checkpoints and data shards together.

For knowledge or model datasets, add your own text cleanup, chunking, quality review,
licensing checks and downstream transformation. Crawlingo currently extracts fields;
it does not automatically produce a finished training corpus or search index.

## Current contract

- Job exports: JSONL and CSV; field values remain strings after validation.
- Exact record identity: declared dedup fields, or all extracted fields by default.
- Recovery: local checkpoint state, one writer, replay of an unfinished batch.
- Large-job API: `DatasetJob`; Page, Dataset and Crawl can retain results in memory.
- Optional capabilities: transport profiles, browser builds, selector recovery and
  monitoring have separate availability and behavior. DatasetJob uses explicit selectors.
- Evidence: the million-page local acceptance is documented in
  [SCALE_RESULTS.md](../SCALE_RESULTS.md), with workload and remaining release gates.

## Product communication

Use this short description in repository metadata and package summaries:

> Generate validated datasets from web pages with a Rust engine, Python and Node.js
> SDKs, exact deduplication, and resumable JSONL/CSV exports.

Feature claims should identify the relevant API, implementation status and evidence.
Published alpha packages, the current source checkout and the hosted docs may be at
different revisions. Release them deliberately and state which version a guide uses.

## Next steps

- [Guided first run](first-run.md)
- [Developer tutorial](developer-tutorial.md)
- [DatasetJob API](api-dataset-job.md)
- [Worker deployment](deployment.md)
- [Production limits and operating guidance](production-guide.md)
