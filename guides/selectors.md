# Selectors

Write explicit extraction rules and test them against representative page shapes.

<a id="columns"></a>

## Select a table column

Use td:nth-child(3) for the third element child of a row. Positive integer positions are supported; odd, even, an+b expressions and other pseudo-classes are not. Element positions ignore whitespace text. Qualify the column before selecting its links so version, author or related-document links do not contaminate a title.

### Directory field

```json
{"name":"title","selector":"td:nth-child(3)","required":true}
```

- [Research directory example](research-datasets.md)

<a id="css"></a>

## CSS and row scope

These are entries in the fields array of a job with row_selector set to article.product. The row container itself can match an attribute field. The selector engine supports a subset of browser selector behavior; check your selectors against real fixtures.

### Field definitions

```json
[
  {"name":"title","selector":"h2","required":true},
  {"name":"sku","selector":"article.product","extract_type":"attr:data-sku"}
]
```

<a id="alternatives"></a>

## Other selector modes

Job selector_type accepts css, xpath, regex, text, after_text and before_text. These are the project’s implementations, not promises of complete browser CSS or full XPath standards compliance. Regular expressions are compiled during configuration validation.

<a id="drift"></a>

## Handle layout changes

Required fields and rejection reports expose broken extraction. DatasetJob does not silently recover selectors. The existing Dataset builder can optionally use stored fingerprints for heuristic CSS recovery.

- [Selector recovery](auto-match.md)
