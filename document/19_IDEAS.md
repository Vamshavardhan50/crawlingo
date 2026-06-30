# 19_IDEAS.md

This document lists future ideas and feature suggestions for Crawlingo.

---

## 1. Proxy Quality & Performance Scoring
- **Concept:** Rank proxies based on latency and success rate.
- **Details:** The session proxy pool should track performance metrics (such as connection success rates and TLS negotiation response times). Slow or blocked proxies can then be de-prioritized or temporarily rotated out of the active pool.

---

## 2. LLM-Based Extraction Fallback
- **Concept:** Fallback to LLM extraction when both static selectors and fingerprint matching fail.
- **Details:** If selectors fail to extract data on modified structures, send the element's text content to a local LLM (via an MCP SSE callback or Ollama interface) to extract fields using natural language analysis.

---

## 3. Visual Selector Generator
- **Concept:** Calculate CSS selector strings using layout coordinate data.
- **Details:** When running a browser-based crawl, capture the bounding box positions of DOM elements. This coordinate data can be used to generate CSS selectors based on visual grouping rather than relying solely on HTML tags.
