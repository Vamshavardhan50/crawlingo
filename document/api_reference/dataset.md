# crawlingo.Dataset

Fluent builder for structured data extraction from a single URL. Define fields with selectors, execute extraction, and export results.

## Constructor

```python
dataset = Dataset(url, session=None)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `url` | `str` | — | Target URL to fetch and extract |
| `session` | `Session` | `None` | Optional session for shared configuration |

## Methods

### `.field(name, selector, selector_type="css", transform=None, default=None)`

Add an extraction field. Returns `self` for chaining.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | `str` | — | Output field name |
| `selector` | `str` | — | Selector expression (CSS, XPath, regex, or text) |
| `selector_type` | `str` | `"css"` | One of `"css"`, `"xpath"`, `"text"`, `"regex"`, `"after_text"`, `"before_text"` |
| `transform` | `Callable` | `None` | Optional Python callable to transform extracted value |
| `default` | `str` | `None` | Default value if selector yields no results |

### `.auto_match(enabled: bool)`

Enable or disable auto-match self-healing. Returns `self`.

### `.timeout(seconds: int)`

Set connection timeout. Returns `self`.

### `.headers(headers: dict)`

Set request headers. Returns `self`.

### `.build()`

Execute extraction synchronously. Returns `DatasetResult`.

### `async .build_async()`

Execute extraction asynchronously. Returns `DatasetResult`.

## DatasetResult

| Method | Description |
|--------|-------------|
| `.to_json(path: str)` | Export fields to a formatted JSON file |
| `.to_csv(path: str)` | Export fields to a CSV file |
| `.to_parquet(path: str)` | Export fields to a Parquet file |
| `.to_dict()` | Return results as a standard dictionary |
| `.df()` | Return results as a Pandas DataFrame |
| `[key]` | Access field by name |

## Examples

```python
from crawlingo import Session

session = Session()
session.auto_match(True)

dataset = session.dataset("https://shop.example.com/item/42")
dataset.field("title", "h1.product-title")
dataset.field("price", "span.price", selector_type="css")
dataset.field("description", "//div[@id='desc']", selector_type="xpath")

result = dataset.build()
print(result["title"])  # "Wireless Mouse"
print(result.to_dict()) # {"title": "...", "price": "...", "description": "..."}

result.to_json("output.json")
result.to_csv("output.csv")
result.to_parquet("output.parquet")
```

## See Also

- [`Page`](page.md): Low-level page fetching and element selection
- [`Crawl`](crawler.md): Multi-page version of dataset extraction
