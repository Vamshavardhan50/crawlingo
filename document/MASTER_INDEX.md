# docs/MASTER_INDEX.md

Welcome to Crawlingo's developer documentation index. This page maps the complete layout of Crawlingo's operational handbook files, design blueprints, and refactoring guidelines.

---

## 1. Developer Operational Handbook (`document/`)

These operational guides outline the project's current status, guidelines, and release workflows:
1. **[00_PROJECT_STATUS.md](file:///d:/Scraper/document/00_PROJECT_STATUS.md):** Active version metadata, dependencies, and known blockers.
2. **[01_ARCHITECTURE.md](file:///d:/Scraper/document/01_ARCHITECTURE.md):** System architecture overview, request/crawling lifecycles, and error propagation.
3. **[02_CODEBASE_MAP.md](file:///d:/Scraper/document/02_CODEBASE_MAP.md):** File-by-file codebase roadmap mapping all core modules.
4. **[03_ENGINEERING_AUDIT.md](file:///d:/Scraper/document/03_ENGINEERING_AUDIT.md):** Codebase evaluation, execution flowcharts, and audit scores.
5. **[04_TECHNICAL_DEBT.md](file:///d:/Scraper/document/04_TECHNICAL_DEBT.md):** Severity rankings of code issues and connection bottlenecks.
6. **[05_ROADMAP.md](file:///d:/Scraper/document/05_ROADMAP.md):** Development milestones from v0.1 to v1.0.
7. **[06_DEVELOPMENT_GUIDE.md](file:///d:/Scraper/document/06_DEVELOPMENT_GUIDE.md):** Local setup, building, testing, and benchmark execution commands.
8. **[07_CONTRIBUTING_GUIDE.md](file:///d:/Scraper/document/07_CONTRIBUTING_GUIDE.md):** Branch naming, commit standards, and PR checklists.
9. **[08_TESTING_GUIDE.md](file:///d:/Scraper/document/08_TESTING_GUIDE.md):** Testing layers, integration test suite details, and templates.
10. **[09_RELEASE_GUIDE.md](file:///d:/Scraper/document/09_RELEASE_GUIDE.md):** Release workflows, version check configurations, and publish actions.
11. **[10_PERFORMANCE.md](file:///d:/Scraper/document/10_PERFORMANCE.md):** Flat DOM tree memory structures, SIMD matching, and parallel Rayon checks.
12. **[11_SECURITY_REVIEW.md](file:///d:/Scraper/document/11_SECURITY_REVIEW.md):** Transport SSL/TLS safety, proxy credential security, and dependency checks.
13. **[12_API_DESIGN.md](file:///d:/Scraper/document/12_API_DESIGN.md):** Public API reference maps for Python/Node.js SDKs, CLI, and MCP.
14. **[13_FEATURE_INDEX.md](file:///d:/Scraper/document/13_FEATURE_INDEX.md):** Feature matrix of active and planned capabilities.
15. **[14_CHANGELOG_INTERNAL.md](file:///d:/Scraper/document/14_CHANGELOG_INTERNAL.md):** Internal changelog starting from version 0.1.0.
16. **[15_AGENT_MEMORY.md](file:///d:/Scraper/document/15_AGENT_MEMORY.md):** Live context memory for future AI agent sessions.
17. **[16_DESIGN_DECISIONS.md](file:///d:/Scraper/document/16_DESIGN_DECISIONS.md):** Rationale behind flat DOM layouts, FFI selections, and embedded stores.
18. **[17_CODE_STYLE.md](file:///d:/Scraper/document/17_CODE_STYLE.md):** Linter configurations and checking commands for Rust, Python, and JavaScript.
19. **[18_BUG_TRACKER.md](file:///d:/Scraper/document/18_BUG_TRACKER.md):** Known bug registry mapping root causes and mitigations.
20. **[19_IDEAS.md](file:///d:/Scraper/document/19_IDEAS.md):** Future proposals like LLM fallback, visual selectors, and proxy quality scoring.
21. **[20_PRODUCTION_CHECKLIST.md](file:///d:/Scraper/document/20_PRODUCTION_CHECKLIST.md):** Checklist of validations before production deployments.

---

## 2. Redesign & Architectural Blueprints (`docs/`)

These design blueprints propose the target architecture for Crawlingo as a modular web data engine:
- **[PROJECT_AUDIT.md](file:///d:/Scraper/docs/PROJECT_AUDIT.md) (Phase 1):** Detailed code audit, execution flows, and engineering evaluations.
- **[NEW_ARCHITECTURE.md](file:///d:/Scraper/docs/NEW_ARCHITECTURE.md) (Phase 2):** Pipeline redesign, traits, ownership models, and lifetimes.
- **[FETCH_DESIGN.md](file:///d:/Scraper/docs/FETCH_DESIGN.md) (Phase 3):** `FetchStrategy` trait, `FetchManager` loop, and strategy providers.
- **[PAGE_DESIGN.md](file:///d:/Scraper/docs/PAGE_DESIGN.md) (Phase 4):** Schema layout and API signatures of the target `Page` model.
- **[DATASET_ENGINE.md](file:///d:/Scraper/docs/DATASET_ENGINE.md) (Phase 5):** Tabular dataset rules, streaming schema pipeline, and deduplication models.
- **[MASTER_ROADMAP.md](file:///d:/Scraper/docs/MASTER_ROADMAP.md) (Phase 6):** Development milestones outlining complexity, risks, and files.
- **[REFACTOR_PLAN.md](file:///d:/Scraper/docs/REFACTOR_PLAN.md) (Phase 7):** Refactoring strategy and risk mitigations.

---

## 3. Reverse-Engineered Internals (`document/` 21–35)

These deep-dive documents explain how the engine operates internally:
- **[21_EXECUTION_FLOW.md](file:///d:/Scraper/document/21_EXECUTION_FLOW.md):** Step-by-step trace of `Page::fetch(url)` showing function and FFI boundaries.
- **[22_DATA_FLOW.md](file:///d:/Scraper/document/22_DATA_FLOW.md):** In-depth memory allocations and data structures from URL input to file export.
- **[23_MODULE_DEPENDENCY.md](file:///d:/Scraper/document/23_MODULE_DEPENDENCY.md):** Internal module dependencies, circular checks, and coupling audits.
- **[24_PAGE_MODEL.md](file:///d:/Scraper/document/24_PAGE_MODEL.md):** Fields, methods, lifecycle stages, and design boundaries of the Page struct.
- **[25_FETCH_PIPELINE.md](file:///d:/Scraper/document/25_FETCH_PIPELINE.md):** Fetch manager network transport, rate-limiting, and retries.
- **[26_PARSER_PIPELINE.md](file:///d:/Scraper/document/26_PARSER_PIPELINE.md):** HTML parsing, flat vector allocations, and data extraction.
- **[27_SELECTOR_ENGINE.md](file:///d:/Scraper/document/27_SELECTOR_ENGINE.md):** Query parser, matching algorithms, cache designs, and complexities.
- **[28_DATASET_ENGINE.md](file:///d:/Scraper/document/28_DATASET_ENGINE.md):** Extraction schemas, validation pipelines, and exporter APIs.
- **[29_PLUGIN_SYSTEM.md](file:///d:/Scraper/document/29_PLUGIN_SYSTEM.md):** Plugin traits, dynamic loading patterns, and registry managers.
- **[30_DIRECTORY_GUIDE.md](file:///d:/Scraper/document/30_DIRECTORY_GUIDE.md):** File directory purposes, module locations, and restructuring recommendations.
- **[31_CODE_OWNERSHIP.md](file:///d:/Scraper/document/31_CODE_OWNERSHIP.md):** Struct creation, mutation, and destruction.
- **[32_RUNTIME.md](file:///d:/Scraper/document/32_RUNTIME.md):** Async runtime, threading models, and FFI memory boundaries.
- **[33_REFACTOR_STRATEGY.md](file:///d:/Scraper/document/33_REFACTOR_STRATEGY.md):** Prioritized refactoring strategy mapping efforts and risks.
- **[34_ARCHITECTURE_DECISIONS.md](file:///d:/Scraper/document/34_ARCHITECTURE_DECISIONS.md):** design decisions, evaluations, and recommended changes.
- **[35_ENGINE_SCORECARD.md](file:///d:/Scraper/document/35_ENGINE_SCORECARD.md):** Brutally honest engine scorecard backed by codebase evidence.
