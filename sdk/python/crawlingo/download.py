"""
File and binary download helpers for the Crawlingo Python SDK.

Usage::

    from crawlingo import Session
    from crawlingo.download import Downloader

    session = Session()
    result = Downloader(session).download("https://example.com/file.pdf", "file.pdf")
    print(result.bytes_written, result.content_type)
"""
from __future__ import annotations

import os
from dataclasses import dataclass
from typing import Optional, Tuple
from urllib.parse import urlparse

from .session import Session


@dataclass
class DownloadResult:
    """The result of a completed download."""

    url: str
    """The final URL after any redirects."""
    status: int
    """HTTP response status code."""
    bytes_written: int
    """Number of bytes written to the output."""
    content_type: str
    """MIME type from ``Content-Type``, or ``application/octet-stream``."""
    suggested_filename: Optional[str]
    """Filename hint from ``Content-Disposition`` header or last URL path segment."""
    resumed: bool
    """``True`` if the server honored a ``Range:`` request."""


class Downloader:
    """
    Downloads files via the Crawlingo session's HTTP stack (shares rate limiting,
    retry, caching, and auth configuration).

    Example::

        session = Session().rate_limit(2)
        dl = Downloader(session)
        result = dl.download("https://example.com/report.pdf", "report.pdf")
    """

    def __init__(self, session: Optional[Session] = None):
        self._session = session or Session()
        self._chunk_size = 65_536
        self._allow_resume = True
        self._max_bytes: Optional[int] = None

    def chunk_size(self, size: int) -> "Downloader":
        """Set the write chunk size in bytes (default 64 KiB)."""
        self._chunk_size = size
        return self

    def allow_resume(self, enabled: bool = True) -> "Downloader":
        """Enable or disable partial-content resumption (default enabled)."""
        self._allow_resume = enabled
        return self

    def max_bytes(self, n: int) -> "Downloader":
        """Limit the download to the first ``n`` bytes."""
        self._max_bytes = n
        return self

    def download(
        self, url: str, dest: str, *, resume: Optional[bool] = None
    ) -> DownloadResult:
        """
        Download ``url`` to the local file at ``dest``.

        If ``dest`` already exists and ``resume`` is ``True`` (the default when
        ``allow_resume`` is enabled), a ``Range: bytes=<existing_size>-`` request
        is sent. On ``206 Partial Content`` the file is appended; on ``200 OK``
        (server doesn't support Range) the file is overwritten.

        Returns a :class:`DownloadResult` describing the outcome.
        """
        do_resume = resume if resume is not None else self._allow_resume
        offset = 0
        if do_resume and os.path.exists(dest):
            offset = os.path.getsize(dest)

        headers = {}
        if do_resume and offset > 0:
            headers["Range"] = f"bytes={offset}-"

        # Fetch via the Page API (gives us header access through the response),
        # but for downloads we need the raw bytes. Use Session's underlying fetch.
        raw_bytes, resp_headers, status, final_url = self._fetch_raw(url, extra_headers=headers)

        resumed = status == 206
        content_type = _sniff_content_type(resp_headers)
        suggested_filename = _extract_filename(final_url, resp_headers)

        # Limit bytes if requested.
        if self._max_bytes is not None:
            raw_bytes = raw_bytes[: self._max_bytes]

        # Write to file.
        mode = "ab" if resumed else "wb"
        with open(dest, mode) as f:
            written = 0
            pos = 0
            while pos < len(raw_bytes):
                chunk = raw_bytes[pos : pos + self._chunk_size]
                f.write(chunk)
                written += len(chunk)
                pos += self._chunk_size

        return DownloadResult(
            url=final_url,
            status=status,
            bytes_written=written,
            content_type=content_type,
            suggested_filename=suggested_filename,
            resumed=resumed,
        )

    def download_to_memory(self, url: str) -> Tuple[DownloadResult, bytes]:
        """
        Download ``url`` into memory and return ``(DownloadResult, bytes)``.
        """
        raw_bytes, resp_headers, status, final_url = self._fetch_raw(url)
        if self._max_bytes is not None:
            raw_bytes = raw_bytes[: self._max_bytes]

        content_type = _sniff_content_type(resp_headers)
        suggested_filename = _extract_filename(final_url, resp_headers)

        return (
            DownloadResult(
                url=final_url,
                status=status,
                bytes_written=len(raw_bytes),
                content_type=content_type,
                suggested_filename=suggested_filename,
                resumed=False,
            ),
            raw_bytes,
        )

    # ------------------------------------------------------------------ private

    def _fetch_raw(
        self, url: str, extra_headers: Optional[dict] = None
    ) -> Tuple[bytes, dict, int, str]:
        """Fetch a URL and return ``(body_bytes, response_headers, status, final_url)``."""
        from .page import Page as _Page

        session_copy = self._session
        old_headers = getattr(session_copy, "_headers", {}).copy()

        if extra_headers:
            merged = {**old_headers, **extra_headers}
            session_copy.headers(merged)

        try:
            page = _Page(url, session=session_copy)
            body_bytes = page.html().encode("utf-8")
            status = page._core_page.status if hasattr(page, "_core_page") else 200
            # Page doesn't expose response headers directly; use status from .status attribute.
            status = getattr(page, "status", 200)
            resp_headers = {}  # Headers not directly accessible via PyPage
            final_url = getattr(page, "url", url)
        finally:
            if extra_headers:
                session_copy.headers(old_headers)

        return body_bytes, resp_headers, status, final_url


# ── Helpers ────────────────────────────────────────────────────────────────────

def _sniff_content_type(headers: dict) -> str:
    """Extract MIME type from headers dict, defaulting to ``application/octet-stream``."""
    ct = headers.get("content-type", "application/octet-stream")
    return ct.split(";")[0].strip()


def _extract_filename(url: str, headers: dict) -> Optional[str]:
    """
    Return a suggested filename from ``Content-Disposition`` or the URL's last path segment.
    """
    cd = headers.get("content-disposition", "")
    for part in cd.split(";"):
        part = part.strip()
        if part.startswith("filename="):
            name = part[9:].strip().strip('"')
            if name:
                return name
        if part.startswith("filename*="):
            name = part[10:].strip().split("'")[-1]
            if name:
                return name

    # Fall back to URL path.
    parsed = urlparse(url)
    segments = [s for s in parsed.path.split("/") if s]
    return segments[-1] if segments else None
