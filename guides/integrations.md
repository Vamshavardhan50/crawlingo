# Integrations

Use Crawlingo as the collection step in your existing data pipeline.

<a id="contract"></a>

## The integration contract

Crawlingo takes a UTF-8 URL file and a field schema. It writes validated records, rejection details and a completion report to a local directory. Your application owns discovery, scheduling and the destination for the finished dataset.

### Pipeline

```text
Discover URLs → urls.txt + job.json → Crawlingo worker
             → JSONL / CSV + report → analysis, database or search
```

<a id="choose"></a>

## Choose a handoff

| Your tool | Connect through | Who does what |
| --- | --- | --- |
| Playwright / Puppeteer | A saved URL file | The browser discovers links; Crawlingo fetches and extracts destination pages. |
| pandas / DuckDB | Completed JSONL or CSV shards | Crawlingo validates strings; the analysis tool casts types and queries records. |
| FastAPI / Express | A queued SDK or CLI worker | Your service controls access and job status; Crawlingo produces files. |
| Airflow / cron / CI | The CLI and its exit status | The scheduler owns retries and storage; Crawlingo resumes an unchanged job. |
| Object storage / databases | A separate export stage | Upload committed output after completion; retain state locally for resume. |
| Search / RAG tools | JSONL records with source metadata | Your downstream code cleans text, chunks, embeds and indexes it. |

- [Playwright and Puppeteer](integrations-playwright.md)
- [pandas, DuckDB and Parquet](integrations-data.md)
- [Services and scheduled workers](integrations-workers.md)

<a id="ownership"></a>

## Keep the boundaries explicit

- Build the current SDK from source for DatasetJob. Older registry packages do not include the new job API.
- Give each job a stable input, configuration and output directory. Only one process may own that directory.
- Publish a dataset only after reviewing complete, cancelled, pages_failed and records_rejected.
- Use JSONL for incremental processing. Do not collect a million records into an application response.

- [Job configuration](api-job-config.md)
- [Report fields](report-reference.md)

<a id="support"></a>

## Adapters and support

The recipes here compose existing APIs and file formats. They are not maintained Playwright, Airflow, pandas or database plugins. Crawlingo has no hosted REST API, built-in queue, embedding service or direct object-storage checkpoint backend. Choose the recipe whose boundary matches your application.
