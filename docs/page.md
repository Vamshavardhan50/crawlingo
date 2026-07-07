# Page API

The `Page` object represents a fetched web page with a parsed DOM tree. It is the primary interface for extracting data from a single URL.

## Constructor

```python
# Python
from crawlingo import Page
page = Page(url, session=None)
```

```javascript
// Node.js
const { Page } = require('crawlingo');
const page = await Page.create(url, session?);
```

```rust
// Rust
use crawlingo::Page;
let page = Page::new(url)?;
```

**Parameters:**

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | Target URL to fetch |
| `session` | `Session` | `None` | Shared configuration (headers, proxy, rate limit, etc.) |

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `status` | `int` | HTTP response status code |
| `url` | `str` | Final URL after redirects |
| `html()` | `str` | Raw HTML content |
| `markdown()` | `str` | Clean markdown conversion of page content |

## CSS Selectors

```python
# Python
elements = page.css("h1")
elements = page.css("div.price")
elements = page.css("#main-container")
elements = page.css("div > p:first-child")

# Access results
for el in elements:
    print(el.text())       # Inner text
    print(el.html())       # Inner HTML
    print(el.attr("href")) # Attribute value

# First match shorthand
first = page.css_first("h1")
```

```javascript
// Node.js
const elements = page.css('h1');
console.log(elements.text);       // Array of texts
console.log(elements.at(0).html); // Inner HTML
console.log(elements.first);      // First match
```

**Supported selectors:** See [Selectors Guide](selectors.md) for full syntax.

## XPath Selectors

```python
# Python
elements = page.xpath("//h1")
elements = page.xpath("//div[@class='price']")
elements = page.xpath("//a/@href")
elements = page.xpath("//p[position()<3]")

for el in elements:
    print(el.text())
```

```javascript
// Node.js
const elements = page.xpath('//h1');
console.log(elements.at(0).text);
```

## Regex Selectors

```python
# Python
emails = page.regex(r'[\w.+-]+@[\w-]+\.[\w.]+')
phones = page.regex(r'\+?1?\d{10,14}')

for match in emails:
    print(match.text())
```

## Text Anchor Selectors

```python
# Python
# Find by text content (SIMD-accelerated)
el = page.find_text("Buy Now")
el = page.find_text("Add to cart", case_sensitive=False)

# Text boundaries
price = page.after_text("Price:")
name = page.before_text(" - Product Details")

# Between two texts
desc = page.between_texts("Description:", "Reviews:")
```

```javascript
// Node.js
const el = page.findText('Buy Now');
const nextEl = page.afterText('Price:');
```

## Extraction Types

Apply type transformations to extracted values:

```python
# Python
price = page.css("span.price").extract("price")
# "$1,234.56" → "1234.56"

date = page.css("time").extract("datetime")
# "Jan 15, 2024" → "2024-01-15"

url = page.css("a").extract("url")
# "/path" → "https://base.com/path"

links = page.css("a[href]").extract("datalink_url")
emails = page.css("[href^=mailto]").extract("datalink_email")
phones = page.css("[href^=tel]").extract("datalink_phone")
```

**Extraction types:** `text`, `price`, `datetime`, `url`, `datalink_url`, `datalink_email`, `datalink_phone`.

## Lifecycle Hooks

```python
# Python
from crawlingo import Page
from crawlingo.hooks import strip_whitespace, uppercase

page = (
    Page("https://example.com")
    .before_fetch(log_request)
    .before_parse(lambda html: html.replace("Old", "New"))
    .after_extract(strip_whitespace)
    .after_extract(uppercase)
)
```

## Error Handling

```python
# Python
from crawlingo import Page, CrawlingoError

try:
    page = Page("https://example.com")
    title = page.css("h1").text()
except CrawlingoError as e:
    print(f"Scraping failed: {e}")
```

| Exception | Cause |
|-----------|-------|
| `ConnectionError` | DNS resolution or TCP connection failure |
| `TimeoutError` | Request exceeded timeout |
| `HttpError` | Unsuccessful HTTP status (4xx/5xx) |
| `SelectorError` | Invalid selector syntax |
| `ParseError` | HTML parsing failure |
