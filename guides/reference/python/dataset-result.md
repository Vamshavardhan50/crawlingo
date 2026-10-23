# DatasetResult · Python

Read and export a single extracted field map.

<a id="overview"></a>

## Purpose and lifecycle

Returned by Dataset.build. Constructing it manually requires the correct native result handle.

Export methods create or replace files. A small-result export is not a resumable dataset shard or checkpoint. Python df() needs pandas installed separately.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class DatasetResult
```

- [sdk/python/crawlingo/dataset.py:118](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L118)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| to_json | method | Not annotated; see behavior notes | Present in source |
| to_csv | method | Not annotated; see behavior notes | Present in source |
| to_parquet | method | Not annotated; see behavior notes | Present in source |
| to_dict | method | dict | Present in source |
| df | method | Not annotated; see behavior notes | Present in source |
| __getitem__ | method | str | Present in source |
| __repr__ | method | str | Present in source |

- [__init__](dataset-result.md#init)
- [to_json](dataset-result.md#to-json)
- [to_csv](dataset-result.md#to-csv)
- [to_parquet](dataset-result.md#to-parquet)
- [to_dict](dataset-result.md#to-dict)
- [df](dataset-result.md#df)
- [__getitem__](dataset-result.md#getitem)
- [__repr__](dataset-result.md#repr)

<a id="init"></a>

## __init__()

Wrap a native DatasetResult handle returned by the engine. A plain Python dictionary is not a compatible handle.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, core_result: _CoreDatasetResult)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| core_result | _CoreDatasetResult | Required | Native result handle produced by the engine. |

- [Source · sdk/python/crawlingo/dataset.py:122](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L122)

<a id="to-json"></a>

## to_json()

Serialize the field map to one JSON object file.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def to_json(self, path: str)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/dataset.py:125](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L125)

<a id="to-csv"></a>

## to_csv()

Write one header row and one data row.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def to_csv(self, path: str)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/dataset.py:132](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L132)

<a id="to-parquet"></a>

## to_parquet()

Write the fields as a one-row Parquet dataset.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def to_parquet(self, path: str)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/dataset.py:139](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L139)

<a id="to-dict"></a>

## to_dict()

Return a copy of the extracted string-valued fields.

Returns / value: dict.

### Signature

```python
def to_dict(self) -> dict
```

- [Source · sdk/python/crawlingo/dataset.py:146](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L146)

<a id="df"></a>

## df()

Return a one-row pandas DataFrame; requires pandas.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def df(self)
```

- [Source · sdk/python/crawlingo/dataset.py:153](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L153)

<a id="getitem"></a>

## __getitem__()

Look up a field by key; the native result can return None for an absent key.

Returns / value: str.

### Signature

```python
def __getitem__(self, key: str) -> str
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| key | str | Required | Field-map key or extraction field name. |

- [Source · sdk/python/crawlingo/dataset.py:158](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L158)

<a id="repr"></a>

## __repr__()

Return a short debugging representation of the wrapper. This is not a data export or a stable serialization format.

Returns / value: str.

### Signature

```python
def __repr__(self) -> str
```

- [Source · sdk/python/crawlingo/dataset.py:161](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/dataset.py#L161)

<a id="errors"></a>

## Errors and operational limits

Filesystem and serialization failures propagate. Node export methods reject with ExportError. Python can also expose OSError directly.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
