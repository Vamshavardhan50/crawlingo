"""
Crawlingo request/extraction pipeline hooks.
Exposes common pre-built hook functions for value cleaning and request logging.
"""

def strip_whitespace(val: str) -> str:
    """Strips leading and trailing whitespace from extracted content."""
    return val.strip() if val else val

def uppercase(val: str) -> str:
    """Converts extracted content to uppercase."""
    return val.upper() if val else val

def lowercase(val: str) -> str:
    """Converts extracted content to lowercase."""
    return val.lower() if val else val

def log_request(req):
    """Simple logger hook that prints the target URL before fetching."""
    print(f"[crawlingo] Fetching: {req.url}")

def log_response(res):
    """Simple logger hook that prints the HTTP status code after fetching."""
    print(f"[crawlingo] Received Status: {res.status}")
