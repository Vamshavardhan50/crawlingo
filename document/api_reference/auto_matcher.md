# Auto Matcher

Self-healing selector recovery system. When a primary selector fails to match any elements, the auto-matcher finds the most similar element using configurable similarity scoring against stored fingerprints.

## How It Works

1. **Fingerprint recording**: When a selector successfully matches an element, its structural fingerprints (tag, class names, ID, text content, attributes, position, depth) are stored.
2. **Selector failure**: On subsequent runs, if the primary selector returns no results, the auto-matcher is activated.
3. **Fingerprint comparison**: The current page's DOM is scanned and each candidate element is scored against the stored fingerprint.
4. **Best match**: The element with the highest similarity score above the threshold is returned.

## Configuration

### Similarity Weights

| Weight | Default | Description |
|--------|---------|-------------|
| `text` | `2.0` | Text content similarity importance |
| `class` | `1.0` | CSS class similarity importance |
| `id` | `1.5` | Element ID similarity importance |
| `attributes` | `1.0` | Other attribute similarity importance |
| `position` | `0.5` | DOM position similarity importance |
| `depth` | `0.5` | DOM tree depth similarity importance |
| `tag` | `1.0` | HTML tag name importance |
| `href` | `1.0` | Link URL pattern importance |
| `structure` | `0.8` | Parent-child structure similarity |

```python
session.auto_match(True)
session.auto_match_weights({"text": 3.0, "class": 0.5})
```

### Fingerprint Storage

```python
session.fingerprint_path("./fingerprints")
```

Fingerprints are stored as JSON files. Each file corresponds to a URL pattern.

## Threshold

The minimum similarity score to accept a match is `0.7`. If no candidate scores above this threshold, `AutoMatchFailed` is raised.

## See Also

- [Selector Engine](selector_engine.md): Supported selector types
- [Fingerprint](fingerprint.md): Fingerprint file format and lifecycle
