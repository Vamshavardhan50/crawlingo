# Development changes

Changes in the current working tree, separate from published package availability.

<a id="dataset"></a>

## Dataset pipeline

- Added DatasetJob and matching language wrappers.
- Added bounded fetching, validation, exact deduplication and resumable shards.
- Added a scale harness that verifies all exported records and idempotent restart.

<a id="fixes"></a>

## Correctness fixes

- Bounded stream queues and concurrent Parquet production/consumption.
- Stable CSV column ordering and propagated export errors.
- Indexed persistent frontier membership and atomic progress updates.
- Response-size limits, retry pacing and lazy browser initialization.
- Robots wildcard, query-string and custom-agent handling.

<a id="docs"></a>

## Documentation

Reorganized around dataset generation, corrected SDK examples and replaced unsupported performance comparisons with measured acceptance evidence. These changes have not been represented as a new published release.

- [Repository changelog](https://github.com/Vamshavardhan50/crawlingo/blob/main/CHANGELOG.md)
