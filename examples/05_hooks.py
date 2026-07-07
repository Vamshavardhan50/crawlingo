"""Crawlingo: Lifecycle hooks and middleware.

Usage:
    pip install crawlingo
    python 05_hooks.py
"""

from crawlingo import Page
from crawlingo.hooks import strip_whitespace, uppercase, log_request


def main():
    print("=== Crawlingo Hooks and Middleware Example ===")

    url = "https://httpbin.org/html"

    page = (
        Page(url)
        .before_fetch(log_request)
        .before_parse(lambda html: html.replace("Melville", "Melville (PRE-PARSED)"))
        .after_extract(strip_whitespace)
        .after_extract(uppercase)
    )

    print("\nTriggering page load and extraction...")
    h1_text = page.css("h1").text()

    print(f"\nFinal Extracted (and hooks-processed) H1:")
    print(f"  '{h1_text}'")


if __name__ == "__main__":
    main()
