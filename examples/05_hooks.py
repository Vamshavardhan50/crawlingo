import sys
import os

# Allow importing crawlingo from the local python source tree directly
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../sdk/python")))

from crawlingo import Page
from crawlingo.hooks import strip_whitespace, uppercase, log_request

def main():
    print("=== Crawlingo Hooks and Middlewares Example ===")
    
    url = "https://httpbin.org/html"
    
    # Configure page with lifecycle hooks
    page = (
        Page(url)
        .before_fetch(log_request)  # Built-in request log hook
        .before_parse(lambda html: html.replace("Melville", "Melville (PRE-PARSED)")) # Custom HTML override
        .after_extract(strip_whitespace) # Clean extracted text
        .after_extract(uppercase)        # Convert final outputs to uppercase
    )
    
    print("\nTriggering page load and extraction...")
    h1_text = page.css("h1").text()
    
    print(f"\nFinal Extracted (and hooks-processed) H1:")
    print(f"  '{h1_text}'")

if __name__ == "__main__":
    main()
