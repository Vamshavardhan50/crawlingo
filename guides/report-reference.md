# Read reports and errors

Interpret every job counter, distinguish row and page failures, and decide what output is ready to use.

> Current development source. New dataset APIs require a matching source build; published alpha packages may differ.

## A report from the local product example

The example processes three input lines referring to two distinct pages. Its returned report includes cumulative counters, the committed shard count and the state of this invocation. The elapsed time below is illustrative because timing changes between runs.

### report.json

```json
{
  "source_lines": 3,
  "pages_fetched": 2,
  "pages_failed": 0,
  "duplicate_urls": 1,
  "records_written": 2,
  "records_rejected": 1,
  "duplicate_records": 1,
  "shards": 1,
  "complete": true,
  "cancelled": false,
  "elapsed_ms": 447
}
```

## Counter reference

A page can produce many rows, so records_written is not expected to equal pages_fetched. Duplicate URLs and duplicate records describe different operations. On resume, cumulative counts remain; the elapsed time and invocation state describe the latest run.

| Field | How to read it |
| --- | --- |
| source_lines | Input lines consumed by committed work; this is not a unique URL or record count. |
| pages_fetched | Pages successfully processed through fetching and extraction. |
| pages_failed | Page-level failures, including fetch failures or a missing required row container. |
| duplicate_urls | Input URLs skipped because their identities were already seen. |
| records_written | Accepted, nonduplicate records committed to data shards. |
| records_rejected | Rows that did not pass extraction/validation requirements. |
| duplicate_records | Valid rows skipped by exact record identity. |
| shards | Number of committed data/error shard pairs. Use this boundary when consuming a completed invocation. |
| complete | The input reached its end. This can be true when failure/rejection counts are nonzero. |
| cancelled | This invocation received a cancellation request. Resume with a new instance. |
| elapsed_ms | Wall time for this invocation, including restart validation; not cumulative job time. |

## Read the error shards

For each committed data shard there is a corresponding part-XXXXXXXX.errors.jsonl file. Read each line as a separate JSON object. Page-level errors identify the URL; row errors also identify the zero-based row index. The error text supplies the reason.

Treat error messages as diagnostics rather than a stable machine-readable taxonomy. Applications should use report counters for high-level decisions and preserve the original error objects for review.

### Rejected product row

```json
{
  "error": "Dataset error: missing required field 'name'",
  "row": 2,
  "url": "http://127.0.0.1:8765/catalog.html"
}
```

## Classify the outcome

The CLI returns 0 for a clean completion, 2 for cancellation or reported page/row failures, and 1 for a fatal error. An SDK returns a report or raises/rejects on a fatal error; it does not choose your application’s exit code.

| Observed state | Interpretation | Next action |
| --- | --- | --- |
| complete=true, no page/row errors | Input completed without reported page or row failures. | Review data quality and use the committed shards. |
| complete=true, errors present | Input completed with partial results. | Inspect errors and decide whether the partial dataset is acceptable. |
| cancelled=true | The invocation stopped before finishing all work. | Wait for the writer to release storage; resume unchanged input/config. |
| Exception / CLI status 1 | The job could not finish normally. | Investigate the error and storage state before consuming output. |
| Process killed or machine restarted | The report may not describe the last written files. | Resume first to reconcile state and output. |

## Retry deliberately

Resuming a finished job is not a retry queue. Failed URLs already committed to its checkpoint are not fetched again. After fixing a selector, source problem or request policy, build a new input file containing the URLs you want to revisit and use a new output directory.

Decide how the corrected output will be reconciled with the earlier dataset. Merging two jobs is an application operation: choose which record wins and how identity is applied. Do not merge state databases or append files into an existing job directory by hand.

- [Checkpoint and recovery details](https://crawlingo-docs.vercel.app/recovery)
- [Schema rules](https://crawlingo-docs.vercel.app/schema)
