# Run a complete example

A local product catalog, two SDK runners and output you can inspect. No third-party website required.

<a id="fixture"></a>

## 1. Serve the sample catalog

The repository includes examples/product_dataset: HTML, robots rules, a URL file and a shared job configuration. Run these commands from the repository root. Keep the server running in a separate terminal.

### Terminal 1

```shell
python -m http.server 8765 --bind 127.0.0.1 --directory examples/product_dataset/site
```

- [Browse the example files](https://github.com/Vamshavardhan50/crawlingo/tree/main/examples/product_dataset)

<a id="extract"></a>

## 2. Run the same job in your language

Build the current source first. Each product is an article.product container. The SKU and name are required; price must be a finite number. Deduplication uses SKU. All runners resolve their input and output paths from the example directory.

### CLI

```shell
cd examples/product_dataset
cargo run --release --features cli -- dataset job.json
```

### Python

```shell
python examples/product_dataset/run.py
```

### Node.js

```shell
node examples/product_dataset/run.cjs
```

- [Build an SDK from source](installation.md)

<a id="output"></a>

## 3. Inspect accepted and rejected rows

The fixture intentionally includes one product without a name and one repeated SKU. The first run writes two valid products, counts one duplicate record, one rejected row and one duplicate URL. Its three URL lines refer to two distinct pages. Output lives in examples/product_dataset/output/.

The CLI exits with status 2 because the example deliberately rejects a row. The SDK runners print the report and also return 2 for rejected rows. This is the expected result; inspect part-00000000.errors.jsonl to find the missing-name row.

### Accepted record shape

```json
{
  "sku": "A-101",
  "name": "Café mug",
  "price": "12.5",
  "_source_url": "http://127.0.0.1:8765/catalog.html",
  "_row": "0"
}
```

<a id="resume"></a>

## 4. Resume and use the data

Repeat the command without changing job.json or urls.txt. The report keeps cumulative counts and the committed pages are not fetched again. For a fresh experiment, change output_dir to a new directory.

Read JSONL one line at a time. Numeric fields are validated strings: convert price explicitly in downstream code. CSV is available by setting format to csv and choosing a fresh output directory.

### Python consumer

```python
import json
from pathlib import Path

root = Path("examples/product_dataset/output")
report = json.loads((root / "report.json").read_text())
for index in range(report["shards"]):
    with (root / f"part-{index:08d}.jsonl").open(encoding="utf-8") as source:
        for line in source:
            row = json.loads(line)
            print(row["sku"], float(row["price"]))
```

- [Output and recovery contract](recovery.md)
