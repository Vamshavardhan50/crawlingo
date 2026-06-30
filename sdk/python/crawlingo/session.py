from typing import TYPE_CHECKING
from ._crawlingo_core import Session as _CoreSession

if TYPE_CHECKING:
    from .page import Page
    from .dataset import Dataset
    from .crawl import Crawl
    from .watch import Watch

class Session:
    """
    Context manager representing a shared scraping session (shares cookies, proxies, headers).
    """
    def __init__(self):
        self._core_session = _CoreSession()
        self._headers = {}
        self._cookies = {}
        self._proxy = None
        self._rate_limit_rps = 0.0
        self._auto_match = False
        self._timeout = 30
        self._fingerprint_path = ".crawlingo"
        self._fetcher_tier = "standard"
        self._browser_profile = None
        self._auto_match_weights = {}
        self._proxy_pool = []
        self._proxy_provider = None

    def headers(self, headers: dict) -> "Session":
        """Set default request headers for the session."""
        self._headers = headers.copy()
        self._core_session.headers(headers)
        return self

    def cookies(self, cookies: dict) -> "Session":
        """Set default cookies for the session."""
        self._cookies = cookies.copy()
        self._core_session.cookies(cookies)
        return self

    def proxy(self, proxy_url: str) -> "Session":
        """Configure session proxy server."""
        self._proxy = proxy_url
        self._core_session.proxy(proxy_url)
        return self

    def rate_limit(self, requests_per_second: float) -> "Session":
        """Configure per-host rate limiter."""
        self._rate_limit_rps = requests_per_second
        self._core_session.rate_limit(requests_per_second)
        return self

    def auto_match(self, enabled: bool) -> "Session":
        """Enable auto-matching selector self-healing globally."""
        self._auto_match = enabled
        self._core_session.auto_match(enabled)
        return self

    def timeout(self, seconds: int) -> "Session":
        """Set request timeouts."""
        self._timeout = seconds
        self._core_session.timeout(seconds)
        return self

    def fingerprint_path(self, path: str) -> "Session":
        """Set fingerprint storage folder path."""
        self._fingerprint_path = path
        self._core_session.fingerprint_path(path)
        return self

    def fetcher_tier(self, tier: str) -> "Session":
        """Set fetcher mode: 'standard' or 'stealthy'."""
        self._fetcher_tier = tier
        self._core_session.fetcher_tier(tier)
        return self

    def browser_profile(self, profile: str) -> "Session":
        """Set browser impersonation profile: 'chrome', 'firefox', 'safari'."""
        self._browser_profile = profile
        self._core_session.browser_profile(profile)
        return self

    def auto_match_weights(self, weights: dict) -> "Session":
        """Set similarity scoring weights (e.g. {"text": 2.0, "class": 1.0})."""
        self._auto_match_weights = weights.copy()
        self._core_session.auto_match_weights(weights)
        return self

    def proxy_pool(self, proxies: list) -> "Session":
        """Set a list of rotating proxy URLs."""
        self._proxy_pool = proxies.copy()
        self._core_session.proxy_pool(proxies)
        return self

    def proxy_provider(self, url: str) -> "Session":
        """Set a proxy list provider API endpoint URL."""
        self._proxy_provider = url
        self._core_session.proxy_provider(url)
        return self

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

    def __enter__(self) -> "Session":
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        pass
