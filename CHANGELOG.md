# Changelog

Published versions below were checked against registry metadata on 2026-10-18.
See [release status](guides/RELEASE_STATUS.md) for artifact and platform details.

## Unreleased — 1.0.0-alpha.2

Python uses the equivalent version `1.0.0a2`. This source has not been published.

### Dataset generation

- Added Rust, Python, Node.js and CLI URL-file jobs with bounded processing.
- Added scoped records, strict field validation and exact record/URL deduplication.
- Added JSONL/CSV shards, cancellation, single-writer state and local resume.
- Added a reproducible product catalog and documented million-record fixture evidence.
- Reduced configuration clones, dedup buffer copies and JSONL temporary allocations.
  No new measured speedup is claimed.

### SDKs and releases

- Aligned manifests for the next version and retained Cargo lockfiles for release builds.
- Declared the current Python 3.8–3.13 and Node.js 22+ source requirements.
- Added an artifact gate, CLI archives, checksums and a release manifest.
- Separated artifact preparation from registry publication and checked prerelease flags.
- Removed generated binaries/reports from source and fixed SDK README links.

### Documentation and project maintenance

- Replaced unsupported performance comparisons with scoped measurements.
- Added API references, usage/deployment guides and copying controls in the website.
- Added issue forms, contribution/security policies, code ownership and dependency updates.
- Replaced obsolete roadmap-as-release entries and unsafe blanket database-deletion advice.

Hosted CI, packaged SDK installation and new performance measurements remain release gates.

## 1.0.0-alpha.1 — 2026-07-19 GitHub release

- A GitHub release and npm alpha package exist for this version.
- The GitHub release has no uploaded artifacts or release notes and is incorrectly
  marked as stable; a hosted metadata correction is pending.
- The inspected npm tarball includes a Windows x64 native addon and does not export
  the new DatasetJob API. It does not establish cross-platform SDK support.

## 0.1.2 — published packages

- Published to npm, PyPI and crates.io.
- The PyPI release provides a CPython 3.14 Windows x64 wheel and source archive.
- The inspected npm archive includes a Windows x64 native addon.

The previous `1.0.0-beta.1` heading was an internal roadmap log, not a verified
published release. Its feature/coverage claims are superseded by source references,
release metadata and the evidence in [SCALE_RESULTS.md](SCALE_RESULTS.md).
