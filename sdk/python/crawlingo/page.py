from typing import Optional
from ._crawlingo_core import Page as _CorePage
from .element import ElementCollection
from .exceptions import handle_core_exception

class Page:
    """
    Core page extraction class. Implements lazy-loading fetching and hook lifecycles.
    """
    def __init__(
        self,
        url: str,
        auto_match: bool = False,
        timeout: int = 30,
        retries: int = 3,
        headers: dict = None,
        cookies: dict = None,
        proxy: str = None,
        session: Optional["Session"] = None,
    ):
        self._url = url
        self._auto_match = auto_match
        self._timeout = timeout
        self._retries = retries
        self._headers = headers or {}
        self._cookies = cookies or {}
        self._proxy = proxy
        self._session = session
        
        self._core_page = None
        self._before_fetch_hooks = []
        self._after_fetch_hooks = []
        self._before_parse_hooks = []
        self._after_extract_hooks = []

    def _ensure_loaded(self):
        if self._core_page is not None:
            return

        # Merge session values as defaults, explicit Page kwargs override
        headers = {}
        cookies = {}
        proxy = None
        auto_match = self._auto_match
        timeout = self._timeout

        if self._session is not None:
            headers.update(self._session._headers)
            cookies.update(self._session._cookies)
            proxy = self._session._proxy if self._proxy is None else self._proxy
            auto_match = self._session._auto_match if not self._auto_match else self._auto_match
            timeout = self._session._timeout if self._timeout == 30 else self._timeout

        headers.update(self._headers)
        cookies.update(self._cookies)
        proxy = self._proxy or proxy
        auto_match = self._auto_match or auto_match

        # 1. Trigger before_fetch hooks
        class RequestContext:
            def __init__(self, url, headers):
                self.url = url
                self.headers = headers
        
        req_ctx = RequestContext(self._url, headers)
        for hook in self._before_fetch_hooks:
            hook(req_ctx)

        # 2. Fetch via Rust Core
        try:
            self._core_page = _CorePage(
                self._url,
                auto_match=auto_match,
                timeout=timeout,
                retries=self._retries,
                headers=headers,
                cookies=cookies,
                proxy=proxy,
            )
        except Exception as e:
            raise handle_core_exception(e)

        # 3. Trigger after_fetch hooks
        class ResponseContext:
            def __init__(self, status):
                self.status = status

        res_ctx = ResponseContext(self._core_page.status)
        for hook in self._after_fetch_hooks:
            hook(res_ctx)

        # 4. Trigger before_parse hooks
        html_content = self._core_page.html()
        for hook in self._before_parse_hooks:
            html_content = hook(html_content) or html_content

    @property
    def url(self) -> str:
        return self._url

    @property
    def status(self) -> int:
        self._ensure_loaded()
        return self._core_page.status

    def html(self) -> str:
        self._ensure_loaded()
        return self._core_page.html()

    def title(self) -> str:
        self._ensure_loaded()
        return self._core_page.title()

    def css(self, selector: str) -> ElementCollection:
        self._ensure_loaded()
        return ElementCollection(self._core_page.css(selector), self._after_extract_hooks)

    def xpath(self, query: str) -> ElementCollection:
        self._ensure_loaded()
        return ElementCollection(self._core_page.xpath(query), self._after_extract_hooks)

    def find_text(self, text: str) -> ElementCollection:
        self._ensure_loaded()
        return ElementCollection(self._core_page.find_text(text), self._after_extract_hooks)

    def after_text(self, text: str) -> ElementCollection:
        self._ensure_loaded()
        return ElementCollection(self._core_page.after_text(text), self._after_extract_hooks)

    def before_text(self, text: str) -> ElementCollection:
        self._ensure_loaded()
        return ElementCollection(self._core_page.before_text(text), self._after_extract_hooks)

    def regex(self, pattern: str) -> ElementCollection:
        self._ensure_loaded()
        try:
            return ElementCollection(self._core_page.regex(pattern), self._after_extract_hooks)
        except Exception as e:
            raise handle_core_exception(e)

    # Hooks configuration
    def before_fetch(self, fn) -> "Page":
        self._before_fetch_hooks.append(fn)
        return self

    def after_fetch(self, fn) -> "Page":
        self._after_fetch_hooks.append(fn)
        return self

    def before_parse(self, fn) -> "Page":
        self._before_parse_hooks.append(fn)
        return self

    def after_extract(self, fn) -> "Page":
        self._after_extract_hooks.append(fn)
        return self
