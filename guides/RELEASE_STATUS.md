# Release status and next SDK release

Last audited: **2026-10-18**. Local changes are not published packages.

## What users can install today

| Channel | Observed version | What was verified |
| --- | --- | --- |
| GitHub | `v1.0.0-alpha.1`, published 2026-07-19 | No release body or uploaded assets; `prerelease` is incorrectly false for an alpha |
| npm latest | `0.1.2` | Archive contains only the Windows x64 native addon; no DatasetJob wrapper |
| npm alpha | `1.0.0-alpha.1` | Same native platform limitation; no DatasetJob wrapper |
| PyPI | `0.1.2` | One CPython 3.14 Windows x64 wheel and one source archive |
| crates.io | `0.1.2` | One published version, not yanked |

Sources: [GitHub release metadata](https://api.github.com/repos/Vamshavardhan50/crawlingo/releases),
[npm metadata](https://registry.npmjs.org/crawlingo),
[PyPI metadata](https://pypi.org/pypi/crawlingo/json), and
[crate metadata](https://crates.io/api/v1/crates/crawlingo).
The npm archives were listed without executing their code. Full installation and
runtime behavior of those old packages was not retested.

Local evidence is preserved in `target/published-release-audit-20261018.json`,
`target/published-node-alpha1.tgz` and `target/published-node-012.tgz`.

## Prepared next version

**Rust/npm: `1.0.0-alpha.2`; Python: `1.0.0a2`. Unpublished.**

The current dataset workflow must be built from source until this version is
released. Existing immutable registry versions must not be overwritten or reused.

### Intended artifact matrix

| Artifact | Build targets | Release checks |
| --- | --- | --- |
| Rust crate | Source package | Locked dependency resolution and Cargo package compilation |
| CLI | Linux x64 GNU, macOS x64/ARM64, Windows x64 | Host build, archive with license |
| npm | Node.js 22+; the same four native targets | Native SDK/API checks, complete addon set, clean consumer import |
| Python | CPython 3.8–3.13 where the wheel builder supports the platform; Linux x64/ARM64, macOS x64/ARM64, Windows x64 | Wheel import and dataset checks, source distribution metadata |

These are intended targets. The revised hosted matrix has **not** passed yet.
Current PyO3 does not support Python 3.14; the package metadata now rejects it
instead of relying on a forward-compatibility override.

## Publication sequence

1. Commit the engine/SDK changes and both Cargo lockfiles. Website changes live
   in a separate repository and need their own deployment.
2. Run CI and the Release workflow manually on the same reviewed commit. Manual
   dispatch builds artifacts without registry publication.
3. Inspect the `release-assets` workflow artifact: CLI archives, npm tarball,
   Python wheels/source, Rust crate, `release-manifest.json` and `SHA256SUMS`.
4. Resolve every failing runtime/platform check. Review the documented local
   dataset example, cancellation and resume evidence before publishing.
5. Configure the protected `npm`, `pypi` and `crates-io` environments and their
   publishing credentials/identities as described in [publishing.md](publishing.md).
6. Tag that commit `v1.0.0-alpha.2` and publish a GitHub **prerelease** using the
   [prepared notes](../releases/v1.0.0-alpha.2.md). The workflow rebuilds and gates
   all packages before any upload. npm prereleases use `next`.
7. Confirm the installed version from each registry. The final workflow job
   attaches the reviewed artifacts to the GitHub release after registry jobs succeed.

Publication across three registries is not atomic. If one upload fails after
another succeeds, inspect which versions exist before rerunning. Never silently
replace a release asset or claim all registries are synchronized after a partial run.

## Existing GitHub release repair

Mark `v1.0.0-alpha.1` as a prerelease and add factual notes describing its Windows
npm artifact and missing dataset API. Do not attach the new alpha.2 binaries to
that old tag. These hosted changes have not been applied locally.

## Remaining technical gates

Local preparation checks on 2026-10-19 passed Rust release-profile compilation,
Node TypeScript checks, documentation production build and Python source-package
metadata validation. These do not replace runtime or cross-platform acceptance.

- Run the revised platform matrix and install packaged artifacts in fresh consumers.
- Rebuild both native bindings after the latest Rust allocation changes.
- Measure the same controlled workloads before making any new speed claim.
- Continue fault-recovery coverage and representative multi-record/multi-origin
  workloads in [SCALE_RESULTS.md](../SCALE_RESULTS.md).
