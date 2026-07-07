# Dataset API

The `Dataset` class provides schema-driven structured data extraction from web pages. It supports multiple fields, auto-match, and export to JSON, CSV, and Parquet.

## Python

```python
from crawlingo import Dataset

dataset = (
    Dataset("https://example.com/product/1")
    .auto_match(True)
    .field("title", "h1.product-title")
    .field("price", "span.price-value", extraction_type="price")
    .field("description", "p.description")
    .field("availability", ".stock-status", default="unknown")
    .field("image_url", "img.main", extraction_type="url")
    .build()
)

# Access results
print(dataset.to_dict())
# {"title": "Premium Widget", "price": "1234.56", ...}

# Export
dataset.to_json("output.json")
dataset.to_csv("output.csv")
dataset.to_parquet("output.parquet")
```

## Node.js

```javascript
const { Dataset } = require('crawlingo');

const dataset = new Dataset('https://example.com/product/1')
  .autoMatch(true)
  .field('title', 'h1.product-title')
  .field('price', 'span.price-value', { extractionType: 'price' });

const result = await dataset.build();
console.log(result.toDict());

await result.toJson('output.json');
await result.toCsv('output.csv');
```

## Rust

```rust
use crawlingo::Dataset;

let mut dataset = Dataset::new("https://example.com");
dataset.add_field(DatasetField {
    name: "title".into(),
    selector: "h1".into(),
    selector_type: "css".into(),
    default: None,
    extract_type: ExtractionType::Text,
});
let result = dataset.build()?;
println!("{:?}", result.fields);
```

## Field API

```python
.field(
    name: str,           # Field name in output
    selector: str,        # CSS/XPath/Regex selector
    selector_type="css",  # "css", "xpath", "regex", "text"
    extraction_type=None, # "text", "price", "datetime", "url", "datalink_*"
    default=None          # Fallback value if selector finds nothing
)
```

## Extraction Types

| Type | Input → Output | Use Case |
|------|---------------|----------|
| `text` | `"  Hello  "` → `"Hello"` | Trim whitespace |
| `price` | `"$1,234.56"` → `"1234.56"` | Normalize currency |
| `datetime` | `"Jan 15, 2024"` → `"2024-01-15"` | Standardize dates |
| `url` | `"/path"` → `"https://base.com/path"` | Resolve relative URLs |
| `datalink_url` | `<a href="...">` → href value | Extract links |
| `datalink_email` | `"mailto:a@b.com"` → `"a@b.com"` | Extract emails |
| `datalink_phone` | `"tel:+1234"` → `"+1234"` | Extract phone numbers |

## Schema Validation

```python
from crawlingo import Dataset, DatasetSchema, FieldConstraint, FieldType

schema = DatasetSchema([
    FieldConstraint("title", FieldType.String, required=True),
    FieldConstraint("price", FieldType.Float, required=True),
    FieldConstraint("description", FieldType.String, required=False),
])

dataset = (
    Dataset("https://example.com")
    .with_schema(schema)
    .field("title", "h1")
    .field("price", "span.price", extraction_type="price")
    .build()
)
```

## Streaming Dataset

Process large URL lists with bounded memory:

```python
dataset = Dataset("https://example.com")
dataset.field("title", "h1")

stream = dataset.build_many_streamed(
    urls=["https://example.com/a", "https://example.com/b", ...],
    concurrency=10
)

for record in stream:
    print(record.fields)
```

## Export Formats

| Method | Format | Notes |
|--------|--------|-------|
| `to_json(path)` | JSON | Pretty-printed array of objects |
| `to_jsonl(path)` | JSONL | One JSON object per line |
| `to_csv(path)` | CSV | Header row + data rows |
| `to_parquet(path)` | Parquet | Columnar, compressed, fast |
| `to_dict()` | dict | In-memory dictionary |
| `to_df()` | DataFrame | Pandas DataFrame (Python only) |
