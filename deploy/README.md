# Python and Node.js dataset workers

These source-built containers run one persistent `DatasetJob`, print its report,
and exit. They use the Rust engine through the Python or Node.js SDK. The default
Compose configuration collects a deterministic local research directory: two
pages, four extracted rows, three accepted records and one exact duplicate.

## Run the example

From the repository root with Docker Engine and Compose installed:

```sh
docker compose -f deploy/compose.yaml --profile python build python
docker compose -f deploy/compose.yaml --profile python run --rm python

docker compose -f deploy/compose.yaml --profile node build node
docker compose -f deploy/compose.yaml --profile node run --rm node
```

Python and Node use separate named volumes. Repeat the same command to verify
the saved input and shards and resume without fetching completed pages again.
`report.json`, `manifest.jsonl`, data/error shards and `state/` live together in
`/data/research-demo`. Preserve the whole volume. `docker compose down` preserves
named volumes; adding `--volumes` would delete the dataset and recovery state.

## Use your own URL file

Build an image, create a writable output directory owned by UID 10001, and mount
your configuration as read-only. Paths inside job.json must use container paths:

```json
{
  "urls_file": "/config/urls.txt",
  "output_dir": "/data/my-dataset",
  "fields": [{"name": "title", "selector": "h1", "required": true}],
  "concurrency": 4,
  "respect_robots": true
}
```

```sh
docker build -f deploy/Dockerfile.python -t crawlingo-python .
docker volume create crawlingo-data
docker run --rm --init --stop-timeout 60 \
  --mount type=bind,src="$PWD/my-config",dst=/config,readonly \
  --mount type=volume,src=crawlingo-data,dst=/data \
  crawlingo-python
```

Use `deploy/Dockerfile.nodejs` and an image name of your choice for Node.js.
`my-config` must contain job.json, urls.txt and session.toml. The sample
`deploy/config/session.toml` sets per-host pacing, timeout, retry and User-Agent.
Both workers accept `--job PATH --session PATH` and shared `CRAWLINGO_*` settings.

## Operating contract

- Exit **0**: complete, with no failed pages or rejected records.
- Exit **2**: cancelled, incomplete, or completed with quality failures. Inspect
  the report and errors; retrying a completed job does not retry its failed URLs.
  Create a new URL file and output directory for a targeted retry.
- Exit **1**: configuration, storage or fatal engine error.
- SIGTERM/SIGINT request cancellation and wait for native cleanup. The current
  uncommitted batch is replayed on restart. A hard kill requires restart validation.
- One worker owns one output directory. For parallel workers, partition inputs
  and use different volumes/directories; deduplication and pacing are local to a
  job/session, not coordinated across machines. Never share Sled state over NFS.
- Upload or ingest committed shards only after checking the report and manifest.
  Queue systems should acknowledge a job after the worker finishes and its output
  is durably stored. Use a stable job identifier to locate retries' persistent volume.
- Pin built image digests for deployments. Build tags here select supported base
  image families; published registry images are not supplied by this repository.

These examples are source artifacts. Docker is not installed on the preparation
machine; container runtime verification is a separate deployment gate. SDK and
engine verification does not establish production orchestration or storage durability.
