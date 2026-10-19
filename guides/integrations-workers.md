# Services & scheduled workers

Run Crawlingo behind an API or scheduler with explicit ownership, retries and output publication.

> Current development source. New dataset APIs require a matching source build; published alpha packages may differ.

## Keep long jobs in a worker

For FastAPI, Express or another web framework, accept a request, enqueue an application job ID, and return that ID. A separate worker loads a trusted configuration and runs Crawlingo. The web handler returns progress metadata or file links rather than holding an HTTP connection open for a million records.

### Application architecture

```text
API request → your job store / queue → one worker per output_dir
GET status  ← application job status ← report + error summary
Download    ← completed dataset     ← publication step
```

- Persist your own queued/running/failed/published states. report.json is an engine report, not a queue or a live status service.
- Validate submitted destinations and file paths in the application. Do not accept arbitrary output directories from an API caller.
- Use durable storage for both shards and state. An ephemeral container filesystem cannot provide restart recovery.

## A Python worker entry point

Install the source SDK, save a trusted job.json, and run this script in the worker’s assigned directory. It returns a non-zero status when the result needs review. Configure your queue to retry only one worker for this output directory at a time.

### worker.py

```python
import json
from pathlib import Path
from crawlingo import DatasetJob

config = json.loads(Path("job.json").read_text(encoding="utf-8"))
report = DatasetJob(config).run()
print(json.dumps(report))
if report["cancelled"] or not report["complete"]:
    raise SystemExit(1)
if report["pages_failed"] or report["records_rejected"]:
    raise SystemExit(2)
# Your publication step runs only after this point.

```

- The same synchronous entry point can run inside a queue task. In an async worker, await DatasetJob(config).run_async(). Cancellation requests a cooperative stop; wait for the worker to exit before starting a replacement on the same directory.

## A Node.js worker entry point

Use the native SDK in a Node server or worker process. It cannot run in a browser bundle or an edge runtime without native-addon support. Await completion after cancel() so the next owner does not race the current writer.

### worker.mjs

```javascript
import { readFile } from 'node:fs/promises';
import { DatasetJob } from 'crawlingo';

const config = JSON.parse(await readFile('job.json', 'utf8'));
const job = new DatasetJob(config);
const stop = () => job.cancel();
process.once('SIGTERM', stop);
process.once('SIGINT', stop);
try {
  const report = await job.run();
  console.log(JSON.stringify(report));
  process.exitCode = report.cancelled || !report.complete ? 1
    : report.pages_failed || report.records_rejected ? 2 : 0;
} finally {
  process.removeListener('SIGTERM', stop);
  process.removeListener('SIGINT', stop);
}
```

## cron, Airflow and CI jobs

Build the CLI once in your worker image. Configure the scheduler to run the command below with a fixed working directory and a persistent mounted job directory. In Airflow, the task or container operator runs the same command; this guide does not require a custom Crawlingo operator.

### Worker command

```shell
crawlingo dataset /work/job.json
```

| CLI status | Meaning | Scheduler action |
| --- | --- | --- |
| 0 | Complete with no page failures or rejected rows | Proceed to the publication task. |
| 2 | Page failures, rejected rows, or cooperative cancellation | Inspect the report; resume cancellation after the old worker exits. |
| 1 | Fatal startup, configuration or storage error | Resolve the error before retrying. |

- A retry uses the same immutable inputs and directory. A scheduled refresh uses a new directory for that collection date. A finished job resumes without refetching completed URLs; it does not refresh pages or retry previously committed failures automatically. A process killed by the OS may return a different exit status.

- [Container and storage setup](https://crawlingo-docs.vercel.app/deployment)
- [Recovery and retry semantics](https://crawlingo-docs.vercel.app/recovery)

## Publish a finished dataset

- Check report counters against your quality threshold.
- Keep data shards, error shards, report, input and configuration traceable to one run.
- Write a destination manifest or completion marker only after all transfers succeed.
- Use stable record keys to make downstream retries idempotent. Dataset deduplication is scoped to a job, not your entire database.

- [FastAPI: background work considerations](https://fastapi.tiangolo.com/tutorial/background-tasks/)
- [Processing the output](https://crawlingo-docs.vercel.app/integrations/data)
