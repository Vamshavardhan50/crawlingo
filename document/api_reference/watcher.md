# crawlingo.Watch

Periodic change-detection monitor that polls a webpage and fires callbacks when field values change (content, price, stock, element added/removed).

## Constructor

```python
watch = Watch(url, session=None)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `url` | `str` | — | URL to monitor for changes |
| `session` | `Session` | `None` | Optional session for shared configuration |

## Methods

All configuration methods return `self` for fluent chaining.

| Method | Parameters | Description |
|--------|-----------|-------------|
| `.field(name, selector, selector_type="css", default=None)` | Field params | Define a selector field to monitor for changes |
| `.interval(seconds: int)` | Poll interval | Set page poll interval in seconds |
| `.auto_match(enabled: bool)` | Enable/disable | Enable auto-match self-healing |
| `.on_change(fn)` | `Callable[[ChangeEvent], None]` | Register callback for any value change |
| `.on_price_change(fn)` | `Callable[[ChangeEvent], None]` | Register callback for price changes |
| `.on_stock_change(fn)` | `Callable[[ChangeEvent], None]` | Register callback for stock changes |
| `.on_element_added(fn)` | `Callable[[ChangeEvent], None]` | Register callback when new element appears |
| `.on_element_removed(fn)` | `Callable[[ChangeEvent], None]` | Register callback when element disappears |
| `.run()` | — | Start the watch loop synchronously |
| `async .run_async()` | — | Start the watch loop asynchronously |
| `.stop()` | — | Cancel and stop the watcher |

## ChangeEvent

| Attribute | Type | Description |
|-----------|------|-------------|
| `.url` | `str` | The page URL that changed |
| `.field` | `str` | The field name that changed |
| `.change_type` | `str` | Type: `"ContentChange"`, `"PriceChange"`, `"StockChange"`, `"ElementAdded"`, `"ElementRemoved"` |
| `.old_value` | `str` | Previous value |
| `.new_value` | `str` | New value |
| `.diff` | `str` | Difference description |
| `.detected_at` | `str` | When the change was detected |
| `.similarity_score` | `float` | Similarity score between old and new |

## Examples

```python
from crawlingo import Session

session = Session()
watch = session.watch("https://example.com/item")
watch.field("price", ".price")
watch.interval(60)  # Check every 60 seconds

watch.on_price_change(lambda ev: print(f"Price changed: {ev.old_value} -> {ev.new_value}"))
watch.on_change(lambda ev: print(f"Change detected on {ev.url}: {ev.field}"))

watch.run()  # Blocks and polls indefinitely
```

## See Also

- [`Session`](session.md): Shared configuration
- [`Dataset`](dataset.md): One-time structured extraction
