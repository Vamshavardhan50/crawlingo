# Change monitoring

Poll selected fields and handle changes in your application.

<a id="example"></a>

## A Node.js watcher

### TypeScript

```typescript
import { Watch } from "crawlingo";

const watcher = new Watch("https://example.com/")
  .field("title", "h1")
  .interval(60);

watcher.run((error, event) => {
  if (error) { console.error(error); return; }
  console.log(event.field, event.oldValue, event.newValue);
});
process.once("SIGINT", () => watcher.stop());
```

<a id="behavior"></a>

## Polling behavior

The first successful poll establishes a baseline. Later polls compare extracted fields. Choose a positive interval in seconds and stop the watcher when it is no longer needed. Watch callbacks run in your application; a durable notification queue or webhook delivery service is not built in.

<a id="dataset"></a>

## Monitoring and datasets

A watcher is separate from DatasetJob. If you want to retain changed observations in a dataset, plan record identity carefully: deduplicating only by product ID would discard later observations of the same product.

- [Change event types](change-detection.md)
- [Schema and identity](schema.md)
