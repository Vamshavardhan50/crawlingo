"""Crawlingo: Price extraction with normalization and change detection.

Usage:
    pip install crawlingo
    python 08_price_extraction.py
"""

from crawlingo import Page, Dataset
from crawlingo.change import detect_changes


def main():
    print("=== Crawlingo Price Extraction Example ===\n")

    # 1. Extract price with type normalization
    print("1. Extracting price from page")
    page = Page("https://httpbin.org/html")

    # Use Dataset with price extraction type
    dataset = (
        Dataset("https://httpbin.org/html")
        .field("title", "h1")
        .field("content", "p", selector_type="xpath")
        .build()
    )

    print(f"   Result: {dataset.to_dict()}\n")

    # 2. Simulate change detection
    print("2. Change detection demo")
    old_data = {
        "title": "Premium Wireless Headphone",
        "price": "299.99",
    }
    new_data = {
        "title": "Premium Wireless Headphone",
        "price": "249.99",
    }

    changes = detect_changes("https://shop.example.com/item/1", old_data, new_data)
    for change in changes:
        print(f"   Field '{change.field}' changed:")
        print(f"     {change.old_value} → {change.new_value}")
        print(f"     Type: {change.change_type}")


if __name__ == "__main__":
    main()
