from typing import List, Callable, Iterator, Optional
from ._crawlingo_core import Element as _CoreElement, ElementCollection as _CoreElementCollection

class Element:
    """
    Represents a single DOM element with traversal capabilities.
    """
    def __init__(self, core_element: _CoreElement, after_extract_hooks: List[Callable] = None):
        self._core = core_element
        self._after_extract_hooks = after_extract_hooks or []

    def _apply_extract_hooks(self, val: str) -> str:
        for hook in self._after_extract_hooks:
            val = hook(val) or val
        return val

    def text(self) -> str:
        """Get the text of this element recursively."""
        return self._apply_extract_hooks(self._core.text())

    def html(self) -> str:
        """Get the outer html of this element."""
        return self._core.html()

    def attr(self, name: str) -> str:
        """Get an attribute value by name."""
        return self._core.attr(name)

    def attrs(self) -> dict:
        """Get all attributes of this element."""
        return self._core.attrs()

    def parent(self) -> "Element":
        """Get the parent element."""
        p = self._core.parent()
        return Element(p, self._after_extract_hooks) if p else None

    def children(self) -> "ElementCollection":
        """Get child elements."""
        return ElementCollection(self._core.children(), self._after_extract_hooks)

    def next(self) -> "Element":
        """Get the next sibling element."""
        n = self._core.next()
        return Element(n, self._after_extract_hooks) if n else None

    def prev(self) -> "Element":
        """Get the previous sibling element."""
        p = self._core.prev()
        return Element(p, self._after_extract_hooks) if p else None

    def siblings(self) -> "ElementCollection":
        """Get all sibling elements."""
        return ElementCollection(self._core.siblings(), self._after_extract_hooks)

    def __repr__(self) -> str:
        return f"Element(tag='{self._core.attrs().get('tag', '')}')"


class ElementCollection:
    """
    Wraps multiple DOM elements, supporting lazy chaining and standard collection mapping.
    """
    def __init__(
        self,
        core_collection: _CoreElementCollection = None,
        after_extract_hooks: List[Callable] = None,
        items: List[Element] = None
    ):
        self._core = core_collection
        self._after_extract_hooks = after_extract_hooks or []
        self._items = items

    def _apply_extract_hooks(self, val: str) -> str:
        for hook in self._after_extract_hooks:
            val = hook(val) or val
        return val

    def text(self) -> str:
        """Concatenate and return text of all items."""
        if self._items is not None:
            return " ".join([item.text() for item in self._items])
        return self._apply_extract_hooks(self._core.text()) if self._core else ""

    def texts(self) -> List[str]:
        """Return list of text content for each element."""
        if self._items is not None:
            return [item.text() for item in self._items]
        return [self._apply_extract_hooks(t) for t in self._core.texts()] if self._core else []

    def attr(self, name: str) -> str:
        """Get attribute from first element."""
        if self._items is not None:
            return self._items[0].attr(name) if self._items else None
        return self._core.attr(name) if self._core else None

    def attrs(self) -> dict:
        """Get attributes from first element."""
        if self._items is not None:
            return self._items[0].attrs() if self._items else {}
        return self._core.attrs() if self._core else {}

    def first(self) -> Optional[Element]:
        """Get the first element."""
        if self._items is not None:
            return self._items[0] if self._items else None
        e = self._core.first() if self._core else None
        return Element(e, self._after_extract_hooks) if e else None

    def last(self) -> Optional[Element]:
        """Get the last element."""
        if self._items is not None:
            return self._items[-1] if self._items else None
        e = self._core.last() if self._core else None
        return Element(e, self._after_extract_hooks) if e else None

    def nth(self, n: int) -> Optional[Element]:
        """Get the nth element."""
        if self._items is not None:
            return self._items[n] if 0 <= n < len(self._items) else None
        e = self._core.nth(n) if self._core else None
        return Element(e, self._after_extract_hooks) if e else None

    def __getitem__(self, index: int) -> Optional[Element]:
        return self.nth(index)

    def __iter__(self) -> Iterator[Element]:
        if self._items is not None:
            return iter(self._items)
        for i in range(len(self)):
            yield self.nth(i)

    def __len__(self) -> int:
        if self._items is not None:
            return len(self._items)
        return len(self._core) if self._core else 0

    def filter(self, fn: Callable[[Element], bool]) -> "ElementCollection":
        """Filter items matching python callback criteria."""
        filtered = [item for item in self if fn(item)]
        return ElementCollection(after_extract_hooks=self._after_extract_hooks, items=filtered)

    def map(self, fn: Callable[[Element], any]) -> list:
        """Map elements to custom objects using a callback."""
        return [fn(item) for item in self]

    def __repr__(self) -> str:
        return f"ElementCollection(elements={len(self)})"
