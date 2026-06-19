import sys
import os
import asyncio

# Allow importing crawlingo from the local python source tree directly
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../sdk/python")))

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
        .interval(2)  # Check every 2 seconds for this example
        .on_change(on_any_change)
    )
    
    # Run the watcher loop asynchronously
    watch_task = asyncio.create_task(watcher.run_async())
    
    print("Watcher is running in the background. We will stop it after 5 seconds...")
    await asyncio.sleep(5.0)
    
    print("Stopping watcher...")
    watcher.stop()
    await watch_task
    print("Monitor stopped successfully.")

if __name__ == "__main__":
    asyncio.run(main())
