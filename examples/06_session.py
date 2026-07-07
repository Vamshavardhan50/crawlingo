"""Crawlingo: Session configuration with headers, proxy, and rate limits.

Usage:
    pip install crawlingo
    python 06_session.py
"""

from crawlingo import Session, Page, Dataset


def main():
    print("=== Crawlingo Session Configuration Example ===")

    with Session() as session:
        session.headers({
            "User-Agent": "CrawlingoExample/1.0",
            "X-My-Header": "MyValue",
        })
        session.timeout(15)
        session.auto_match(True)

        print("Fetching page with session...")
        page = Page("https://httpbin.org/headers")
        print("Response headers (partial):")
        print(page.html()[:300])

        print("\nExtracting data through session...")
        dataset = (
            Dataset("https://httpbin.org/html", session=session)
            .field("title", "h1")
            .build()
        )
        print(f"Dataset result: {dataset.to_dict()}")


if __name__ == "__main__":
    main()
