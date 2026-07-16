import crawlingo
import time

def on_any_change(event):
    print(f"  [CHANGE EVENT] Field '{event.field}' changed!")
    print(f"    Old value: '{event.old_value}'")
    print(f"    New value: '{event.new_value}'")

def on_price_drop(event):
    print(f"  🔥 [PRICE DROP] Honey, price for '{event.field}' dropped to {event.new_value}!")

def main():
    print("=== Operation Silk Sheets: Product Monitor ===")
    
    session = crawlingo.Session()
    # Watch httpbin.org for testing
    url = "https://httpbin.org/html"
    
    print(f"Watching URL darlin': {url}")
    watcher = crawlingo.Watch(url, session)
    
    # We will query the header and body text
    watcher.field("title", "h1")
    watcher.field("price", "p")
    
    # Register callbacks
    watcher.on_change(on_any_change)
    watcher.on_price_change(on_price_drop)
    watcher.interval(2)
    
    print("Starting background watch loop. Press Ctrl+C to terminate.")
    try:
        # Run asynchronously in a background thread
        import asyncio
        asyncio.run(watcher.run_async())
        
        # Keep running for 10 seconds for the example
        time.sleep(10)
        watcher.stop()
        print("Watcher stopped cleanly.")
    except KeyboardInterrupt:
        watcher.stop()
        print("Watcher stopped by user.")
    except Exception as e:
        print(f"Watch loop encountered error: {e}")

if __name__ == "__main__":
    main()
