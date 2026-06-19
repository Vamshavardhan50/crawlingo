import sys
import os

# Allow importing crawlingo from the local python source tree directly
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../sdk/python")))

from crawlingo import Session, Page, Dataset

def main():
    print("=== Crawlingo Session Configuration Example ===")
    
    # Initialize a shared cookie and rate-limiting context
    with Session() as session:
        session.headers({"User-Agent": "Customcrawlingor/1.0", "X-My-Header": "MyValue"})
        session.timeout(15)
        session.auto_match(True)
        session.fingerprint_path(".crawlingo_custom_fingerprints")
        
        # Requests using this session context will share headers and connection limits
        print("Fetching page 1 with session...")
        page1 = Page("https://httpbin.org/headers")
        print("Page 1 body response headers:")
        print(page1.html()[:300]) # Preview returned request headers
        
        # Build dataset utilizing the session configuration
        print("\nExtracting data through session...")
        dataset = (
            Dataset("https://httpbin.org/html", session=session)
            .field("title", "h1")
            .build()
        )
        print(f"Dataset result: {dataset.to_dict()}")

if __name__ == "__main__":
    main()
