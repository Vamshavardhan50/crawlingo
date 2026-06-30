# 16_DESIGN_DECISIONS.md

This document records the design decisions and technical rationales behind Crawlingo's architecture.

---

## 1. contigous Flat `DomTree` (`Vec<DomNode>`)

- **Context:** Storing an HTML tree in Rust can be difficult because parent-child references must be managed without memory leaks, cycle problems, or slow pointer chasing.
- **Decision:** Store the DOM as a flat vector `Vec<DomNode>`, where relationships are recorded as `usize` index markers.
- **Rationale:**
  1. Contiguous arrays fit inside CPU caches, improving traversal performance.
  2. Element collections can be passed to Python and Node.js as lists of `usize` index markers, avoiding memory copies across FFI boundaries.
  3. No cycle references or `Rc<RefCell<Node>>` overhead.

---

## 2. Using PyO3 and NAPI-RS for Multi-Language Support

- **Context:** High-performance web scrapers need to be written in Rust for speed but consumed in Python (data science, scraping scripts) and Node.js (web backend applications).
- **Decision:** Write the engine in Rust and expose native Python and Node.js extensions using PyO3 and NAPI-RS.
- **Rationale:**
  - Standard JSON-RPC or HTTP sidecars add inter-process network overhead.
  - Native FFI bindings allow direct memory access, enabling JavaScript and Python to read elements directly from the Rust-allocated `DomTree`.

---

## 3. `sled` as the Fingerprint Storage Engine

- **Context:** Self-healing selectors require storing structural signatures (DOM fingerprints) between runs to detect page changes.
- **Decision:** Use `sled` as the embedded key-value database.
- **Rationale:**
  - Lock-free, transactional, and fast.
  - Embedded library: requires no system installation or local daemon processes (unlike PostgreSQL or Redis).
  - Integrates smoothly with Rust (keys/values serialized via `bincode`).

---

## 4. `Rayon` for Parallel Selector Healing

- **Context:** If a selector fails, comparing all DOM elements against the cached fingerprint database using similarity metrics is compute-intensive.
- **Decision:** Use Rayon to parallelize candidate evaluations.
- **Rationale:**
  - Evaluates similarity matrices (Jaro-Winkler, Jaccard tag similarity, depth differences) in parallel.
  - Distributes the computational load across all available CPU cores, keeping query resolution latency low.
