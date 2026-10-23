# Playwright & Puppeteer

Discover links with a browser, then hand a stable URL file to a dataset job.

<a id="fit"></a>

## 1. Choose the right browser boundary

Use this pattern when a listing needs JavaScript or interaction, but each destination page contains the fields you need in its HTTP response. The browser runs only for discovery. The dataset job owns fetching, field validation, deduplication and checkpoints.

If the data exists only after JavaScript runs on each destination, this URL handoff is insufficient. Crawlingo will fetch the destination again without the Playwright page state. See the rendered-content section before choosing this design.

<a id="install"></a>

## 2. Prepare the tools

Build the matching Crawlingo SDK first. Install Playwright in the same Python environment or Node application. Browser installation is a separate step. The examples accept a listing URL and link selector so you can use a site you control.

### Python

```shell
python -m pip install playwright
python -m playwright install chromium
```

### Node.js

```shell
npm install playwright
npx playwright install chromium
```

- [Crawlingo source installation](installation.md)

<a id="discover"></a>

## 3. Save the discovered URLs

Save one of these examples and pass a listing URL plus the CSS selector for destination links. It waits for a link and keeps HTTP(S) URLs on the same origin. A real listing may require an explicit “all results loaded” signal, pagination or button clicks before collecting links. The link list is in memory for this discovery step; split very large listings into bounded batches.

### Python · discover.py

```python
import sys
from pathlib import Path
from urllib.parse import urlsplit
from playwright.sync_api import sync_playwright

listing, selector = sys.argv[1:3]
with sync_playwright() as p:
    browser = p.chromium.launch()
    try:
        page = browser.new_page()
        page.goto(listing, wait_until="domcontentloaded")
        links = page.locator(selector)
        links.first.wait_for(state="attached")
        origin = urlsplit(page.url)
        urls = links.evaluate_all("(nodes) => nodes.map(a => a.href)")
        accepted = set()
        for value in urls:
            if not value:
                continue
            url = urlsplit(value)
            if url.scheme in ("http", "https") and (url.scheme, url.netloc) == (origin.scheme, origin.netloc):
                accepted.add(url._replace(fragment="").geturl())
        if not accepted:
            raise RuntimeError("No destination URLs matched")
        Path("urls.txt").write_text("\n".join(sorted(accepted)) + "\n", encoding="utf-8")
    finally:
        browser.close()
```

### Node.js · discover.mjs

```javascript
import { chromium } from 'playwright';
import { writeFile } from 'node:fs/promises';

const [listing, selector] = process.argv.slice(2);
if (!listing || !selector) throw new Error('Pass a URL and link selector');
const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  await page.goto(listing, { waitUntil: 'domcontentloaded' });
  const links = page.locator(selector);
  await links.first().waitFor({ state: 'attached' });
  const origin = new URL(page.url()).origin;
  const hrefs = await links.evaluateAll(nodes => nodes.map(a => a.href));
  const urls = new Set();
  for (const href of hrefs) {
    if (!href) continue;
    const url = new URL(href);
    if (['http:', 'https:'].includes(url.protocol) && url.origin === origin) {
      url.hash = '';
      urls.add(url.href);
    }
  }
  if (!urls.size) throw new Error('No destination URLs matched');
  await writeFile('urls.txt', [...urls].sort().join('\n') + '\n');
} finally {
  await browser.close();
}
```

<a id="run"></a>

## 4. Run a dataset job

Inspect urls.txt, then keep it unchanged for the lifetime of the job. Save job.json below in the same directory. Replace h1 with the field selector in the destination response. This example produces one title record per page.

### job.json

```json
{
  "urls_file": "urls.txt",
  "output_dir": "datasets/discovered",
  "fields": [
    {
      "name": "title",
      "selector": "h1",
      "required": true
    }
  ],
  "concurrency": 8,
  "format": "jsonl"
}
```

### Python · run_job.py

```python
import json
from crawlingo import DatasetJob

with open("job.json", encoding="utf-8") as source:
    report = DatasetJob(json.load(source)).run()
print(report)
if not report["complete"] or report["cancelled"] or report["pages_failed"] or report["records_rejected"]:
    raise SystemExit("Review the report before publishing this dataset")
```

### Node.js · run-job.mjs

```javascript
import { readFile } from 'node:fs/promises';
import { DatasetJob } from 'crawlingo';

const config = JSON.parse(await readFile('job.json', 'utf8'));
const report = await new DatasetJob(config).run();
console.log(report);
if (!report.complete || report.cancelled || report.pages_failed || report.records_rejected) {
  throw new Error('Review the report before publishing this dataset');
}
```

- [Output and recovery contract](api-dataset-job.md)

<a id="puppeteer"></a>

## Using Puppeteer instead

In a Node project with Puppeteer installed, use its browser for the discovery stage. The output is the same urls.txt, so the job configuration and execution above are unchanged. Adjust the selector to the page and wait for the complete listing before taking a snapshot.

### Node.js · discover-puppeteer.mjs

```javascript
import puppeteer from 'puppeteer';
import { writeFile } from 'node:fs/promises';

const [listing, selector] = process.argv.slice(2);
if (!listing || !selector) throw new Error('Pass a URL and link selector');
const browser = await puppeteer.launch();
try {
  const page = await browser.newPage();
  await page.goto(listing, { waitUntil: 'domcontentloaded' });
  await page.waitForSelector(selector);
  const origin = new URL(page.url()).origin;
  const hrefs = await page.$$eval(selector, nodes => nodes.map(a => a.href));
  const urls = new Set();
  for (const href of hrefs) {
    if (!href) continue;
    const url = new URL(href);
    if (['http:', 'https:'].includes(url.protocol) && url.origin === origin) {
      url.hash = '';
      urls.add(url.href);
    }
  }
  if (!urls.size) throw new Error('No destination URLs matched');
  await writeFile('urls.txt', [...urls].sort().join('\n') + '\n');
} finally {
  await browser.close();
}
```

<a id="rendered-content"></a>

## When the destination also needs JavaScript

page.content() returns the rendered document HTML. The current Python and Node Page wrappers have no public from_html / fromHtml constructor, and DatasetJob accepts a URL file rather than HTML records. Do not pass HTML to Page.create: it expects a URL.

For a browser-owned extraction workflow, keep extraction in Playwright or Puppeteer and write its records using your own pipeline. Alternatively, a custom Rust application can use Crawlingo’s optional browser feature and a browser-tier Session. That path launches its own Chromium process; it does not attach to the Playwright context.

Playwright cookies, local storage, authentication and proxy configuration are not shared automatically. Session HTTP settings must be configured separately. A copied cookie may still be insufficient for a site whose state lives in the browser. Start with public destinations and inspect the actual response before scaling.

- [Session reference](api-session.md)
- [Rust browser tier](advanced.md)

<a id="sources"></a>

## API sources

- [Playwright Python: locator.evaluate_all](https://playwright.dev/python/docs/api/class-locator#locator-evaluate-all)
- [Playwright: page.content](https://playwright.dev/docs/api/class-page#page-content)
- [Puppeteer: page.$$eval](https://pptr.dev/api/puppeteer.page.__eval)
