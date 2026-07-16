from .page import Page
from .element import ElementCollection
from .dataset import Dataset, DatasetResult
from .crawl import Crawl
from .watch import Watch
from .session import Session
from .types import ChangeEvent
from .sitemap import Sitemap, SitemapEntry, sitemap_url_for_origin
from .download import Downloader, DownloadResult
from .pagination import PaginationConfig
from .schema import DatasetSchema, FieldType, FieldConstraint
from . import hooks
from .exceptions import (
    CrawlingoError,
    FetchError,
    ParseError,
    SelectorError,
    AutoMatchFailed,
    TimeoutError,
    RateLimitError,
    ChangeDetectionError,
    ExportError,
    DnsError,
    FingerprintStoreError,
)

__all__ = [
    "Page",
    "ElementCollection",
    "Dataset",
    "DatasetResult",
    "Crawl",
    "Watch",
    "Session",
    "ChangeEvent",
    "Sitemap",
    "SitemapEntry",
    "PaginationConfig",
    "DatasetSchema",
    "FieldType",
    "FieldConstraint",
    "sitemap_url_for_origin",
    "Downloader",
    "DownloadResult",
    "hooks",
    "CrawlingoError",
    "FetchError",
    "ParseError",
    "SelectorError",
    "AutoMatchFailed",
    "TimeoutError",
    "RateLimitError",
    "ChangeDetectionError",
    "ExportError",
    "DnsError",
    "FingerprintStoreError",
]

__version__ = "1.0.0a1"

