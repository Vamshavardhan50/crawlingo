import sys
import os

# Allow importing crawlingo from the local python source tree directly
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../sdk/python")))

from crawlingo import Crawl

def main():
    print("=== Crawlingo Web Crawler Example ===")
    
    # Start URL to crawl
    start_url = "https://httpbin.org/links/5/0"
    print(f"Starting crawl from {start_url}...")
    
    # Configure and build the crawl job
    crawl_job = (
        Crawl(start_url)
        .follow("a")          # Selector for links to discover and queue
        .limit(3)             # Limit total pages fetched
        .depth(2)             # Max crawling depth hops
        .concurrency(2)       # Fetching thread count
        .delay(0.5)           # Politeness delay between fetches
        .field("title", "h1")
        .field("links", "a")
    )
    
    results = crawl_job.build()
    
    print(f"\nCrawled {len(results)} pages:")
    for i, res in enumerate(results):
        print(f"  Page {i+1} title: '{res['title']}'")
        
    # Export all results
    results.to_json("crawl_results.json")
    print("\nSaved crawl outcomes to 'crawl_results.json'.")

if __name__ == "__main__":
    main()
