# What is Crawlingo?

Understand what the tool does, what you need to give it, and what you receive at the end.

> Current development source. New dataset APIs require a matching source build; published alpha packages may differ.

## From a web page to a row of useful information

A website presents information for people to read. A dataset organizes selected information into consistent records that software can process. Crawlingo connects the two: it visits the pages you specify, extracts named fields, checks those fields, removes exact duplicates and writes the accepted records to files.

For example, a product page may contain navigation, images, reviews, a product name, a price and an item code. Your dataset might need only the item code, name and price. You define that shape once and apply it to pages with a compatible structure.

Rust runs the extraction pipeline. You can control it through the command line, Python, Node.js or Rust. Choosing Python does not mean you need to rewrite the processing engine in Python.

## Who is it for?

The current product is a library and command-line tool. It does not include a hosted dashboard or a point-and-click scraping application. Setting it up and changing extraction rules requires some technical work. The beginner guides explain that work without assuming Rust knowledge.

| You want to… | Start with… |
| --- | --- |
| Understand whether it fits your project | Read this overview, the use cases and feature status. No code is needed to understand the workflow. |
| Collect a small table for analysis | Run the supplied example, inspect the output, then work with a developer to adapt the selectors to your pages. |
| Add dataset collection to an application | Use the developer tutorial and a Python, Node.js or Rust SDK. |
| Operate a large collection job | Read resource limits, report counters, checkpoints and worker deployment before increasing volume. |

## The three things you provide

A configuration file called job.json brings these choices together. A selector tells the engine which part of a page contains a value. Required fields and type rules describe what an acceptable record looks like. You can begin with the included catalog instead of finding a live website.

- A URL file: one page address per line. This determines which pages the dataset job attempts.
- A record definition: field names such as sku, name and price, plus instructions for finding each value.
- An output directory: a folder owned by this job, where accepted data, errors and restart state are stored.

- [Try the local catalog](https://crawlingo-docs.vercel.app/first-run)
- [Learn the terms](https://crawlingo-docs.vercel.app/concepts)

## What you receive

A shard is simply one part of the dataset. Splitting output avoids keeping every record in memory. A million records do not need to become a million-element Python or JavaScript array. The SDK returns a small report while the engine writes records to disk.

| Output | What it contains | What you do with it |
| --- | --- | --- |
| Data shards | Accepted records in JSONL or CSV. | Read them in a script, analysis tool or downstream import. |
| Error shards | Failed pages and rejected rows, with a reason. | Identify missing fields, incorrect selectors or fetch failures. |
| report.json | Counts, completion state and the number of committed shards. | Check the outcome before trusting the exported data. |
| state/ | Progress and exact-deduplication indexes. | Keep it with the data so an unchanged job can resume. |

## A concrete product example

The local fixture has two distinct pages, a repeated URL, a repeated SKU and a product without a name. It exports two valid products, rejects the missing name and counts the duplicate SKU. These values come from the local product example. The real output also includes _source_url and _row. Numeric values are currently validated strings, so an application should explicitly convert price when it needs a number.

| sku | name | price |
| --- | --- | --- |
| A-101 | Café mug | 12.5 |
| B-202 | Notebook | 6 |

- [Run and inspect the complete example](https://crawlingo-docs.vercel.app/examples)

## What Crawlingo can do today

- Extract one record from a page or multiple records from repeated row containers.
- Validate required fields, minimum lengths, integers, finite decimal numbers and boolean values.
- Normalize whitespace, apply defaults and keep rejected rows visible.
- Deduplicate exact records using all fields or selected identity fields.
- Export JSONL or CSV in checkpoints and resume the same unchanged input.
- Use related APIs for page extraction, crawling, sitemap discovery, monitoring and optional Chromium rendering.

- [Detailed capability matrix](https://crawlingo-docs.vercel.app/features)

## What needs your judgment

Crawlingo needs explicit extraction rules. It does not infer the meaning of every website or automatically produce a finished training dataset from a topic prompt. A valid string can still be the wrong product name if the selector points to the wrong element. Review samples against their source pages.

Browser rendering, selector recovery and monitoring are separate capabilities with their own scope. Dataset jobs do not silently repair selectors. The HTTP benchmark does not measure browser-rendered pages. Exact deduplication does not find semantically similar text.

The current job engine uses local storage and one writer per directory. It does not provide distributed scheduling or remote storage commits. New job APIs describe the current source checkout; an older published alpha may not include them.

## What “one million” means here

A measured acceptance run fetched 1,000,000 small pages from a local HTTP fixture, exported 1,000,000 records, checked every record and resumed without refetching. It used a Windows release build with 50 concurrent requests. The observed peak process working set was 181.5 MiB, including the fixture server and verification.

That result demonstrates the pipeline on the measured workload. It does not predict the time or memory required for a different website, larger documents or JavaScript rendering. Begin with a small representative sample and increase the workload after checking its output.

- [Read the full measurement method](https://crawlingo-docs.vercel.app/benchmarks)

## Choose your next step

- [I want a guided first run](https://crawlingo-docs.vercel.app/first-run)
- [I am building an application](https://crawlingo-docs.vercel.app/developer-tutorial)
- [I want to see possible use cases](https://crawlingo-docs.vercel.app/use-cases)
