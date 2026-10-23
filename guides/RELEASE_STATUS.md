# Release status and next SDK release

Last updated: **2026-10-23** (registry versions audited October 22). Local changes are not published packages.

## What users can install today

| Channel | Observed version | What was verified |
| --- | --- | --- |
| GitHub | `v1.0.0-alpha.1`, published 2026-07-19 | Prerelease label and factual package limitations corrected October 23; no uploaded assets |
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
| CLI | Linux x64 GNU (glibc 2.35+), macOS x64/ARM64, Windows x64 | Host build, archive with license |
| npm | Node.js 22+; the same four native targets | Native SDK/API checks, Debian 12 consumer check, complete addon set, clean consumer import |
| Python | CPython 3.8–3.13 where the wheel builder supports the platform; Linux x64/ARM64 (glibc 2.28+), macOS x64/ARM64, Windows x64 | Wheel import and dataset checks, source distribution metadata |

These are intended targets. The revised hosted matrix has **not** passed yet.
The alpha.2 artifact matrix and package metadata target CPython 3.8–3.13.
The patched PyO3 dependency does not by itself certify additional Python versions.

## Publication sequence

1. Commit the engine/SDK changes and both Cargo lockfiles. Website changes live
   in a separate repository and need their own deployment.
2. Run CI and the Release workflow manually on the same reviewed commit. Manual
   dispatch builds artifacts without registry publication.
3. Inspect the `release-assets` workflow artifact: CLI archives, npm tarball,
   Python wheels/source, Rust crate, `release-manifest.json` and `SHA256SUMS`.
4. Resolve every failing runtime/platform check. Review the documented local
   dataset example, cancellation and resume evidence before publishing.
5. Configure registry publishing identities as described in [publishing.md](publishing.md).
   GitHub environments `npm`, `pypi` and `crates-io` were created on October 22,
   restricted to main and release tags. Registry-side trusted publisher setup
   was confirmed for PyPI on October 23; crates.io setup still requires the
   package owner's account. An existing repository `NPM_TOKEN`
   is available as a fallback; its validity has not been confirmed.
6. Tag that commit `v1.0.0-alpha.2` and publish a GitHub **prerelease** using the
   [prepared notes](../releases/v1.0.0-alpha.2.md). The workflow rebuilds and gates
   all packages before any upload. npm prereleases use `next`.
7. Confirm the installed version from each registry. The final workflow job
   attaches the reviewed artifacts to the GitHub release after registry jobs succeed.

Publication across three registries is not atomic. If one upload fails after
another succeeds, inspect which versions exist before rerunning. Never silently
replace a release asset or claim all registries are synchronized after a partial run.

## Existing GitHub release repair

Completed October 23: `v1.0.0-alpha.1` is now marked as a prerelease and has factual
notes describing its Windows npm artifact and missing dataset API. The historical
tag and package contents were preserved. No alpha.2 binaries were attached to it.

## Release preparation evidence — October 23

- Removed the unused Chromium downloader feature that broke browser compilation.
- The manual release build passed the dependency audit; Linux core tests, browser
  feature compilation and CLI packaging passed at commit `f6882fd`.
- CI passed Rust checks on Linux/macOS, Node 22 on Linux/Windows, and Python
  3.12/3.13 on Linux/Windows at `f6882fd`. Remaining jobs were still running when
  this entry was written; this is not a completed matrix claim.
- A Python source distribution from `1de5760` rebuilt into a CPython 3.12 Windows
  wheel independently of the checkout. Its installed import and dataset/resume
  checks passed. The later `f6882fd` change adjusts Linux wheel tooling and docs.
- The first Linux wheel build exposed an outdated Clang in manylinux2014. The
  corrected configuration uses manylinux_2_28 for x64 and ARM64.
- Inspection of the first Linux Node addon found glibc 2.38 symbols. Its release
  builder was changed to Ubuntu 22.04 and a Debian 12 consumer check was added;
  the revised artifact still needs its hosted result.

The new SDK versions remain unpublished. PyPI's saved GitHub publisher was
verified on October 23 with owner `Vamshavardhan50`, repository `crawlingo`,
workflow `release.yml` and environment `pypi`. crates.io publisher setup remains
unconfirmed. The Linux wheel build now compiles and repairs successfully, but
its first test invocation used the repository root instead of the SDK directory;
the workflow now uses cibuildwheel's `{package}` path.

## Remaining technical gates

Local preparation checks on 2026-10-19 passed Rust release-profile compilation,
Node TypeScript checks, documentation production build and Python source-package
metadata validation. These do not replace runtime or cross-platform acceptance.

- Run the revised platform matrix and install packaged artifacts in fresh consumers.
- Rebuild both native bindings after the latest Rust allocation changes.
- Measure the same controlled workloads before making any new speed claim.
- Continue fault-recovery coverage and representative multi-record/multi-origin
  workloads in [SCALE_RESULTS.md](../SCALE_RESULTS.md).
