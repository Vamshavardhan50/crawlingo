# Page · Python

Fetch one page and inspect its parsed HTML before defining a dataset schema.

<a id="overview"></a>

## Purpose and lifecycle

Python Page(url) is lazy; reading status, html or selectors triggers loading. Python Page.create fetches immediately. Node Page.create returns a promise; the Node constructor accepts an existing native page.

A page retains HTML and a DOM tree. Use DatasetJob for large inputs rather than keeping a list of Page objects.

HTTP statuses such as 404, 429 and 500 are returned through status, including after retries are exhausted. They do not automatically become typed HTTP exceptions.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. 1 member(s) have a known limitation; see the marked entries below.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class Page
```

- [sdk/python/crawlingo/page.py:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L6)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
from crawlingo import Page

page = Page.create("https://example.com/", timeout=30, retries=1)
print(page.status, page.title())
print(page.css("h1").texts())
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| create | static method | 'Page' | Present in source |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| url | property | str | Present in source |
| status | property | int | Present in source |
| html | method | str | Present in source |
| title | method | str | Present in source |
| css | method | ElementCollection | Present in source |
| xpath | method | ElementCollection | Present in source |
| find_text | method | ElementCollection | Present in source |
| after_text | method | ElementCollection | Present in source |
| before_text | method | ElementCollection | Present in source |
| regex | method | ElementCollection | Present in source |
| before_fetch | method | 'Page' | Present in source |
| after_fetch | method | 'Page' | Present in source |
| before_parse | method | 'Page' | Known limitation — read entry |
| after_extract | method | 'Page' | Present in source |

- [create](page.md#create)
- [__init__](page.md#init)
- [url](page.md#url)
- [status](page.md#status)
- [html](page.md#html)
- [title](page.md#title)
- [css](page.md#css)
- [xpath](page.md#xpath)
- [find_text](page.md#find-text)
- [after_text](page.md#after-text)
- [before_text](page.md#before-text)
- [regex](page.md#regex)
- [before_fetch](page.md#before-fetch)
- [after_fetch](page.md#after-fetch)
- [before_parse](page.md#before-parse)
- [after_extract](page.md#after-extract)

<a id="create"></a>

## create()

Fetch and parse the URL. Node request headers/cookies merge over the session defaults; explicit timeout, proxy and profile take precedence. Python passes its Page defaults to native code, including 30 seconds and 3 retries.

Returns / value: 'Page'.

### Signature

```python
def create(cls, url: str, auto_match: bool=False, timeout: int=30, retries: int=3, headers: dict=None, cookies: dict=None, proxy: str=None, session: Optional['Session']=None) -> 'Page'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| auto_match | bool | Default: False | Heuristic recovery option; read Page and Dataset behavior because their native paths differ. |
| timeout | int | Default: 30 | Per-attempt timeout in whole seconds; retries and pacing add elapsed time. |
| retries | int | Default: 3 | Additional request attempts after the initial one. |
| headers | dict | Default: None | Header names and values; avoid sharing credentials across unrelated sites. |
| cookies | dict | Default: None | Explicit cookie-name/value defaults. |
| proxy | str | Default: None | Per-request proxy URL; overrides the relevant session default. |
| session | Optional['Session'] | Default: None | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/python/crawlingo/page.py:11](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L11)

<a id="init"></a>

## __init__()

Create a lazy Python Page. Network access starts on create() or the first content/status/selector operation. Register hooks before triggering that load.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, url: str, auto_match: bool=False, timeout: int=30, retries: int=3, headers: dict=None, cookies: dict=None, proxy: str=None, session: Optional['Session']=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| auto_match | bool | Default: False | Heuristic recovery option; read Page and Dataset behavior because their native paths differ. |
| timeout | int | Default: 30 | Per-attempt timeout in whole seconds; retries and pacing add elapsed time. |
| retries | int | Default: 3 | Additional request attempts after the initial one. |
| headers | dict | Default: None | Header names and values; avoid sharing credentials across unrelated sites. |
| cookies | dict | Default: None | Explicit cookie-name/value defaults. |
| proxy | str | Default: None | Per-request proxy URL; overrides the relevant session default. |
| session | Optional['Session'] | Default: None | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/python/crawlingo/page.py:35](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L35)

<a id="url"></a>

## url

Source/final URL according to the operation; check redirects and provenance needs.

Returns / value: str.

### Signature

```python
def url(self) -> str
```

- [Source · sdk/python/crawlingo/page.py:107](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L107)

<a id="status"></a>

## status

HTTP response status code.

Returns / value: int.

### Signature

```python
def status(self) -> int
```

- [Source · sdk/python/crawlingo/page.py:111](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L111)

<a id="html"></a>

## html()

Load the page if necessary and return its HTML string. Inspect status separately; an HTTP error response can still have a body.

Returns / value: str.

### Signature

```python
def html(self) -> str
```

- [Source · sdk/python/crawlingo/page.py:115](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L115)

<a id="title"></a>

## title()

Return the text of the title element, or an empty string when absent.

Returns / value: str.

### Signature

```python
def title(self) -> str
```

- [Source · sdk/python/crawlingo/page.py:119](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L119)

<a id="css"></a>

## css()

Evaluate the supported CSS subset and return a collection in document order. This is not a browser querySelectorAll implementation.

Returns / value: ElementCollection.

### Signature

```python
def css(self, selector: str) -> ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| selector | str | Required | Selector evaluated against the fetched DOM. |

- [Source · sdk/python/crawlingo/page.py:123](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L123)

<a id="xpath"></a>

## xpath()

Evaluate the supported basic XPath subset; advanced XPath axes/functions are not implemented.

Returns / value: ElementCollection.

### Signature

```python
def xpath(self, query: str) -> ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| query | str | Required | Supported XPath expression. |

- [Source · sdk/python/crawlingo/page.py:127](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L127)

<a id="find-text"></a>

## find_text()

Find DOM nodes by text content using the engine text selector.

Returns / value: ElementCollection.

### Signature

```python
def find_text(self, text: str) -> ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| text | str | Required | Text anchor used for matching. |

- [Source · sdk/python/crawlingo/page.py:131](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L131)

<a id="after-text"></a>

## after_text()

Select the element after a matching text anchor; inspect sample output before relying on a repeated label.

Returns / value: ElementCollection.

### Signature

```python
def after_text(self, text: str) -> ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| text | str | Required | Text anchor used for matching. |

- [Source · sdk/python/crawlingo/page.py:135](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L135)

<a id="before-text"></a>

## before_text()

Select the element before a matching text anchor.

Returns / value: ElementCollection.

### Signature

```python
def before_text(self, text: str) -> ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| text | str | Required | Text anchor used for matching. |

- [Source · sdk/python/crawlingo/page.py:139](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L139)

<a id="regex"></a>

## regex()

Compile the Rust regular expression and match text. Invalid syntax produces SelectorError. Rust regex does not support look-around or backreferences.

Returns / value: ElementCollection.

### Signature

```python
def regex(self, pattern: str) -> ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| pattern | str | Required | Rust regular expression. |

- [Source · sdk/python/crawlingo/page.py:143](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L143)

<a id="before-fetch"></a>

## before_fetch()

Register a Python hook before lazy loading. Mutating the shared headers dictionary affects request headers; assigning a new context URL does not change the Page URL used to fetch.

Returns / value: 'Page'.

### Signature

```python
def before_fetch(self, fn) -> 'Page'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Not annotated | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/page.py:151](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L151)

<a id="after-fetch"></a>

## after_fetch()

Register a Python callback receiving a response context with status. It runs after the native fetch and parse have completed.

Returns / value: 'Page'.

### Signature

```python
def after_fetch(self, fn) -> 'Page'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Not annotated | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/page.py:155](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L155)

<a id="before-parse"></a>

## before_parse()

Known limitation: The callback runs after native parsing and its returned HTML is discarded. It does not modify the DOM.

Known limitation: this hook runs after the native DOM has already been parsed. Returned replacement HTML is not fed back into the parser; it cannot transform the parsed DOM.

Returns / value: 'Page'.

### Signature

```python
def before_parse(self, fn) -> 'Page'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Not annotated | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/page.py:159](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L159)

<a id="after-extract"></a>

## after_extract()

Apply Python transforms to extracted text. A falsy hook return currently keeps the previous value, so returning an empty string does not erase it.

Returns / value: 'Page'.

### Signature

```python
def after_extract(self, fn) -> 'Page'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| fn | Not annotated | Required | Application callback invoked with the documented value or event. |

- [Source · sdk/python/crawlingo/page.py:163](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/page.py#L163)

<a id="errors"></a>

## Errors and operational limits

Network failures reject/raise. Node maps native timeout, DNS and regex selector failures to typed errors. Missing selector matches return an empty collection. Python exception conversion is message-based.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
