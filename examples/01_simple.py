import sys
import os

# Allow importing crawlingo from the local python source tree directly
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../sdk/python")))

from crawlingo import Page

def main():
    print("=== Crawlingo Simple Extraction Example ===")
    
    # Fetch a simple HTML page
    url = "https://httpbin.org/html"
    print(f"Fetching {url}...")
    page = Page(url)
    
    print(f"\nResponse Status: {page.status}")
    print(f"Page Title: '{page.title()}'")
    
    # Extract using CSS
    print("\n--- CSS Selector (h1) ---")
    h1 = page.css("h1")
    print(f"Found {len(h1)} h1 elements. Text: '{h1.text()}'")
    
    # Extract using XPath
    print("\n--- XPath Selector (//p) ---")
    paragraphs = page.xpath("//p")
    for i, p in enumerate(paragraphs):
        print(f"Paragraph {i+1}: '{p.text()}'")
        
    # Extract using Text Anchors
    print("\n--- Text Anchors ---")
    # Finding text near "Herman Melville" (part of the default httpbin html content)
    melville_el = page.find_text("Herman Melville")
    print(f"Found 'Herman Melville': {len(melville_el) > 0}")
    if melville_el:
        print(f"Outer HTML: {melville_el.first().html()}")

if __name__ == "__main__":
    main()
