# 10_PERFORMANCE.md

This document details the performance optimization strategies, memory layout decisions, and benchmarking configurations in Crawlingo.

---

## 1. High-Performance Memory Layout: Flat `DomTree`

Traditional HTML tree parsers construct nested pointer architectures using heap wrappers like `Rc<RefCell<Node>>` or recursive pointer lists. In Rust, this introduces significant overhead, pointer-chasing latency, and memory cleanup complexity.

Crawlingo bypasses this by utilizing a **flat vector layout**:
- **Representation:** The DOM is compiled into a single contiguous vector of nodes: `Vec<DomNode>`.
- **References:** Parent, child, and sibling relationships are recorded using raw `usize` indices rather than pointers.
- **Cache Locality:** contiguous vectors fit efficiently inside CPU L1/L2 caches. Traversals simply increment or index into the array, avoiding pointer-chasing and cache misses.
- **FFI Cleanliness:** Element references are represented as `node_index: usize` integers, allowing the Python and Node.js SDK bindings to reference elements without copying HTML text or passing pointers across FFI boundaries.

---

## 2. Zero-Copy & Streaming Parser

- **lol_html integration:** Parsing is executed streamingly on raw response byte arrays as they arrive from `wreq`.
- **Heap Overhead:** Tokens are processed on-the-fly to construct the `DomTree` nodes, minimizing intermediate memory allocations.
- **UTF-8 Safety:** Raw bytes are handled natively, with conversion to Rust `String` objects limited only to required text nodes and attributes.

---

## 3. SIMD Text Anchor Matching

- **memchr Crate:** Finding target anchor nodes via text patterns is accelerated using `memchr` SIMD routines.
- **Throughput:** Text searching scans memory blocks at hardware speeds, processing text occurrences faster than standard regex loops.

---

## 4. Parallel Auto-Matcher Scoring

- **Rayon Parallelism:** When CSS selectors fail and the auto-matcher is activated, candidate matching calculations can become compute-intensive. Crawlingo wraps the candidate scoring loop in a Rayon parallel iterator (`par_iter`).
- **Work Stealing:** Scoring computations (Jaro-Winkler for content text, Jaccard for tags and class lists) are distributed across all available CPU cores automatically, completing fingerprint evaluations quickly.

---

## 5. Multi-Tier Cache Layering

To minimize compile times and execution overhead:
1. **CSS Selector Cache:** Compiled CSS selectors are stored inside a global thread-safe `DashMap<String, Selector>` to avoid recurrent parsing of selector query strings.
2. **Regex Cache:** Compiled regex patterns are cached in a thread-safe `DashMap`.
3. **DNS resolver cache:** A hickory-resolver combined with a `moka` cache stores IP addresses to minimize network roundtrip times.
