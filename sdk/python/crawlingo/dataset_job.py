"""Bounded dataset jobs; URLs and records stay in Rust rather than Python lists."""
import asyncio
import json
from typing import Optional
from ._crawlingo_core import DatasetJob as _CoreJob
from .session import Session


class DatasetJob:
    def __init__(self, config: dict, session: Optional[Session] = None):
        self._session = session or Session()
        self._core = _CoreJob(json.dumps(config), self._session._core_session)

    def run(self) -> dict:
        """Run or resume, returning counters and the committed shard count."""
        return json.loads(self._core.run())

    async def run_async(self) -> dict:
        loop = asyncio.get_running_loop()
        future = loop.run_in_executor(None, self.run)
        try:
            return await asyncio.shield(future)
        except asyncio.CancelledError:
            self.cancel()
            # Keep the native worker alive until it releases the output-directory lock.
            # Shield prevents cancellation of the asyncio wrapper from hiding completion.
            try:
                await asyncio.shield(future)
            except Exception:
                pass
            raise

    def cancel(self) -> None:
        """Stop active requests; replay the uncommitted batch on a new instance."""
        self._core.cancel()
