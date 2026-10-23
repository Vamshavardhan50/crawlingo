# hooks.strip_whitespace · Python

Strips leading and trailing whitespace from extracted content.

<a id="overview"></a>

## Purpose and lifecycle

Strips leading and trailing whitespace from extracted content.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Reachable result/callback type; not a top-level package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
def strip_whitespace(val: str) -> str
```

- [sdk/python/crawlingo/hooks.py:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/hooks.py#L6)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| strip_whitespace | method | str | Present in source |

- [strip_whitespace](hooks-strip-whitespace.md#strip-whitespace)

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

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
