# Change Detection

The Watch system detects changes in extracted fields over time by comparing current extraction results against stored baselines.

## Change Types

### ContentChange
Triggered when text content changes beyond the configured tolerance.

```python
# Detect any text change
watch.field("title", "h1")
```

### PriceChange
Specialized numeric change detection with percentage calculation.

```python
# Price with 5% tolerance
watch.field("price", "span.price", extraction_type="price")
watch.tolerance(0.05)  # Only fires if change > 5%
```

### StockChange
Triggered when availability indicators change.

```python
watch.field("stock", ".availability")
```

### ElementAdded / ElementRemoved
Triggered when a selector starts matching new elements or stops matching existing ones.

```python
watch.field("products", ".product-item")  # Fires ElementAdded/Removed
```

## Event Object

```python
{
    "field": "price",        # Changed field name
    "old_value": "299.99",   # Previous value
    "new_value": "249.99",   # New value
    "change_type": "PriceChange",
    "change_pct": -16.67,    # Percentage change (numeric fields only)
    "url": "https://...",
    "timestamp": "2024-01-15T10:30:00Z"
}
```

## Change Detection Function

Low-level access to the detection algorithm:

```python
from crawlingo import detect_changes

old_data = {"price": "299.99", "title": "Widget"}
new_data = {"price": "249.99", "title": "Widget"}

changes = detect_changes("https://example.com", old_data, new_data)
for change in changes:
    print(f"{change.field}: {change.change_type} ({change.old_value} → {change.new_value})")
```

## Integration with Webhooks

```python
import requests

def on_change(event):
    requests.post("https://my-api.example.com/webhook", json={
        "field": event.field,
        "old_value": event.old_value,
        "new_value": event.new_value,
        "change_type": event.event_type,
        "url": event.url,
    })

Watch("https://example.com")
    .field("price", "span.price", extraction_type="price")
    .interval(300)
    .on_change(on_change)
```
