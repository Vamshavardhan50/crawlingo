# Measured scale

## Live public directory — one million unique domains in 7.02 seconds

On November 6, 2026 (India time), the Crawlingo Rust HTTP transport downloaded
Tranco snapshot `JZPGY` over HTTPS. Its 22,639,581-byte CSV arrived in 4.14 s.
The importer checked consecutive ranks and case-insensitive domain uniqueness
and flushed the JSONL file to disk by 5.87 s. It then reopened the file and
compared each rank and domain to the corresponding CSV row. Total measured
process time was **7.02 s**. The final JSONL contains 1,000,000 rows and
41,639,581 bytes. The source CDN reported a cache hit. This was one fresh client
download of a bulk ranking. The HTML job and per-URL measurements follow below.

To reproduce, use a fresh output directory under the existing build directory:

```powershell
cargo build --release --locked --example tranco_directory
./target/release/examples/tranco_directory.exe JZPGY target/tranco-JZPGY
```

[Tranco snapshot](https://tranco-list.eu/list/JZPGY) · [Run evidence and hashes](../measurements/2026-11-06-tranco.json)

## One million verified records in under 60 seconds

On November 6, 2026 the optimized Rust dataset job passed three consecutive
Windows release runs against fresh local HTTP fixtures. Each run fetched 1,000
HTML pages with 1,000 rows per page, validated three extracted fields, performed
exact deduplication and committed 20 JSONL shards. The harness independently
read every row and checked its ID, title, Unicode content, source URL and position.
It then restarted the job and counted zero extra HTTP requests.

| Trial | Export complete | Whole process including verification and restart |
| --- | ---: | ---: |
| Previous build, fresh baseline | 76.18 s | 80.27 s |
| Optimized, first run | 40.16 s | 44.37 s |
| Optimized, second run | 50.10 s | 54.60 s |
| Optimized, third run | 40.49 s | 44.78 s |

At eight logical CPUs and 50 concurrent requests, the median export rate was
24,698 records/s. Process peak working sets ranged from 588 to 623 MiB, including
the fixture server. The 60-second gate applies to the whole process. Source
latency, rows per response and the site's allowed request rate determine whether
a particular public dataset can meet the same deadline. On the same optimized
binary, one million separate local HTTP requests at one row per page took
195.28 seconds to export and 200.82 seconds including verification and restart.
That workload missed the 60-second target.

[Trial reports and executable hashes](../measurements/2026-11-06-minute.json) · [Run the time gate](PERFORMANCE_ARCHITECTURE.md#reproduce-measurements)

## Earlier November 2026 working-tree results

Windows x86_64, eight logical CPUs, release builds. Each HTTP scale case used 50 concurrent requests and a local fixture serving UTF-8 HTML. Dataset times include parsing, validation, deduplication, checkpointing and JSONL export. The harness then read each row, compared all fields and source URLs with the expected values and checked that restarting produced zero extra requests.

| Workload | Observed result | Trials |
| --- | --- | --- |
| HTTP fetch and verify 10,000 bodies, about 179 bytes each | 11,781 requests/s median | 3 |
| Fetch 10,000 pages, export one row per page | 6,754 pages/s median | 3 |
| Fetch 1,000,000 pages, export one row per page | 221.89 s, 4,507 pages/s, 195.4 MiB peak working set | 1 |
| Fetch 1,000 directory pages, export 1,000 rows per page | 142.64 s, 7,011 rows/s, 685.6 MiB peak working set | 1 |
| One idle native Session, four Tokio workers | 5.58 MiB median working set | 45 samples of one process |
| Live Python PEP index, robots enabled, rate limit 1 request/s | 746 verified records from one page, zero rejected, unchanged shards on restart | 1 |

The short HTTP tests time in-process work after startup. Both million-scale jobs shared the machine with some development activity. Their memory figures cover the local server, engine and subsequent verification. The live PEP run measures extraction quality on a public directory. Its network conditions differ from the local load tests.

### Same-input comparison

Each engine started a fresh process, extracted one title and URL from each of 200 small static pages on the same local server, and wrote JSONL. The harness verified all 200 pairs after each run and rotated the engine order. The rates below are medians over three runs per engine at each concurrency. Crawlingo also wrote recovery checkpoints. Playwright launched Chromium and loaded each page. Startup contributes materially to a job this short.

| Requested concurrency | Crawlingo CLI | Scrapy 2.19.0 | Playwright 1.64.1 |
| --- | ---: | ---: | ---: |
| 4 | 979.3 pages/s | 75.7 pages/s | 35.9 pages/s |
| 50 | 814.1 pages/s | 68.7 pages/s | 24.9 pages/s |

See the [individual timings, row counts, binary hashes and process measurements](../measurements/2026-11-06-performance.json) and [architecture and reproduction commands](PERFORMANCE_ARCHITECTURE.md).

<a id="result"></a>

## October 2026 historical run

Recorded on 2026-10-14 on Windows x86_64 with eight logical processors, using a release build and small generated UTF-8 HTML served over local HTTP. The fixture server, output checks and restart run in the same process. Other development builds shared the machine.

This used the modified working tree based on d19a62c04ab09713d04f86e958434665b60d131f, before final robots edge-case fixes. Robots checks were disabled for the fixture.

| Measurement | Observed result |
| --- | --- |
| Pages fetched / records exported | 1,000,000 / 1,000,000 |
| Pages failed / rows rejected | 0 / 0 |
| Records individually checked | 1,000,000 |
| Concurrency / checkpoint pages | 50 / 1,000 |
| Initial job time | 1,094.569 seconds |
| Harness wall time | 1,101.961 seconds |
| Peak working set | 181.5 MiB |
| Resume without refetching | Passed |

<a id="meaning"></a>

## What the result establishes

The production job processed the input, exported the expected records, checked every value and resumed without refetching. It does not predict throughput on arbitrary websites, JavaScript rendering, many hosts or larger pages.

The October run measured one dataset workload. The November checks above separately measure HTTP dispatch, dataset export, native idle memory and a cold-start comparison.

<a id="reproduce"></a>

## Reproduce the run

Use a new output directory. acceptance.json records the original job and verification. The PowerShell wrapper records process memory and wall time alongside it. After the harness resumes, dataset/report.json describes the short restart invocation.

### PowerShell

```powershell
cargo build --release --example dataset_scale
./scripts/measure-dataset-scale.ps1 -Pages 1000000 -OutputDirectory D:/Scraper/target/scale-new -Mode http
```

### Any supported platform

```shell
cargo run --release --example dataset_scale -- 1000000 target/scale-new http
```

- [Full result and release gates](https://github.com/Vamshavardhan50/crawlingo/blob/main/SCALE_RESULTS.md)
