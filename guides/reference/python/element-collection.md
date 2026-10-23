# ElementCollection · Python

Read, iterate and select from multiple DOM matches.

<a id="overview"></a>

## Purpose and lifecycle

Node text/html properties return arrays. Python text() concatenates text, while texts() returns a list. Node attr(name) returns one value per match; Python attr(name) reads the first match.

Node indices are zero-based. first()/at() return null when no element exists; Python first()/last()/nth() return None.

Python filter/map run application callbacks in Python. Known issue: iteration over a materialized filtered collection currently exits without yielding its stored items; use texts() or explicit nth() access until corrected.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. 1 member(s) have a known limitation; see the marked entries below.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class ElementCollection
```

- [sdk/python/crawlingo/element.py:60](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L60)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
links = page.css("a")
print(len(links), links.texts())
for link in links:
    print(link.attr("href"))
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| text | method | str | Present in source |
| texts | method | List[str] | Present in source |
| attr | method | str | Present in source |
| attrs | method | dict | Present in source |
| first | method | Optional[Element] | Present in source |
| last | method | Optional[Element] | Present in source |
| nth | method | Optional[Element] | Present in source |
| __getitem__ | method | Optional[Element] | Present in source |
| __iter__ | method | Iterator[Element] | Known limitation — read entry |
| __len__ | method | int | Present in source |
| filter | method | 'ElementCollection' | Present in source |
| map | method | list | Present in source |
| __repr__ | method | str | Present in source |

- [__init__](element-collection.md#init)
- [text](element-collection.md#text)
- [texts](element-collection.md#texts)
- [attr](element-collection.md#attr)
- [attrs](element-collection.md#attrs)
- [first](element-collection.md#first)
- [last](element-collection.md#last)
- [nth](element-collection.md#nth)
- [__getitem__](element-collection.md#getitem)
- [__iter__](element-collection.md#iter)
- [__len__](element-collection.md#len)
- [filter](element-collection.md#filter)
- [map](element-collection.md#map)
- [__repr__](element-collection.md#repr)

<a id="init"></a>

## __init__()

Wrap a native match collection or materialized Python Element list. See the known iteration limitation for materialized lists.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, core_collection: _CoreElementCollection=None, after_extract_hooks: List[Callable]=None, items: List[Element]=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| core_collection | _CoreElementCollection | Default: None | Native collection of matched DOM elements. |
| after_extract_hooks | List[Callable] | Default: None | Python string callbacks applied after text extraction. Empty return values retain the previous string. |
| items | List[Element] | Default: None | Materialized Python Element list. Read the iteration limitation before using it. |

- [Source · sdk/python/crawlingo/element.py:64](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L64)

<a id="text"></a>

## text()

Node: array of each match’s text. Python: concatenated text for the collection.

Returns / value: str.

### Signature

```python
def text(self) -> str
```

- [Source · sdk/python/crawlingo/element.py:79](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L79)

<a id="texts"></a>

## texts()

Return each selected element’s text as a Python list.

Returns / value: List[str].

### Signature

```python
def texts(self) -> List[str]
```

- [Source · sdk/python/crawlingo/element.py:85](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L85)

<a id="attr"></a>

## attr()

Node: array of attribute values or null. Python: attribute of the first match, or None.

Returns / value: str.

### Signature

```python
def attr(self, name: str) -> str
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |

- [Source · sdk/python/crawlingo/element.py:91](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L91)

<a id="attrs"></a>

## attrs()

Return attributes of the first Python match, or an empty dictionary.

Returns / value: dict.

### Signature

```python
def attrs(self) -> dict
```

- [Source · sdk/python/crawlingo/element.py:97](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L97)

<a id="first"></a>

## first()

Return the first match, or null/None.

Returns / value: Optional[Element].

### Signature

```python
def first(self) -> Optional[Element]
```

- [Source · sdk/python/crawlingo/element.py:103](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L103)

<a id="last"></a>

## last()

Return the final match, or None.

Returns / value: Optional[Element].

### Signature

```python
def last(self) -> Optional[Element]
```

- [Source · sdk/python/crawlingo/element.py:110](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L110)

<a id="nth"></a>

## nth()

Return a zero-based match, or None if out of bounds.

Returns / value: Optional[Element].

### Signature

```python
def nth(self, n: int) -> Optional[Element]
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| n | int | Required | Count or index; see the method description. |

- [Source · sdk/python/crawlingo/element.py:117](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L117)

<a id="getitem"></a>

## __getitem__()

Delegate to nth(index). Use a non-negative index; out-of-range matches return None.

Returns / value: Optional[Element].

### Signature

```python
def __getitem__(self, index: int) -> Optional[Element]
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| index | int | Required | Zero-based element index. |

- [Source · sdk/python/crawlingo/element.py:124](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L124)

<a id="iter"></a>

## __iter__()

Known limitation: For a materialized collection (for example filter() output), this generator returns an iterator instead of yielding from it. Iteration/map can produce no items.

Iterate over matched Element wrappers in collection order. See any known limitation above before using a materialized Python collection.

Returns / value: Iterator[Element].

### Signature

```python
def __iter__(self) -> Iterator[Element]
```

- [Source · sdk/python/crawlingo/element.py:127](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L127)

<a id="len"></a>

## __len__()

Return the number of stored results or matched elements; no additional pages are fetched.

Returns / value: int.

### Signature

```python
def __len__(self) -> int
```

- [Source · sdk/python/crawlingo/element.py:133](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L133)

<a id="filter"></a>

## filter()

Build a materialized Python collection from elements satisfying the predicate. See the iteration limitation above.

Returns / value: 'ElementCollection'.

### Signature

```python
def filter(self, fn: Callable[[Element], bool]) -> 'ElementCollection'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Callable[[Element], bool] | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/element.py:138](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L138)

<a id="map"></a>

## map()

Return a Python list of callback results; see the materialized-collection iteration limitation above.

Returns / value: list.

### Signature

```python
def map(self, fn: Callable[[Element], any]) -> list
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Callable[[Element], any] | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/element.py:143](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L143)

<a id="repr"></a>

## __repr__()

Return a short debugging representation of the wrapper. This is not a data export or a stable serialization format.

Returns / value: str.

### Signature

```python
def __repr__(self) -> str
```

- [Source · sdk/python/crawlingo/element.py:147](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/element.py#L147)

<a id="errors"></a>

## Errors and operational limits

An empty match is valid. Reject negative indices in your Node caller; the native index type is unsigned. Callback exceptions can propagate from Python map/filter.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
