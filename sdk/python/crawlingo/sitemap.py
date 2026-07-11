"""
Sitemap crawling helpers for the Crawlingo Python SDK.

These are **pure-Python** helpers that use the core Crawl/Session APIs to implement
sitemap fetching and URL seeding — no additional Rust bindings are needed.

Usage::

    from crawlingo import Session
    from crawlingo.sitemap import Sitemap

    session = Session()
    results = Sitemap("https://example.com/sitemap.xml", session=session).build()
    for r in results:
        print(r.url, r.fields)
"""
from __future__ import annotations

import xml.etree.ElementTree as ET
from typing import List, Optional, Dict
from urllib.parse import urljoin, urlparse

from .session import Session
from .crawl import Crawl, CrawlResults
from .dataset import DatasetResult


# Sitemap XML namespaces.
_NS_SITEMAP = "http://www.sitemaps.org/schemas/sitemap/0.9"
_NSMAP = {"sm": _NS_SITEMAP}


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


def parse_sitemap_xml(xml_text: str) -> tuple[list[SitemapEntry], list[str]]:
    """
    Parse a sitemap XML string.

    Returns a ``(url_entries, child_sitemap_urls)`` pair:
    - ``url_entries``: list of :class:`SitemapEntry` from a ``<urlset>``
    - ``child_sitemap_urls``: list of ``<loc>`` strings from a ``<sitemapindex>``

    Exactly one of the two lists will be non-empty (or both empty for unknown docs).
    """
    try:
        root = ET.fromstring(xml_text)
    except ET.ParseError:
        return [], []

    tag = root.tag
    local = tag.split("}")[-1] if "}" in tag else tag

    url_entries: list[SitemapEntry] = []
    child_sitemaps: list[str] = []

    if local == "urlset":
        for url_el in root.iter():
            if url_el.tag.split("}")[-1] != "url":
                continue
            loc = ""
            lastmod = changefreq = priority = None
            for child in url_el:
                child_local = child.tag.split("}")[-1]
                text = (child.text or "").strip()
                if child_local == "loc":
                    loc = text
                elif child_local == "lastmod":
                    lastmod = text
                elif child_local == "changefreq":
                    changefreq = text
                elif child_local == "priority":
                    priority = text
            if loc:
                url_entries.append(SitemapEntry(loc, lastmod, changefreq, priority))
    elif local == "sitemapindex":
        for sm_el in root.iter():
            if sm_el.tag.split("}")[-1] != "sitemap":
                continue
            for child in sm_el:
                if child.tag.split("}")[-1] == "loc":
                    loc = (child.text or "").strip()
                    if loc:
                        child_sitemaps.append(loc)
                    break

    return url_entries, child_sitemaps


def sitemap_url_for_origin(origin: str) -> str:
    """Return the canonical ``/sitemap.xml`` URL for a given origin.

    Example::

        sitemap_url_for_origin("https://example.com")
        # → "https://example.com/sitemap.xml"
    """
    return origin.rstrip("/") + "/sitemap.xml"


class Sitemap:
    """
    Fetch and parse a sitemap (or sitemap index), optionally running a downstream
    :class:`~crawlingo.Crawl` against all discovered URLs.

    Sitemap index files are resolved recursively up to ``max_depth`` levels.

    Example::

        session = Session()
        results = (
            Sitemap("https://example.com/sitemap.xml", session=session)
            .follow("a.article-link")
            .field("title", "h1")
            .limit(50)
            .build()
        )
    """

    def __init__(self, sitemap_url: str, *, session: Optional[Session] = None):
        self._sitemap_url = sitemap_url
        self._session = session or Session()
        self._max_depth = 5
        # Downstream Crawl config overrides
        self._follow: Optional[str] = None
        self._limit: Optional[int] = None
        self._depth: Optional[int] = None
        self._concurrency: Optional[int] = None
        self._delay: Optional[float] = None
        self._fields: list[dict] = []
        self._webhook: Optional[str] = None

    def max_depth(self, depth: int) -> "Sitemap":
        """Maximum nesting depth for sitemap-index resolution (default 5)."""
        self._max_depth = depth
        return self

    def follow(self, selector: str) -> "Sitemap":
        """CSS selector for follow-links on each discovered page."""
        self._follow = selector
        return self

    def limit(self, pages: int) -> "Sitemap":
        """Maximum number of pages to crawl."""
        self._limit = pages
        return self

    def depth(self, max_depth: int) -> "Sitemap":
        """Maximum crawl depth from each seed URL."""
        self._depth = max_depth
        return self

    def concurrency(self, n: int) -> "Sitemap":
        """Number of concurrent crawl workers."""
        self._concurrency = n
        return self

    def delay(self, seconds: float) -> "Sitemap":
        """Politeness delay between requests in seconds."""
        self._delay = seconds
        return self

    def field(self, name: str, selector: str, selector_type: str = "css", default: Optional[str] = None) -> "Sitemap":
        """Add a field extraction definition applied to each crawled page."""
        self._fields.append({"name": name, "selector": selector, "selector_type": selector_type, "default": default})
        return self

    def webhook(self, url: str) -> "Sitemap":
        """Webhook endpoint for real-time result delivery."""
        self._webhook = url
        return self

    def list_urls(self) -> List[SitemapEntry]:
        """
        Fetch the sitemap and return all discovered :class:`SitemapEntry` objects
        without running any downstream crawl.
        """
        from .page import Page as _Page
        seen: set[str] = set()
        entries: list[SitemapEntry] = []
        self._collect(self._sitemap_url, 0, seen, entries)
        return entries

    def build(self) -> CrawlResults:
        """
        Fetch the sitemap, seed all discovered URLs into a crawl, run it, and return
        the aggregated :class:`~crawlingo.crawl.CrawlResults`.
        """
        seed_entries = self.list_urls()
        if not seed_entries:
            return CrawlResults([])

        # Use the first URL as the "start_url" for the Crawl object, but seed all
        # discovered URLs by building a Crawl per URL. For large sitemaps this is
        # inefficient — a future improvement would expose SitemapCrawler::fetch from
        # the Rust FFI layer so we can seed the frontier directly.
        # For now we run one Crawl seeded with the first URL + follow=None and collect
        # all URLs by crawling with concurrency.
        from .page import Page as _Page
        import itertools

        # Build the Crawl from the first seed URL, then we collect results per URL.
        # Real implementation: batch fetch all sitemap URLs directly via the session.
        all_results: list[DatasetResult] = []

        # Chunk into batches to avoid creating thousands of Crawl objects.
        # Real-world use: each sitemap URL is fetched as depth-0 page.
        for entry in seed_entries:
            crawl = Crawl(entry.loc, session=self._session)
            if self._follow:
                crawl.follow(self._follow)
            crawl.limit(1)  # Each URL is one page by default
            if self._depth is not None:
                crawl.depth(self._depth)
            if self._concurrency is not None:
                crawl.concurrency(self._concurrency)
            if self._delay is not None:
                crawl.delay(self._delay)
            for f in self._fields:
                crawl.field(f["name"], f["selector"], f["selector_type"], f.get("default"))
            if self._webhook:
                crawl.webhook(self._webhook)
            try:
                result = crawl.build()
                all_results.extend(list(result))
            except Exception:
                pass  # Skip individual fetch failures (e.g. 404s, timeouts)

        return CrawlResults(all_results)

    # ------------------------------------------------------------------ private

    def _fetch_xml(self, url: str) -> str:
        """Fetch a URL and return the response body as a string."""
        from .page import Page as _Page
        # Use PyPage to fetch – it returns HTML, which for XML sitemaps is the raw XML.
        try:
            page = _Page(url, session=self._session)
            return page.html()
        except Exception:
            return ""

    def _collect(
        self,
        url: str,
        depth: int,
        seen: set[str],
        entries: list[SitemapEntry],
    ) -> None:
        if depth > self._max_depth or url in seen:
            return
        seen.add(url)
        xml_text = self._fetch_xml(url)
        if not xml_text:
            return
        url_entries, child_sitemaps = parse_sitemap_xml(xml_text)
        entries.extend(url_entries)
        for child_url in child_sitemaps:
            self._collect(child_url, depth + 1, seen, entries)
