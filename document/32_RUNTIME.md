# document/32_RUNTIME.md

This document explains Crawlingo's runtime environment, async execution boundaries, threading model, and memory management.

---

## 1. Runtime Processing Pipeline

```
[FFI Call] -> [Tokio Thread Pool] -> [Fetcher & DNS] -> [lol_html Stream] -> [DomTree Heap Allocation] -> [GC Drop]
```

### A. Initialization & Configuration
- **Tokio Runtime:** The Rust library initializes a global static `Runtime` instance (`TOKIO_RUNTIME`) on startup using `once_cell::sync::Lazy`.
- **Async wrappers:** This runtime runs async network futures synchronously for the Python GIL thread:
  ```rust
  pub static TOKIO_RUNTIME: Lazy<Runtime> =
      Lazy::new(|| Runtime::new().expect("Failed to initialize static Tokio runtime"));
  ```
- **Node.js runtime:** Node.js napi-rs handles async tasks natively, spawning them directly on the Tokio task loop and returning JavaScript Promises.

### B. Concurrency & Thread Safety
- **Core Safety:** All core engine components implement `Send` and `Sync` to ensure thread-safety across concurrent workers.
- **Worker Pools:** The `Crawler` uses a Tokio `JoinSet` to run concurrent HTTP fetch tasks on the thread pool.
- **Lock-free Read Maps:** The CSS selector cache and DNS resolver cache use `DashMap` (a lock-free hash map) to prevent read/write thread contention.
- **Shared States:** The `Session` manages headers, cookies, and rate limits using thread-safe `RwLock` wrappers, allowing concurrent reads while blocking writers during updates.

---

## 2. Memory Allocations & Garbage Collection

### A. DOM Representation
- Contiguous vector layout `Vec<DomNode>` stores nodes on the Heap.
- Relationships are represented as `usize` index offsets, avoiding the pointer overhead and reference cycles of traditional tree structures.

### B. Zero-Copy Streaming
- Raw bytes stream directly from the network socket into `lol_html::HtmlRewriter`.
- HTML tokens are parsed on-the-fly to construct `DomTree` nodes, minimizing intermediate memory allocations.

### C. FFI Memory Passing
- Page wrappers and element collections share the DOM tree using atomic reference pointers (`Arc<DomTree>`).
- When Python or Node.js elements are created, they only receive a copy of the `Arc` pointer and a `usize` index offset. The DOM tree memory is automatically freed once the client garbage collector drops the last reference and the `Arc` count reaches `0`.

---

## 3. Observability & Shutdown

- **Tracing Spans:** The engine utilizes the `tracing` crate to instrument function boundaries.
- **Graceful Shutdown:** The `Watch` monitor task accepts `CancellationToken` handlers. Triggering the token cancels background polling loops immediately.
