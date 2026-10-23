# Publish the SDKs

Maintainer instructions for building release artifacts and distributing Crawlingo through npm, PyPI and crates.io.

> Current development source. New dataset APIs require a matching source build; published alpha packages may differ.

## Prepare one version across three packages

The prepared next version is 1.0.0-alpha.2 (Python 1.0.0a2), currently unpublished. The October 18 audit found npm latest 0.1.2 plus alpha.1, and PyPI/crates.io 0.1.2. The inspected npm archives contain only a Windows x64 addon and no DatasetJob. Build the new source until the next release is published. Keep Cargo.toml, sdk/nodejs/native/Cargo.toml, sdk/nodejs/package.json, sdk/nodejs/package-lock.json sdk/python/pyproject.toml and sdk/python/crawlingo/__init__.py aligned. The version checker normalizes Python prerelease notation.

Run the Release workflow manually first: workflow_dispatch builds and checks artifacts without uploading packages to registries. Commit both Cargo lockfiles. The workflow prepares all SDK and CLI artifacts, verifies the complete set and creates SHA256SUMS and release-manifest.json before allowing registry uploads. Review a manual run before publishing the matching GitHub prerelease. The tag and prerelease flag must match the package version. Final asset upload runs after all three registries succeed; registry publication is not atomic.

### Repository root

```shell
node scripts/check-release.cjs
cargo test --locked --lib --tests
cd sdk/nodejs
npm ci
npm run build
npm test
node --test tests/dataset_job.test.cjs
```

- [Release workflow](https://github.com/Vamshavardhan50/crawlingo/blob/main/.github/workflows/release.yml)
- [Remaining release gates](https://crawlingo-docs.vercel.app/roadmap)

## One-time registry setup

Sign in as an owner of the existing `crawlingo` package on each registry. GitHub
environments alone do not give GitHub permission to publish a package. Save a
GitHub Actions trusted publisher in each registry with these exact values:

| Field | npm | PyPI | crates.io |
| --- | --- | --- | --- |
| GitHub owner | `Vamshavardhan50` | `Vamshavardhan50` | `Vamshavardhan50` |
| Repository | `crawlingo` | `crawlingo` | `crawlingo` |
| Workflow filename | `release.yml` | `release.yml` | `release.yml` |
| Environment | `npm` | `pypi` | `crates-io` |

Use the workflow filename only, without `.github/workflows/`. The environment
names must match the corresponding publication job in this repository.

### PyPI account steps

1. Open [Crawlingo publishing settings](https://pypi.org/manage/project/crawlingo/settings/publishing/)
   and sign in. Alternatively, choose **Your projects → crawlingo → Manage → Publishing**.
2. Select **GitHub Actions** and enter the PyPI column above.
3. Choose **Add** and check that the publisher appears in the project's publisher list.
4. If a token has been shared in chat or a public issue, revoke it under the account's
   API token settings. Trusted publishing does not require a replacement token in GitHub.

See [PyPI's setup instructions](https://docs.pypi.org/trusted-publishers/adding-a-publisher/).

### crates.io account steps

Open [Crawlingo on crates.io](https://crates.io/crates/crawlingo), sign in as a crate
owner and add a GitHub Actions trusted publisher in the crate's settings. Enter
the crates.io column above and confirm the saved publisher matches the repository,
workflow and environment. See the [crates.io trusted publishing guide](https://crates.io/docs/trusted-publishing).

### npm account steps

Open the `crawlingo` package settings on npm, find **Trusted Publisher**, select
**GitHub Actions** and enter the npm column above. Enable direct **npm publish**
under allowed actions: this workflow does not use staged publishing. Save the
publisher and verify the saved configuration. See [npm's setup instructions](https://docs.npmjs.com/trusted-publishers/).

### Run a release

1. In GitHub **Actions → Release**, use **Run workflow** on the reviewed branch.
   This builds artifacts without uploading them to the registries.
2. Wait for all platform and package jobs to pass. Download `release-assets` and
   review its manifest, checksums and package contents.
3. Create the `v1.0.0-alpha.2` tag on that exact reviewed commit and publish a
   GitHub release with **Set as a pre-release** enabled. Use the prepared release
   notes after replacing the draft notice with the verified artifact matrix.
4. Check all three publication jobs, then install each exact registry version in
   a clean consumer. Update [release status](RELEASE_STATUS.md) with the results.

If a registry rejects authentication, compare its saved publisher against the
table before changing the workflow. For a partial publication, use GitHub's
**Re-run failed jobs** after fixing the failed registry's settings; rerunning all
jobs would attempt to upload immutable versions that already exist. A build or
package correction needs a new version and a complete build.

## Node.js → npm

The package includes JavaScript, declarations and native addons in dist/. The release matrix builds Linux x64 GNU, macOS Intel, macOS ARM64 and Windows x64 addons. These are build targets, not a claim that all platforms have passed the new release.

Configure npm trusted publishing for owner `Vamshavardhan50`, repository `crawlingo`, workflow `release.yml` and environment `npm`, with permission for direct npm publish. The workflow uses Node 22 and npm 11, with id-token: write for publication. The existing `NPM_TOKEN` repository secret is supplied as a fallback. Prereleases publish under `next`; stable versions use `latest`. Install a prerelease with `npm install crawlingo@next`.

### Inspect a local artifact

```shell
cd sdk/nodejs
npm run build
npm pack --dry-run
# Verify dist/index.js, declarations and the native binary list.
# Local builds contain only the addons you built; CI assembles the matrix.
```

- [npm trusted publisher setup](https://docs.npmjs.com/trusted-publishers/)

## Python → PyPI

The workflow targets Python 3.8–3.13 wheels and a source distribution. Python 3.14 is outside this release matrix; the old published Windows 3.14 wheel is a different artifact. Linux wheels use `manylinux_2_28` for x64 and ARM64 and require glibc 2.28 or newer; Alpine/musl wheels are not built. This image provides a newer Clang than the old manylinux2014 distro packages, required by the native TLS build. Wheel installation must be checked in a clean environment with the Python version and architecture you intend to support. Local validation uses CPython 3.12 on Windows x64.

Configure a PyPI trusted publisher for the owner, repository, release.yml and environment pypi. GitHub supplies the publishing identity; a PyPI password is not stored in the repository. TestPyPI uses its own project and publisher configuration if you add a rehearsal upload.

### Build locally

```shell
cd sdk/python
python -m pip install maturin
maturin build --locked --release --out dist
maturin sdist --out dist
# In a fresh virtual environment, install the exact generated wheel path.
# Then run: python -c "from crawlingo import DatasetJob; print(DatasetJob)"
```

- [PyPI publisher configuration](https://docs.pypi.org/trusted-publishers/adding-a-publisher/)

## Rust → crates.io

Inspect the crate contents and let Cargo compile the packaged source before uploading. The workflow runs a dry run, then uses `rust-lang/crates-io-auth-action@v1` to obtain a short-lived publishing token in the `crates-io` environment for published releases. No long-lived crate token is required.

In the Crawlingo crate settings on crates.io, add a GitHub trusted publisher with owner `Vamshavardhan50`, repository `crawlingo`, workflow `release.yml`, and environment `crates-io`. For PyPI, use the same owner/repository/workflow and environment `pypi` in the project's Publishing settings. Do not paste registry tokens into issues or chat.

### Repository root

```shell
cargo package --locked --list
cargo publish --locked --dry-run
# After reviewing the artifact and signing in as a crate owner:
# cargo publish
```

- [Cargo publishing reference](https://doc.rust-lang.org/cargo/reference/publishing.html)

## Confirm the installed release

- Install from each registry in a fresh project, not from this checkout.
- Run the local product example and verify accepted, duplicate and rejected counts.
- Repeat the job to confirm resume; check the package reports the intended version.
- Publish release notes listing tested OS/runtime combinations and link the scale methodology.
- A registry upload cannot replace the contents of an existing version. Fix a bad artifact with a new version.

- [Runnable product example](https://crawlingo-docs.vercel.app/examples)
- [Deploy the documentation](https://crawlingo-docs.vercel.app/docs-hosting)
