# crawlingo.Page

Downloaded web page with a parsed, immutable DOM tree and a suite of selector methods.

## Constructor

```python
page = Page(url, auto_match=False, timeout=30, retries=3, headers=None, cookies=None, proxy=None, session=None)
```

### Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `url` | `str` | — | Target URL to fetch |
| `auto_match` | `bool` | `False` | Enable self-healing selector fallback |
| `timeout` | `int` | `30` | Request timeout in seconds |
| `retries` | `int` | `3` | Number of automatic retries on transient failures |
| `headers` | `dict[str, str]` | `None` | Custom HTTP headers |
| `cookies` | `dict[str, str]` | `None` | Custom cookies |
| `proxy` | `str` | `None` | Proxy URL |
| `session` | `Session` | `None` | Session to inherit settings from |

If a `session` is provided, its headers, cookies, proxy, auto_match, and timeout settings are used as defaults, with explicit constructor parameters taking precedence.

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `.url` | `str` | Fully resolved target URL (after redirects) |
| `.status` | `int` | HTTP status code of the response |

## Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.html()` | `str` | Raw HTML source of the page |
| `.title()` | `str` | Content of `<title>` tag |
| `.css(selector: str)` | `ElementCollection` | Query by CSS selector |
| `.xpath(query: str)` | `ElementCollection` | Query by XPath expression |
| `.find_text(text: str)` | `ElementCollection` | Find nodes containing exact text |
| `.after_text(text: str)` | `ElementCollection` | Elements following a text anchor |
| `.before_text(text: str)` | `ElementCollection` | Elements preceding a text anchor |
| `.regex(pattern: str)` | `ElementCollection` | Match text against regex pattern |

## Hooks

| Method | Returns | Description |
|--------|---------|-------------|
| `.before_fetch(fn)` | `Page` | Register hook called before HTTP fetch |
| `.after_fetch(fn)` | `Page` | Register hook called after HTTP fetch |
| `.before_parse(fn)` | `Page` | Register hook called before HTML parsing |
| `.after_extract(fn)` | `Page` | Register hook called after value extraction |

## Examples

```python
from crawlingo import Page, Session

# Direct creation
page = Page("https://example.com", auto_match=True)

# Via session
session = Session()
session.auto_match(True)
page = session.page("https://example.com/products")

# CSS selectors
titles = page.css("h2.product-title")
for el in titles:
    print(el.text())

# XPath
prices = page.xpath("//span[@class='price']")

# Text anchors
sku = page.after_text("SKU:")

# Regex
emails = page.regex(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")
```

## See Also

- [`Session`](session.md): Shared configuration container
- [`Element`](sdk_python.md#element): Single DOM element with traversal
- [`ElementCollection`](sdk_python.md#elementcollection): Collection of DOM elements
