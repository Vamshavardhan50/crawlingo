# Implementation status

A source audit of available APIs, binding gaps and delivery limits. No fabricated coverage score.

<a id="snapshot"></a>

## Inventory provenance

Version 1.0.0-alpha.2; base engine commit c76f4bd12844f98afe49a8fe7c50da5e542353e3. The source snapshot includes unpublished working-tree changes.

Source digest: dc4274011af056b0a4008555ae7f5f521b4b589da5fad77bcbf63279880555db. The JSON inventory includes individual file SHA-256 hashes. Generate a new snapshot when source changes.

Node: all exports from src/index.ts are enumerated with the TypeScript compiler. Python: __all__, wrapper definitions, reachable return/callback types and hook helpers are read without loading native binaries. Rust: rustdoc is the authority for modules, structs, traits and implementations.

| Surface | Package exports | Documented symbols | Declared members |
| --- | --- | --- | --- |
| node | 34 | 34 | 193 |
| python | 30 | 39 | 198 |

- [Node reference](node.md)
- [Python reference](python.md)
- [Rust modules and rustdoc](rust.md)

<a id="available"></a>

## Implemented source paths

These paths exist in source. Consult the recorded workload evidence separately; this inventory did not exercise every runtime path or certify production readiness.

- DatasetJob URL-file processing, scoped records, validation, exact deduplication, JSONL/CSV shards, checkpoint resume, cancellation and reports.
- Node custom error classes and native conversion; session retry setters and cookie read/delete/clear; shared Page session manager.
- Page inspection, supported CSS/basic XPath/text/regex selectors, Dataset extraction and small-result JSON/CSV/Parquet exports.
- Rust HTTP connection reuse, per-host pacing, retry policy, middleware, metrics, sitemap discovery, crawl frontier and optional browser feature.

- [Measured public-site results](../real-world-results.md)
- [Scale methodology](../benchmarks.md)

<a id="issues"></a>

## Known issues and restricted behavior

| API | Finding |
| --- | --- |
| python:Crawl.depth | The wrapper calls native depth(), but PyCrawl exposes max_depth(). This method cannot currently be used through the public wrapper. |
| python:Crawl.webhook | The wrapper calls webhook(), which is not exported by PyCrawl. The Rust and Node APIs expose a webhook setting. |
| python:Crawl.schedule | The wrapper calls schedule(), which is not exported by PyCrawl. Use an application scheduler around a supported operation. |
| python:Crawl.build | PyCrawl.build returns dictionaries, while this wrapper constructs DatasetResult objects expecting native result handles. Their to_dict/export calls can fail. Prefer DatasetJob for production file exports. |
| python:Page.before_parse | The callback runs after native parsing and its returned HTML is discarded. It does not modify the DOM. |
| python:ElementCollection.__iter__ | For a materialized collection (for example filter() output), this generator returns an iterator instead of yielding from it. Iteration/map can produce no items. |
| node:Crawl.schedule | A background native loop is started without a public cancellation/join handle. Use an external scheduler for controlled worker lifecycle. |

- [python:Crawl.depth](python/crawl.md#depth)
- [python:Crawl.webhook](python/crawl.md#webhook)
- [python:Crawl.schedule](python/crawl.md#schedule)
- [python:Crawl.build](python/crawl.md#build)
- [python:Page.before_parse](python/page.md#before-parse)
- [python:ElementCollection.__iter__](python/element-collection.md#iter)
- [node:Crawl.schedule](node/crawl.md#schedule)

<a id="parity"></a>

## Not exposed or not implemented

| Capability | Current boundary |
| --- | --- |
| Python Session retry/cookie-management setters | The native PySession has additional methods, but the public Python Session wrapper does not expose the new Node methods. Do not assume snake_case aliases exist. |
| Node auth/cache/metrics helpers | The public Node wrapper does not expose Python’s auth middleware helpers, response-cache setter or metrics snapshot. Set headers explicitly where appropriate. |
| Browser navigation on Page | No Node Page.navigate/reload/back/forward/close API. Use Playwright/Puppeteer and pass discovered URLs or rendered snapshots to a defined integration. |
| Universal SDK parity | Python traversal and hook APIs differ from Node. Error conversion and crawl return types differ. Read the language-specific contract. |
| Distributed processing | No multi-host shared checkpoint writer or bundled durable task queue. Use one process per job directory. |
| Data quality beyond exact checks | No semantic deduplication or automatic correctness guarantee for heuristic selector recovery. |
| Performance promises | No verified universal req/s, idle-memory or Scrapy/Playwright superiority claim. |

<a id="refresh"></a>

## Regenerate this reference

Commit the engine change and the corresponding website snapshot in their respective repositories. This generator reads declarations; behavior notes and known issues require review when implementation changes. The docs release gate rejects a dirty or stale snapshot.

### Engine checkout

```shell
cd sdk/nodejs
npm ci
cd ../..
# Requires Python 3 and the Node SDK TypeScript dev dependency.
node scripts/export-sdk-api.cjs crawlingo-docs/src/content/generated/sdk-api.json
cd crawlingo-docs
npm run lint
npm run export:guides -- ../docs
npm run build
```

- [Documentation release procedure](docs-release.md)
