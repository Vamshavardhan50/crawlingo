"""Crawlingo: Basic page fetch with CSS and XPath selectors.

Usage:
    pip install crawlingo
    python 01_simple.py
"""

from crawlingo import Page


def main():
    print("=== Crawlingo Simple Extraction Example ===")

    url = "https://httpbin.org/html"
    print(f"Fetching {url}...")

    page = Page(url)

    print(f"\nResponse Status: {page.status}")
    print(f"Page Title: '{page.title()}'")

    print("\n--- CSS Selector (h1) ---")
    h1 = page.css("h1")
    print(f"Found {len(h1)} h1 elements. Text: '{h1.text()}'")

    print("\n--- XPath Selector (//p) ---")
    paragraphs = page.xpath("//p")
    for i, p in enumerate(paragraphs):
        print(f"Paragraph {i+1}: '{p.text()}'")

    print("\n--- Text Anchors ---")
    melville_el = page.find_text("Herman Melville")
    print(f"Found 'Herman Melville': {len(melville_el) > 0}")
    if melville_el:
        print(f"Outer HTML: {melville_el.first().html()}")

    print("\n--- Clean Markdown ---")
    print(page.markdown()[:300])


if __name__ == "__main__":
    main()
