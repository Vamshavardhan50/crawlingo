"""Run one persistent dataset job; suitable for Docker, batch jobs and queues."""
import argparse
import asyncio
import json
import signal
import sys
from pathlib import Path

from crawlingo import DatasetJob, Session


async def run(args):
    session = Session.from_config(args.session)
    config = json.loads(Path(args.job).read_text(encoding="utf-8"))
    job = DatasetJob(config, session=session)
    loop = asyncio.get_running_loop()
    for signum in (signal.SIGINT, signal.SIGTERM):
        # add_signal_handler is supported by the Linux container event loop.
        loop.add_signal_handler(signum, job.cancel)
    try:
        report = await job.run_async()
    finally:
        for signum in (signal.SIGINT, signal.SIGTERM):
            loop.remove_signal_handler(signum)
    print(json.dumps(report), flush=True)
    return 0 if (
        report["complete"] and not report["cancelled"]
        and report["pages_failed"] == 0 and report["records_rejected"] == 0
    ) else 2


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--job", default="/config/job.json")
    parser.add_argument("--session", default="/config/session.toml")
    args = parser.parse_args()
    try:
        return asyncio.run(run(args))
    except Exception as exc:
        print(f"Dataset worker failed ({type(exc).__name__}): {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
