# Downloader · Python

Save a response to a local file or receive its bytes in memory.

<a id="overview"></a>

## Purpose and lifecycle

allowResume uses HTTP Range when resuming an existing file. Inspect resumed and status rather than assuming the server honored the range.

The underlying transport buffers the response and applies its response-size cap. This is not an unlimited streaming downloader; chunk_size controls file writes, not network memory.

maxBytes/max_bytes limits the bytes retained/written after fetching. It does not make the upstream transfer stop at that exact byte count.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class Downloader
```

- [sdk/python/crawlingo/download.py:33](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L33)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
from crawlingo import Downloader
result = Downloader().allow_resume(True).max_bytes(1048576).download(
    "https://example.com/", "page.html"
)
print(result.status, result.bytes_written, result.resumed)
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| chunk_size | method | 'Downloader' | Present in source |
| allow_resume | method | 'Downloader' | Present in source |
| max_bytes | method | 'Downloader' | Present in source |
| download | method | DownloadResult | Present in source |
| download_to_memory | method | Tuple[DownloadResult, bytes] | Present in source |

- [__init__](downloader.md#init)
- [chunk_size](downloader.md#chunk-size)
- [allow_resume](downloader.md#allow-resume)
- [max_bytes](downloader.md#max-bytes)
- [download](downloader.md#download)
- [download_to_memory](downloader.md#download-to-memory)

<a id="init"></a>

## __init__()

Create a downloader using a supplied session or a fresh default session. Configure limits and call download or download_to_memory.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, session: Optional[Session]=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| session | Optional[Session] | Default: None | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/python/crawlingo/download.py:38](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L38)

<a id="chunk-size"></a>

## chunk_size()

Choose the Python file-write chunk size in bytes; default 65536.

Returns / value: 'Downloader'.

### Signature

```python
def chunk_size(self, size: int) -> 'Downloader'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| size | int | Required | Size in bytes. |

- [Source · sdk/python/crawlingo/download.py:42](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L42)

<a id="allow-resume"></a>

## allow_resume()

Enable or disable range-based continuation of an existing destination file.

Returns / value: 'Downloader'.

### Signature

```python
def allow_resume(self, enabled: bool=True) -> 'Downloader'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | bool | Default: True | Enable or disable this behavior. |

- [Source · sdk/python/crawlingo/download.py:47](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L47)

<a id="max-bytes"></a>

## max_bytes()

Limit the returned/written body bytes. Choose a positive integer.

Returns / value: 'Downloader'.

### Signature

```python
def max_bytes(self, n: int) -> 'Downloader'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| n | int | Required | Count or index; see the method description. |

- [Source · sdk/python/crawlingo/download.py:52](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L52)

<a id="download"></a>

## download()

Fetch into the destination and return DownloadResult. This can append or replace depending on resume/server response.

Returns / value: DownloadResult.

### Signature

```python
def download(self, url: str, dest: str) -> DownloadResult
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| dest | str | Required | Destination file path; existing content may be replaced or resumed. |

- [Source · sdk/python/crawlingo/download.py:57](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L57)

<a id="download-to-memory"></a>

## download_to_memory()

Return result metadata and the response bytes; retain only small downloads this way.

Returns / value: Tuple[DownloadResult, bytes].

### Signature

```python
def download_to_memory(self, url: str) -> Tuple[DownloadResult, bytes]
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/python/crawlingo/download.py:73](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L73)

<a id="errors"></a>

## Errors and operational limits

Network, timeout and filesystem failures can propagate. Node uses typed native errors; Python downloader methods may propagate native exceptions directly.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
