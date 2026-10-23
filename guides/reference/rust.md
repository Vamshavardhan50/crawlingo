# Rust API and rustdoc

The complete generated module/type/trait reference, plus a map of the entry points used to build datasets.

<a id="generate"></a>

## Open the exact reference for your source

rustdoc reads the Rust declarations directly and includes functions, public fields, enum variants, trait implementations, generics and source links. Its search is the authority for the full Rust surface; the Node and Python indexes document their separate wrappers.

Build with the browser feature to include optional Chromium modules. CLI and PyO3 FFI symbols are feature-specific; add cli or python only when you need those surfaces and have their build dependencies. These commands build documentation; they do not run examples as tests.

### Engine checkout

```shell
cargo doc --locked --no-deps --lib --features browser --open
# Output: target/doc/crawlingo/index.html
# All feature-gated declarations, including the binding layer:
cargo doc --locked --no-deps --all-features --open
```

<a id="entry-points"></a>

## Public module map

| Module | Purpose | Start with |
| --- | --- | --- |
| dataset | Bulk jobs, small builders, validation and exports | DatasetJob, JobConfig, JobField, JobReport; builder::Dataset; stream::DatasetStream |
| engine::session | Shared request state and pooled transports | Session::new, from_config, fetch_manager, clone_session |
| engine::fetcher | Requests, responses and injectable transports | FetchRequest, NormalizedResponse, FetchManager, Transport, MockTransport |
| engine::retry / rate_limiter / pool | Backoff, pacing and connection limits | RetryPolicy, HostRateLimiter, ConnectionPoolConfig |
| engine::auth / cache / middleware | Authentication, response caching and layers | AuthScheme, InMemoryCache, MiddlewareStack, Layer |
| engine::download | File and in-memory body downloads | Downloader, DownloadResult |
| engine::browser | Optional local Chromium/CDP rendering | Enable the browser Cargo feature |
| crawl | Discovery, pagination, robots and frontier storage | crawler::Crawler; sitemap; pagination; frontier; robots |
| parser | HTML parsing, DOM structure and Page | streaming::HtmlParser; document::Page, DomTree |
| selector | CSS, basic XPath, regex and text selection | css, xpath, regex_selector, text_anchor |
| extraction | Field normalization and attribute extraction | ExtractionEngine, ExtractionType |
| fingerprint / matcher | Stored element identity and heuristic recovery | FingerprintStore and auto_matcher |
| change / watch | Value comparison and polling | ChangeEvent, ChangeType; watch::monitor::Watch |
| config | TOML/JSON configuration and environment overrides | CrawlingoConfig, RetryConfigSpec, PoolConfigSpec |
| metrics | Fetch counts and timings | Metrics, MetricsSnapshot, MetricsLayer |
| error / util | Typed failures and URL/header/value utilities | CrawlingoError, Result; util::url |

<a id="job"></a>

## Asynchronous job example

Use async APIs inside an existing Tokio runtime. The shared TOKIO_RUNTIME exists for blocking callers and bindings; avoid nesting its block_on inside another async runtime.

Keep the Arc<Session> alive for related work. Cancellation is cooperative. Do not release a job directory to another writer before the active job has stopped.

### Rust

```rust
use crawlingo::dataset::{DatasetJob, JobConfig};
use crawlingo::engine::session::Session;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: JobConfig = serde_json::from_reader(
        std::fs::File::open("job.json")?
    )?;
    let session = Arc::new(Session::new());
    *session.rate_limit_rps.write().unwrap() = 2.0;
    let job = DatasetJob::new(config, session)?;
    let report = job.run().await?;
    println!("written={}, failed={}", report.records_written, report.pages_failed);
    Ok(())
}
```

- [Job configuration fields](../api-job-config.md)
- [Report semantics](../report-reference.md)

<a id="artifacts"></a>

## Versioned documentation artifacts

The engine Rust API documentation workflow builds browser-enabled rustdoc with broken intra-doc links treated as errors. It uploads the rust-docs artifact. The SDK release requires that artifact and includes crawlingo-VERSION-rustdoc.tar.gz in its checksummed release manifest.

Download a successful workflow artifact or a published release archive, extract it, and open doc/crawlingo/index.html. Prepared workflow code is not proof of a successful hosted build or upload. Unpublished versions do not have a registry docs.rs page yet.

- [Engine documentation workflow](https://github.com/Vamshavardhan50/crawlingo/actions/workflows/rustdoc.yml)
- [Engine releases](https://github.com/Vamshavardhan50/crawlingo/releases)
- [Documentation release procedure](docs-release.md)
