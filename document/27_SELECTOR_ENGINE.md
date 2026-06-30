# document/27_SELECTOR_ENGINE.md

This document explains the technical implementation of Crawlingo's query selectors, matching algorithms, complexities, and cache designs.

---

## 1. Selector Core Implementations

### CSS Selectors
- **Parsing:** Manual parser splits query strings on whitespace into compound groups. For each group, parses `.class` and `#id` properties.
- **Evaluation:** Evaluates matching elements starting from leaf nodes and traversing ancestors recursively.
- **Cache:** Utilizes a global static `DashMap<String, CompiledSelector>` to avoid parsing the same CSS query strings repeatedly.

### XPath
- **Parsing:** Parses basic expressions like `//div[@class='product']/span` by segment splits.
- **Evaluation:** Traverses descendant elements recursively for double slashes (`//`) or checks direct parent node tags for single slashes (`/`).

### Regular Expressions (Regex)
- **Parsing:** Compiles regex pattern queries.
- **Evaluation:** Evaluates regex pattern matches on text nodes inside the DOM tree.
- **Cache:** Stores compiled regex rules inside a thread-safe `DashMap`.

### Text Search (SIMD Anchors)
- **Evaluation:** Scans element contents for text matching target patterns using `memchr` SIMD routines.
- **Before/After Anchoring:** Exposes methods to retrieve sibling elements immediately preceding or following matching text anchor nodes.

---

## 2. Match Algorithms & Complexities

| Selector Type | Match Algorithm | Time Complexity | Cache Level | Memory Overhead |
| :--- | :--- | :--- | :--- | :--- |
| **CSS** | Leaf evaluation + recursive ancestor check | $O(N \cdot D)$ where $N$ = DOM size, $D$ = tree depth | Global `DashMap` | Low |
| **XPath** | Leaf evaluation + stateful segment traversal | $O(N \cdot S)$ where $S$ = segment count | None | Low |
| **Regex** | Text node traversal + regex execution | $O(N \cdot M)$ where $M$ = pattern complexity | Global `DashMap` | Medium |
| **Text Anchor** | SIMD memory scanning via `memchr` | $O(N + K)$ where $K$ = pattern size | None | Extremely Low |

---

## 3. Performance & Future Improvements

### Current Strengths
- Caching CSS and regex rules inside thread-safe maps avoids repeatedly parsing queries.
- The contiguous flat DOM vector structure keeps traversals cache-friendly.
- Using SIMD-based text searches is extremely fast.

### Recommended Enhancements
1. **Replace Custom Parsers with standard crates:** Currently, CSS and XPath parsers use basic string splitting, which does not support advanced selectors like sibling connectors (`+`, `~`), child selectors (`>`), or pseudo-classes (`:nth-child`). We should replace the custom parsers with standard crates like `selectors` or `nipper` for complete CSS support.
2. **Query Indexing:** Currently, every query scans the entire `DomTree` from index `0` to `len()`. Creating index caches mapping tag names, classes, or IDs to their corresponding vector index locations would speed up evaluations to $O(M)$ (matching elements) instead of $O(N)$ (scanning the whole tree).
