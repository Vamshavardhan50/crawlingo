# Change events

Compare extracted values and route events to your own consumers.

<a id="types"></a>

## Event categories

Price and stock classifications are heuristics. The old and new field values remain the primary evidence for your application.

| Type | Meaning |
| --- | --- |
| ContentChange | A field value changed. |
| PriceChange | Values were interpreted as prices. |
| StockChange | Values matched stock-status heuristics. |
| ElementAdded | A key appeared in the new map. |
| ElementRemoved | A key disappeared from the new map. |

<a id="integration"></a>

## Deliver events

Use the Watch callback to persist an event or enqueue an application notification. Add your own retries, deduplication and delivery tracking for external services. Crawlingo does not automatically deliver webhooks.

- [Watch example](watch.md)
