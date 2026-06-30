# docs/DATASET_ENGINE_V2.md

This document outlines the pipeline architecture, schema validation rules, and export layers of the Crawlingo Dataset Engine V2.

---

## 1. Pipeline Flow Diagram

```
+------------+      +------------+      +------------+
|   Page 1   |      |   Page 2   |      |   Page 3   |
+------------+      +------------+      +------------+
      |                   |                   |
      +---------+---------+-------------------+
                |
                v
  +---------------------------+
  |    Extraction Engine      | (Applies Extraction Rules)
  +---------------------------+
                |
                v
  +---------------------------+
  |   Tabular Record Map      | (Flat HashMap: Field -> Val)
  +---------------------------+
                |
                v
  +---------------------------+
  |  Deduplication Check      | (Sled Fingerprint Database)
  +---------------------------+
                |
                v
  +---------------------------+
  |      Schema Validation    | (Checks Types & Required Fields)
  +---------------------------+
                |
                v
  +---------------------------+
  |  Streaming Buffer Queue   | (Async tokio::sync::mpsc)
  +---------------------------+
                |
                +-------------------+-------------------+
                |                   |                   |
                v                   v                   v
        +---------------+   +---------------+   +---------------+
        |  Exporter CSV |   | Exporter JSON |   |ExporterParquet|
        +---------------+   +---------------+   +---------------+
```

---

## 2. Single Responsibility Boundaries

The Dataset Engine V2 is strictly an **aggregation and export pipeline**.

- **The Dataset Engine MUST:**
  - Consume pre-compiled `Page` objects.
  - Define data schemas (field names, types, mandatory checks).
  - Apply deduplication logic based on page and element content hashes.
  - Buffer extracted records and stream them asynchronously to exporters.
- **The Dataset Engine MUST NOT:**
  - Connect to the network or make fetch calls.
  - Parse raw HTML byte slices.
  - Traverse selectors directly (it delegates queries to the `SelectorEngine`).

---

## 3. Core Architecture Details

### A. Schema & Validation
Every dataset defines a target Schema constraint:
```rust
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
}

pub struct FieldConstraint {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
}

pub struct DatasetSchema {
    pub fields: Vec<FieldConstraint>,
}
```
Validation ensures fields conform to types (e.g. throwing error logs or falling back to default values when pricing fields cannot be converted to floats).

### B. Deduplication
Rather than opening sled databases on every run, the database connection (`FingerprintStore`) is held inside a long-lived `Session` and passed to the validation block.
- **Fingerprinting:** A record is fingerprinted by hashing the combination of its URL and primary key values. If the hash exists in Sled, the record is flagged as a duplicate and ignored.

### C. Streaming & Buffering
For large-scale crawls, buffering all records in memory before writing to disk leads to memory exhaustion. The Dataset Engine solves this via asynchronous channel streaming:
1. When a worker extracts a record, it sends it to an async channel (`tokio::sync::mpsc::channel`).
2. A background writing task listens on the channel receiver.
3. The writer streamingly appends rows directly to the target output file (CSV, JSON-lines, or Parquet row-group writer), flushing data incrementally.
4. This ensures constant-memory scaling regardless of whether the crawl fetches 10 or 10,000 pages.
