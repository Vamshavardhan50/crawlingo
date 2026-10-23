# Dataset · Python

Extract named fields from one page, with optional validation and small-result exports.

<a id="overview"></a>

## Purpose and lifecycle

field adds a named selector; build fetches once and returns DatasetResult. This API keeps results in memory.

buildStructured/extractStructured zip selector results by their positions. Missing values can change row alignment. Prefer DatasetJob with row_selector for reliable repeated records.

The Python wrapper exposes transform callbacks for ordinary build/build_async. The Node field options expose extractType. Do not infer parity from the underlying Rust field type.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class Dataset
```

- [sdk/python/crawlingo/dataset.py:12](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L12)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
from crawlingo import Dataset
result = Dataset("https://example.com/").field("title", "h1").build()
print(result.to_dict())
result.to_json("page.json")
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| field | method | 'Dataset' | Present in source |
| with_schema | method | 'Dataset' | Present in source |
| auto_match | method | 'Dataset' | Present in source |
| timeout | method | 'Dataset' | Present in source |
| headers | method | 'Dataset' | Present in source |
| build | method | 'DatasetResult' | Present in source |
| build_async | method | 'DatasetResult' | Present in source |
| extract_structured | method | list[dict[str, str]] | Present in source |
| build_structured | method | list[dict[str, str]] | Present in source |
| save_json | static method | Not annotated; see behavior notes | Present in source |
| save_csv | static method | Not annotated; see behavior notes | Present in source |

- [__init__](dataset.md#init)
- [field](dataset.md#field)
- [with_schema](dataset.md#with-schema)
- [auto_match](dataset.md#auto-match)
- [timeout](dataset.md#timeout)
- [headers](dataset.md#headers)
- [build](dataset.md#build)
- [build_async](dataset.md#build-async)
- [extract_structured](dataset.md#extract-structured)
- [build_structured](dataset.md#build-structured)
- [save_json](dataset.md#save-json)
- [save_csv](dataset.md#save-csv)

<a id="init"></a>

## __init__()

Configure extraction for one URL with the supplied or default session. Add fields, then call build to fetch and produce a result.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, url: str, session: Optional[Session]=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| session | Optional[Session] | Default: None | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/python/crawlingo/dataset.py:16](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L16)

<a id="field"></a>

## field()

Append a field definition. name is its output key; selector_type/selectorType chooses the selector family; default/defaultVal supplies a fallback when empty. Node extractType supports text, price, datetime, url and attr:name.

Returns / value: 'Dataset'.

### Signature

```python
def field(self, name: str, selector: str, selector_type: str='css', transform: Optional[Callable[[str], str]]=None, default: Optional[str]=None) -> 'Dataset'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |
| selector | str | Required | Selector evaluated against the fetched DOM. |
| selector_type | str | Default: 'css' | Selector engine for this field; supported values differ between builders. |
| transform | Optional[Callable[[str], str]] | Default: None | Python callback applied to the extracted string; application exceptions propagate. |
| default | Optional[str] | Default: None | Fallback field value when extraction is empty. |

- [Source · sdk/python/crawlingo/dataset.py:23](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L23)

<a id="with-schema"></a>

## with_schema()

Attach DatasetSchema to validate/coerce the collected field map before returning it.

Returns / value: 'Dataset'.

### Signature

```python
def with_schema(self, schema: DatasetSchema) -> 'Dataset'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| schema | DatasetSchema | Required | DatasetSchema containing validation constraints. |

- [Source · sdk/python/crawlingo/dataset.py:44](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L44)

<a id="auto-match"></a>

## auto_match()

Set heuristic recovery on the attached session for ordinary Dataset extraction.

Returns / value: 'Dataset'.

### Signature

```python
def auto_match(self, enabled: bool) -> 'Dataset'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | bool | Required | Enable or disable this behavior. |

- [Source · sdk/python/crawlingo/dataset.py:52](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L52)

<a id="timeout"></a>

## timeout()

Set timeout on the attached session in seconds.

Returns / value: 'Dataset'.

### Signature

```python
def timeout(self, seconds: int) -> 'Dataset'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | int | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/python/crawlingo/dataset.py:57](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L57)

<a id="headers"></a>

## headers()

Replace headers on the attached session.

Returns / value: 'Dataset'.

### Signature

```python
def headers(self, headers: dict) -> 'Dataset'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| headers | dict | Required | Header names and values; avoid sharing credentials across unrelated sites. |

- [Source · sdk/python/crawlingo/dataset.py:62](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L62)

<a id="build"></a>

## build()

Fetch, parse and extract one field map. Node returns Promise<DatasetResult>; Python blocks and returns DatasetResult.

Returns / value: 'DatasetResult'.

### Signature

```python
def build(self) -> 'DatasetResult'
```

- [Source · sdk/python/crawlingo/dataset.py:67](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L67)

<a id="build-async"></a>

## build_async()

Run the Python native build through an executor and await its result. This is an in-memory result, not a streaming DatasetJob.

Returns / value: 'DatasetResult'.

### Signature

```python
async def build_async(self) -> 'DatasetResult'
```

- [Source · sdk/python/crawlingo/dataset.py:75](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L75)

<a id="extract-structured"></a>

## extract_structured()

Extract positional rows from an already parsed Page without making another fetch.

Returns / value: list[dict[str, str]].

### Signature

```python
def extract_structured(self, page: 'Page') -> list[dict[str, str]]
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| page | 'Page' | Required | Page object to extract from; distinct from a page-number pagination parameter. |

- [Source · sdk/python/crawlingo/dataset.py:85](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L85)

<a id="build-structured"></a>

## build_structured()

Fetch the configured URL and return positional multi-row records in memory.

Returns / value: list[dict[str, str]].

### Signature

```python
def build_structured(self) -> list[dict[str, str]]
```

- [Source · sdk/python/crawlingo/dataset.py:93](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L93)

<a id="save-json"></a>

## save_json()

Write the supplied records as a JSON array. The destination file is created or replaced.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def save_json(records: list[dict[str, str]], path: str)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| records | list[dict[str, str]] | Required | In-memory row maps to export. |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/dataset.py:101](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L101)

<a id="save-csv"></a>

## save_csv()

Write a CSV header and the supplied records. The destination file is created or replaced. Use a consistent record schema.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def save_csv(records: list[dict[str, str]], path: str)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| records | list[dict[str, str]] | Required | In-memory row maps to export. |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/dataset.py:109](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L109)

<a id="errors"></a>

## Errors and operational limits

Non-success HTTP statuses abort a Dataset build. Required schema violations, regex syntax errors, fingerprint storage failures and export errors can fail the operation. Node uses the typed error bridge; Python uses its wrapper-specific conversion.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
