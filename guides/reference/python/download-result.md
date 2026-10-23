# DownloadResult · Python

Response metadata and bytes actually retained/written. Check status and resumed explicitly.

<a id="overview"></a>

## Purpose and lifecycle

Response metadata and bytes actually retained/written. Check status and resumed explicitly.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](../node/downloader.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class DownloadResult
```

- [sdk/python/crawlingo/download.py:16](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L16)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| url | property | str | Present in source |
| status | property | int | Present in source |
| bytes_written | property | int | Present in source |
| content_type | property | str | Present in source |
| suggested_filename | property | Optional[str] | Present in source |
| resumed | property | bool | Present in source |

- [url](download-result.md#url)
- [status](download-result.md#status)
- [bytes_written](download-result.md#bytes-written)
- [content_type](download-result.md#content-type)
- [suggested_filename](download-result.md#suggested-filename)
- [resumed](download-result.md#resumed)

<a id="url"></a>

## url

Source/final URL according to the operation; check redirects and provenance needs.

Returns / value: str.

### Signature

```python
url: str
```

- [Source · sdk/python/crawlingo/download.py:19](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L19)

<a id="status"></a>

## status

HTTP response status code.

Returns / value: int.

### Signature

```python
status: int
```

- [Source · sdk/python/crawlingo/download.py:21](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L21)

<a id="bytes-written"></a>

## bytes_written

Bytes actually retained or written.

Returns / value: int.

### Signature

```python
bytes_written: int
```

- [Source · sdk/python/crawlingo/download.py:23](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L23)

<a id="content-type"></a>

## content_type

Reported or inferred response MIME type.

Returns / value: str.

### Signature

```python
content_type: str
```

- [Source · sdk/python/crawlingo/download.py:25](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L25)

<a id="suggested-filename"></a>

## suggested_filename

Untrusted response-suggested filename; choose your destination explicitly.

Returns / value: Optional[str].

### Signature

```python
suggested_filename: Optional[str]
```

- [Source · sdk/python/crawlingo/download.py:27](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L27)

<a id="resumed"></a>

## resumed

Whether a range response was accepted for the download.

Returns / value: bool.

### Signature

```python
resumed: bool
```

- [Source · sdk/python/crawlingo/download.py:29](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/download.py#L29)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
