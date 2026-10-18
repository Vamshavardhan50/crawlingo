# Getting help

## Start with a working example

- New users: [what Crawlingo does](guides/PRODUCT.md) and the
  [local product example](examples/product_dataset/README.md).
- SDK users: [Python](sdk/python/README.md), [Node.js](sdk/nodejs/README.md),
  [DatasetJob API](guides/api-dataset-job.md) and [JobConfig](guides/api-job-config.md).
- Operators: [deployment](guides/deployment.md), [report counters](guides/report-reference.md)
  and [recovery](DATASET_JOBS.md).

These files describe the source in this repository. Check your installed version
before assuming a published package contains an unreleased API.

## Report a problem

Use the [issue forms](https://github.com/Vamshavardhan50/crawlingo/issues/new/choose)
for bugs, documentation problems and feature proposals. Include:

1. The package version or commit and whether it was built from source.
2. OS, CPU architecture and Rust / Python / Node.js versions.
3. A minimal configuration, command and small HTML fixture.
4. Expected records, actual records and relevant report counters.
5. Whether this was a new run or a resumed job.

Remove cookies, tokens, credentials and private source data before posting. For
security vulnerabilities, follow [SECURITY.md](SECURITY.md).

Support is community based. Response times and a fix date are not guaranteed.
