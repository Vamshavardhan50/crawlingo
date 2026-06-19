class CrawlingoError(Exception):
    """Base exception for Crawlingo."""
    pass

class FetchError(CrawlingoError):
    """Raised when request fetching fails."""
    pass

class ParseError(CrawlingoError):
    """Raised when parsing fails."""
    pass

class SelectorError(CrawlingoError):
    """Raised when selection query syntax or evaluation fails."""
    pass

class AutoMatchFailed(CrawlingoError):
    """Raised when DOM fingerprint auto matching fails."""
    pass

class TimeoutError(CrawlingoError):
    """Raised when an operation times out."""
    pass

class RateLimitError(CrawlingoError):
    """Raised when host request rate limit is hit."""
    pass

class ChangeDetectionError(CrawlingoError):
    """Raised when change detection fails."""
    pass

class ExportError(CrawlingoError):
    """Raised when exporting dataset to CSV/Parquet/JSON fails."""
    pass

class DnsError(CrawlingoError):
    """Raised when DNS resolution or caching fails."""
    pass

class FingerprintStoreError(CrawlingoError):
    """Raised when reading or writing to the fingerprint store fails."""
    pass

def handle_core_exception(e: Exception) -> Exception:
    """Maps standard exceptions (like RuntimeError from PyO3) to specific crawlingo exceptions."""
    if not isinstance(e, RuntimeError):
        return e
    
    msg = str(e)
    if "Fetch failed:" in msg or "HTTP client error:" in msg:
        return FetchError(msg)
    elif "Parse failed:" in msg:
        return ParseError(msg)
    elif "Selector failed:" in msg:
        return SelectorError(msg)
    elif "Auto-match failed" in msg:
        return AutoMatchFailed(msg)
    elif "Timeout after" in msg:
        return TimeoutError(msg)
    elif "Rate limit reached" in msg:
        return RateLimitError(msg)
    elif "Change detection failed:" in msg:
        return ChangeDetectionError(msg)
    elif "Export failed:" in msg or "CSV write error" in msg or "Parquet error" in msg or "Arrow error" in msg:
        return ExportError(msg)
    elif "DNS resolution failed:" in msg:
        return DnsError(msg)
    elif "Fingerprint store error:" in msg or "Sled DB error:" in msg:
        return FingerprintStoreError(msg)
    
    return CrawlingoError(msg)
