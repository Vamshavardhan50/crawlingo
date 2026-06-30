# docs/REFACTOR_PROGRESS.md

This document tracks the current status and roadmap of Crawlingo's Core Architecture Refactoring.

---

## 1. Refactoring Status Checklist

- [x] **Phase 1: Analyze Current Core**
  - [x] Audit fetch, parser, selector, extractor, dataset, and watch modules
  - [x] Identify coupling, duplicated code, and incorrect ownership
  - [x] Create [docs/CORE_REFACTOR_ANALYSIS.md](file:///d:/Scraper/docs/CORE_REFACTOR_ANALYSIS.md)
- [x] **Phase 2: Design New Core**
  - [x] Map out structural flow from `User` to `Exporter`
  - [x] Define responsibilities, ownership, thread safety, and async constraints
  - [x] Create [docs/CORE_ARCHITECTURE.md](file:///d:/Scraper/docs/CORE_ARCHITECTURE.md)
- [ ] **Phase 3: Fetch Layer Refactor**
  - [ ] Implement `FetchStrategy` trait in core
  - [ ] Implement polymorphic strategies: `HttpFetcher`, `SessionFetcher`
  - [ ] Integrate strategy selection inside `FetchManager`
  - [x] Create design documentation: [docs/FETCH_REDESIGN.md](file:///d:/Scraper/docs/FETCH_REDESIGN.md)
- [ ] **Phase 4: Normalize Responses**
  - [ ] Create `NormalizedResponse` structure
  - [ ] Update all fetch strategies to return the normalized response format
- [ ] **Phase 5: Page Object Refactor**
  - [ ] Redesign `Page` struct to own DOM, html, and lazy metadata indices
  - [ ] Implement lazy-loading using thread-safe `OnceCell` structures
  - [x] Create design documentation: [docs/PAGE_MODEL_V2.md](file:///d:/Scraper/docs/PAGE_MODEL_V2.md)
- [ ] **Phase 6: Parser Refactor**
  - [ ] Refactor `HtmlParser` to strictly consume `NormalizedResponse` and return `Page`
  - [ ] Move any inline dataset/export hooks out of parsing functions
  - [x] Create design documentation: [docs/PARSER_V2.md](file:///d:/Scraper/docs/PARSER_V2.md)
- [ ] **Phase 7: Selector Refactor**
  - [ ] Decouple selector queries from dataset fields
  - [ ] Unify under `SelectorEngine::query` router
  - [x] Create design documentation: [docs/SELECTOR_ENGINE_V2.md](file:///d:/Scraper/docs/SELECTOR_ENGINE_V2.md)
- [ ] **Phase 8: Extraction Refactor**
  - [ ] Create standalone `ExtractionEngine`
  - [ ] Map field rules to clean, type-normalized strings
  - [x] Create design documentation: [docs/EXTRACTION_ENGINE_V2.md](file:///d:/Scraper/docs/EXTRACTION_ENGINE_V2.md)
- [ ] **Phase 9: Dataset Engine Refactor**
  - [ ] Update `Dataset` to consume `Page` objects
  - [ ] Implement streaming rows through async channels (`mpsc`)
  - [ ] Open fingerprint sled instances at session level
  - [x] Create design documentation: [docs/DATASET_ENGINE_V2.md](file:///d:/Scraper/docs/DATASET_ENGINE_V2.md)
- [ ] **Phase 10: Validate Architecture**
  - [x] Create target dependency blueprint: [docs/DEPENDENCY_GRAPH_V2.md](file:///d:/Scraper/docs/DEPENDENCY_GRAPH_V2.md)
  - [ ] Run clippy, formatting, and unit/integration tests to verify compliance

---

## 2. Refactoring Timeline & Milestones

1. **Milestone 1: Architectural Alignment (Current)**
   - Completes design plans and analyzes core vulnerabilities.
2. **Milestone 2: Fetch & Response Normalization**
   - Implements Fetch Strategy traits and resolves response standardization.
3. **Milestone 3: Page & Parser Consolidation**
   - Unifies parsing outputs and integrates the lazy-evaluated Page object.
4. **Milestone 4: Extraction & Exporter Streaming**
   - Decouples dataset builder, introduces mpsc channels, and wraps FFI adapters cleanly.
