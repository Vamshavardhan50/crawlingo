from typing import Optional, List
from ._crawlingo_core import Crawl as _CoreCrawl
from .session import Session
from .dataset import DatasetResult
from .exceptions import handle_core_exception

class CrawlResults:
    """
    Holds bulk dataset extraction results from a crawl.
    """
    def __init__(self, results: List[DatasetResult]):
        self._results = results

    def to_json(self, path: str):
        """Export all crawled pages to a JSON list file."""
        import json
        data = [res.to_dict() for res in self._results]
        with open(path, "w") as f:
            json.dump(data, f, indent=2)

    def to_csv(self, path: str):
        """Export all crawled pages to a CSV file."""
        import csv
        if not self._results:
            return
        keys = self._results[0].to_dict().keys()
        with open(path, "w", newline="", encoding="utf-8") as f:
            writer = csv.DictWriter(f, fieldnames=keys)
            writer.writeheader()
            for res in self._results:
                writer.writerow(res.to_dict())

    def to_parquet(self, path: str):
        """Export all crawled pages to a Parquet database file."""
        self.df().to_parquet(path, index=False)

    def df(self):
        """Convert results to a Pandas DataFrame."""
        import pandas as pd
        return pd.DataFrame([res.to_dict() for res in self._results])

    def __iter__(self) -> iter:
        return iter(self._results)

    def __len__(self) -> int:
        return len(self._results)

    def __getitem__(self, idx) -> DatasetResult:
        return self._results[idx]

    def __repr__(self) -> str:
        return f"CrawlResults(pages={len(self)})"


class Crawl:
    """
    Configuration and execution unit for multi-page spider crawls.
    """
    def __init__(self, start_url: str, session: Optional[Session] = None):
        self._session = session or Session()
        try:
            self._core_crawl = _CoreCrawl(start_url, self._session._core_session)
        except Exception as e:
            raise handle_core_exception(e)

    def follow(self, selector: str) -> "Crawl":
        """Set CSS selector pointing to links that crawler should queue and follow."""
        try:
            self._core_crawl.follow(selector)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def limit(self, pages: int) -> "Crawl":
        """Set maximum page count limit."""
        try:
            self._core_crawl.limit(pages)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def depth(self, max_depth: int) -> "Crawl":
        """Set maximum links hops/crawling depth level."""
        try:
            self._core_crawl.depth(max_depth)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def field(
        self,
        name: str,
        selector: str,
        selector_type: str = "css",
        default: Optional[str] = None,
    ) -> "Crawl":
        """Define an extraction field for every crawled page."""
        try:
            self._core_crawl.field(
                name,
                selector,
                selector_type=selector_type,
                default=default,
            )
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def auto_match(self, enabled: bool) -> "Crawl":
        """Enable auto-matching self healing selector recovery."""
        self._session.auto_match(enabled)
        return self

    def concurrency(self, n: int) -> "Crawl":
        """Set concurrent fetching worker thread counts."""
        try:
            self._core_crawl.concurrency(n)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def delay(self, seconds: float) -> "Crawl":
        """Set request pacing delays in seconds."""
        try:
            self._core_crawl.delay(seconds)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def webhook(self, url: str) -> "Crawl":
        """Set webhook endpoint URL to deliver JSON results in real-time."""
        try:
            self._core_crawl.webhook(url)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def schedule(self, interval_seconds: int):
        """Schedule crawling loop to execute periodically in the background."""
        try:
            self._core_crawl.schedule(interval_seconds)
        except Exception as e:
            raise handle_core_exception(e)

    def build(self) -> CrawlResults:
        """Run the crawler crawl loops synchronously."""
        try:
            core_results = self._core_crawl.build()
            results_list = [DatasetResult(res) for res in core_results]
            return CrawlResults(results_list)
        except Exception as e:
            raise handle_core_exception(e)
