# What you can build

Practical dataset workflows, the fields they need, and the boundaries to check before starting.

> Current development source. New dataset APIs require a matching source build; published alpha packages may differ.

## Product and directory catalogs

Collect a list of products, businesses, courses or documents from pages with repeated entries. Define one row container for each entry, then extract its stable identifier, name and other useful fields. The product fixture demonstrates this pattern with SKUs and prices.

Choose identity deliberately. Deduplicating on a SKU keeps one record per product, even if a later page contains a different price. If your goal is to retain each distinct observation, include the observation fields in identity or create a separate job for each collection date. A date-stamped output folder is your convention, not an automatic timestamp field.

| Decision | Example |
| --- | --- |
| Row | One article.product container |
| Fields | sku, name, price |
| Quality rule | sku and name required; price must be finite |
| Identity | sku for one record per product |
| Output | CSV for review, JSONL for a software pipeline |

- [Product fixture and configuration](https://crawlingo-docs.vercel.app/examples)

## Article and documentation collections

Create one record per page with a title, a body and source metadata. Use a minimum body length to reject empty or navigation-only matches. Start with a few representative pages, especially when a site has different templates.

For text datasets, inspect paragraph order, whitespace, boilerplate and missing sections. The current HTML parser can place local text before descendant text in mixed-content elements. Required-field validation does not establish document fidelity. Crawlingo also does not split text into model tokens, build embeddings, label examples or select a training/evaluation split. Those are downstream steps.

- [Article configuration recipe](https://crawlingo-docs.vercel.app/cookbook#articles)
- [Schema and validation](https://crawlingo-docs.vercel.app/schema)

## Repeatable data collection for analysis

Make the collection process inspectable by retaining its URL file, configuration, output, report and source revision. The _source_url field lets you trace an accepted row back to the page that produced it. Error shards show which records did not meet your rules.

A completed report can include failed pages or rejected rows. Decide what completeness means for your project before running the large job: for example, whether every source must succeed or whether a reviewed set of missing sources is acceptable. The engine reports counts; it does not choose an acceptable error rate for you.

- [Read the job report](https://crawlingo-docs.vercel.app/report-reference)

## Change monitoring

The existing Watch API polls selected page fields and calls your callback when it detects a change. Use it when you need an application to react to a changed title, price or other selected value.

A callback does not automatically create a historical dataset, send a notification or persist an audit log. Your application owns those actions. Choose a polling interval appropriate for the source and keep monitoring results separate from dataset-job progress. Change classification is heuristic, so inspect representative changes before automating a consequential response.

- [Watch API](https://crawlingo-docs.vercel.app/watch)
- [Change event fields](https://crawlingo-docs.vercel.app/change-detection)

## A dataset feature inside your application

A backend can accept a prepared collection request, write its URL file, start a worker and show the resulting report to a user. Use a queue or your existing job system to control worker ownership. Keep records on disk or in your downstream store; do not return a million rows in one web response.

Run the native SDK on a server process. The Node addon is not browser JavaScript. If end users can submit arbitrary URLs, your service needs its own destination and access policy: the library URL validator is not an SSRF security boundary.

- [Developer walkthrough](https://crawlingo-docs.vercel.app/developer-tutorial)
- [Deploy a worker](https://crawlingo-docs.vercel.app/deployment)

## Decide whether the current engine fits

| Requirement | Current fit |
| --- | --- |
| Known URLs and explicit field rules | A direct fit for DatasetJob. |
| Automatically discover every relevant page | Use separate crawler/sitemap APIs to prepare input; discovery is not built into DatasetJob. |
| JavaScript-rendered content | Optional Rust browser feature; validate the rendering path separately. |
| JSONL or CSV with resume | Implemented for local single-writer jobs. |
| Typed JSON values or checkpointed Parquet | Not available in DatasetJob yet. |
| No-code hosted app | Not part of the current product. |
| Distributed cloud crawl | Needs application-level orchestration and additional recovery design. |
