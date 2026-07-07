# Selectors Guide

Crawlingo supports four selector engines: **CSS**, **XPath**, **Regex**, and **Text Anchor** (SIMD-accelerated).

## CSS Selectors

Standard CSS selector syntax powered by `scraper` crate.

```python
page.css("h1")                    # Tag name
page.css(".product-title")        # Class
page.css("#main-content")         # ID
page.css("[data-id]")             # Attribute presence
page.css("[href^=https]")         # Attribute starts-with
page.css("[class~=highlight]")    # Attribute contains word
page.css("div > p")               # Direct child
page.css("div p")                 # Descendant
page.css("h1, h2, h3")           # Multiple selectors
page.css("ul li:first-child")     # Pseudo-class
page.css("tr:nth-child(even)")    # Positional
page.css(":not(.hidden)")         # Negation
page.css("input[type=text]")      # Attribute equals
page.css("a[href$=.pdf]")         # Attribute ends-with
page.css("div[data-*]")           # Attribute contains substring
```

## XPath Selectors

Full XPath 1.0 support for complex traversals.

```python
page.xpath("//h1")                            # All h1 elements
page.xpath("//div[@class='price']")            # Attribute filter
page.xpath("//a/@href")                        # Extract attribute
page.xpath("//p[position()<3]")                # Positional
page.xpath("//div[contains(@class,'active')]") # Contains
page.xpath("//p/text()")                       # Direct text nodes
page.xpath("//*[@id='main']//a")               # Descendant
page.xpath("//div[not(@class)]")               # Negation
page.xpath("//a | //button")                   # Union
page.xpath("//table/tbody/tr[1]/td[2]")        # Table navigation
```

## Regex Selectors

Extract text patterns from the raw HTML or rendered text.

```python
# From rendered text
page.regex(r'\b[A-Z][a-z]+ [A-Z][a-z]+\b')        # Proper names
page.regex(r'[\w.+-]+@[\w-]+\.[\w.]+')             # Emails
page.regex(r'\+\d{1,3}\s?\(?\d{3}\)?\s?\d{3}[-.]?\d{4}')  # Phones
page.regex(r'(?:https?://)?[\w.-]+\.\w{2,}(?:/\S*)?')      # URLs

# Limits and extraction types
page.regex(r'SKU-\d+').first()           # First match only
page.regex(r'\$[\d,]+\.?\d*').extract("price")  # With type conversion
```

## Text Anchor Selectors

SIMD-accelerated text boundary detection using `memchr`.

```python
# Find element by text content
page.find_text("Buy Now")                     # Case-sensitive
page.find_text("add to cart", case_sensitive=False)

# Text boundaries (returns sibling/adjancent element)
page.after_text("Price:")                      # Element after text
page.before_text(" - Product Details")         # Element before text
page.between_texts("Description:", "Reviews:")  # Elements between two texts

# Combined with regex anchors
page.after_text(page.regex(r'Price:\s+\$[\d.]+').first().text())
```

## Performance Characteristics

| Selector | Speed | Flexibility | Use When |
|----------|-------|-------------|----------|
| CSS | Fastest | Moderate | Stable page structures |
| XPath | Fast | Highest | Complex DOM traversal |
| Regex | Moderate | High | Pattern matching in text |
| Text Anchor | Fastest (SIMD) | Low | Known text content near target |

## Pseudo-classes Reference

| Pseudo-class | Description |
|-------------|-------------|
| `:first-child` | First child of parent |
| `:last-child` | Last child of parent |
| `:nth-child(n)` | Nth child (1-indexed) |
| `:nth-of-type(n)` | Nth child of its type |
| `:not(selector)` | Negation |
| `:empty` | Elements with no children |
| `:contains(text)` | Elements containing text (non-standard) |
