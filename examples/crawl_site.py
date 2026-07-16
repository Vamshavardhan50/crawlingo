import crawlingo
import os

def main():
    print("=== Operation Warm Lover: Site Crawler ===")
    
    session = crawlingo.Session()
    session.rate_limit(5.0)  # Politeness first, darlin'
    
    # We will crawl a mock site
    start_url = "https://quotes.toscrape.com/"
    print(f"Starting crawl at: {start_url}")
    
    crawl = crawlingo.Crawl(start_url, session)
    crawl.follow("a")  # Follow outbound links
    crawl.limit(5)      # Limit to 5 pages
    crawl.concurrency(2)
    
    # Extract quote text and author name
    crawl.field("quote", ".quote .text")
    crawl.field("author", ".quote .author")
    
    try:
        results = crawl.build()
        print(f"Crawl completed! Crawled {len(results)} pages.")
        
        output_path = "quotes_extracted.json"
        results.to_json(output_path)
        print(f"Saved results to: {os.path.abspath(output_path)}")
        
        # Display sample
        if len(results) > 0:
            print("\nSample records:")
            for idx, r in enumerate(results[:3]):
                print(f"  [{idx + 1}] {r.to_dict()}")
    except Exception as e:
        print(f"Crawl failed, honey: {e}")

if __name__ == "__main__":
    main()
