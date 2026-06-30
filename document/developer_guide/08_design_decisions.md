# Design Decisions

## Flat DomTree (Vec<DomNode>)

**Decision:** Represent the DOM as a contiguous flat vector `Vec<DomNode>` with parent/child indices instead of a pointer-based tree.

**Rationale:**
- **Cache locality**: Sequential memory access during selector traversal is CPU-cache-friendly.
- **FFI cleanliness**: A single `(*mut DomNode, usize)` pair crosses the FFI boundary instead of a complex pointer graph.
- **Atomic sharing**: `Arc<DomTree>` is a single atomic reference count.

## Streaming Parser (lol_html)

**Decision:** Use `lol_html` for HTML parsing instead of `html5ever` or `servo`.

**Rationale:**
- Zero-copy streaming: HTML bytes pass through the tokeniser without intermediate allocations.
- Low overhead: No full DOM tree allocation before building our flat representation.
- Rewriter API: Allows content filtering during parsing if needed.

## Embedded Database (Sled)

**Decision:** Use Sled for fingerprint storage instead of SQLite or a network database.

**Rationale:**
- No system daemon: Sled is embedded; no external process required.
- Lock-free: Sled's lock-free tree avoids contention in concurrent extraction scenarios.
- Transactional: Crash-safe writes without complex recovery logic.

## Parallel Scorer (Rayon)

**Decision:** Use Rayon for parallel fingerprint similarity computation.

**Rationale:**
- DOM elements can be scored independently — an embarrassingly parallel workload.
- Rayon's work-stealing scheduler efficiently distributes work across all CPU cores.
- Drop-in: `par_iter()` replaces `iter()` with no structural changes.

## Native FFI (PyO3 / NAPI-RS)

**Decision:** Build native FFI bindings instead of HTTP-based IPC.

**Rationale:**
- Zero serialisation overhead: Shared memory via `Arc`, no JSON marshalling.
- Synchronous calls: No network latency; DOM access is instantaneous from Python/Node.js.
- Single process: No subprocess management, port allocation, or health checking.
