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
    msg_lower = msg.lower()

    if any(k in msg_lower for k in ("fetch failed", "http client error", "connection refused", "connection reset", "dns error", "ssl error", "unreachable", "proxy connect")):
        return FetchError(msg)
    elif any(k in msg_lower for k in ("parse failed", "html parse", "lol_html", "invalid html", "encoding error")):
        return ParseError(msg)
    elif any(k in msg_lower for k in ("selector failed", "invalid selector", "selector error", "css parse", "xpath parse")):
        return SelectorError(msg)
    elif any(k in msg_lower for k in ("auto-match failed", "auto_match failed", "fingerprint not found", "auto match failed")):
        return AutoMatchFailed(msg)
    elif any(k in msg_lower for k in ("timeout", "timed out")):
        return TimeoutError(msg)
    elif any(k in msg_lower for k in ("rate limit", "rate_limit", "too many requests")):
        return RateLimitError(msg)
    elif any(k in msg_lower for k in ("change detection failed", "detect_changes")):
        return ChangeDetectionError(msg)
    elif any(k in msg_lower for k in ("export failed", "csv write error", "parquet error", "arrow error", "csv error", "io error")):
        return ExportError(msg)
    elif any(k in msg_lower for k in ("dns resolution failed", "dns lookup", "dns error")):
        return DnsError(msg)
    elif any(k in msg_lower for k in ("fingerprint store error", "sled db error", "fingerprint")):
        return FingerprintStoreError(msg)

    return CrawlingoError(msg)
