# Self-Healing Selectors

Crawlingo can automatically recover from broken CSS/XPath selectors by matching DOM fingerprints. This makes scrapers resilient to website layout changes.

## How It Works

1. **First run**: When a selector successfully matches elements, Crawlingo computes a DOM fingerprint (tag, attributes, text content, position, depth) and stores it as a JSON file in the fingerprint directory.
2. **Subsequent runs**: If the selector returns zero results, the `AutoMatcher` scans the current DOM for elements whose fingerprint closely matches the stored one.
3. **Scoring**: Similarity is computed using configurable weights across text content, class names, element ID, attributes, position, depth, tag name, href patterns, and parent structure.

## Enabling Auto-Healing

```python
from crawlingo import Session

session = Session()
session.auto_match(True)

# Even if the CSS class changes, Crawlingo finds the closest fingerprint match
page = session.page("https://example.com")
result = page.css("div.old-classname")
```

## Customizing Weights

```python
session.auto_match_weights({
    "text": 3.0,
    "class": 0.5,
    "id": 2.0,
    "position": 0.5,
})
```

## Fingerprint Storage Path

```python
session.fingerprint_path("./fingerprints")
```

## When Auto-Healing Activates

1. A selector query returns zero results.
2. The page or session has `auto_match=True`.
3. The fingerprint store contains a fingerprint for that selector.
4. A DOM element with similarity score ≥ 0.7 (default threshold) is found.

If no element passes the threshold, an `AutoMatchFailed` exception is raised.

## Best Practices

- Enable `auto_match=True` for production scrapers targeting frequently updated websites.
- Use specific selectors (with classes or IDs) rather than generic tag selectors for better fingerprint matching.
- Periodically verify selector health with the `Watch` change detector.

## Next Steps

- [Watching for Changes](06_change_detection.md): Monitor pages for DOM changes.
- [AutoMatcher API Reference](../api_reference/auto_matcher.md): Full API details.
