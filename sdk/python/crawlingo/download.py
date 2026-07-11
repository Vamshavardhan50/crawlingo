"""
File and binary download helpers for the Crawlingo Python SDK.

These are native python wrappers that call the Rust core Downloader directly.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Optional, Tuple

from ._crawlingo_core import Downloader as _CoreDownloader
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
    Downloads files via the Crawlingo session's HTTP stack.
    """

    def __init__(self, session: Optional[Session] = None):
        self._session = session or Session()
        self._core_downloader = _CoreDownloader(self._session._core_session)

    def chunk_size(self, size: int) -> "Downloader":
        """Set the write chunk size in bytes (default 64 KiB)."""
        self._core_downloader.chunk_size(size)
        return self

    def allow_resume(self, enabled: bool = True) -> "Downloader":
        """Enable or disable partial-content resumption (default enabled)."""
        self._core_downloader.allow_resume(enabled)
        return self

    def max_bytes(self, n: int) -> "Downloader":
        """Limit the download to the first ``n`` bytes."""
        self._core_downloader.max_bytes(n)
        return self

    def download(
        self, url: str, dest: str
    ) -> DownloadResult:
        """
        Download ``url`` to the local file at ``dest``.
        """
        res = self._core_downloader.download(url, dest)
        return DownloadResult(
            url=res.url,
            status=res.status,
            bytes_written=res.bytes_written,
            content_type=res.content_type,
            suggested_filename=res.suggested_filename,
            resumed=res.resumed,
        )

    def download_to_memory(self, url: str) -> Tuple[DownloadResult, bytes]:
        """
        Download ``url`` into memory and return ``(DownloadResult, bytes)``.
        """
        res, body = self._core_downloader.download_to_memory(url)
        return (
            DownloadResult(
                url=res.url,
                status=res.status,
                bytes_written=res.bytes_written,
                content_type=res.content_type,
                suggested_filename=res.suggested_filename,
                resumed=res.resumed,
            ),
            body,
        )
