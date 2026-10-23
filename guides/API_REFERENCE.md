# API reference maintenance

Crawlingo converts explicit URL inputs and extraction schemas into validated,
deduplicated datasets. `DatasetJob` is the bulk export entry point; `Page`,
`Dataset`, `Crawl`, `Sitemap` and `Watch` support smaller inspection, discovery
and monitoring workflows.

## Reference surfaces

| Surface | Documentation source | Coverage |
| --- | --- | --- |
| Node / TypeScript | Website `/reference/node` | Every package export, declared method and field |
| Python | Website `/reference/python` | `__all__`, reachable result/context types and hook helpers |
| Rust | `cargo doc --locked --no-deps --lib --features browser --open` | Public modules, types, traits and implementations with browser support |
| CLI | Website `/reference/cli` | Commands, flags, defaults and exit behavior |
| Availability | Website `/reference/implementation-status` | Source provenance, binding gaps and operational boundaries |

These website routes are prepared in the documentation source; deploying the
website is a separate operation. The public website may still describe an older
snapshot until its hosting deployment succeeds.

## Refresh SDK declarations

From the engine checkout, install the compiler dependency and generate the
inventory without importing a native SDK:

```sh
cd sdk/nodejs
npm ci --ignore-scripts
cd ../..
node scripts/export-sdk-api.cjs crawlingo-docs/src/content/generated/sdk-api.json
```

Requires Node and Python 3. Set `PYTHON` when your interpreter has another name
or location. Source hashes use UTF-8 with LF line endings so Windows and Linux
checkouts produce the same provenance. Rust binding inputs are included.

The generator extracts declarations, not runtime test results. Review the
authored behavior, examples and known-issue notes when changing implementation.
The current audit records Python Crawl depth/webhook/schedule binding gaps, a
Crawl result-shape mismatch, ineffective `Page.before_parse` replacement and
materialized `ElementCollection` iteration. Node scheduled crawls also lack a
public cancellation/join handle. Do not advertise those paths as production-ready.

## Build and export the book

`crawlingo-docs/` is a separate Git repository. In that checkout:

```sh
npm ci
npm run lint
npm run export:guides -- ../docs
npm run build -- --webpack
npm run package:docs
```

The Markdown exporter writes every content page, relative internal links,
explicit section anchors, a complete index and the source inventory. It preserves
files outside its named exports. In this workspace `docs/` is an older independent
checkout; publish website changes from `crawlingo-docs/`.

## Release requirements

Commit the engine changes before generating a release snapshot. Commit the
matching website content and snapshot separately. The website release requires
the tag `vVERSION`, matching package/snapshot versions and the correct prerelease
flag. It checks the recorded engine commit, file hashes and regenerated inventory
before uploading server/Markdown archives, a manifest and SHA256SUMS.

Manual documentation runs prepare reviewable artifacts; they do not publish SDKs
or deploy hosting. Engine SDK releases additionally require the `rust-docs`
artifact. Rust documentation failures block release artifact preparation.

See the website `/reference/docs-release` for the full procedure and recovery
steps. Local builds do not establish that hosted CI or a public deployment passed.
