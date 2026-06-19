from typing import Callable, Optional
from ._crawlingo_core import Dataset as _CoreDataset, DatasetResult as _CoreDatasetResult
from .session import Session
from .exceptions import handle_core_exception

class Dataset:
    """
    Structured field data extraction builder.
    """
    def __init__(self, url: str, session: Optional[Session] = None):
        self._session = session or Session()
        try:
            self._core_dataset = _CoreDataset(url, self._session._core_session)
        except Exception as e:
            raise handle_core_exception(e)

    def field(
        self,
        name: str,
        selector: str,
        selector_type: str = "css",
        transform: Optional[Callable[[str], str]] = None,
        default: Optional[str] = None,
    ) -> "Dataset":
        """Define an extraction field mapping."""
        try:
            self._core_dataset.field(
                name,
                selector,
                selector_type=selector_type,
                transform=transform,
                default=default,
            )
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def auto_match(self, enabled: bool) -> "Dataset":
        """Enable or disable auto-selector self-healing."""
        self._session.auto_match(enabled)
        return self

    def timeout(self, seconds: int) -> "Dataset":
        """Set connection timeout."""
        self._session.timeout(seconds)
        return self

    def headers(self, headers: dict) -> "Dataset":
        """Set request headers."""
        self._session.headers(headers)
        return self

    def build(self) -> "DatasetResult":
        """Build and run the dataset query synchronously."""
        try:
            res = self._core_dataset.build()
            return DatasetResult(res)
        except Exception as e:
            raise handle_core_exception(e)

    async def build_async(self) -> "DatasetResult":
        """Build and run the dataset query asynchronously."""
        try:
            res = self._core_dataset.build_async()
            return DatasetResult(res)
        except Exception as e:
            raise handle_core_exception(e)


class DatasetResult:
    """
    Holds and exports structured dataset results.
    """
    def __init__(self, core_result: _CoreDatasetResult):
        self._core = core_result

    def to_json(self, path: str):
        """Export fields to a formatted JSON file."""
        try:
            self._core.to_json(path)
        except Exception as e:
            raise handle_core_exception(e)

    def to_csv(self, path: str):
        """Export fields to a CSV file."""
        try:
            self._core.to_csv(path)
        except Exception as e:
            raise handle_core_exception(e)

    def to_parquet(self, path: str):
        """Export fields to a Parquet file."""
        try:
            self._core.to_parquet(path)
        except Exception as e:
            raise handle_core_exception(e)

    def to_dict(self) -> dict:
        """Return results as a standard dictionary."""
        try:
            return self._core.to_dict()
        except Exception as e:
            raise handle_core_exception(e)

    def df(self):
        """Return results loaded into a Pandas DataFrame."""
        import pandas as pd
        return pd.DataFrame([self.to_dict()])

    def __getitem__(self, key: str) -> str:
        return self._core[key]

    def __repr__(self) -> str:
        return repr(self._core)
