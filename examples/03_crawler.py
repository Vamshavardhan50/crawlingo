"""Crawlingo: Multi-page web crawler with field extraction.

Usage:
    pip install crawlingo
    python 03_crawler.py
"""

from crawlingo import Crawl


def main():
    print("=== Crawlingo Web Crawler Example ===")

    start_url = "https://httpbin.org/links/5/0"
    print(f"Starting crawl from {start_url}...")

    results = (
        Crawl(start_url)
        .follow("a")
        .limit(3)
        .depth(2)
        .concurrency(2)
        .delay(0.5)
        .field("title", "h1")
        .build()
    )

    print(f"\nCrawled {len(results)} pages:")
    for i, res in enumerate(results):
        print(f"  Page {i+1}: {res['url']} — title: '{res.get('title', 'N/A')}'")

    results.to_json("crawl_results.json")
    print("\nSaved crawl results to 'crawl_results.json'.")


if __name__ == "__main__":
    main()
