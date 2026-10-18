# Contributing to Crawlingo

Start with the [product definition](guides/PRODUCT.md),
[workspace map](guides/WORKSPACE.md) and [code of conduct](CODE_OF_CONDUCT.md).
Small reproducible fixes, accurate examples and measured improvements are useful
contributions.

## Development setup

Use the stable Rust toolchain selected by `rust-toolchain.toml`. Native dependencies
also need a C/C++ toolchain, CMake and libclang. CI installs these on Linux and
configures LLVM on Windows. Set `LIBCLANG_PATH` when clang cannot be found locally.

For the Python SDK, use a virtual environment with CPython 3.12 or 3.13 for current
CI targets. The broader wheel matrix is in the release workflow. For the Node SDK,
use Node 22 or 24 and its npm installation.

```sh
git clone https://github.com/YOUR_USERNAME/crawlingo.git
cd crawlingo
git switch -c fix/describe-the-change
cargo build
```

### Rust engine

```sh
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --lib --tests
cargo check --features cli,browser
cargo doc --no-deps
```

Tests use writable temporary storage and local HTTP fixtures. Browser runtime
tests additionally need a local Chrome/Chromium executable and the `browser`
feature. Set `CRAWLINGO_CHROME_PATH`, then run:

```sh
cargo test --features browser --test browser_phase3_test
```

Browser tests can skip when Chrome is unavailable; record whether Chrome actually
ran. Compilation alone is not browser execution evidence.

### Python SDK

Activate a virtual environment, then run from the repository root:

```sh
python -m pip install maturin
cd sdk/python
maturin develop
cd ../..
python -m unittest discover -s sdk/python/tests -p test_import.py -v
python -m unittest discover -s sdk/python/tests -p test_dataset_job.py -v
python sdk/python/tests/test_all_apis.py
```

### Node.js SDK

```sh
cd sdk/nodejs
npm ci
npm run build
npm test
npm run test:all
node --test tests/dataset_job.test.cjs
npm run check:package
```

`npm test` checks exports; `npm run test:all` runs the implemented API suite.
Experimental entries excluded by that runner are not passing coverage. The dataset
job test uses the freshly built native module. Rebuild for the target OS and architecture.

The Python comprehensive script is a standalone runner, not a pytest fixture suite.
CI runs it explicitly and fails if it reports failures, including incomplete APIs.
Run installed-package checks from the repository root so the wrapper source directory
does not shadow the installed native extension.

### Website and written guides

The website at `crawlingo-docs/` and Markdown docs at `docs/` are separate repositories
in the maintainer's workspace; they are not included in a normal engine clone.
Engine-side guides live in `guides/`. Coordinate a separate website change when a
public API changes.

In the website checkout:

```sh
npm ci
npm run lint
npm run build
npm run dev
```

Use the webpack option documented in that repository when the environment cannot
run Turbopack. Keep code examples aligned with the actual SDK source and describe
unreleased APIs as development source.

## Where changes belong

| Area | Location |
| --- | --- |
| Fetching, sessions and resource limits | `src/engine/` |
| Parsing, selection and extraction | `src/parser/`, `src/selector/`, `src/extraction/` |
| Dataset job, schemas and output | `src/dataset/` |
| Discovery and robots policy | `src/crawl/` |
| Rust integration tests | `tests/` |
| Python wrapper and tests | `sdk/python/` |
| Node wrapper, native binding and tests | `sdk/nodejs/` |
| Runnable fixture | `examples/product_dataset/` |
| User and operator guides | `guides/` |
| CI and contribution forms | `.github/` |

## Pull requests

1. Describe the user problem and the smallest change that solves it.
2. Update relevant examples and add a reproducible check for changed behavior.
3. Run the checks appropriate to your changes. State what was not run.
4. Add an entry under Unreleased in `CHANGELOG.md` for user-visible changes.
5. Open a pull request using the template; keep generated data and local artifacts out.

Use clear titles such as `fix(dataset): preserve row identity when a field is missing`
or `docs: explain checkpoint ownership`. Performance claims need the workload,
machine, build profile, before/after results and correctness checks. Avoid presenting
a synthetic fixture as a general web throughput guarantee.

## Releases

Versions are checked by `node scripts/check-release.cjs`. Maintainers first run the
Release workflow manually to build artifacts without publishing. Publishing a GitHub
Release with the matching `v<version>` tag triggers registry publishing after its
build jobs and environment gates pass. Follow [the release guide](guides/publishing.md)
and [repository setup](.github/REPOSITORY_SETUP.md); a tag push alone does not publish.

## Help and sensitive reports

Use [SUPPORT.md](SUPPORT.md) for usage questions and [SECURITY.md](SECURITY.md) for
vulnerabilities. Do not include credentials or private datasets in public reports.
