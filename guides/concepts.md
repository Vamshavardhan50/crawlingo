# The basic concepts

A shared vocabulary for planning a dataset, reading the configuration and understanding the result.

<a id="records"></a>

## Pages, rows and fields

A page is one HTTP response. A record is one structured item extracted from it. A field is one named value inside that item. One article page might produce one record; a catalog page might produce many.

A row selector identifies the repeated containers that belong to individual records. Field selectors operate within those containers. Without a row selector, the entire page is treated as one record. This distinction prevents a missing field in one product from borrowing a value from the next product.

| Term | Product catalog example |
| --- | --- |
| Page | catalog.html |
| Row container | article.product |
| Field name | price |
| Field selector | .price |
| Extracted value | 12.5 |

<a id="selectors"></a>

## Selectors describe where a value lives

A CSS selector is a small rule for finding HTML elements. h1 matches a heading, .price matches elements with the price class, and article.product matches product containers. The extractor can read text or an attribute such as data-sku.

Selectors depend on the actual HTML returned to the engine. What appears after JavaScript runs in your browser may not exist in the original response. The developer should inspect both the source and a sample extraction before choosing a fetch strategy.

- [Selector reference](selectors.md)

<a id="schema"></a>

## A schema defines acceptable records

A schema describes fields and their rules. required rejects a missing value. min_length catches unexpectedly short text. data_type validates a string as an integer, finite float or boolean. A default fills an empty extracted value when that fallback is meaningful.

Rules enforce the checks you choose. They do not prove the real-world accuracy of a value. For example, a price of 999999 is a valid number even if it is wrong for that product. Apply domain-specific checks in your application after extraction.

- [Field schema reference](schema.md)

<a id="identity"></a>

## Deduplication is a choice about identity

A duplicate URL is an input address the job has already processed. A duplicate record is a row whose identity values match an accepted row. They are counted separately.

With dedup_fields empty, all extracted fields define identity. With dedup_fields set to sku, only the SKU defines identity. Metadata such as the source URL is excluded from default record identity. The comparison is exact; it does not detect paraphrases or near-duplicate text.

<a id="checkpoints"></a>

## Checkpoints make work resumable

A checkpoint is a committed boundary in the input and output. The engine writes and flushes a batch of data and errors, then advances its stored progress. If the process stops before that boundary is committed, the unfinished batch is replayed on the next run.

Keep the same input and configuration for resume. A changed URL file, selector or limit defines a different job and needs a fresh directory. Resume is not an instruction to refresh every page or retry all earlier failures.

- [Recovery details](recovery.md)

<a id="formats"></a>

## JSONL, CSV and shards

When combining CSV shards, read each with a CSV parser so its header is interpreted once. Do not concatenate file bytes and treat repeated headers as data. For JSONL, parse each line rather than loading the complete file into memory.

| Term | Meaning |
| --- | --- |
| JSONL | One JSON object per line; useful for processing records incrementally. |
| CSV | A table with a header and delimited values; each shard includes its own header. |
| Shard | One numbered output file containing part of the dataset. |
| Error shard | A JSONL file listing failed pages and rejected rows. |
| State directory | The database containing checkpoints and identity indexes. |

<a id="runtime"></a>

## Concurrency, rate and memory

Concurrency is how many page operations can be in progress at once. A request rate limit controls how quickly requests may start. They solve different problems: increasing concurrency does not remove source-side limits or make every network connection faster.

Memory includes response bodies, parsed documents, extracted rows, pending deduplication values and the database cache. Bounds on these individual parts make large jobs manageable, but no single setting guarantees a fixed process memory size. Measure representative pages before raising limits.

- [Running larger jobs](production-guide.md)
