# ResponseContext · Python

class declaration and members.

<a id="overview"></a>

## Purpose and lifecycle



- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Reachable result/callback type; not a top-level package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class ResponseContext
```

- [sdk/python/crawlingo/page.py:93](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L93)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| status | property | Not annotated | Present in source |

- [__init__](response-context.md#init)
- [status](response-context.md#status)

<a id="init"></a>

## __init__()

Internal value constructed by Page for after_fetch callbacks. The status field reports the fetched HTTP status.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, status)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| status | Not annotated | Required | HTTP response status code. |

- [Source · sdk/python/crawlingo/page.py:94](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L94)

<a id="status"></a>

## status

Instance field assigned by the constructor.

Returns / value: Not annotated.

### Signature

```python
status: Not annotated
```

- [Source · sdk/python/crawlingo/page.py:95](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L95)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
