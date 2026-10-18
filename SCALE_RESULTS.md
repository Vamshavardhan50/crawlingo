# Dataset implementation and measured acceptance

Recorded 2026-10-14. These measurements are from the modified working tree based
on commit `d19a62c04ab09713d04f86e958434665b60d131f`, not a published release.

## Million-page result

The release `dataset_scale` example fetched **1,000,000 distinct pages over local
HTTP and exported 1,000,000 records** using the production `DatasetJob` pipeline.

| Measurement | Result |
| --- | ---: |
| Concurrent requests | 50 |
| Checkpoint size | 1,000 pages |
| Exported JSONL shards | 1,000 |
| Failed pages / rejected records | 0 / 0 |
| Records checked against expected values | 1,000,000 |
| Job time, initial invocation | 1,094.569 seconds |
| Complete harness wall time | 1,101.961 seconds |
| Observed peak working set | 190,300,160 bytes (181.5 MiB) |
| Resume without refetching | Passed |

Environment: Windows NT 10.0.26300.0, x86_64, eight logical processors, release
profile. The fixture server runs in the same process; memory includes that server,
output verification and restart. Payloads are small generated UTF-8 HTML pages with
unique IDs and Unicode text. Other development builds ran on the same machine;
this was a correctness and capacity run, not an isolated throughput comparison.

The earlier 10,000-page run completed with 10,000 verified records, no failures,
4.360 seconds job time and 64,958,464 bytes peak working set (61.9 MiB).

These results do not establish performance on arbitrary websites, JavaScript-heavy
pages, large responses, many origins, or distributed workers. They do not support
the previous 3,500 requests/second or 2.4 MB memory claims, or comparisons with
Scrapy and Playwright. The million run used the release executable built before
the final robots edge-case fixes; robots checks were disabled for its local fixture.

## Reproduce

```powershell
cargo build --release --example dataset_scale
./scripts/measure-dataset-scale.ps1 -Pages 1000000 -OutputDirectory D:/Scraper/target/scale-new -Mode http
```

Use a new directory. The harness fails on wrong counts, incorrect record contents
or refetching on resume. It writes `acceptance.json` inside the output directory,
and `.measurement.json`, `.stdout.log` and `.stderr.log` alongside that directory.
`dataset/report.json` describes the last invocation, so after restart its elapsed
time is the restart time; use `acceptance.json` for the initial job time.

Local evidence for this run:

- `target/scale-http-1m-20261014/acceptance.json`
- `target/scale-http-1m-20261014.measurement.json`
- `target/scale-http-10k-20261014/acceptance.json`
- `target/scale-http-10k-20261014.measurement.json`

## Implemented architecture

`URL file → bounded fetch/parse → scoped records → validation → exact dedup → shards`

- Rust owns the complete pipeline; Python and Node receive a small report.
- URL input is streamed; fetch concurrency, response size, page record count,
  record size and checkpoint dedup buffers have independent limits.
- Sled stores exact record identities, URL identities and checkpoints. Output
  files are flushed before checkpoint advancement. Interrupted batches replay.
- Extraction is scoped to each row container. Missing fields cannot shift the
  values of subsequent rows. Invalid records go to error shards.
- JSONL and CSV are supported by jobs. The existing stream API also supports
  Parquet, now with bounded channels and concurrent producer/consumer operation.
- The persistent crawler frontier uses indexed membership and atomic updates.
  Fetch bodies and parser node/depth counts are bounded; retries respect pacing.

See [DATASET_JOBS.md](DATASET_JOBS.md) for configuration and SDK examples.

## Remaining release gates and implementation order

### Local regression evidence (2026-10-17)

- Rust: 161 library/integration checks passed (126 library + 35 integration);
  separate browser suite: seven checks passed with local Chrome.
- Node: 141 implemented API checks passed; the existing runner separately excludes
  five browser checks and 27 experimental/planned checks. Two additional native
  dataset/monitoring tests passed. Package exports were inspected after prepack.
- Python: rebuilt with CPython 3.12; the native dataset, rejection and resume test
  passed. This is a source-extension check, not a published-wheel matrix check.
- The documented product fixture exported two records, rejected one row, counted
  one duplicate record and URL, and resumed without refetching.
- A packed Node tarball installed in a fresh local consumer and exported DatasetJob.
- Documentation: production build, TypeScript and 32-guide/61-example checks passed.
  All 33 content routes returned 200; `/docs` redirected with 308 and a missing page
  returned 404. Desktop/mobile navigation, theme switching, search ranking and code
  tabs were inspected. Copy-button success feedback was checked; the browser tool's
  separate virtual clipboard prevented validating the OS clipboard contents.

Logs: `target/rust-tests-final.log`, `target/browser-tests-final.log`,
`target/node-full-tests-final.log`, `target/node-job-tests.log`,
`target/python-tests-final.log`, `target/product-example-check.log`.
Website evidence: `target/docs-build-final.log`, `target/docs-route-check.json`,
`target/docs-preview.jpg`. Package evidence: `target/npm-pack-final.log` and
`target/package-validation/`.

### Documentation and repository setup (2026-10-18)

- The website production build, TypeScript and content checks passed with 43
  guides and 96 guide code examples. All 46 content routes returned 200;
  `/docs` returned its expected 308 redirect and a missing route returned 404.
- The landing page and developer documentation now share the logo and teal/navy
  palette. API references cover DatasetJob, JobConfig, Session and Page, with
  Python, Node.js and Rust examples, copy controls and Markdown page exports.
- Desktop and mobile navigation, language tabs, search, light/dark themes and
  copy success feedback were inspected. OS clipboard contents and an actual
  Markdown file download were not validated.
- Repository setup validation parsed nine GitHub YAML files and two metadata
  JSON files, checked 76 local links across 24 Markdown files, and passed the
  shared release version check for `1.0.0-alpha.1`.
- Contribution, security, support, issue forms, ownership, dependency updates,
  release notes and CI configuration are prepared locally. The revised CI matrix
  has not run on GitHub; core suites were not rerun during this documentation pass.
  Repository settings and publication still require authenticated GitHub access.

Evidence: `target/docs-developer-build.log`,
`target/docs-developer-routes.json`, `target/docs-developer-preview.jpg`.
See [.github/REPOSITORY_SETUP.md](.github/REPOSITORY_SETUP.md) for the prepared
repository settings and the steps that still need to be applied on GitHub.

### Alpha.2 preparation checks (2026-10-19)

- Updated Rust and npm to `1.0.0-alpha.2`, Python to `1.0.0a2`; both Cargo lockfiles
  and SDK manifests pass the version consistency check.
- `cargo check --locked --offline --release --lib --example dataset_scale` passed.
  The first sandboxed attempt stopped in the native TLS build; the retry with the
  installed LLVM path outside the sandbox completed successfully.
- Rust formatting, Node TypeScript checks, nine GitHub YAML files and all 12 release
  job dependency names passed local checks. Seventy-one local links across 23
  Markdown files resolved. Git diff whitespace checks passed.
- The updated documentation production build passed with 43 guides and 96 guide
  code examples. The final preview shows the alpha.2 publication instructions.
- A Python source archive was built and passed `twine check`. Inspection confirmed
  the Rust/Python job source, lockfile, license, rewritten manifest path and inline
  Python registry description. This is not a compiled wheel installation check.
- The Rust crate file list excludes SDK binaries, GitHub workflow files and local
  output. Generated Windows addon and Python JSON outputs were preserved under
  `target/repository-cleanup-20261018/` and removed from source.

The job now shares its configuration and pending dedup payloads and writes JSONL
directly into buffered output. This pass did not run test suites or benchmarks;
the earlier million-record evidence predates these allocation changes. No new
speedup, platform acceptance or completed release is claimed.

Evidence: `target/alpha2-rust-check-20261019.log`,
`target/docs-alpha2-final-build.log`, `target/docs-alpha2-preview.jpg`,
`target/alpha2-python-sdist.log`, `target/alpha2-python-dist/`, and
`target/alpha2-crate-files.txt`.
See [the published-release audit](guides/RELEASE_STATUS.md) and
[performance tuning](guides/PERFORMANCE.md) for the next checks.

### Remaining work

1. **SDK and platform coverage:** run the same acceptance on Linux and macOS and
   test packaged wheels/binaries. The revised release workflow has not yet run on
   hosted runners or uploaded a release.
2. **Recovery under faults:** add subprocess termination at every commit boundary,
   disk-full/corrupt-state fixtures and recovery checks. Current cancellation and
   restart tests do not establish power-loss durability.
3. **Broader datasets:** multiple records per page at million-record scale, varied
   payload sizes, many origins, redirects, slow/erroring servers and sustained
   runs. Measure stage timings before choosing further CPU optimizations.
4. **Export contract:** add checkpointed Parquet jobs and explicit typed schemas;
   current job values are strings, with numeric/boolean validation. Add provenance
   timestamps and dataset manifests before external storage integrations.
5. **Feature-specific acceptance:** retain separate tests for selector recovery,
   TLS profile configuration, browser execution and change monitoring. Selector
   recovery is heuristic, TLS emulation does not guarantee access, and browser
   results must not be pooled with HTTP throughput measurements. DatasetJob does
   not silently heal selectors: changes require an explicit configuration decision.
6. **Scale beyond one process:** durable discovery input, leased worker partitions,
   shared identity rules and remote shard commits, once single-node failure tests
   and representative workload measurements pass.

SIMD is supplied by selected dependencies where supported; no independent SIMD
speedup or universal “fastest Rust architecture” claim has been established.
