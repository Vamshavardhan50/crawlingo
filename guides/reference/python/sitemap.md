# Sitemap · Python

Discover URLs from a sitemap or use them to seed a bounded crawl.

<a id="overview"></a>

## Purpose and lifecycle

listUrls/list_urls returns an in-memory list of sitemap entries. It does not fetch each listed page.

maxDepth/max_depth limits sitemap-index recursion. depth limits subsequent crawling from each seed; these limits have different meanings.

For a large dataset, persist discovered URLs to a file and use DatasetJob. Inspect gzip and child-sitemap handling for your binding before relying on compressed sitemap support.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class Sitemap
```

- [sdk/python/crawlingo/sitemap.py:42](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L42)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
from crawlingo import Sitemap
entries = Sitemap("https://example.com/sitemap.xml").max_depth(3).list_urls()
with open("urls.txt", "w", encoding="utf-8") as out:
    for entry in entries:
        out.write(entry.loc + "\n")
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| max_depth | method | 'Sitemap' | Present in source |
| follow | method | 'Sitemap' | Present in source |
| limit | method | 'Sitemap' | Present in source |
| depth | method | 'Sitemap' | Present in source |
| concurrency | method | 'Sitemap' | Present in source |
| delay | method | 'Sitemap' | Present in source |
| field | method | 'Sitemap' | Present in source |
| webhook | method | 'Sitemap' | Present in source |
| list_urls | method | List[SitemapEntry] | Present in source |
| build | method | CrawlResults | Present in source |

- [__init__](sitemap.md#init)
- [max_depth](sitemap.md#max-depth)
- [follow](sitemap.md#follow)
- [limit](sitemap.md#limit)
- [depth](sitemap.md#depth)
- [concurrency](sitemap.md#concurrency)
- [delay](sitemap.md#delay)
- [field](sitemap.md#field)
- [webhook](sitemap.md#webhook)
- [list_urls](sitemap.md#list-urls)
- [build](sitemap.md#build)

<a id="init"></a>

## __init__()

Create a sitemap discovery builder. Fetching begins with listUrls/list_urls or build; the constructor does not materialize discovered URLs.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, sitemap_url: str, *, session: Optional[Session]=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| sitemap_url | str | Required | URL of a sitemap XML document or index. |
| session | Optional[Session] | Default: None | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/python/crawlingo/sitemap.py:48](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L48)

<a id="max-depth"></a>

## max_depth()

Maximum sitemap-index nesting depth.

Returns / value: 'Sitemap'.

### Signature

```python
def max_depth(self, depth: int) -> 'Sitemap'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| depth | int | Required | Nesting or crawl-hop limit, as specified by this method. |

- [Source · sdk/python/crawlingo/sitemap.py:52](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L52)

<a id="follow"></a>

## follow()

Configure links followed after a seed page is fetched.

Returns / value: 'Sitemap'.

### Signature

```python
def follow(self, selector: str) -> 'Sitemap'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| selector | str | Required | Selector evaluated against the fetched DOM. |

- [Source · sdk/python/crawlingo/sitemap.py:57](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L57)

<a id="limit"></a>

## limit()

Configure the downstream crawl page limit.

Returns / value: 'Sitemap'.

### Signature

```python
def limit(self, pages: int) -> 'Sitemap'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| pages | int | Required | Maximum downstream crawl pages. |

- [Source · sdk/python/crawlingo/sitemap.py:62](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L62)

<a id="depth"></a>

## depth()

Configure the downstream crawl hop limit.

Returns / value: 'Sitemap'.

### Signature

```python
def depth(self, max_depth: int) -> 'Sitemap'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| max_depth | int | Required | Maximum sitemap nesting or crawl depth, according to the API. |

- [Source · sdk/python/crawlingo/sitemap.py:67](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L67)

<a id="concurrency"></a>

## concurrency()

Configure concurrent downstream crawl workers.

Returns / value: 'Sitemap'.

### Signature

```python
def concurrency(self, n: int) -> 'Sitemap'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| n | int | Required | Count or index; see the method description. |

- [Source · sdk/python/crawlingo/sitemap.py:72](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L72)

<a id="delay"></a>

## delay()

Configure downstream crawl delay in seconds.

Returns / value: 'Sitemap'.

### Signature

```python
def delay(self, seconds: float) -> 'Sitemap'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | float | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/python/crawlingo/sitemap.py:77](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L77)

<a id="field"></a>

## field()

Add a downstream extraction field.

Returns / value: 'Sitemap'.

### Signature

```python
def field(self, name: str, selector: str, selector_type: str='css', default: Optional[str]=None) -> 'Sitemap'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |
| selector | str | Required | Selector evaluated against the fetched DOM. |
| selector_type | str | Default: 'css' | Selector engine for this field; supported values differ between builders. |
| default | Optional[str] | Default: None | Fallback field value when extraction is empty. |

- [Source · sdk/python/crawlingo/sitemap.py:82](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L82)

<a id="webhook"></a>

## webhook()

Set a downstream result endpoint; delivery is not durable.

Returns / value: 'Sitemap'.

### Signature

```python
def webhook(self, url: str) -> 'Sitemap'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/python/crawlingo/sitemap.py:87](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L87)

<a id="list-urls"></a>

## list_urls()

Fetch sitemap documents and return discovered URL entries; results are collected in memory.

Returns / value: List[SitemapEntry].

### Signature

```python
def list_urls(self) -> List[SitemapEntry]
```

- [Source · sdk/python/crawlingo/sitemap.py:92](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L92)

<a id="build"></a>

## build()

Seed a crawl with sitemap URLs and return its in-memory results.

Returns / value: CrawlResults.

### Signature

```python
def build(self) -> CrawlResults
```

- [Source · sdk/python/crawlingo/sitemap.py:100](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/sitemap.py#L100)

<a id="errors"></a>

## Errors and operational limits

Node listUrls now propagates fetch/parse errors. Python list_urls can skip failing sitemap fetches in its native binding. Empty results can therefore have different meanings.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
