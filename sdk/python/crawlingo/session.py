from typing import TYPE_CHECKING, Optional
from ._crawlingo_core import Session as _CoreSession

if TYPE_CHECKING:
    from .page import Page
    from .dataset import Dataset
    from .crawl import Crawl
    from .watch import Watch

class Session:
    """
    Context manager representing a shared scraping session (shares cookies, proxies, headers).
    All configuration state is managed strictly by the underlying Rust engine.
    """
    def __init__(self):
        self._core_session = _CoreSession()

    def headers(self, headers: dict) -> "Session":
        """Set default request headers for the session."""
        self._core_session.headers(headers)
        return self

    def cookies(self, cookies: dict) -> "Session":
        """Set default cookies for the session."""
        self._core_session.cookies(cookies)
        return self

    def proxy(self, proxy_url: str) -> "Session":
        """Configure session proxy server."""
        self._core_session.proxy(proxy_url)
        return self

    def rate_limit(self, requests_per_second: float) -> "Session":
        """Configure per-host rate limiter."""
        self._core_session.rate_limit(requests_per_second)
        return self

    def auto_match(self, enabled: bool) -> "Session":
        """Enable auto-matching selector self-healing globally."""
        self._core_session.auto_match(enabled)
        return self

    def timeout(self, seconds: int) -> "Session":
        """Set request timeouts."""
        self._core_session.timeout(seconds)
        return self

    def fingerprint_path(self, path: str) -> "Session":
        """Set fingerprint storage folder path."""
        self._core_session.fingerprint_path(path)
        return self

    def fetcher_tier(self, tier: str) -> "Session":
        """Set fetcher mode: 'standard', 'stealthy', or 'browser'."""
        self._core_session.fetcher_tier(tier)
        return self

    def browser_profile(self, profile: str) -> "Session":
        """Set browser impersonation profile: 'chrome', 'firefox', 'safari'."""
        self._core_session.browser_profile(profile)
        return self

    def auto_match_weights(self, weights: dict) -> "Session":
        """Set similarity scoring weights (e.g. {"text": 2.0, "class": 1.0})."""
        self._core_session.auto_match_weights(weights)
        return self

    def proxy_pool(self, proxies: list) -> "Session":
        """Set a list of rotating proxy URLs."""
        self._core_session.proxy_pool(proxies)
        return self

    def proxy_provider(self, url: str) -> "Session":
        """Set a proxy list provider API endpoint URL."""
        self._core_session.proxy_provider(url)
        return self

    def enable_response_cache(self, max_entries: int = 1024, default_ttl_seconds: int = 300) -> "Session":
        """Enable in-memory HTTP response caching (honors `Cache-Control`/`ETag`/`Last-Modified`).
        Must be called before the first fetch made through this session.
        """
        self._core_session.enable_response_cache(max_entries, default_ttl_seconds)
        return self

    def basic_auth(self, username: str, password: str) -> "Session":
        """Authenticate every fetch with HTTP Basic auth. Must be called before the first fetch."""
        self._core_session.basic_auth(username, password)
        return self

    def bearer_auth(self, token: str) -> "Session":
        """Authenticate every fetch with a fixed `Authorization: Bearer <token>` header. Must be
        called before the first fetch."""
        self._core_session.bearer_auth(token)
        return self

    def header_auth(self, name: str, value: str) -> "Session":
        """Authenticate every fetch with a fixed custom header (e.g. an API key header). Must be
        called before the first fetch."""
        self._core_session.header_auth(name, value)
        return self

    def api_key_auth(self, name: str, value: str) -> "Session":
        """Authenticate every fetch by appending an API key query parameter to the request URL.
        Must be called before the first fetch."""
        self._core_session.api_key_auth(name, value)
        return self

    @classmethod
    def from_config(cls, path: Optional[str] = None) -> "Session":
        """Build a Session from a config file (`.toml`/`.json`), layered with `CRAWLINGO_*`
        environment variable overrides. Pass no path to load from defaults + environment only.
        """
        session = cls.__new__(cls)
        session._core_session = _CoreSession.from_config(path)
        return session

    def metrics(self) -> dict:
        """Return a snapshot of this session's aggregated fetch metrics."""
        return self._core_session.metrics()

    def page(self, url: str) -> "Page":
        """Create a new lazy Page attached to this session."""
        from .page import Page
        return Page(url, session=self)

    def dataset(self, url: str) -> "Dataset":
        """Create a new Dataset builder attached to this session."""
        from .dataset import Dataset
        return Dataset(url, session=self)

    def watch(self, url: str) -> "Watch":
        """Create a new Watch poller attached to this session."""
        from .watch import Watch
        return Watch(url, session=self)

    def crawl(self, url: str) -> "Crawl":
        """Create a new Crawl spider crawler attached to this session."""
        from .crawl import Crawl
        return Crawl(url, session=self)

    def clone(self) -> "Session":
        """Clone this session and all its configurations into an independent Session."""
        cloned = Session.__new__(Session)
        cloned._core_session = self._core_session.clone()
        return cloned

    def destroy(self) -> None:
        """Destroy the session and release its resources."""
        self._core_session.destroy()

    def __enter__(self) -> "Session":
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        pass
