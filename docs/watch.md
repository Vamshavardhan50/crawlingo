# Watch API

The `Watch` class periodically polls a web page and detects changes in extracted fields. It supports typed callbacks for content, price, stock, and structural changes.

## Python

```python
from crawlingo import Watch
import asyncio

def on_change(event):
    print(f"[{event.event_type}] '{event.field}' changed!")
    print(f"  Old: {event.old_value}")
    print(f"  New: {event.new_value}")

async def main():
    watcher = (
        Watch("https://example.com/product/1")
        .field("title", "h1")
        .field("price", "span.price", extraction_type="price")
        .interval(60)                           # Poll every 60 seconds
        .on_change(on_change)
    )

    watch_task = asyncio.create_task(watcher.run_async())

    # Run for 1 hour
    await asyncio.sleep(3600)
    watcher.stop()
    await watch_task

asyncio.run(main())
```

## Node.js

```javascript
const { Watch } = require('crawlingo');

const watcher = new Watch('https://example.com/product/1')
  .field('title', 'h1')
  .field('price', 'span.price', { extractionType: 'price' })
  .interval(60);

watcher.run((err, event) => {
  if (err) return console.error(err);
  console.log(`[${event.changeType}] ${event.field} changed!`);
  console.log(`  Old: ${event.oldValue}`);
  console.log(`  New: ${event.newValue}`);
});

// Stop after 1 hour
setTimeout(() => watcher.stop(), 3600000);
```

## Parameters

| Method | Default | Description |
|--------|---------|-------------|
| `field(name, selector)` | — | Field to monitor |
| `interval(secs)` | `300` | Polling interval in seconds |
| `on_change(callback)` | — | Change notification callback |
| `session(session)` | — | Shared Session configuration |
| `tolerance(pct)` | `0.0` | Price change tolerance (e.g. `0.05` = 5%) |
| `compare_old(bool)` | `True` | Store baseline for comparison |

## Change Event Properties

| Property | Type | Description |
|----------|------|-------------|
| `field` | `str` | Name of changed field |
| `old_value` | `str` | Previous value |
| `new_value` | `str` | Current value |
| `event_type` | `str` | Type of change detected |
| `timestamp` | `datetime` | When change was detected |
| `url` | `str` | Monitored URL |
| `change_pct` | `float` | Percentage change (price only) |

## Change Types

| Type | Triggered When |
|------|----------------|
| `ContentChange` | Text content differs from baseline |
| `PriceChange` | Numeric price crosses tolerance threshold |
| `StockChange` | Availability indicator changes |
| `ElementAdded` | New element appears in selection |
| `ElementRemoved` | Previously matched element disappears |

## Session Integration

```python
from crawlingo import Session, Watch

with Session() as session:
    session.rate_limit(1.0)
    session.auto_match(True)

    watcher = Watch("https://example.com", session=session)
    watcher.field("price", "span.price", extraction_type="price")
    watcher.interval(300)
    watcher.on_change(lambda e: print(f"{e.field}: {e.old_value} → {e.new_value}"))
```
