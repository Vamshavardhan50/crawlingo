# crawlingo Fingerprint System

Persistent storage for element fingerprints used by the auto-matcher for self-healing selector recovery.

## Fingerprint Structure

Each fingerprint is a JSON object:

```json
{
  "url_pattern": "https://example.com/products/*",
  "field": "price",
  "selector": "span.price",
  "created_at": "2025-01-15T10:30:00Z",
  "updated_at": "2025-01-15T10:30:00Z",
  "fingerprints": [
    {
      "tag": "span",
      "id": "",
      "classes": ["price", "final"],
      "text_hash": "ab12cd34",
      "attributes": {"data-price": "19.99"},
      "depth": 5,
      "position": 3,
      "parent_tag": "div",
      "parent_classes": ["product-card"]
    }
  ]
}
```

## Lifecycle

1. **Recording**: When a selector matches during extraction, the system records the matched element's fingerprint.
2. **Retrieval**: On later failures, fingerprints are loaded and compared to current DOM candidates.
3. **Update**: Successful re-matches update the fingerprint timestamp; persistent failures may evict stale fingerprints.
4. **Cleanup**: Fingerprints not matched for 30+ days are automatically removed.

## Storage Path

```python
session.fingerprint_path("./fingerprints")
```

Default: `./crawlingo_fingerprints` in the current working directory.

## See Also

- [Auto Matcher](auto_matcher.md): The self-healing recovery system
- [Selector Engine](selector_engine.md): Available selector types
