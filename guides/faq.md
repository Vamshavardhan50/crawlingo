# Troubleshooting

Start with the reported error, the invocation counters and the committed output boundary.

<a id="missing-api"></a>

## DatasetJob cannot be imported

You may be using a published alpha or a stale native binary. Build the current source, rebuild the language wrapper and confirm your application resolves that local package. Python requires the matching extension module; Node requires matching native assets in dist.

- [Installation](installation.md)

<a id="empty"></a>

## A job finished with fewer records than expected

Check pages_failed, records_rejected, duplicate_urls and duplicate_records. Read the error shards. Confirm row_selector matches containers, required fields exist, and dedup_fields reflects your intended identity.

A completed job does not retry previously failed URLs on an unchanged resume. Create a new input/job after correcting the source or extraction rules.

<a id="resume"></a>

## Resume rejects my configuration

Input and job configuration are fingerprinted. Changes to either require a fresh output directory. Restore missing committed shards before resuming; do not delete the state directory to force continuation.

<a id="memory"></a>

## Memory or checkpoint limits are exceeded

Use smaller concurrency, response limits, page record limits and checkpoint batches in a new job. checkpoint_bytes bounds pending identity payloads; process RSS also includes DOMs, network buffers and the database cache. Large pages need separate profiling.

<a id="blocked"></a>

## Pages are blocked or JavaScript content is missing

Inspect HTTP status and robots failures. standard and stealthy fetchers do not render JavaScript. Use the optional Rust browser feature when rendering is necessary. TLS profiles do not guarantee access or solve challenges automatically.

<a id="build"></a>

## Native builds or tests fail on Windows

Confirm the C/C++ build tools and CMake are available. Use Python 3.12 for the current PyO3 build. Local HTTP tests need loopback networking, and Sled tests need a writable temporary directory. Report the exact build log rather than treating an environment failure as a throughput result.
