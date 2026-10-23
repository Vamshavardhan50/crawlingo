# Your first run, step by step

Follow a small local example, understand each file, and learn what a successful or partially successful run looks like.

<a id="prepare"></a>

## 1. Prepare the tools once

You need a terminal, the current Crawlingo source checkout, Python to serve the sample HTML, and the Rust/native build tools listed in Installation. If native build setup is unfamiliar, ask your developer to help with this step. You do not need to write Rust code to use the CLI.

The terminal is where you enter commands. Its working directory matters: relative paths such as urls.txt are resolved from that directory. Keep one terminal for the local server and open another for running the job.

### Repository checkout

```shell
git clone https://github.com/Vamshavardhan50/crawlingo.git
cd crawlingo
cargo build --release --features cli
```

- [Build prerequisites and SDK alternatives](installation.md)

<a id="files"></a>

## 2. Open the example folder

Find examples/product_dataset in the checkout. You will use the supplied files as they are on the first run. The example is local so the expected data does not depend on a changing third-party website.

| File or folder | Purpose |
| --- | --- |
| site/catalog.html and site/more.html | The two sample pages. |
| site/robots.txt | Allows the local sample to be fetched. |
| urls.txt | Three URL lines; one is deliberately repeated. |
| job.json | The row, field, quality and identity rules. |
| run.py / run.cjs | Alternative Python and Node entry points. |
| output/ | Created by the job; holds data, errors and state. |

<a id="serve"></a>

## 3. Start the local sample website

In the first terminal, from the repository root, run the command below. Leave it running. Open http://127.0.0.1:8765/catalog.html in your browser to see the source products. The page intentionally contains a product with no name.

### Terminal 1

```shell
python -m http.server 8765 --bind 127.0.0.1 --directory examples/product_dataset/site
```

<a id="run"></a>

## 4. Generate the dataset

In the second terminal, start in the repository root, move into the example folder and run its configuration. You will see a JSON report in the terminal. The job also writes that report to output/report.json.

### Terminal 2

```shell
cd examples/product_dataset
cargo run --release --features cli -- dataset job.json
```

- This example deliberately ends with status 2 because one row is rejected. That means you need to inspect the reported issue; it does not mean the two accepted products disappeared. A fatal startup or storage error uses status 1. A fully clean completion uses status 0.

<a id="review"></a>

## 5. Read the result before using it

Open output/part-00000000.jsonl in a text editor. Each line is one accepted product. Then open output/part-00000000.errors.jsonl: its missing-name error points to the affected page and row. The source row index begins at zero.

A complete report means the input reached its end. It can still contain rejected rows. Check the counters every time, including after an automated run.

| Counter | Expected value | Meaning |
| --- | --- | --- |
| pages_fetched | 2 | The two distinct catalog pages were processed successfully. |
| records_written | 2 | Two valid products were exported. |
| records_rejected | 1 | A product had no required name. |
| duplicate_records | 1 | The repeated SKU was not exported again. |
| duplicate_urls | 1 | The repeated address was not fetched again. |
| pages_failed | 0 | No page-level failures occurred. |

- [Every report field explained](report-reference.md)

<a id="resume"></a>

## 6. Run the command again

Keep the server running and repeat the same job command. The accepted and error counts stay cumulative. The finished pages are not fetched again. elapsed_ms changes because it measures the new invocation.

Do not delete state/ to start over. For a separate experiment, choose a new output_dir in a copy of the configuration. Keep a finished job together so you can trace what produced its data.

<a id="csv"></a>

## 7. Make a table for review

To create CSV, copy job.json to job-csv.json, change format to csv and change output_dir to output-csv. Leave the other settings alone. Run the new file. Each part-XXXXXXXX.csv has a header row and can be imported into a spreadsheet or read with a CSV parser.

Identifiers such as SKU should usually stay text when importing. Otherwise a spreadsheet may remove leading zeros. Crawlingo does not control the type inference performed by your spreadsheet application.

### From examples/product_dataset

```shell
cargo run --release --features cli -- dataset job-csv.json
```

<a id="adapt"></a>

## 8. Adapt the workflow to your pages

- Write down the fields you need and what one record represents.
- Collect a small sample of page URLs, including pages with missing or unusual values.
- Inspect the HTML and choose selectors for those fields.
- Choose required values and a stable record identity.
- Use a fresh output directory and review every output row in the small sample.
- Increase the input only after the sample behaves as expected.

- [Developer tutorial for adapting the example](developer-tutorial.md)
- [Use cases and limitations](use-cases.md)

<a id="trouble"></a>

## If something goes wrong

| Symptom | First thing to check |
| --- | --- |
| Connection refused | Is the sample HTTP server still running on port 8765? |
| job.json or urls.txt not found | Is the second terminal inside examples/product_dataset? |
| Missing DatasetJob in an SDK | Did you build the current wrapper and native extension together? |
| Configuration changed error | Use a fresh output directory after changing the config or input. |
| The output folder is locked | Another process may still own that job; let it stop before resuming. |

- [Full troubleshooting guide](faq.md)
