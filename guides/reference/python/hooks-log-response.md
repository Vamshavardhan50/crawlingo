# hooks.log_response · Python

Simple logger hook that prints the HTTP status code after fetching.

<a id="overview"></a>

## Purpose and lifecycle

Simple logger hook that prints the HTTP status code after fetching.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Reachable result/callback type; not a top-level package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
def log_response(res)
```

- [sdk/python/crawlingo/hooks.py:22](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/hooks.py#L22)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| log_response | method | Not annotated; see behavior notes | Present in source |

- [log_response](hooks-log-response.md#log-response)

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
