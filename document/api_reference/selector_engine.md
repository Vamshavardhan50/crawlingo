# crawlingo Selector Engine

Unified query engine supporting five selector strategies for DOM element retrieval.

## Selector Types

| Type | Prefix | Example | Description |
|------|--------|---------|-------------|
| **CSS** | (none) | `div.product h2.title` | Standard CSS selector |
| **XPath** | `xpath=` | `xpath=//div[@class='price']` | XPath 1.0 expression |
| **Text** | `text=` | `text=Welcome` | Elements whose text content equals the query |
| **After Text** | `after=` | `after=SKU:` | First sibling following a text anchor |
| **Before Text** | `before=` | `before=Reviews` | First sibling preceding a text anchor |
| **Regex** | `re=` or `regex=` | `re=[A-Z]{2}\d{4}` | Elements whose text matches a regex |

## Selection Priority

When multiple selectors are provided, the engine tries each in order and stops at the first non-empty result:

1. CSS selectors first
2. XPath expressions second
3. Text matchers third
4. After/Before text anchors next
5. Regex patterns last

## Auto-Match Integration

When auto-match is enabled and a primary selector fails, the engine uses fuzzy matching against known fingerprints to find a replacement selector. See [Auto Matcher](auto_matcher.md).

## Usage

```python
# Page methods accept raw selector strings
page.css("div.product")
page.xpath("//div[@class='product']")
page.find_text("Welcome")
page.after_text("SKU:")
page.before_text("Reviews")
page.regex(r"\d+\.\d{2}")

# Dataset/Crawl field selector_type parameter
dataset.field("price", "span.price", selector_type="css")
dataset.field("price", "//span[@class='price']", selector_type="xpath")
```

## See Also

- [Auto Matcher](auto_matcher.md): Self-healing selector recovery
- [Page](page.md): Page methods that use the selector engine
- [Element](sdk_python.md#element): DOM element result of a selector query
