# Element · Python

Read one selected DOM element and its attributes.

<a id="overview"></a>

## Purpose and lifecycle

Obtain elements from a Page collection rather than constructing wrappers around native handles yourself.

Node uses text and html properties. Python uses text() and html() methods and exposes additional traversal helpers. Python Element is available through crawlingo.element and as a collection result, not as a top-level import.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Reachable result/callback type; not a top-level package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class Element
```

- [sdk/python/crawlingo/element.py:4](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L4)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
heading = page.css("h1").first()
if heading is not None:
    print(heading.text(), heading.attr("id"))
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| text | method | str | Present in source |
| html | method | str | Present in source |
| attr | method | str | Present in source |
| attrs | method | dict | Present in source |
| parent | method | 'Element' | Present in source |
| children | method | 'ElementCollection' | Present in source |
| next | method | 'Element' | Present in source |
| prev | method | 'Element' | Present in source |
| siblings | method | 'ElementCollection' | Present in source |
| __repr__ | method | str | Present in source |

- [__init__](element.md#init)
- [text](element.md#text)
- [html](element.md#html)
- [attr](element.md#attr)
- [attrs](element.md#attrs)
- [parent](element.md#parent)
- [children](element.md#children)
- [next](element.md#next)
- [prev](element.md#prev)
- [siblings](element.md#siblings)
- [__repr__](element.md#repr)

<a id="init"></a>

## __init__()

Wrap a native DOM element and optional extraction hooks. Normally obtained through Page selectors and collection access.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, core_element: _CoreElement, after_extract_hooks: List[Callable]=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| core_element | _CoreElement | Required | Native DOM element handle. |
| after_extract_hooks | List[Callable] | Default: None | Python string callbacks applied after text extraction. Empty return values retain the previous string. |

- [Source · sdk/python/crawlingo/element.py:8](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L8)

<a id="text"></a>

## text()

Read descendant text in document order. Python after-extract hooks apply to this value.

Returns / value: str.

### Signature

```python
def text(self) -> str
```

- [Source · sdk/python/crawlingo/element.py:17](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L17)

<a id="html"></a>

## html()

Read the reconstructed outer HTML for the selected node.

Returns / value: str.

### Signature

```python
def html(self) -> str
```

- [Source · sdk/python/crawlingo/element.py:21](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L21)

<a id="attr"></a>

## attr()

Look up one attribute by name. Missing values return null in Node or None in Python.

Returns / value: str.

### Signature

```python
def attr(self, name: str) -> str
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |

- [Source · sdk/python/crawlingo/element.py:25](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L25)

<a id="attrs"></a>

## attrs()

Return all attributes as a dictionary.

Returns / value: dict.

### Signature

```python
def attrs(self) -> dict
```

- [Source · sdk/python/crawlingo/element.py:29](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L29)

<a id="parent"></a>

## parent()

Return the parent element, or None when absent.

Returns / value: 'Element'.

### Signature

```python
def parent(self) -> 'Element'
```

- [Source · sdk/python/crawlingo/element.py:33](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L33)

<a id="children"></a>

## children()

Return the child-element collection.

Returns / value: 'ElementCollection'.

### Signature

```python
def children(self) -> 'ElementCollection'
```

- [Source · sdk/python/crawlingo/element.py:38](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L38)

<a id="next"></a>

## next()

Return the next sibling, or None.

Returns / value: 'Element'.

### Signature

```python
def next(self) -> 'Element'
```

- [Source · sdk/python/crawlingo/element.py:42](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L42)

<a id="prev"></a>

## prev()

Return the previous sibling, or None.

Returns / value: 'Element'.

### Signature

```python
def prev(self) -> 'Element'
```

- [Source · sdk/python/crawlingo/element.py:47](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L47)

<a id="siblings"></a>

## siblings()

Return sibling elements as a collection.

Returns / value: 'ElementCollection'.

### Signature

```python
def siblings(self) -> 'ElementCollection'
```

- [Source · sdk/python/crawlingo/element.py:52](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L52)

<a id="repr"></a>

## __repr__()

Return a short debugging representation of the wrapper. This is not a data export or a stable serialization format.

Returns / value: str.

### Signature

```python
def __repr__(self) -> str
```

- [Source · sdk/python/crawlingo/element.py:56](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L56)

<a id="errors"></a>

## Errors and operational limits

Absent attributes and absent relatives may return null/None even where a Python annotation is narrower. Node exposes only text, html and attr; do not translate Python traversal calls directly.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
