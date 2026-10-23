# SitemapEntry · Python

Sitemap URL metadata. lastmod and priority are source strings, not validated dates or numbers.

<a id="overview"></a>

## Purpose and lifecycle

Sitemap URL metadata. lastmod and priority are source strings, not validated dates or numbers.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](../node/sitemap.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class SitemapEntry
```

- [sdk/python/crawlingo/sitemap.py:16](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L16)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| loc | property | str | Present in source |
| lastmod | property | Optional[str] | Present in source |
| changefreq | property | Optional[str] | Present in source |
| priority | property | Optional[str] | Present in source |
| __repr__ | method | str | Present in source |

- [__init__](sitemap-entry.md#init)
- [loc](sitemap-entry.md#loc)
- [lastmod](sitemap-entry.md#lastmod)
- [changefreq](sitemap-entry.md#changefreq)
- [priority](sitemap-entry.md#priority)
- [__repr__](sitemap-entry.md#repr)

<a id="init"></a>

## __init__()

Store the source URL and optional sitemap metadata. The wrapper does not validate dates, update-frequency strings or priority values.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, loc: str, lastmod: Optional[str]=None, changefreq: Optional[str]=None, priority: Optional[str]=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| loc | str | Required | URL from a leaf sitemap entry. |
| lastmod | Optional[str] | Default: None | Optional original last-modified string. |
| changefreq | Optional[str] | Default: None | Optional advisory update-frequency string. |
| priority | Optional[str] | Default: None | Optional original priority string; not a numeric validation guarantee. |

- [Source · sdk/python/crawlingo/sitemap.py:21](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L21)

<a id="loc"></a>

## loc

Instance field assigned by the constructor.

Returns / value: str.

### Signature

```python
loc: str
```

- [Source · sdk/python/crawlingo/sitemap.py:28](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L28)

<a id="lastmod"></a>

## lastmod

Instance field assigned by the constructor.

Returns / value: Optional[str].

### Signature

```python
lastmod: Optional[str]
```

- [Source · sdk/python/crawlingo/sitemap.py:29](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L29)

<a id="changefreq"></a>

## changefreq

Instance field assigned by the constructor.

Returns / value: Optional[str].

### Signature

```python
changefreq: Optional[str]
```

- [Source · sdk/python/crawlingo/sitemap.py:30](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L30)

<a id="priority"></a>

## priority

Instance field assigned by the constructor.

Returns / value: Optional[str].

### Signature

```python
priority: Optional[str]
```

- [Source · sdk/python/crawlingo/sitemap.py:31](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L31)

<a id="repr"></a>

## __repr__()

Return a short debugging representation of the wrapper. This is not a data export or a stable serialization format.

Returns / value: str.

### Signature

```python
def __repr__(self) -> str
```

- [Source · sdk/python/crawlingo/sitemap.py:33](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L33)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
