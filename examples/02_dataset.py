"""Crawlingo: Structured data extraction with Dataset.

Usage:
    pip install crawlingo
    python 02_dataset.py
"""

from crawlingo import Dataset


def main():
    print("=== Crawlingo Structured Dataset Example ===")

    url = "https://httpbin.org/html"
    print(f"Building dataset query for {url}...")

    dataset = (
        Dataset(url)
        .auto_match(True)
        .field("title", "h1")
        .field("author", "p", selector_type="xpath")
        .field("content", "div")
        .build()
    )

    print("\nExtracted Fields:")
    result_dict = dataset.to_dict()
    for field, value in result_dict.items():
        print(f"  {field}: {value[:60]}...")

    print("\nExporting results...")
    dataset.to_json("dataset_result.json")
    dataset.to_csv("dataset_result.csv")
    print("Created 'dataset_result.json' and 'dataset_result.csv'.")


if __name__ == "__main__":
    main()
