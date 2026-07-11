"""
Sitemap crawling helpers for the Crawlingo Python SDK.

These are native python wrappers that call the Rust core SitemapCrawler.
"""
from __future__ import annotations

from typing import List, Optional

from ._crawlingo_core import Sitemap as _CoreSitemap, sitemap_url_for_origin as _core_sitemap_url_for_origin
from .session import Session
from .crawl import CrawlResults
from .dataset import DatasetResult


class SitemapEntry:
    """A single `<url>` entry from a leaf sitemap."""

    __slots__ = ("loc", "lastmod", "changefreq", "priority")

    def __init__(
        self,
        loc: str,
        lastmod: Optional[str] = None,
        changefreq: Optional[str] = None,
        priority: Optional[str] = None,
    ):
        self.loc = loc
        self.lastmod = lastmod
        self.changefreq = changefreq
        self.priority = priority

    def __repr__(self) -> str:
        return f"SitemapEntry(loc={self.loc!r})"


def sitemap_url_for_origin(origin: str) -> str:
    """Return the canonical ``/sitemap.xml`` URL for a given origin."""
    return _core_sitemap_url_for_origin(origin)


class Sitemap:
    """
    Fetch and parse a sitemap (or sitemap index), optionally running a downstream
    crawl against all discovered URLs.
    """

    def __init__(self, sitemap_url: str, *, session: Optional[Session] = None):
        self._session = session or Session()
        self._core_sitemap = _CoreSitemap(sitemap_url, self._session._core_session)

    def max_depth(self, depth: int) -> "Sitemap":
        """Maximum nesting depth for sitemap-index resolution."""
        self._core_sitemap.max_depth(depth)
        return self

    def follow(self, selector: str) -> "Sitemap":
        """CSS selector for follow-links on each discovered page."""
        self._core_sitemap.follow(selector)
        return self

    def limit(self, pages: int) -> "Sitemap":
        """Maximum number of pages to crawl."""
        self._core_sitemap.limit(pages)
        return self

    def depth(self, max_depth: int) -> "Sitemap":
        """Maximum crawl depth from each seed URL."""
        self._core_sitemap.depth(max_depth)
        return self

    def concurrency(self, n: int) -> "Sitemap":
        """Number of concurrent crawl workers."""
        self._core_sitemap.concurrency(n)
        return self

    def delay(self, seconds: float) -> "Sitemap":
        """Politeness delay between requests in seconds."""
        self._core_sitemap.delay(seconds)
        return self

    def field(self, name: str, selector: str, selector_type: str = "css", default: Optional[str] = None) -> "Sitemap":
        """Add a field extraction definition applied to each crawled page."""
        self._core_sitemap.field(name, selector, selector_type, default)
        return self

    def webhook(self, url: str) -> "Sitemap":
        """Webhook endpoint for real-time result delivery."""
        self._core_sitemap.webhook(url)
        return self

    def list_urls(self) -> List[SitemapEntry]:
        """Fetch the sitemap and return all discovered SitemapEntry objects."""
        core_entries = self._core_sitemap.list_urls()
        return [
            SitemapEntry(e.loc, e.lastmod, e.changefreq, e.priority)
            for e in core_entries
        ]

    def build(self) -> CrawlResults:
        """Fetch the sitemap, crawl, and return results."""
        core_results = self._core_sitemap.build()
        results_list = [DatasetResult(res) for res in core_results]
        return CrawlResults(results_list)
