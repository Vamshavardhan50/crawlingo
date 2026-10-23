# hooks · Python

Value-cleaning and logging helpers for Python Page hooks.

<a id="overview"></a>

## Purpose and lifecycle

Value-cleaning and logging helpers for Python Page hooks.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
from crawlingo import hooks
```

- [sdk/python/crawlingo/hooks.py:1](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/hooks.py#L1)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| strip_whitespace | method | str | Present in source |
| uppercase | method | str | Present in source |
| lowercase | method | str | Present in source |
| log_request | method | Not annotated; see behavior notes | Present in source |
| log_response | method | Not annotated; see behavior notes | Present in source |

- [strip_whitespace](hooks.md#strip-whitespace)
- [uppercase](hooks.md#uppercase)
- [lowercase](hooks.md#lowercase)
- [log_request](hooks.md#log-request)
- [log_response](hooks.md#log-response)

<a id="strip-whitespace"></a>

## strip_whitespace()

Strips leading and trailing whitespace from extracted content.

Returns / value: str.

### Signature

```python
def strip_whitespace(val: str) -> str
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| val | str | Required | Extracted string to clean or transform. |

- [Source · sdk/python/crawlingo/hooks.py:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/hooks.py#L6)

<a id="uppercase"></a>

## uppercase()

Converts extracted content to uppercase.

Returns / value: str.

### Signature

```python
def uppercase(val: str) -> str
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| val | str | Required | Extracted string to clean or transform. |

- [Source · sdk/python/crawlingo/hooks.py:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/hooks.py#L10)

<a id="lowercase"></a>

## lowercase()

Converts extracted content to lowercase.

Returns / value: str.

### Signature

```python
def lowercase(val: str) -> str
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| val | str | Required | Extracted string to clean or transform. |

- [Source · sdk/python/crawlingo/hooks.py:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/hooks.py#L14)

<a id="log-request"></a>

## log_request()

Simple logger hook that prints the target URL before fetching.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def log_request(req)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| req | Not annotated | Required | before_fetch RequestContext with url and headers. |

- [Source · sdk/python/crawlingo/hooks.py:18](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/hooks.py#L18)

<a id="log-response"></a>

## log_response()

Simple logger hook that prints the HTTP status code after fetching.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def log_response(res)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| res | Not annotated | Required | after_fetch ResponseContext with HTTP status. |

- [Source · sdk/python/crawlingo/hooks.py:22](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/hooks.py#L22)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
