# Configuration

Separate dataset shape, request settings and runtime resource limits.

<a id="job"></a>

## Dataset configuration

job.json describes input, output, extraction, validation, deduplication and job limits. It is distinct from the CLI’s global TOML/JSON session configuration. Input and job settings are checked when resuming.

- [Dataset job reference](dataset-jobs.md)

<a id="session"></a>

## Request configuration

Use a Session for headers, cookies, proxies, timeouts and per-host pacing. SDK method names differ between Python and Node.js; the job JSON keys are the same in both.

- [Session examples](session.md)

<a id="cli"></a>

## Rust CLI flags

Use --help on the built CLI to see its supported subcommands and flags. The Python command-line wrapper is separate; the dataset command shown here belongs to the Rust CLI.

### CLI

```shell
cargo run --release --features cli -- --rate-limit-rps 2 --timeout 30 dataset job.json
```

| Flag | Environment variable |
| --- | --- |
| --config | CRAWLINGO_CONFIG |
| --proxy | CRAWLINGO_PROXY |
| --rate-limit-rps | CRAWLINGO_RATE_LIMIT_RPS |
| --timeout | CRAWLINGO_TIMEOUT_SECONDS |
