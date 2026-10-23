# Documentation release

Update the API snapshot, build the website and Markdown book, then publish a versioned, checksummed documentation bundle.

<a id="ownership"></a>

## Repositories and generated files

| Location | Role | Commit policy |
| --- | --- | --- |
| crawlingo (engine) | Rust, SDK wrappers, inventory generator and Rust API workflow | Commit implementation and generator changes here. |
| crawlingo-docs (website) | Authored guides, source snapshot, reference renderer and docs workflows | Commit website content and generated/sdk-api.json here. |
| docs (workspace folder) | Markdown exports from all website guides and API pages | An older independent checkout; preserve local edits outside named exports. Push website changes from crawlingo-docs. |
| target/doc | Generated rustdoc HTML | Build artifact; do not commit generated HTML. |

<a id="refresh"></a>

## 1. Refresh source inventory

Requires Node and Python 3. PYTHON may point to a specific interpreter. The generator parses SDK source without loading the native package.

For release, commit the engine changes first, regenerate the snapshot from that clean commit, review behavior notes and known issues, and commit the website changes. sourceCommit and sourceFiles hashes bind the docs to exact code. Do not manually clear sourceModified or change hashes to bypass the gate.

### Engine repository root

```shell
cd sdk/nodejs
npm ci
cd ../..
node scripts/export-sdk-api.cjs crawlingo-docs/src/content/generated/sdk-api.json
```

<a id="prepare"></a>

## 2. Build a reviewable bundle

Docs CI runs TypeScript and content/link checks, builds the Next.js standalone server, exports every content page to Markdown, and creates server/Markdown archives plus SHA256SUMS and a docs-release manifest. It does not publish packages to npm, PyPI or crates.io.

Manual Docs release runs prepare artifacts without uploading a GitHub release. The standard CI no longer duplicates deployments, installs unpinned global CLIs, swallows build failures or uploads private environment files.

### Website checkout

```shell
npm ci
npm run lint
npm run export:guides -- ../docs
npm run build -- --webpack
npm run package:docs
```

<a id="publish"></a>

## 3. Publish matching documentation assets

- Use a clean engine snapshot and a website package version matching snapshot.version.
- Create the website tag vVERSION at the reviewed website commit. Use a prerelease flag for an alpha/beta version.
- Publishing that GitHub release starts Docs release. Its gate checks the tag/version, clean snapshot, source hashes against the recorded engine commit, and regenerated SDK inventory.
- The job rebuilds the exact website ref, uploads artifacts, and attaches the archives/checksums after all required checks succeed.
- An upload failure does not roll back another uploaded file. Inspect the release assets; do not silently overwrite an existing release archive.

- [Website workflows](https://github.com/Vamshavardhan50/crawlingo-docs/actions)

<a id="hosting"></a>

## 4. Deploy the website separately

GitHub documentation releases distribute artifacts. Hosting is a separate step: use Vercel/Netlify Git integration for the reviewed website commit, or deploy the standalone server archive.

For Vercel, import the website repository, use Node 22, npm ci and npm run build, and select the reviewed production branch. The Git integration owns preview/production deployment; no VERCEL_TOKEN is needed in the documentation artifact workflow.

For a standalone archive: extract it, set HOSTNAME=0.0.0.0 and PORT as needed, then run node server.js with Node 22+. The archive includes traced runtime dependencies, public assets and static chunks. Place an HTTPS reverse proxy in front of it.

Do not mark a release as deployed until the hosting provider reports success and the target URLs load. Updating local docs alone does not update the public website.

- [Hosting guide](../docs-hosting.md)

<a id="rustdoc"></a>

## 5. Release the Rust API reference

The engine workflow rustdoc.yml builds the browser-enabled public crate reference. The SDK release requires its rust-docs artifact and adds the versioned archive to SHA256SUMS and release-manifest.json.

Rust docs and the website must describe the same intended engine version. A failed rustdoc job blocks SDK release artifact preparation. Fix broken links in source instead of suppressing the gate.

- [Rust API reference](rust.md)

<a id="troubleshooting"></a>

## Failure and recovery

| Failure | Action |
| --- | --- |
| Snapshot marked modified | Commit the engine source, regenerate the snapshot, then commit the docs. |
| Source hash or inventory mismatch | Regenerate from the recorded engine commit; review signatures and behavioral notes. |
| Tag/version mismatch | Align website version, snapshot version and release tag before publication. |
| Broken route/anchor | Fix the link or preserve an alias; do not suppress content checking. |
| Native rustdoc build failure | Install the C/C++ toolchain, CMake and libclang; keep Cargo.lock. |
| Missing hosting configuration | Configure the hosting provider Git integration or deploy the standalone archive. A successful artifact job is not a live deployment. |
