# document/22_DATA_FLOW.md

This document maps how data changes formats, structures, and memory ownership as it flows through the Crawlingo scraping pipeline.

---

## 1. Data Flow Stages Table

| Stage | Input | Output | Primary Structures | Memory Ownership | Purpose |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **1. Target URL** | `&str` / `String` | Parsed `url::Url` | `String`, `url::Url` | Owned by caller, cloned into `FetchRequest`. | Defines target host. |
| **2. Request Config** | FFI parameters | `FetchRequest` | `FetchRequest`, `HashMap` | Struct allocated on Heap; fields are owned. | Holds timeout, header map, cookies, and proxy. |
| **3. Network Stream** | TCP packets | Streamed HTTP bytes | `wreq::Response`, `bytes::Bytes` | Streamed directly into buffers. | Downloads page bytes over socket. |
| **4. Raw HTML Ingest** | `bytes::Bytes` | `DomTree` | `lol_html::HtmlRewriter`, `Vec<DomNode>` | Ingested as byte slices. Outputs heap-allocated vector. | Parses tags and builds DOM tree. |
| **5. DOM Structure** | Flat vector nodes | Indexed search matches | `DomTree`, `DomNode`, `usize` index | Owned by `Page`. Shared via `Arc<DomTree>`. | Memory-indexed representation of DOM. |
| **6. Selectors** | Query strings | Matched element offsets | `CompiledSelector`, `Vec<usize>` | Selector patterns cached in global static `DashMap`. | Finds node index arrays matching patterns. |
| **7. Field Extraction** | Element indices | Structured row | `DatasetResult`, `HashMap` | Heap-allocated text strings cloned into maps. | Captures key-value data fields. |
| **8. Dataset Output** | List of extracted maps | Apache Arrow batches | `Dataset`, `RecordBatch`, `ArrayRef` | Arrow structures allocate contiguous off-heap memory. | Organizes rows into columns. |
| **9. File Export** | Arrow RecordBatches | Parquet/CSV/JSON files | `SerializedFileWriter` (Parquet) | Flushed directly to disk, freeing heap memory. | Archives data to storage. |

---

## 2. In-Depth Memory Ownership Analysis

### A. The In-Memory DOM (`DomTree`)
- **Traditional Pointer DOM:** Traditional HTML parsers use pointer-based trees (such as `Rc<RefCell<Node>>`). This results in reference fragmentation and pointer-chasing latency.
- **Crawlingo Contiguous Vector:** Crawlingo stores the DOM as a contiguous vector `Vec<DomNode>`.
- **References:** Relationships are represented as `usize` index offsets, meaning node index operations bypass Rust's reference checking.
- **FFI Sharing:** The `DomTree` is wrapped in `Arc<DomTree>`. When elements are passed to Python or Node.js, the wrappers only clone a cheap `Arc` reference and a `usize` index.

### B. Element Selectors (`CompiledSelector`)
- **Compilation Caches:** Selectors are parsed and compiled into `CompiledSelector` structures containing tag, class, and ID filter arrays.
- **Global Map:** Compiled selector objects are stored inside a global `SELECTOR_CACHE` (`DashMap<String, CompiledSelector>`). This cache is static and survives the lifespan of individual page parses, avoiding recompilation on repeated CSS executions.

### C. Tabular Dataset Generation
- **Row Mapping:** The dataset builder extracts field strings from nodes, compiling them into a `HashMap<String, String>` per page.
- **Apache Arrow Batches:** For export, these maps are converted into Apache Arrow `RecordBatch` columns. Arrow writes memory contiguously, allowing fast zero-copy serialization during file exports.
