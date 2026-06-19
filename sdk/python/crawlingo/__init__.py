from .page import Page
from .element import ElementCollection
from .dataset import Dataset, DatasetResult
from .crawl import Crawl
from .watch import Watch
from .session import Session
from .types import ChangeEvent
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

__version__ = "0.1.0"
