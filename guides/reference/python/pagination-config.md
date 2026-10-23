# PaginationConfig · Python

Choose how a crawler discovers the next page.

<a id="overview"></a>

## Purpose and lifecycle

Use the named factories rather than calling the constructor. Attach the returned configuration with Crawl.withPagination/with_pagination.

Set crawl limits even when pagination has its own maximum. A URL pattern is not evidence that every generated page exists.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class PaginationConfig
```

- [sdk/python/crawlingo/pagination.py:4](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/pagination.py#L4)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
from crawlingo import Crawl, PaginationConfig
crawl = Crawl("https://example.com/catalog?page=1").with_pagination(
    PaginationConfig.page_number("https://example.com/catalog?page={page}", 1, 5)
).limit(5)
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| next_link | static method | 'PaginationConfig' | Present in source |
| page_number | static method | 'PaginationConfig' | Present in source |
| url_pattern | static method | 'PaginationConfig' | Present in source |

- [__init__](pagination-config.md#init)
- [next_link](pagination-config.md#next-link)
- [page_number](pagination-config.md#page-number)
- [url_pattern](pagination-config.md#url-pattern)

<a id="init"></a>

## __init__()

Wrap an existing native pagination configuration. Calling without it raises ValueError; use next_link, page_number or url_pattern.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, _core_config=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| _core_config | Not annotated | Default: None | Native pagination handle; use the public pagination factories. |

- [Source · sdk/python/crawlingo/pagination.py:8](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/pagination.py#L8)

<a id="next-link"></a>

## next_link()

Use the href of the next-link selector.

Returns / value: 'PaginationConfig'.

### Signature

```python
def next_link(cls, selector: str) -> 'PaginationConfig'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| selector | str | Required | Selector evaluated against the fetched DOM. |

- [Source · sdk/python/crawlingo/pagination.py:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/pagination.py#L14)

<a id="page-number"></a>

## page_number()

Construct numbered URLs from a template containing {page}, starting at startPage/start_page and bounded by maxPages/max_pages.

Returns / value: 'PaginationConfig'.

### Signature

```python
def page_number(cls, url_template: str, start_page: int, max_pages: int) -> 'PaginationConfig'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url_template | str | Required | URL template containing the literal {page} placeholder. |
| start_page | int | Required | First page number used by numbered pagination. |
| max_pages | int | Required | Numbered-pagination bound passed to the native configuration. |

- [Source · sdk/python/crawlingo/pagination.py:19](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/pagination.py#L19)

<a id="url-pattern"></a>

## url_pattern()

Increment the page number captured by a regular expression in the current URL; stop at the configured maximum.

Returns / value: 'PaginationConfig'.

### Signature

```python
def url_pattern(cls, page_regex: str, max_page: int) -> 'PaginationConfig'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| page_regex | str | Required | Regular expression capturing the current page number in a URL. |
| max_page | int | Required | Upper page-number bound for pattern pagination. |

- [Source · sdk/python/crawlingo/pagination.py:24](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/pagination.py#L24)

<a id="errors"></a>

## Errors and operational limits

Invalid patterns or templates can fail when evaluated by the crawler. Validate a small URL sequence before a larger crawl.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
