# 18_BUG_TRACKER.md

This registry tracks verified codebase bugs, documenting their symptoms, root causes, and fixes.

---

## 1. Rate Limiter Isolation Leak
- **Issue ID:** BUG-001
- **Severity:** High
- **Symptom:** Parallel crawl requests exceed configured rate limits, risking IP bans from target hosts.
- **Root Cause:** Both the `Crawler` and `Dataset::build_async` inline a new `HostRateLimiter` instance for each request cycle. This isolates token buckets to individual requests instead of applying them across the entire session.
- **Files Affected:** [crawler.rs](file:///d:/Scraper/src/crawl/crawler.rs), [builder.rs](file:///d:/Scraper/src/dataset/builder.rs).
- **Remedy:** Pass a shared `Arc<Session>` or `Arc<HostRateLimiter>` reference into the crawler work loops and dataset builders.

---

## 2. Sled DB File Lock Collisions
- **Issue ID:** BUG-002
- **Severity:** High
- **Symptom:** Running concurrent dataset extractions results in `SledError: Io(Os { code: 32, ... })` or lock contention failures.
- **Root Cause:** Sled utilizes filesystem lock files to restrict database access to a single process. Opening `FingerprintStore::open()` inside `Dataset::build_async()` on every extraction call creates file lock race conditions.
- **Files Affected:** [builder.rs](file:///d:/Scraper/src/dataset/builder.rs).
- **Remedy:** Open the sled DB connection once at the session initialization level and share it across calls.

---

## 3. Crawler Extraction Selectors Bypass
- **Issue ID:** BUG-003
- **Severity:** Medium
- **Symptom:** The crawler fails to resolve elements defined with XPath, Regex, or Text selectors.
- **Root Cause:** The crawler's element parser loop calls `css::query()` directly, ignoring the schema's `selector_type` property.
- **Files Affected:** [crawler.rs](file:///d:/Scraper/src/crawl/crawler.rs).
- **Remedy:** Read the selector configuration and call the correct query evaluator module dynamically.
