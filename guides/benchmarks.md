# Measured scale

A reproducible acceptance result, with the workload and limits shown alongside the numbers.

<a id="result"></a>

## One million pages → one million records

Recorded on 2026-10-14 on Windows x86_64 with eight logical processors, using a release build and small generated UTF-8 HTML served over local HTTP. The fixture server, output checks and restart run in the same process. Other development builds shared the machine.

This used the modified working tree based on d19a62c04ab09713d04f86e958434665b60d131f, before final robots edge-case fixes. Robots checks were disabled for the fixture.

| Measurement | Observed result |
| --- | --- |
| Pages fetched / records exported | 1,000,000 / 1,000,000 |
| Pages failed / rows rejected | 0 / 0 |
| Records individually checked | 1,000,000 |
| Concurrency / checkpoint pages | 50 / 1,000 |
| Initial job time | 1,094.569 seconds |
| Harness wall time | 1,101.961 seconds |
| Peak working set | 181.5 MiB |
| Resume without refetching | Passed |

<a id="meaning"></a>

## What the result establishes

The production job processed the input, exported the expected records, checked every value and resumed without refetching. It does not predict throughput on arbitrary websites, JavaScript rendering, many hosts or larger pages.

The earlier 3,500 requests/second and 2.4 MB claims were unsupported and have been withdrawn. No equivalent Scrapy or Playwright comparison was run. SIMD dependency use does not establish an independent SIMD speedup.

<a id="reproduce"></a>

## Reproduce the run

Use a new output directory. acceptance.json records the original job and verification. The PowerShell wrapper records process memory and wall time alongside it. After the harness resumes, dataset/report.json describes the short restart invocation.

### PowerShell

```powershell
cargo build --release --example dataset_scale
./scripts/measure-dataset-scale.ps1 -Pages 1000000 -OutputDirectory D:/Scraper/target/scale-new -Mode http
```

### Any supported platform

```shell
cargo run --release --example dataset_scale -- 1000000 target/scale-new http
```

- [Full result and release gates](https://github.com/Vamshavardhan50/crawlingo/blob/main/SCALE_RESULTS.md)
