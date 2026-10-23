# RequestContext · Python

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
class RequestContext
```

- [sdk/python/crawlingo/page.py:68](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L68)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| url | property | Not annotated | Present in source |
| headers | property | Not annotated | Present in source |

- [__init__](request-context.md#init)
- [url](request-context.md#url)
- [headers](request-context.md#headers)

<a id="init"></a>

## __init__()

Internal value constructed by Page for before_fetch callbacks. Headers refer to the request map; assigning url does not change the actual fetch URL.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, url, headers)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | Not annotated | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| headers | Not annotated | Required | Header names and values; avoid sharing credentials across unrelated sites. |

- [Source · sdk/python/crawlingo/page.py:69](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L69)

<a id="url"></a>

## url

Instance field assigned by the constructor.

Returns / value: Not annotated.

### Signature

```python
url: Not annotated
```

- [Source · sdk/python/crawlingo/page.py:70](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L70)

<a id="headers"></a>

## headers

Instance field assigned by the constructor.

Returns / value: Not annotated.

### Signature

```python
headers: Not annotated
```

- [Source · sdk/python/crawlingo/page.py:71](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L71)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
