# Change Detection

Monitor a web page for DOM changes and receive callbacks when content updates.

## Basic Usage

```python
from crawlingo import Session

session = Session()

watch = session.watch("https://example.com/prices")
watch.field("price", "span.price")
watch.interval(300)  # Check every 5 minutes

watch.on_change(lambda ev: print(f"Change detected on {ev.url}: {ev.field} {ev.old_value} -> {ev.new_value}"))
watch.run()  # Blocks and polls indefinitely
```

## How It Works

1. On first poll, the `Watch` extracts field values using the configured selectors.
2. On subsequent polls, field values are re-extracted and compared.
3. If a value differs, registered callbacks fire with a `ChangeEvent`.

## ChangeEvent

```python
watch.on_change(lambda ev: print(
    f"URL: {ev.url}",
    f"Field: {ev.field}",
    f"Type: {ev.change_type}",
    f"Old: {ev.old_value}",
    f"New: {ev.new_value}",
))
```

| Field | Type | Description |
|-------|------|-------------|
| `url` | `str` | Page URL |
| `field` | `str` | Field name that changed |
| `change_type` | `str` | `ContentChange`, `PriceChange`, `StockChange`, `ElementAdded`, `ElementRemoved` |
| `old_value` | `str` | Previous value |
| `new_value` | `str` | New value |
| `diff` | `str` | Difference description |
| `detected_at` | `str` | Detection timestamp |
| `similarity_score` | `float` | Similarity between old and new |

## Specialized Callbacks

```python
watch.on_price_change(lambda ev: print(f"Price alert: {ev.old_value} -> {ev.new_value}"))
watch.on_stock_change(lambda ev: print(f"Stock changed!"))
watch.on_element_added(lambda ev: print(f"New element: {ev.field}"))
watch.on_element_removed(lambda ev: print(f"Element removed: {ev.field}"))
```

## Stopping the Watcher

```python
watch.stop()
```

## Combining with Auto-Healing

Use `Watch` to detect when a selector breaks, and auto-match to recover:

```python
session = Session()
session.auto_match(True)

watch = session.watch("https://example.com")
watch.field("data", "div.important-data")
watch.interval(3600)

watch.on_change(lambda ev: print(f"Selector healed for {ev.url}: {ev.field}"))
```

## Next Steps

- [Watcher API Reference](../api_reference/watcher.md): Full API documentation.
