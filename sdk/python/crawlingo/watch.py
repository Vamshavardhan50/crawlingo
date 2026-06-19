from typing import Optional, Callable
from ._crawlingo_core import Watch as _CoreWatch
from .session import Session
from .exceptions import handle_core_exception

class Watch:
    """
    Poller watcher to monitor website changes and execute event callbacks.
    """
    def __init__(self, url: str, session: Optional[Session] = None):
        self._session = session or Session()
        try:
            self._core_watch = _CoreWatch(url, self._session._core_session)
        except Exception as e:
            raise handle_core_exception(e)

    def field(self, name: str, selector: str) -> "Watch":
        """Define a CSS selector field to monitor for changes."""
        try:
            self._core_watch.field(name, selector)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def interval(self, seconds: int) -> "Watch":
        """Set page poll intervals in seconds."""
        try:
            self._core_watch.interval(seconds)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def auto_match(self, enabled: bool) -> "Watch":
        """Enable auto-matching self-healing selector recovery."""
        self._session.auto_match(enabled)
        return self

    def on_change(self, fn: Callable) -> "Watch":
        """Register a callback for any value change event."""
        try:
            self._core_watch.on_change(fn)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def on_price_change(self, fn: Callable) -> "Watch":
        """Register a callback for price change events."""
        try:
            self._core_watch.on_price_change(fn)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def on_stock_change(self, fn: Callable) -> "Watch":
        """Register a callback for stock status changes."""
        try:
            self._core_watch.on_stock_change(fn)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def on_element_added(self, fn: Callable) -> "Watch":
        """Register a callback when a new field element is found."""
        try:
            self._core_watch.on_element_added(fn)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def on_element_removed(self, fn: Callable) -> "Watch":
        """Register a callback when a monitored element disappears."""
        try:
            self._core_watch.on_element_removed(fn)
        except Exception as e:
            raise handle_core_exception(e)
        return self

    def run(self):
        """Starts the watch loop synchronously on the current thread."""
        try:
            self._core_watch.run()
        except Exception as e:
            raise handle_core_exception(e)

    async def run_async(self):
        """Starts the watch loop asynchronously in a background thread."""
        import asyncio
        loop = asyncio.get_running_loop()
        await loop.run_in_executor(None, self.run)

    def stop(self):
        """Cancels and stops the watcher loop execution."""
        try:
            self._core_watch.stop()
        except Exception as e:
            raise handle_core_exception(e)
