# pandas, DuckDB & Parquet

Read completed output incrementally, convert field types, and produce an analytical dataset.

<a id="input"></a>

## Start with committed output

Run the product example first. The snippets below read its output directory from examples/product_dataset. Review report.json before importing records. Use only part-XXXXXXXX.jsonl files: a broad *.jsonl glob also matches error shards. Crawlingo stores validated field values as strings, so define downstream types deliberately.

- [Run the product example](product-example.md)

<a id="pandas"></a>

## Read JSONL in pandas batches

Install pandas in a separate analysis environment if needed. Process each chunk before reading the next. Concatenating every chunk recreates an in-memory dataset and loses the benefit of chunked reading. Keep identifiers as strings so leading zeros survive.

### Terminal

```shell
python -m pip install pandas
```

### Python · summarize.py

```python
import json
from pathlib import Path
import pandas as pd

root = Path("examples/product_dataset/output")
report = json.loads((root / "report.json").read_text(encoding="utf-8"))
if not report["complete"] or report["cancelled"]:
    raise RuntimeError("Wait for a completed job")
# The sample intentionally has one rejected row. Review its error shard first.
total = 0
for shard in sorted(root.glob("part-" + "[0-9]" * 8 + ".jsonl")):
    for chunk in pd.read_json(shard, lines=True, chunksize=10_000, dtype=False, convert_dates=False):
        chunk["sku"] = chunk["sku"].astype("string")
        chunk["price"] = pd.to_numeric(chunk["price"], errors="raise")
        total += len(chunk)
        print(chunk[["sku", "price"]].head())
print({"rows_processed": total, "rejected_rows": report["records_rejected"]})
```

<a id="duckdb"></a>

## Query and convert with DuckDB

Install DuckDB and run this script from the repository root after the product job completes. The explicit columns preserve field types at import. Casting price creates a typed numeric column. This Parquet file is a downstream artifact; the dataset job itself still checkpoints JSONL or CSV.

### Terminal

```shell
python -m pip install duckdb
```

### Python · to_parquet.py

```python
from pathlib import Path
import duckdb

destination = Path("products.parquet")
if destination.exists():
    raise RuntimeError("Choose a new destination for this export")
with duckdb.connect() as con:
    con.execute("""
        CREATE TEMP VIEW products AS
        SELECT sku, name, CAST(price AS DOUBLE) AS price, _source_url
        FROM read_json(
          'examples/product_dataset/output/part-[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9].jsonl',
          format = 'newline_delimited',
          columns = {sku: 'VARCHAR', name: 'VARCHAR', price: 'VARCHAR',
                     _source_url: 'VARCHAR', _row: 'VARCHAR'}
        )
    """)
    print(con.execute("SELECT COUNT(*), AVG(price) FROM products").fetchone())
    con.execute("COPY products TO 'products.parquet' (FORMAT PARQUET)")

```

<a id="destinations"></a>

## Databases, object storage and search

Do not upload a live state database as a consistent recovery snapshot. A remote bucket is not the local state backend. Snapshot a stopped job as a whole when you need a recoverable backup.

| Destination | Application responsibility |
| --- | --- |
| SQL database | Batch records into transactions; define an identity key and use an intentional insert/upsert policy. |
| Object storage | Upload completed data and error shards with the report and a file manifest; publish a completion marker last. |
| Search or RAG index | Review extracted text, remove boilerplate, chunk it, create embeddings, and keep _source_url for attribution. |
| CSV / spreadsheets | Read each shard with its own header; preserve identifiers as text and convert numbers explicitly. |

<a id="sources"></a>

## API sources

- [pandas.read_json: lines, chunksize and dtype](https://pandas.pydata.org/docs/reference/api/pandas.read_json.html)
- [DuckDB JSON reader](https://duckdb.org/docs/stable/data/json/overview)
- [Crawlingo recovery](recovery.md)
