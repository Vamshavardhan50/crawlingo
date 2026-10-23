# Schema and quality

Make valid records explicit. Keep rejected rows visible and preserve source information.

<a id="fields"></a>

## Field reference

| Field option | Default | Use |
| --- | --- | --- |
| name | Required | Unique output key; must not begin with _. |
| selector | Required | Selector evaluated inside the row container. |
| selector_type | css | css, xpath, regex, text, after_text or before_text. |
| extract_type | text | text, price, datetime, url or attr:name. |
| required | false | Reject an empty or whitespace-only value. |
| default | None | Fallback when extraction returns an empty string. |
| data_type | string | string, integer, float or boolean validation. |
| normalize_whitespace | true | Collapse extracted whitespace runs. |
| min_length | 0 | Minimum character count after normalization. |

<a id="types"></a>

## Validation and output types

Job records currently contain string values. Integer, float and boolean types validate and canonicalize those strings; they do not produce JSON numbers or booleans. Integers must parse strictly. Floats must be finite. Booleans accept true/false, yes/no and 1/0.

Invalid rows are counted in records_rejected and written to error shards. Missing row containers and HTTP failures count as page failures. Empty records are rejected. Unicode symbols are preserved; Unicode NFC normalization is not currently performed.

<a id="identity"></a>

## Choose the deduplication key

By default, all extracted fields define record identity; source metadata is excluded. Set dedup_fields to declared keys such as sku for entity deduplication. If only sku defines identity, later price observations with the same sku are duplicates too.

Exact dedup compares canonical content stored on disk, with hash-collision checking. Semantic and near-duplicate detection are not implemented. URLs are deduplicated separately.

<a id="review"></a>

## Review rejected records

Fix the selector, schema or source input after reviewing errors, then use a new job directory. Completed failed URLs are part of the checkpoint; rerunning an unchanged completed job does not retry them.

### Error shard shape

```json
{"url":"https://example.com/item","row":0,"error":"missing required field 'title'"}
```

- [Recovery and job ownership](recovery.md)
