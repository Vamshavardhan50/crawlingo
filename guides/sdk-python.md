# Python SDK

Run the Rust dataset pipeline from Python and receive a compact report.

<a id="setup"></a>

## Build the native extension

These guides describe the current development source. DatasetJob is new in this working tree; published alpha packages may not contain it. Build the matching source before using these examples.

- [Python installation](installation.md#python)

<a id="job"></a>

## Generate a dataset

DatasetJob(config, session=None) accepts the shared JSON configuration as a dictionary. run() blocks until completion and releases the GIL while the native job runs.

### Python

```python
import json
from crawlingo import DatasetJob

with open("job.json", encoding="utf-8") as source:
    config = json.load(source)

report = DatasetJob(config).run()
print(report["records_written"])
print(report["pages_failed"], report["records_rejected"])
```

<a id="async"></a>

## Use an async application

Cancellation requests stop the native job. Wait for the previous run to release its directory before resuming with a new instance.

### Python

```python
import asyncio
import json
from crawlingo import DatasetJob

async def main():
    with open("job.json", encoding="utf-8") as source:
        job = DatasetJob(json.load(source))
    report = await job.run_async()
    print(report)

asyncio.run(main())
```

<a id="small"></a>

## Extract one page

The Python Dataset.field wrapper currently accepts name, selector, selector_type, transform and default. Use JobField configuration for extract_type in bulk jobs.

### Python

```python
from crawlingo import Dataset

result = Dataset("https://example.com/").field("title", "h1").build()
print(result.to_dict())
result.to_json("page.json")
```

- [Dataset job reference](dataset-jobs.md)
