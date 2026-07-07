"""Crawlingo: Web page change monitoring with Watch.

Usage:
    pip install crawlingo
    python 04_monitor.py
"""

import asyncio
from crawlingo import Watch


def on_any_change(event):
    print(f"\n[EVENT] Field '{event.field}' changed!")
    print(f"  Old value: '{event.old_value}'")
    print(f"  New value: '{event.new_value}'")
    print(f"  Event Type: {event.event_type}")


async def main():
    print("=== Crawlingo Web Monitor Example ===")

    url = "https://httpbin.org/html"
    print(f"Starting async monitor for {url}...")

    watcher = (
        Watch(url)
        .field("title", "h1")
        .interval(2)
        .on_change(on_any_change)
    )

    watch_task = asyncio.create_task(watcher.run_async())

    print("Watcher running for 5 seconds...")
    await asyncio.sleep(5.0)

    print("Stopping watcher...")
    watcher.stop()
    await watch_task
    print("Monitor stopped.")


if __name__ == "__main__":
    asyncio.run(main())
