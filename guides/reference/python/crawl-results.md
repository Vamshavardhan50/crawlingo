# CrawlResults · Python

Python collection of per-page extraction results.

<a id="overview"></a>

## Purpose and lifecycle

This is a return type, not a top-level package export. It retains results in memory. Current Crawl.build wraps native dictionaries as DatasetResult handles, so conversion/export through that path needs correction.

pandas is required for df(); Parquet export through pandas needs an installed Parquet engine.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Reachable result/callback type; not a top-level package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class CrawlResults
```

- [sdk/python/crawlingo/crawl.py:8](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L8)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| to_json | method | Not annotated; see behavior notes | Present in source |
| to_csv | method | Not annotated; see behavior notes | Present in source |
| to_parquet | method | Not annotated; see behavior notes | Present in source |
| df | method | Not annotated; see behavior notes | Present in source |
| __iter__ | method | iter | Present in source |
| __len__ | method | int | Present in source |
| __getitem__ | method | DatasetResult | Present in source |
| __repr__ | method | str | Present in source |

- [__init__](crawl-results.md#init)
- [to_json](crawl-results.md#to-json)
- [to_csv](crawl-results.md#to-csv)
- [to_parquet](crawl-results.md#to-parquet)
- [df](crawl-results.md#df)
- [__iter__](crawl-results.md#iter)
- [__len__](crawl-results.md#len)
- [__getitem__](crawl-results.md#getitem)
- [__repr__](crawl-results.md#repr)

<a id="init"></a>

## __init__()

Store an in-memory list of DatasetResult wrappers. No fetching or exporting happens during construction.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, results: List[DatasetResult])
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| results | List[DatasetResult] | Required | List of DatasetResult wrappers retained in memory. |

- [Source · sdk/python/crawlingo/crawl.py:12](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L12)

<a id="to-json"></a>

## to_json()

Write a JSON array of all field dictionaries.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def to_json(self, path: str)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/crawl.py:15](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L15)

<a id="to-csv"></a>

## to_csv()

Write all field maps using the first result’s keys as columns; inconsistent later keys can fail.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def to_csv(self, path: str)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/crawl.py:22](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L22)

<a id="to-parquet"></a>

## to_parquet()

Build a pandas DataFrame and export it with the installed Parquet engine.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def to_parquet(self, path: str)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/crawl.py:34](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L34)

<a id="df"></a>

## df()

Collect every field dictionary into a pandas DataFrame.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def df(self)
```

- [Source · sdk/python/crawlingo/crawl.py:38](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L38)

<a id="iter"></a>

## __iter__()

Iterate over the stored DatasetResult wrappers. See any known limitation above before using a materialized Python collection.

Returns / value: iter.

### Signature

```python
def __iter__(self) -> iter
```

- [Source · sdk/python/crawlingo/crawl.py:43](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L43)

<a id="len"></a>

## __len__()

Return the number of stored results or matched elements; no additional pages are fetched.

Returns / value: int.

### Signature

```python
def __len__(self) -> int
```

- [Source · sdk/python/crawlingo/crawl.py:46](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L46)

<a id="getitem"></a>

## __getitem__()

Index the backing Python list of results. List indexing rules apply, including negative indices and IndexError outside the list.

Returns / value: DatasetResult.

### Signature

```python
def __getitem__(self, idx) -> DatasetResult
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| idx | Not annotated | Required | Python list index for CrawlResults; negative indices follow list semantics. |

- [Source · sdk/python/crawlingo/crawl.py:49](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L49)

<a id="repr"></a>

## __repr__()

Return a short debugging representation of the wrapper. This is not a data export or a stable serialization format.

Returns / value: str.

### Signature

```python
def __repr__(self) -> str
```

- [Source · sdk/python/crawlingo/crawl.py:52](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L52)

<a id="errors"></a>

## Errors and operational limits

Filesystem, pandas dependency and native result-shape errors may propagate.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
