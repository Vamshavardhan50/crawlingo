# Crawling and discovery

Follow links for discovery; use URL-file jobs for large dataset exports.

<a id="boundary"></a>

## Choose the right API

The existing crawler follows links, supports a frontier, and collects results. It is intended for smaller interactive crawls. DatasetJob streams an existing URL list into output shards and is the API used by the million-page acceptance run.

The persistent frontier stores queued and visited membership on disk and recovers in-flight entries after reopening. This does not make the collecting result vector a streaming dataset.

<a id="cli"></a>

## Discover pages with the Rust CLI

Choose a conservative limit and inspect the results. Robots checks and pacing are separate from the maximum page count.

### CLI

```shell
cargo run --release --features cli -- crawl https://example.com/ --follow a --limit 10 --depth 2 --concurrency 2
```

<a id="large"></a>

## Prepare bulk input

Write discovered HTTP(S) URLs to a UTF-8 file, one per line. DatasetJob separately deduplicates URL identities and record identities. Durable discovery directly feeding a bulk job is a future extension; it is not part of the current URL-file contract.

- [Dataset job reference](dataset-jobs.md)
