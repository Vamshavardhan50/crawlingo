# CLI reference

Every current command and flag from src/bin/crawlingo.rs, with input/output and failure behavior.

<a id="install"></a>

## Build and inspect

On Windows use target/release/crawlingo.exe. Run commands from a working directory with access to input and output paths. Logs go to stderr; command output uses stdout.

### Shell

```shell
cargo build --locked --release --features cli
./target/release/crawlingo --help
./target/release/crawlingo dataset --help
```

<a id="global"></a>

## Global options

Precedence is defaults, config file, environment overrides, then explicit CLI options. Shared configuration validation runs before execution.

| Flag | Value | Behavior |
| --- | --- | --- |
| --config | TOML/JSON path | Loads shared session configuration; CRAWLINGO_CONFIG can supply the path. |
| --proxy | Proxy URL | Overrides config; CRAWLINGO_PROXY is also supported. |
| --rate-limit-rps | Number | Per-host request rate; overrides config. |
| --timeout | Whole seconds | Per-attempt timeout; overrides config. |
| --help / -h | None | Show help for this command. |
| --version / -V | None | Show compiled package version. |

<a id="dataset"></a>

## dataset <job>

Read a JobConfig JSON file, generate/resume shards and print the report. The CLI handles Ctrl+C and Unix SIGTERM by requesting cancellation and waiting for cleanup.

Exit 0 means successful completion without rejected rows or failed pages. Exit 2 signals incomplete/cancelled work or quality failures. Fatal setup/run errors exit 1. Keep state and committed shards together for resume.

### Shell

```shell
crawlingo --rate-limit-rps 2 dataset job.json
```

- [Every JobConfig field](../api-job-config.md)
- [Output and resume](../recovery.md)

<a id="fetch"></a>

## fetch <url>

### Shell

```shell
crawlingo fetch https://example.com/ --markdown
```

| Option | Behavior |
| --- | --- |
| --markdown | Print the parsed page as Markdown instead of HTML. |
| --stealthy | Use HTTP TLS/browser-profile emulation; this does not execute page JavaScript. |

<a id="crawl"></a>

## crawl <url>

This collects a bounded crawl result in memory. It does not create DatasetJob shards or its checkpoint format. Review per-page failures in logs.

### Shell

```shell
crawlingo crawl https://example.com/ --limit 10 --depth 1 --field title:h1 --output json
```

| Option | Default | Behavior |
| --- | --- | --- |
| --follow | a | CSS link selector. |
| --limit | 10 | Maximum pages. |
| --depth | 3 | Maximum link-hop depth. |
| --concurrency | 2 | Crawl workers. |
| --delay | 0 | Pacing delay in seconds. |
| --field NAME:SELECTOR | None | Repeatable named CSS extraction field. |
| --output | json | json or csv stdout format. |
| --resume | None | Persistent frontier directory. |

<a id="sitemap"></a>

## sitemap <url>

Inspect the discovered URLs before using them as a dataset input. URL discovery does not guarantee successful fetching or a shared record schema.

### Shell

```shell
crawlingo sitemap https://example.com/sitemap.xml --urls-only > urls.txt
```

| Option | Default | Behavior |
| --- | --- | --- |
| --urls-only | false | Print loc URLs one per line. |
| --max-depth | 5 | Maximum index nesting depth. |

<a id="download"></a>

## download <url>

### Shell

```shell
crawlingo download https://example.com/ --output page.html --max-bytes 1048576
```

| Option | Default | Behavior |
| --- | --- | --- |
| --output / -o | Required | Destination file. |
| --no-resume | false | Restart instead of attempting a range request. |
| --max-bytes | No explicit retained-byte limit | Limit bytes saved; the transport still has a response-body cap. |

- [Download semantics and limitations](node/downloader.md)
