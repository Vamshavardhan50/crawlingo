# DatasetJob · Python

Process a URL file into bounded, validated, resumable dataset shards.

<a id="overview"></a>

## Purpose and lifecycle

The recommended API for large research and directory datasets. Define one row’s meaning, required fields and identity before scaling.

Records remain in Rust and on disk. run returns a report, not all records. Use one active writer per output directory and keep input, configuration, state and committed shards consistent.

A completed resume does not retry already committed failed URLs. Make a new job with corrected input for those failures. Cancellation requires a new instance for resume.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class DatasetJob
```

- [sdk/python/crawlingo/dataset_job.py:9](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset_job.py#L9)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
from crawlingo import DatasetJob
job = DatasetJob({
    "urls_file": "urls.txt", "output_dir": "datasets/research",
    "fields": [{"name": "title", "selector": "h1", "required": True}],
    "format": "jsonl", "concurrency": 8,
})
report = job.run()
print(report["records_written"], report["pages_failed"])
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| run | method | dict | Present in source |
| run_async | method | dict | Present in source |
| cancel | method | None | Present in source |

- [__init__](dataset-job.md#init)
- [run](dataset-job.md#run)
- [run_async](dataset-job.md#run-async)
- [cancel](dataset-job.md#cancel)

<a id="init"></a>

## __init__()

Validate configuration and allocate the job handle. Network requests begin at run, not construction.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, config: dict, session: Optional[Session]=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| config | dict | Required | Configuration object; see the linked job or pagination reference. |
| session | Optional[Session] | Default: None | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/python/crawlingo/dataset_job.py:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset_job.py#L10)

<a id="run"></a>

## run()

Run or resume the job and return JobReport. Rust/Node are asynchronous; Python run blocks and releases the GIL for native work.

Returns / value: dict.

### Signature

```python
def run(self) -> dict
```

- [Source · sdk/python/crawlingo/dataset_job.py:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset_job.py#L14)

<a id="run-async"></a>

## run_async()

Await the Python job in an executor. Task cancellation requests native cancellation and waits for cleanup before re-raising CancelledError.

Returns / value: dict.

### Signature

```python
async def run_async(self) -> dict
```

- [Source · sdk/python/crawlingo/dataset_job.py:18](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset_job.py#L18)

<a id="cancel"></a>

## cancel()

Request cooperative cancellation. Await the active run before releasing its volume or starting a replacement job. The uncommitted batch may be replayed.

Returns / value: None.

### Signature

```python
def cancel(self) -> None
```

- [Source · sdk/python/crawlingo/dataset_job.py:33](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset_job.py#L33)

<a id="errors"></a>

## Errors and operational limits

Construction rejects invalid configuration. Job-level storage, checkpoint and configuration failures reject/raise; individual failed pages and rows are counted and written to error shards. complete=true does not mean every page succeeded.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
