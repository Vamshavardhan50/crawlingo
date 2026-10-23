# Dataset recipes

Small changes to the shared configuration for common extraction tasks.

<a id="products"></a>

## Product rows

Use article.product as row_selector; extract a required sku attribute, a name and a validated price. Choose whether identity means the product or the full observation before setting dedup_fields.

- [Complete product configuration](dataset-jobs.md#row-containers)

<a id="articles"></a>

## Article pages

Test extraction order and boilerplate on representative documents before treating the output as training data. The pipeline validates fields; it does not guarantee licensing suitability, semantic quality or model readiness.

### job.json

```json
{
  "urls_file": "urls.txt",
  "output_dir": "datasets/articles",
  "fields": [
    {
      "name": "title",
      "selector": "h1",
      "required": true
    },
    {
      "name": "body",
      "selector": "article",
      "required": true,
      "min_length": 100
    }
  ],
  "format": "jsonl"
}
```

<a id="csv"></a>

## CSV output

Set format to csv in a new job directory. Every shard has declared field columns followed by _source_url and _row. Read each shard with a CSV parser; do not concatenate repeated header rows into your data.

- [Export contract](recovery.md)
