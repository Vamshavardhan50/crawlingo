# crawlingo DOM Tree

Immutable in-memory DOM representation constructed from parsed HTML.

## Structure

The DOM is parsed once on page load and stored as a lightweight tree:

```
Document
 └── html
      ├── head
      │    ├── title
      │    ├── meta
      │    └── link
      └── body
           ├── header
           ├── main
           │    ├── div.product
           │    │    ├── h2.name
           │    │    └── span.price
           │    └── div.product
           │         ├── h2.name
           │         └── span.price
           └── footer
```

## Properties

- **Immutable**: The tree cannot be modified after construction
- **Read-once**: Elements are created lazily when accessed via selectors
- **Memory-efficient**: Only parsed nodes that are accessed are fully materialized

## Node Operations

| Operation | Description |
|-----------|-------------|
| `.parent()` | Navigate to parent node |
| `.children()` | Navigate to child collection |
| `.next()` | Navigate to next sibling |
| `.prev()` | Navigate to previous sibling |
| `.siblings()` | All siblings of current node |

All navigation returns `Element` or `ElementCollection`. See [Python SDK Element](sdk_python.md#element).

## See Also

- [Selector Engine](selector_engine.md): Query the DOM tree
- [Element](sdk_python.md#element): Element API with navigation
- [Page](page.md): Creates and holds the DOM tree
