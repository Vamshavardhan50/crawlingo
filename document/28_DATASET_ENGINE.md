# document/28_DATASET_ENGINE.md

This document reverse-engineers the dataset builder engine, extraction schemas, and exporting pipelines.

---

## 1. Schema Specifications & Field Extractions

Schemas specify the output fields and their target selectors:

```json
{
  "fields": [
    {
      "name": "product_name",
      "selector": "h1.title",
      "selector_type": "css",
      "required": true
    }
  ]
}
```

### Extraction Steps
1. **Query Resolution:** The builder executes the query selector (CSS, XPath, text, or regex) on the `DomTree`.
2. **Selector Failure (Self-Healing):** If the query returns no matches and `auto_match` is enabled, the builder queries the Sled database to fetch the cached element fingerprint. It runs a similarity comparison across all DOM elements using parallel Rayon tasks, returning the closest match.
3. **Pipeline Hooks:** The extracted text is run through cleaning pipelines (e.g. whitespace stripping or casing conversions).

---

## 2. Validation, Deduplication & Export

- **Validation:** Verifies datatype conversions (such as parsing floats or validating date formats) and checks required field constraints.
- **Deduplication:** Utilizes Bloom filters or memory-efficient hash sets to identify and filter out duplicate unique keys (e.g., SKUs or canonical URLs) before they are written to disk.
- **Exporting:** Maps row fields to Arrow RecordBatches to serialize the dataset into Parquet, CSV, or JSON formats.

---

## 3. How to Interact with the Dataset Engine

### How Developers Should Interact with the API
Developers configure datasets using the fluent builder and execute extractions on a target URL:

```python
from crawlingo import Dataset

# 1. Instantiate Dataset with a target URL
dataset = Dataset("https://example.com/item")

# 2. Define extraction fields
dataset.field("title", "h1.name", selector_type="css")
dataset.field("price", "span.price", selector_type="css")

# 3. Build and extract
result = dataset.build()
print(result.to_dict())  # {"title": "Laptop", "price": "$999"}
```

### How Internal Core Code Operates
Internally, the `Dataset` struct manages field definitions and orchestrates extraction:
- The builder stores field definitions in a `Vec<DatasetField>`.
- On `.build()`, it fetches the URL, parses HTML into a `DomTree`, and runs each field's selector against the tree.
- Results are mapped through optional transform functions and returned as a `DatasetResult` with export methods (JSON, CSV, Parquet).
