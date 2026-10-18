# Deploy a dataset worker

Run the Rust CLI or a Python/Node SDK worker on a host with persistent disk.
The current checkpoint store supports one local writer per job directory.

## CLI container

From the repository root, using a POSIX shell:

```sh
docker build -f deploy/Dockerfile.cli -t crawlingo-worker .
mkdir -p work
# Put job.json and urls.txt in work/.
# In job.json: urls_file="urls.txt", output_dir="output".
docker run --rm --init --stop-timeout 120 \
  --mount type=bind,src="$(pwd)/work",dst=/work \
  crawlingo-worker dataset job.json
```

The image runs as UID 10001. Give that user write permission on the mounted
directory, or pass `--user "$(id -u):$(id -g)"` on a Linux host. Use source URLs
reachable from the container: `localhost` points to the container itself.
The CLI requests cancellation on SIGTERM and Ctrl+C. The next unchanged run
replays the unfinished batch. Keep the input, state and shards together.

The Dockerfile is a deployment template; it has not been built on this Windows
validation host. Pin a tested image digest and retain the resolved Cargo.lock
in your release build process for repeatable production builds.

## SDK workers

Build/install the SDK for the worker's OS and architecture. Do not copy a Windows
native addon into Linux. Run `DatasetJob` in a backend worker, pass an exclusive
output directory and await cancellation during shutdown. Browser/edge JavaScript
runtimes cannot load the Node addon.

On a VM or container job service, attach persistent storage and run one replica
per job directory. Shared network filesystems and multi-host writers remain
unvalidated. Record report counters and use explicit disk quotas. Copy or back up
the complete job directory while its writer is stopped.

See [the operations guide](../guides/deployment.md) and
[SDK publication guide](../guides/publishing.md). Docs hosting is separate:
use [the website guide](../guides/docs-hosting.md).
