# Public research dataset evaluation

Measured **2026-10-19** using the current Rust CLI and real HTTPS requests to
[Python's PEP directory](https://peps.python.org/numerical/). The source also
publishes a metadata API; this experiment evaluates Crawlingo's HTML path.

## Results

| Workload | Pages fetched | Records written | Job time | Peak CLI working set | JSONL bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| Numerical directory | 1 | 744 | 4.620 s | 42.0 MiB | 151,865 |
| First 20 linked documents | 20 | 20 | 20.539 s | 38.9 MiB | 312,906 |

Both jobs had **0 failed pages, 0 rejected records and 0 duplicate records**.
The directory and documents are related datasets; their counts are not 764 unique
research entities. Both completed-job restarts kept all data/error shard hashes
unchanged. Restart job times were 40 ms and 51 ms respectively.

## Conditions

- Windows 11, 8 logical CPUs, release-mode native Rust CLI, standard HTTP transport.
- One host; concurrency 2; **1 request/second per host**; 30-second request timeout.
- Robots checks enabled. The source's robots.txt returned HTTP 404 during preflight.
- Four pages per checkpoint, 2 MiB job response acceptance limit, 256 KiB record
  limit. Directory row cap explicitly set to 2,000.
- Memory is Windows peak working set for the Rust CLI child only, sampled by the
  harness every 20 ms. It excludes Python, compilation and the remote server.
- Job times come from `JobReport.elapsed_ms`. Process wall times including startup
  were 4.669 s and 20.583 s. This is one measured run after an exploratory pilot;
  it is not a statistical performance study.
- Binary, input, relevant source-file and shard hashes plus full counters are in
  [the machine-readable evidence](measurements/2026-10-19-peps.json).

## Quality checks and corrections

The first pilot produced 744 rows but included a neighboring version link in
522 titles. This was a **selector error despite successful requests and valid row
counts**. Positive-integer `:nth-child(N)` support now selects the intended table
column, and the example uses that selector.

Independent verification with Python's HTMLParser compared all 744 normalized
IDs, titles and classifications against a separately saved index snapshot:
**zero field mismatches** after the fix. Every exported record was also checked
for required values, source/row provenance, unique identity and report count.

Code review found that mixed inline HTML text could be reordered by the custom
DOM. The fix stores text ranges and child references in document order. Three
new parser regression cases check Unicode, inline punctuation, word fragments,
block boundaries, script/style exclusion and preserved local whitespace.
PEP 1's extracted text matches the independently parsed source character order
after entity decoding and ignoring whitespace. This check covers **one** document
body; the remaining 19 bodies are not semantically certified.

Focused checks passed: 4 parser unit tests, 3 CSS selector unit tests, 8 dataset-job
tests and 4 edge-case tests. These are not a claim that the whole repository CI,
all SDK builds or every advertised feature passes.

## What this establishes

The production engine can collect a real research directory, create a fixed
document sample, export bounded local shards, and restart completed jobs without
changing their output. A page can produce many records; pages and records must
be reported separately.

The sample does **not** establish maximum public-web speed, million-page production
capacity, arbitrary-site compatibility, JavaScript rendering, PDF extraction,
failure recovery under process kill, or extraction accuracy across every document.
Pacing dominates this document run. Raising concurrency does not override the
source's allowed rate or make a different website equivalent to this source.

The earlier [million-page local HTTP acceptance](SCALE_RESULTS.md) remains separate
evidence from 2026-10-14. It has **not been rerun after the current parser changes**.
The old 3,500 req/s and 2.4 MB claims are not supported by these measurements.

## Reproduce and scale responsibly

Follow the [research example](examples/research_dataset/README.md). Its measurement
runner uses the compiled CLI for HTTP collection; Python reads local artifacts
and monitors the process. Raw source content remains local under `work/research/`.
Use fresh output directories for a new measurement.

For one million records, first measure accepted unique records per page and
average serialized size. At one allowed page/second, one million single-record
pages require at least **11.6 days** of request time; 50,000 pages yielding 20 valid
records each require at least **13.9 hours**. These are arithmetic planning
examples, before overhead and failures, not measured throughput promises.

See [dataset design](guides/dataset-design.md) for identity, quality gates, memory
limits and storage planning, and [the architecture review](ARCHITECTURE_AUDIT.md)
for remaining engineering work.
