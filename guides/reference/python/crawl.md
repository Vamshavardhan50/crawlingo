# Crawl · Python

Discover linked pages and extract a small in-memory result set.

<a id="overview"></a>

## Purpose and lifecycle

Use follow, limit, depth and concurrency to bound discovery. For a large output dataset, persist discovered URLs and pass them to DatasetJob.

Node run returns DatasetResult[]. Python build returns a CrawlResults wrapper, but the native result conversion currently has a documented mismatch; review known issues.

Persistent crawl frontier state and DatasetJob checkpoint state are different formats and APIs.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. 4 member(s) have a known limitation; see the marked entries below.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class Crawl
```

- [sdk/python/crawlingo/crawl.py:56](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L56)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| resumable | static method | 'Crawl' | Present in source |
| follow | method | 'Crawl' | Present in source |
| with_pagination | method | 'Crawl' | Present in source |
| limit | method | 'Crawl' | Present in source |
| depth | method | 'Crawl' | Known limitation — read entry |
| field | method | 'Crawl' | Present in source |
| auto_match | method | 'Crawl' | Present in source |
| concurrency | method | 'Crawl' | Present in source |
| delay | method | 'Crawl' | Present in source |
| browser | method | 'Crawl' | Present in source |
| respect_robots | method | 'Crawl' | Present in source |
| cancel | method | Not annotated; see behavior notes | Present in source |
| webhook | method | 'Crawl' | Known limitation — read entry |
| schedule | method | Not annotated; see behavior notes | Known limitation — read entry |
| build | method | CrawlResults | Known limitation — read entry |

- [__init__](crawl.md#init)
- [resumable](crawl.md#resumable)
- [follow](crawl.md#follow)
- [with_pagination](crawl.md#with-pagination)
- [limit](crawl.md#limit)
- [depth](crawl.md#depth)
- [field](crawl.md#field)
- [auto_match](crawl.md#auto-match)
- [concurrency](crawl.md#concurrency)
- [delay](crawl.md#delay)
- [browser](crawl.md#browser)
- [respect_robots](crawl.md#respect-robots)
- [cancel](crawl.md#cancel)
- [webhook](crawl.md#webhook)
- [schedule](crawl.md#schedule)
- [build](crawl.md#build)

<a id="init"></a>

## __init__()

Create a bounded link-discovery builder. Configure fields, page/depth limits and pacing before running it. Python can also wrap a supplied native crawl handle.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, start_url: str, session: Optional[Session]=None, _core_crawl=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| start_url | str | Required | Initial page used for crawl discovery. |
| session | Optional[Session] | Default: None | Shared request state. Omit only where the signature provides a default. |
| _core_crawl | Not annotated | Default: None | Optional native crawl handle; normally supplied by the resumable factory. |

- [Source · sdk/python/crawlingo/crawl.py:60](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L60)

<a id="resumable"></a>

## resumable()

Use a persistent frontier directory. Python argument order is start_url, path, session; Node order is startUrl, session, path.

Returns / value: 'Crawl'.

### Signature

```python
def resumable(cls, start_url: str, path: str, session: Optional[Session]=None) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| start_url | str | Required | Initial page used for crawl discovery. |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |
| session | Optional[Session] | Default: None | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/python/crawlingo/crawl.py:71](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L71)

<a id="follow"></a>

## follow()

Set the CSS selector for links to enqueue. The link’s href is resolved against the page URL.

Returns / value: 'Crawl'.

### Signature

```python
def follow(self, selector: str) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| selector | str | Required | Selector evaluated against the fetched DOM. |

- [Source · sdk/python/crawlingo/crawl.py:83](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L83)

<a id="with-pagination"></a>

## with_pagination()

Attach a PaginationConfig built with one of its factories.

Returns / value: 'Crawl'.

### Signature

```python
def with_pagination(self, config: PaginationConfig) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| config | PaginationConfig | Required | Configuration object; see the linked job or pagination reference. |

- [Source · sdk/python/crawlingo/crawl.py:91](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L91)

<a id="limit"></a>

## limit()

Set a maximum page count for this crawl.

Returns / value: 'Crawl'.

### Signature

```python
def limit(self, pages: int) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| pages | int | Required | Maximum downstream crawl pages. |

- [Source · sdk/python/crawlingo/crawl.py:99](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L99)

<a id="depth"></a>

## depth()

Known limitation: The wrapper calls native depth(), but PyCrawl exposes max_depth(). This method cannot currently be used through the public wrapper.

Set the maximum link-hop depth from the starting URL.

Returns / value: 'Crawl'.

### Signature

```python
def depth(self, max_depth: int) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| max_depth | int | Required | Maximum sitemap nesting or crawl depth, according to the API. |

- [Source · sdk/python/crawlingo/crawl.py:107](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L107)

<a id="field"></a>

## field()

Append an extraction field evaluated on each crawled page.

Returns / value: 'Crawl'.

### Signature

```python
def field(self, name: str, selector: str, selector_type: str='css', default: Optional[str]=None, extract_type: str='text') -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |
| selector | str | Required | Selector evaluated against the fetched DOM. |
| selector_type | str | Default: 'css' | Selector engine for this field; supported values differ between builders. |
| default | Optional[str] | Default: None | Fallback field value when extraction is empty. |
| extract_type | str | Default: 'text' | Extraction transformation such as text, price, datetime, url or attr:name, where supported. |

- [Source · sdk/python/crawlingo/crawl.py:115](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L115)

<a id="auto-match"></a>

## auto_match()

Set heuristic recovery on the associated session.

Returns / value: 'Crawl'.

### Signature

```python
def auto_match(self, enabled: bool) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | bool | Required | Enable or disable this behavior. |

- [Source · sdk/python/crawlingo/crawl.py:136](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L136)

<a id="concurrency"></a>

## concurrency()

Set concurrent crawl workers. Start small and respect source pacing.

Returns / value: 'Crawl'.

### Signature

```python
def concurrency(self, n: int) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| n | int | Required | Count or index; see the method description. |

- [Source · sdk/python/crawlingo/crawl.py:141](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L141)

<a id="delay"></a>

## delay()

Set request spacing in seconds; this is separate from retry backoff.

Returns / value: 'Crawl'.

### Signature

```python
def delay(self, seconds: float) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | float | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/python/crawlingo/crawl.py:149](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L149)

<a id="browser"></a>

## browser()

Choose the browser transport for this session. Requires a browser-enabled native build and local Chromium.

Returns / value: 'Crawl'.

### Signature

```python
def browser(self, enabled: bool=True) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | bool | Default: True | Enable or disable this behavior. |

- [Source · sdk/python/crawlingo/crawl.py:157](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L157)

<a id="respect-robots"></a>

## respect_robots()

Set whether robots rules are respected; enabled by default.

Returns / value: 'Crawl'.

### Signature

```python
def respect_robots(self, enabled: bool) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | bool | Required | Enable or disable this behavior. |

- [Source · sdk/python/crawlingo/crawl.py:181](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L181)

<a id="cancel"></a>

## cancel()

Request cancellation of the crawler. Await completion before restarting or reusing its state.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def cancel(self)
```

- [Source · sdk/python/crawlingo/crawl.py:189](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L189)

<a id="webhook"></a>

## webhook()

Known limitation: The wrapper calls webhook(), which is not exported by PyCrawl. The Rust and Node APIs expose a webhook setting.

Send results to an application-owned endpoint. Delivery is not an exactly-once queue and has no durable retry receipt API.

Returns / value: 'Crawl'.

### Signature

```python
def webhook(self, url: str) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/python/crawlingo/crawl.py:196](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L196)

<a id="schedule"></a>

## schedule()

Known limitation: The wrapper calls schedule(), which is not exported by PyCrawl. Use an application scheduler around a supported operation.

Start an unmanaged native background thread that repeats the crawl. Node exposes no stop/join handle for it. Prefer a worker scheduler that owns one run at a time.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def schedule(self, interval_seconds: int)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| interval_seconds | int | Required | Repeat interval in seconds. See lifecycle restrictions before scheduling. |

- [Source · sdk/python/crawlingo/crawl.py:204](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L204)

<a id="build"></a>

## build()

Known limitation: PyCrawl.build returns dictionaries, while this wrapper constructs DatasetResult objects expecting native result handles. Their to_dict/export calls can fail. Prefer DatasetJob for production file exports.

Run the Python crawl synchronously. Current wrapper/native result types differ; see the known issue on this member.

Returns / value: CrawlResults.

### Signature

```python
def build(self) -> CrawlResults
```

- [Source · sdk/python/crawlingo/crawl.py:211](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/crawl.py#L211)

<a id="errors"></a>

## Errors and operational limits

Some page failures are logged while crawling continues; Node run does not expose the complete native CrawlResult diagnostics. A result list alone is not proof that every discovered URL succeeded.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
