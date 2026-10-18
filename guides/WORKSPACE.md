# Workspace map

| Location | Purpose |
| --- | --- |
| `src/` | Rust engine, dataset jobs and CLI |
| `sdk/python/` | Python wrappers and wheel configuration |
| `sdk/nodejs/` | Node wrappers, native addon and package scripts |
| `tests/`, `benches/` | Rust checks and benchmark sources |
| `examples/product_dataset/` | Small catalog with shared job configuration |
| `examples/dataset_scale.rs` | Verified local HTTP scale harness |
| `deploy/` | Worker container template and operating notes |
| `guides/` | Maintainer and deployment guides |
| `scripts/` | Release version check and scale measurement helper |
| `.github/` | CI, release workflow, issue forms and repository setup |
| `assets/` | Brand assets and the preserved architecture board |
| `crawlingo-docs/` | **Separate Git repository:** Next.js docs website |
| `docs/` | Local Markdown exports in an older checkout of the website repository |
| `target/` | Ignored build output and local acceptance evidence |
| `archive/` | Ignored historical material; preserve until reviewed |

## Common commands

From the root:

```sh
npm run docs:dev
npm run docs:check
npm run docs:build
npm run sdk:node
npm run release:check
```

Install website dependencies with `npm ci --prefix crawlingo-docs` first.
These root commands are development conveniences; the root package is private.
The Makefile also points to the same directories.

## Cleanup policy

Generated `target/`, `dist/`, `.next/`, Python bytecode and test reports are not
source. Preserve the million-page acceptance reports before clearing `target/`.
The dataset state and shards are user output: deleting part of a job breaks
resume, so they are never included in generic cleanup commands.

Unused old UI components, their helper, five unused media assets and the obsolete
install script have been removed. Generated Node reports and output have also
been removed from source; local report evidence is in `target/node-test-reports/`.
The architecture board is preserved at `assets/architecture-board.tldr`; the
obsolete static-site merge script was removed from `.github/`.
Generated native addons and Python test exports are ignored. The previously
tracked Windows addon and two Python JSON exports were preserved under
`target/repository-cleanup-20261018/` and removed from source. Rebuild native
addons from the matching Rust source; do not reuse old binaries for a new version.
Root and Node native Cargo lockfiles now belong in source control so CI and
releases can use `--locked`.
API content now lives in `crawlingo-docs/src/content/`; route components,
layout components and styles have separate folders. Do not move nested `.git`
directories or assume a root commit contains changes to the website repository.
The `docs/` checkout and `crawlingo-docs/` share the same GitHub remote. Push website
changes from `crawlingo-docs/`; keep `docs/` as a local export and do not push its
older branch over the website. Exported guides needed by engine users live in
this repository's `guides/` directory.

To update the Markdown copy from website content:

```sh
npm run export:guides --prefix crawlingo-docs -- ../docs
```

The exporter updates its named guides and index. It leaves other authored
documents alone. Review changes in each repository before committing.
