# Page extraction

Inspect a page and choose selectors before defining a bulk job.

<a id="fetch"></a>

## Fetch a page

### Python

```python
from crawlingo import Page

page = Page("https://example.com/")
print(page.status)
print(page.title())
print(page.css("h1").text())
```

### Node.js

```typescript
import { Page } from "crawlingo";

const page = await Page.create("https://example.com/");
console.log(page.status, page.title());
console.log(page.css("h1").text);
```

<a id="scope"></a>

## Find the record container

Inspect one representative page, then test missing fields and multiple rows. Prefer stable attributes and meaningful container selectors. A successful query on one page is not evidence that every page has the same structure.

<a id="bulk"></a>

## Move to a dataset job

Use DatasetJob when the URL list or record count becomes large. Page and collecting crawler APIs retain data in memory and are useful for inspection and smaller tasks.

- [Dataset job reference](dataset-jobs.md)
