# Fetching Pages and Selecting Elements

This guide covers how to fetch web pages and query their DOM tree using Crawlingo's selector engine.

## Fetching a Page

The simplest way to fetch a page:

```python
from crawlingo import Page

page = Page("https://example.com")
print(page.status)    # 200
print(page.html())    # Raw HTML
```

### With a Session

For shared configuration across multiple requests:

```python
from crawlingo import Session

session = Session()
session.headers({"User-Agent": "MyBot/1.0"})
session.timeout(10)

page = session.page("https://example.com")
```

## Querying the DOM Tree

### CSS Selectors

Select elements using familiar CSS syntax:

```python
# Simple tag selector
page.css("h1")

# Class selector
page.css("div.product")

# Descendant combinator
page.css("div.product h2.title")

# ID selector
page.css("#main-content")
```

### XPath

For complex structural queries:

```python
# All div elements with class 'product'
page.xpath("//div[@class='product']")

# Direct child span inside a div
page.xpath("//div/span")
```

### Regex

Search text nodes with regular expressions:

```python
# Find prices
page.regex(r"\$\d+\.\d{2}")

# Find email addresses
page.regex(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")
```

### Text Anchors

Find elements by their text content:

```python
# Find nodes containing exact text
page.find_text("Add to cart")

# Get element before a text anchor
page.before_text("Price:")

# Get element after a text anchor
page.after_text("Description:")
```

## Working with ElementCollections

All selector methods return an `ElementCollection`:

```python
collection = page.css("a.link")

for element in collection:
    print(element.text())
    print(element.attrs())
```

## Performance Considerations

- CSS and Regex selectors are **cached** globally — repeated queries with the same string skip parsing.
- The DOM tree is a **contiguous flat vector** (`Vec<DomNode>`), ensuring cache-friendly traversal.
- Text-anchor search uses SIMD (`memchr`) for near-constant-time pattern scanning.

## Next Steps

- [Dataset Extraction](03_dataset_extraction.md): Extract structured data using schemas.
- [Self-Healing Selectors](04_auto_healing.md): Automatically recover from broken selectors.
- [Selector Engine Reference](../api_reference/selector_engine.md): Full API details.
