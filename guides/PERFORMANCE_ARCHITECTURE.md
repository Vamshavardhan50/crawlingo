# Performance architecture and evidence

The product contract is URL input → validated records → durable resumable output.
Requests/second and records/second are separate measurements. Neither is an
unconditional product guarantee. The [measured workloads](benchmarks.md)
separate synthetic load, live-site extraction and native idle memory.

## Implemented execution path

1. One Session shares a fetch manager, connection pools, retry policy, host rate
   limits and metrics. Do not create a new Session for every URL.
2. DatasetJob reads bounded checkpoints from the URL file. Fetch concurrency
   follows the job configuration. A per-job semaphore limits active blocking
   parse/extraction tasks to the smaller of that concurrency and available CPU
   parallelism. Results retain input order;
   a slow early page can therefore delay later output. Increasing concurrency is
   not automatically faster and consumes more memory.
3. HTTP bodies are read with a byte ceiling. Known-length responses reserve at
   most 64 KiB initially to reduce buffer growth without trusting arbitrary large
   Content-Length declarations. The job's own response limit is also checked
   after transport; it is not a replacement for the transport's streaming ceiling.
4. CSS queries share compiled selectors. The global cache retains at most 4,096
   entries and excludes selector strings over 4,096 bytes. Additional selectors
   still execute but are parsed without retention. Public get_or_compile preserves
   its owned return type for compatibility.
5. Row extraction applies required-field/type rules, exact deduplication and
   sharded writing. Sled checkpoints and committed shard integrity protect restart.
   Deduplication is disk-backed and includes a bounded in-flight checkpoint batch.
   The Sled page cache is sized at four times the checkpoint byte budget, clamped
   to 32–128 MiB. With the default budget it can keep 128 MiB of index pages hot.
   This budget applies to the database cache, not total process memory.
6. Browser rendering remains optional. Studio's animated preview is a separate
   Playwright tool; its collection rate is not the HTTP engine's throughput.

## Reproduce measurements

Build once before timing. Run workloads sequentially on an otherwise idle machine.
Use a fresh output directory every time. Repeat performance runs at least three
times before quoting a representative result; preserve all runs, not just the best.

```powershell
cargo build --locked --release --features cli --bin crawlingo --example dataset_scale --example idle_session --example text_search_probe
# HTTP dispatch and full-body verification, with no parse or export
./scripts/measure-dataset-scale.ps1 -Pages 10000 -Mode http-fetch -OutputDirectory target/perf-fetch-1
# 10,000 HTTP pages, one record per page, 50 concurrent requests
./scripts/measure-dataset-scale.ps1 -Pages 10000 -RowsPerPage 1 -OutputDirectory target/perf-pages-1
# One million records across 1,000 directory pages
./scripts/measure-dataset-scale.ps1 -Pages 1000 -RowsPerPage 1000 -MaxWallSeconds 60 -OutputDirectory target/perf-directory-1
# One million HTTP pages is a distinct, longer acceptance case
./scripts/measure-dataset-scale.ps1 -Pages 1000000 -RowsPerPage 1 -OutputDirectory target/perf-million-pages-1
./scripts/measure-idle-memory.ps1 -OutputFile target/perf-idle-1.json
./target/release/examples/text_search_probe.exe
```

The local HTTP server shares the process. Load-test memory therefore includes
the server, dataset engine, output verification and restart. Idle memory uses a
separate executable with one Session and four Tokio workers, no HTTP clients,
server, browser or language SDK. Do not substitute binary size for working set.

Each scale run checks every record's identity, title, Unicode content and order,
then verifies restart makes no additional fetches. `acceptance.json` reports job
seconds, pages/s, records/s, rows/page and checkpoint size. The PowerShell report
adds memory, wall time, binary SHA-256, source revision and dirty-tree status.
Generated mode bypasses HTTP and must never be labeled network throughput.

To compare cold processes, install the benchmark's isolated Scrapy and Playwright
dependencies. Run from the repository root.

```powershell
python -m venv target/compare-env
./target/compare-env/Scripts/python -m pip install scrapy==2.19.0
npm ci --prefix scripts/compare-engines
Push-Location scripts/compare-engines
npx playwright install chromium
Pop-Location
./target/compare-env/Scripts/python scripts/compare-engines/run.py target/compare-50 --cli target/release/crawlingo.exe --pages 200 --concurrency 50
```

## What the checks cover

| Feature | Evidence and boundary |
| --- | --- |
| Throughput | The million-page run exceeded 3,500 pages/s on tiny local HTTP bodies. Page size, remote latency, per-host limits, browser rendering and output durability all affect throughput. |
| 2.4 MB idle target | The measured native idle process used 5.58 MiB (5.85 MB decimal). Python, Node and Chromium need separate memory measurements. |
| Cross-engine throughput | The comparison uses the same input and checks every exported title and URL. See cold-process times at concurrency 4 and 50 in the results. |
| Self-healing and change detection | The integration test persists a price fingerprint, renames its CSS class, checks the exact recovered price and verifies the expected old and new prices in a change event. Accuracy across arbitrary sites remains unmeasured. |
| TLS profiles | A test captures real Chrome- and Firefox-profile ClientHello cipher lists over TCP, removes GREASE values and checks that the profiles differ. Site acceptance and full browser fingerprint equivalence require separate evidence. |
| Text search | The production text-anchor search and an exhaustive byte-window reference found the same 256 nodes in a 1,065,997-byte fixture. Across six alternating trials of 64 scans, median times were 4.83 ms and 234.34 ms. This measures algorithm and compiler effects together. An isolated SIMD gain requires a same-algorithm scalar build. |

Fetch `total_duration` now includes body consumption. Session average latency
accumulates microseconds and converts to milliseconds after averaging, preserving
fast submillisecond requests. DNS/TCP/TLS breakdowns remain unavailable (zero),
and must not be presented as individually measured stages.
