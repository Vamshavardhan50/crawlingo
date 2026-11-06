# Crawlingo Selector Studio

Selector Studio opens a URL in a temporary Playwright Chromium window. A large,
transparent-background spider stays fixed at the center of the screen; clicking
an element highlights it and produces a CSS selector. Use the inspector to
preview fields and export a Crawlingo dataset job.

This is an **optional local development tool** in the repository, not a hosted
service or a feature included in the published SDK packages.

## Start

For a local interactive demonstration with six clearly labeled synthetic records,
run `npm run demo` after installing the dependencies and Chromium below. It opens
the browser and starts collection automatically. Click **Watch spider collect**
to replay, or **Stop** to take control. Results go to `exports/demo`.

Install Node.js 22 or newer, clone this repository, then run:

```powershell
cd D:\Scraper\tools\selector-studio  # Use your own checkout path.
npm ci
npx playwright install chromium
npm start
```

Enter a URL on the Studio start screen and choose **Open in Studio**. Then click
**Watch spider collect**, or expand the manual selector controls. You can also
skip the start screen with `npm start -- https://peps.python.org/numerical/`.
Use the browser Back button to return to the start screen.

The browser opens in a new,
temporary context; close it to stop the tool. Add `--output <directory>` after
the URL to change the export parent directory (default: `./exports`).

## Pick a dataset

### Watch the spider collect automatically

Click **Watch spider collect**. The browser finds a likely group of repeated
records, scrolls to each one, and animates the spider grabbing it. The panel
shows the collected titles. After the pass it saves `records.json` in a new
`preview-<id>` folder under the export directory; the terminal prints its path.
Each record contains a guessed title, visible text, and the first HTTP(S) link.
The body remains centered while the leg nearest the selected record extends,
grips, and retracts. The other seven legs keep their resting animation. Choose Gentle, Normal, or Quick animation speed. The compact
bottom panel shows progress; expand **Collected records** to review and save,
or **Choose selectors & configure a dataset job** for manual controls.
**Hide spider** temporarily clears the mascot without stopping collection.

This is a heuristic preview of up to **30 records already on the current page**.
It does not follow links, paginate, exhaust infinite scrolling, or infer a
complete domain-specific schema. Review the data. If it selects the wrong group,
set a row selector manually and run it again. **Stop** interrupts the pass;
**Save collected JSON** saves a partial result. Manual selection remains available.

### Choose the fields yourself

1. In **Pick element** mode, hover over the page and click a field. The spider
   catches it and the selector appears in the panel. Scroll the page normally;
   the highlight follows its element.
2. For repeated records, click an element inside a record, use **Select parent**
   until the complete record is selected, and click **Use as rows**. Check the
   row count. You can also type a row selector and select **Apply rows**.
3. Click a field within a record. Choose text or an attribute, enter a field
   name, review the sample values, and click **Add field**. Repeat for each
   column. Use **Preview matches** after editing a selector.
4. Click **Export files**. The terminal prints the new folder containing
   `job.json` and `urls.txt`. **Copy selector** and **Copy job JSON** are also
   available when the page permits clipboard access.
5. Choose **Browse page** or press Escape to follow links and use page controls;
   press Escape again to resume picking. The panel can be collapsed using `−`.

Selections reset when the URL changes. Remove saved fields before changing the
row selector, because each field is defined relative to its record scope.

## Run the exported job

First build the CLI from the repository root:

```powershell
cargo build --locked --release --features cli
```

Then change into the export folder shown in the terminal and run the CLI. For a
default export inside this checkout on Windows, the command is:

```powershell
cd tools\selector-studio\exports\selection-<id>
..\..\..\..\target\release\crawlingo.exe dataset job.json
```

Use the actual folder name in place of `selection-<id>`. Relative paths inside
the job are resolved from the current working directory. The output is written
to `datasets/selected` in that export folder. Read the job report and output
before using the dataset; use a new output directory when changing the input.

The inspector previews the **rendered browser DOM**. The dataset job fetches HTML
through Crawlingo's HTTP engine; it does not reuse the browser DOM, browser login,
cookies, or JavaScript rendering. A preview can therefore differ from actual
job results. Test on a small permitted sample before scaling. The initial export
contains only the current URL, uses JSONL, two workers, and respects robots.txt.
Edit and review the generated job and URL file for your workload.

The picker supports tags, classes, IDs, descendant spaces, and positive
`:nth-child(N)` selectors. It inspects the top-level document only; iframe and
shadow-root contents are not selectable. Some sites may prevent inspection or
load different content in the HTTP engine. Nested row containers can also lead
to different grouping in the preview and engine, so prefer non-overlapping rows.

## Tests

```powershell
npm test
```

To also run the generated job against an already-built local CLI on Windows:

```powershell
$env:CRAWLINGO_CLI = 'D:\Scraper\target\release\crawlingo.exe'
npm test
```

The browser tests use a local fixture; they do not measure real-site throughput.
