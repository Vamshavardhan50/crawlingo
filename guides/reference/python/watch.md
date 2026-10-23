# Watch · Python

Poll selected fields and deliver detected changes to application callbacks.

<a id="overview"></a>

## Purpose and lifecycle

The first successful sample establishes a baseline; later samples are compared with it. Stop the watcher explicitly when the worker shuts down.

Node run takes an error-first callback. Python registers callbacks with on_change and typed event helpers; run blocks and run_async uses an executor.

Set a positive interval in whole seconds. Zero is not a supported polling interval. Callback delivery and baseline state are not a durable event log.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class Watch
```

- [sdk/python/crawlingo/watch.py:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L6)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
from crawlingo import Watch
watch = Watch("https://example.com/").field("title", "h1").interval(60)
watch.on_change(lambda event: print(event.field, event.old_value, event.new_value))
try:
    watch.run()
finally:
    watch.stop()
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| field | method | 'Watch' | Present in source |
| interval | method | 'Watch' | Present in source |
| auto_match | method | 'Watch' | Present in source |
| on_change | method | 'Watch' | Present in source |
| on_price_change | method | 'Watch' | Present in source |
| on_stock_change | method | 'Watch' | Present in source |
| on_element_added | method | 'Watch' | Present in source |
| on_element_removed | method | 'Watch' | Present in source |
| run | method | Not annotated; see behavior notes | Present in source |
| run_async | method | Not annotated; see behavior notes | Present in source |
| stop | method | Not annotated; see behavior notes | Present in source |

- [__init__](watch.md#init)
- [field](watch.md#field)
- [interval](watch.md#interval)
- [auto_match](watch.md#auto-match)
- [on_change](watch.md#on-change)
- [on_price_change](watch.md#on-price-change)
- [on_stock_change](watch.md#on-stock-change)
- [on_element_added](watch.md#on-element-added)
- [on_element_removed](watch.md#on-element-removed)
- [run](watch.md#run)
- [run_async](watch.md#run-async)
- [stop](watch.md#stop)

<a id="init"></a>

## __init__()

Create a monitor with a URL and optional session. Add fields and callbacks before starting the polling loop with run.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, url: str, session: Optional[Session]=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| session | Optional[Session] | Default: None | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/python/crawlingo/watch.py:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L10)

<a id="field"></a>

## field()

Add a named field to compare between polls. Configure all fields before starting.

Returns / value: 'Watch'.

### Signature

```python
def field(self, name: str, selector: str, selector_type: str='css', transform: Optional[Callable[[str], str]]=None, default: Optional[str]=None) -> 'Watch'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |
| selector | str | Required | Selector evaluated against the fetched DOM. |
| selector_type | str | Default: 'css' | Selector engine for this field; supported values differ between builders. |
| transform | Optional[Callable[[str], str]] | Default: None | Python callback applied to the extracted string; application exceptions propagate. |
| default | Optional[str] | Default: None | Fallback field value when extraction is empty. |

- [Source · sdk/python/crawlingo/watch.py:17](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L17)

<a id="interval"></a>

## interval()

Set a positive poll interval in seconds. This is not a request timeout.

Returns / value: 'Watch'.

### Signature

```python
def interval(self, seconds: int) -> 'Watch'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | int | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/python/crawlingo/watch.py:38](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L38)

<a id="auto-match"></a>

## auto_match()

Enable heuristic Dataset selector recovery on the shared session.

Returns / value: 'Watch'.

### Signature

```python
def auto_match(self, enabled: bool) -> 'Watch'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | bool | Required | Enable or disable this behavior. |

- [Source · sdk/python/crawlingo/watch.py:46](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L46)

<a id="on-change"></a>

## on_change()

Register a Python callback for change events.

Returns / value: 'Watch'.

### Signature

```python
def on_change(self, fn: Callable) -> 'Watch'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Callable | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/watch.py:51](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L51)

<a id="on-price-change"></a>

## on_price_change()

Register a callback for detected price changes.

Returns / value: 'Watch'.

### Signature

```python
def on_price_change(self, fn: Callable) -> 'Watch'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Callable | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/watch.py:59](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L59)

<a id="on-stock-change"></a>

## on_stock_change()

Register a callback for stock status changes.

Returns / value: 'Watch'.

### Signature

```python
def on_stock_change(self, fn: Callable) -> 'Watch'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Callable | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/watch.py:67](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L67)

<a id="on-element-added"></a>

## on_element_added()

Register a callback for added-element events.

Returns / value: 'Watch'.

### Signature

```python
def on_element_added(self, fn: Callable) -> 'Watch'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Callable | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/watch.py:75](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L75)

<a id="on-element-removed"></a>

## on_element_removed()

Register a callback for removed-element events.

Returns / value: 'Watch'.

### Signature

```python
def on_element_removed(self, fn: Callable) -> 'Watch'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Callable | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/watch.py:83](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L83)

<a id="run"></a>

## run()

Start polling. Node returns immediately and delivers callbacks; Python blocks.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def run(self)
```

- [Source · sdk/python/crawlingo/watch.py:91](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L91)

<a id="run-async"></a>

## run_async()

Run the blocking Python poll loop in an executor. Cancellation of the asyncio task alone is not a substitute for stop().

Returns / value: Not annotated; see behavior notes.

### Signature

```python
async def run_async(self)
```

- [Source · sdk/python/crawlingo/watch.py:98](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L98)

<a id="stop"></a>

## stop()

Request cancellation of the polling loop. It does not erase a separately owned dataset directory.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def stop(self)
```

- [Source · sdk/python/crawlingo/watch.py:104](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/watch.py#L104)

<a id="errors"></a>

## Errors and operational limits

Node reports poll failures to the callback. Handle err before reading the event. Python native polling logs some failures; it is not equivalent to a rejected DatasetJob.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
