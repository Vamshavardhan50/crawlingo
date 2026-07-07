"""Crawlingo: Authentication helpers (Basic, Bearer, Header, API Key).

Usage:
    pip install crawlingo
    python 07_authentication.py
"""

from crawlingo import Session, Page
from crawlingo.auth import BearerAuth, BasicAuth, HeaderAuth


def main():
    print("=== Crawlingo Authentication Example ===\n")

    # 1. Bearer Token
    print("1. Bearer Token Auth")
    session = Session()
    session.headers(BearerAuth("your-api-token-here").headers())
    print(f"   Authorization: Bearer your-api-token-here\n")

    # 2. Basic Auth
    print("2. Basic Auth")
    session = Session()
    session.headers(BasicAuth("username", "password").headers())
    print(f"   Authorization: Basic base64(username:password)\n")

    # 3. Custom Header Auth
    print("3. Custom Header Auth")
    session = Session()
    session.headers(HeaderAuth("X-API-Key", "abc123").headers())
    print(f"   Headers: X-API-Key: abc123\n")

    # 4. Full example with auth + extraction
    print("4. Authenticated request example")
    session = Session()
    session.headers(BearerAuth("demo-token").headers())
    session.rate_limit(3.0)

    page = Page("https://httpbin.org/headers")
    print(f"   Status: {page.status}")


if __name__ == "__main__":
    main()
