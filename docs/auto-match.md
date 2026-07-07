# Auto-Match — Self-Healing Selectors

Auto-Match is Crawlingo's most distinctive feature. When a CSS selector breaks due to a page layout change, Auto-Match automatically recovers the correct element using DOM fingerprinting and similarity scoring.

## How It Works

1. **First match:** When a CSS selector finds an element, Auto-Match generates a DOM fingerprint for that element (tag name, class names, ID, attributes, parent hierarchy, text content hash).

2. **Fingerprint stored:** The fingerprint is cached in an embedded database (Sled).

3. **Recovery:** On subsequent fetches, if the CSS selector fails to find a match, Auto-Match scans all DOM elements, computes similarity scores against stored fingerprints, and returns the best match above the confidence threshold.

## Usage

```python
from crawlingo import Page, Dataset

# Enable on Page
page = Page("https://example.com").auto_match(True)
title = page.css("h1.product-title").text()  # Auto-healed if selector breaks

# Enable on Dataset
dataset = Dataset("https://example.com").auto_match(True)
dataset.field("price", "span.price-value")
result = dataset.build()  # Recovers if .price-value class changes

# Enable on Session (applies to all)
from crawlingo import Session
session = Session().auto_match(True)
page = Page("https://example.com", session=session)
```

## Fingerprint Components

The DOM fingerprint captures these features:

| Feature | Weight (default) | Description |
|---------|-----------------|-------------|
| Tag name | 1.0 | HTML tag (div, span, h1, etc.) |
| Class names | 0.8 | Ordered list of CSS classes |
| ID | 0.6 | Element ID attribute |
| Attributes | 0.4 | Key-value pairs of all attributes |
| Parent tag | 0.5 | Parent element tag |
| Depth | 0.1 | DOM depth from root |
| Text hash | 0.3 | Hash of text content (optional) |

## Custom Weights

```python
# Python
session.auto_match_weights({
    "tag": 1.0,
    "class_name": 0.9,
    "id": 0.7,
    "attributes": 0.5,
    "parent_tag": 0.6,
    "depth": 0.2,
    "text_hash": 0.4
})
```

## Confidence Threshold

The auto-matcher returns a match only if the best similarity score exceeds the confidence threshold (default: 0.7). You can adjust this:

```python
session.auto_match_confidence(0.8)  # Higher = stricter matching
```

## Fingerprint Database

Fingerprints are stored in a local Sled database:

```python
session.fingerprint_path("/tmp/crawlingo_fingerprints")
```

Default location: `.crawlingo/` in the working directory.

## Best Practices

- Enable auto-match for **production scrapers** targeting sites that may change layout
- Set higher confidence thresholds (0.8-0.9) for **critical data** (prices, stock status)
- Use lower thresholds (0.6-0.7) for **non-critical data** (sidebars, footers)
- Clear the fingerprint database if the **page structure is intentionally redesigned**
