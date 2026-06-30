# Dataset Extraction

Extract structured data from a web page using the fluent Dataset builder and export to JSON, CSV, or Parquet.

## Defining Fields

Fields define what to extract and which selector to use:

```python
from crawlingo import Session

session = Session()
session.auto_match(True)

dataset = session.dataset("https://shop.example.com/item/42")
dataset.field("product_name", "h1.title")
dataset.field("price", "//span[@class='price']", selector_type="xpath")
dataset.field("description", "meta[name='description']")

result = dataset.build()
print(result.to_dict())
# {"product_name": "Wireless Mouse", "price": "$29.99", "description": "..."}
```

## Field Options

Each `.field()` call accepts:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `name` | — | Output field name |
| `selector` | — | CSS, XPath, text, or regex expression |
| `selector_type` | `"css"` | One of: `"css"`, `"xpath"`, `"text"`, `"regex"`, `"after_text"`, `"before_text"` |
| `transform` | `None` | Python callable to transform the extracted value |
| `default` | `None` | Default value when selector yields no results |

## Exporting Results

```python
result.to_csv("output/products.csv")
result.to_parquet("output/products.parquet")
result.to_json("output/products.json")
```

### Using a Session

```python
from crawlingo import Dataset

dataset = Dataset("https://shop.example.com/item/42")
dataset.field("title", "h1")
dataset.field("price", ".price")
dataset.auto_match(True)
dataset.timeout(15)

result = dataset.build()
```

### Asynchronous

```python
result = await dataset.build_async()
```

## Next Steps

- [Self-Healing Selectors](04_auto_healing.md): Understand how failed selectors are recovered.
- [Crawling Multiple Pages](05_crawling.md): Crawl sites at scale.
- [Dataset API Reference](../api_reference/dataset.md): Full API documentation.
