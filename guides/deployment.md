# Deploy a dataset worker

Run the SDK in a backend process with persistent storage, predictable limits and a clear restart procedure.

<a id="choose"></a>

## Choose a runtime

| Platform | How to run Crawlingo |
| --- | --- |
| Local machine or VM | Build the CLI or SDK for the host OS and architecture. Keep the job directory on a local persistent disk. |
| Docker on a Linux host | Use the provided CLI Dockerfile. Mount a working directory at /work; keep input, config and output there. |
| Container job platform | Run the same worker image with a persistent volume and one replica per job directory. Configure termination grace time. |
| React / Next.js application | Queue work from the server and let a separate worker run the native SDK. Display the report in the frontend. |
| Vercel / Netlify docs hosting | Host the documentation website. A million-page job needs a worker and storage outside the docs deployment. |

<a id="container"></a>

## Build a CLI worker image

The image compiles the current Rust source and includes the CLI plus CA certificates. Docker examples use a POSIX shell. Supply your own HTTP(S) source URLs; localhost inside a container refers to that container.

### Build and run

```shell
docker build -f deploy/Dockerfile.cli -t crawlingo-worker .
mkdir -p work
# Save job.json and urls.txt inside work/.
# In job.json: urls_file="urls.txt", output_dir="output".
docker run --rm --init --stop-timeout 120 \
  --mount type=bind,src="$(pwd)/work",dst=/work \
  crawlingo-worker dataset job.json
```

- [Worker Dockerfile and operating notes](https://github.com/Vamshavardhan50/crawlingo/tree/main/deploy)

<a id="sdk"></a>

## Use Python or Node in your own worker

Build or install a package for the worker OS, CPU and runtime. A Windows .node or .pyd file cannot be copied into a Linux image. Until the new release is published, use the source installation guides. Node is a server runtime dependency; the addon cannot execute in a browser or edge isolate.

Use DatasetJob.run() in Python or await job.run() in Node. Pass an explicit output directory per job. Send job.cancel() when your worker is shutting down, await completion, and only then release its volume. The Rust CLI handles Ctrl+C and SIGTERM.

- [Python integration](sdk-python.md)
- [Node integration](sdk-nodejs.md)

<a id="operate"></a>

## Operate and resume

Local Sled storage is the checkpoint backend. Shared network filesystems and multi-host writers are not validated deployment targets. Container builds and cloud execution still require platform acceptance; the measured million-page result is from Windows.

- Start with a representative small input; inspect records, errors and disk growth before increasing volume.
- Keep state and all committed shards on the same persistent volume. Do not share one directory between active workers.
- Record pages_failed, records_rejected, duplicate_records, records_written and elapsed_ms from report.json.
- After interruption, run the unchanged configuration with the same input and directory. Resume before consuming output after a hard crash.
- Back up a stopped job as a unit. Treat input or schema changes as a new job.

- [Recovery details](recovery.md)
- [Resource limits](dataset-jobs.md#limits)
