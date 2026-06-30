# document/31_CODE_OWNERSHIP.md

This document traces the memory lifecycle and ownership of Crawlingo's primary structures.

---

## 1. Struct Ownership Lifecycle Matrix

| Struct Name | Created By | Primary Owner | Mutated By | Destroyed By | Lifespan |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`Session`** | Client wrapper | FFI Wrapper / FFI runtime | Shared threads (`RwLock`) | Client gc / FFI drop | Lifespan of active scrape job. |
| **`Fetcher`** | `Page` / `Crawler` / `Dataset` | Initializing worker thread | Immutable after config | Thread loop exit | Recreated per fetch request in current codebase. |
| **`DomTree`** | `parse_html` | `Page` via `Arc<DomTree>` | Immutable after parse | Reference count = 0 | Lifespan of parent Page. |
| **`DomNode`** | `parse_html` tokenizer | `DomTree` (`Vec<DomNode>`) | Immutable after parse | `DomTree` drop | Lifespan of parent DomTree. |
| **`Page`** | `PyPage::new` / `JsPage` | FFI wrapper | Immutable | FFI drop / GC | Lifespan of client variables. |
| **`Dataset`** | FFI dataset | FFI wrapper | FFI schema configuration | FFI drop | Lifespan of extraction run. |
| **`Crawler`** | FFI crawler | FFI wrapper | Crawler worker threads | FFI drop | Lifespan of site crawl run. |
| **`FingerprintStore`** | `Dataset` extraction | `Dataset` call stack | Immutable after open | Sled lock drop | Lifespan of single extraction. |
| **`Watch`** | FFI watch | FFI monitor task | Polling thread loop | Cancellation token | Until monitor is stopped. |

---

## 2. In-Depth Allocation Profiles

### A. The Shared Session (`Session`)
- **Memory Profile:** Allocated on the Heap.
- **Reference Sharing:** Wrapped in `Arc<Session>` to allow concurrent access.
- **Thread Safety:** Configuration variables are protected by thread-safe `RwLock` or `Atomic` wrappers, allowing concurrent worker threads to read properties (like proxy lists, rate limits, or cookies) safely.

### B. Element Collections (`ElementCollection`)
- **Memory Profile:** Allocated on the Heap as a small struct containing the `Arc<DomTree>` and a `Vec<usize>` representing matching element indices.
- **Reference Sharing:** Cheaply cloned and passed to FFI wrappers without copying HTML text or DOM structure data.

### C. Flat Node Array (`DomTree`)
- **Memory Profile:** Allocated on the Heap as a single flat vector `Vec<DomNode>`.
- **Reference Sharing:** Once parsed, the `DomTree` is wrapped in an `Arc<DomTree>`. This allows FFI wrappers and element collections to read elements without copying the underlying DOM tree.
- **Destruction:** The `DomTree` memory is automatically cleared from the heap once the reference count of the `Arc<DomTree>` reaches `0`.
