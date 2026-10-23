# AutoMatchFailed · Python

Raised when DOM fingerprint auto matching fails.

<a id="overview"></a>

## Purpose and lifecycle

AutoMatchFailed is exported from the Python package. Specialized errors inherit CrawlingoError; the base inherits Python Exception.

Construct with ErrorClass(message). The Python wrapper uses message-based conversion for selected methods; other native failures can remain RuntimeError, ValueError or OSError. An exported class is not proof that every corresponding failure path produces it.

Page returns HTTP statuses, including exhausted 429/500 responses. A missing selector can return an empty collection. DatasetJob records individual failures in reports/error shards. These conditions do not automatically throw every exported error class.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class AutoMatchFailed(CrawlingoError)
```

- [sdk/python/crawlingo/exceptions.py:17](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/exceptions.py#L17)

<a id="catch"></a>

## Construct and catch

### Python

```python
from crawlingo import AutoMatchFailed

try:
    raise AutoMatchFailed("Operation failed")
except AutoMatchFailed as error:
    print(str(error))
```

- [Base exception contract](crawlingo-error.md)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
