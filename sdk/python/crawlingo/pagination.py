from typing import Optional
from ._crawlingo_core import PaginationConfig as _CorePaginationConfig

class PaginationConfig:
    """
    Configuration for automatic pagination within a crawl.
    """
    def __init__(self, _core_config=None):
        if _core_config is None:
            raise ValueError("Use static/class constructors: next_link, page_number, or url_pattern.")
        self._core_config = _core_config

    @classmethod
    def next_link(cls, selector: str) -> "PaginationConfig":
        """Follow the href of the first element matching selector."""
        return cls(_CorePaginationConfig.next_link(selector))

    @classmethod
    def page_number(cls, url_template: str, start_page: int, max_pages: int) -> "PaginationConfig":
        """Construct numbered URLs from a template containing {page}."""
        return cls(_CorePaginationConfig.page_number(url_template, start_page, max_pages))

    @classmethod
    def url_pattern(cls, page_regex: str, max_page: int) -> "PaginationConfig":
        """Increment the page number captured by page_regex."""
        return cls(_CorePaginationConfig.url_pattern(page_regex, max_page))
