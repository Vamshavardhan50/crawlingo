# GitHub repository setup

## Repository identity

- Engine and SDKs: `Vamshavardhan50/crawlingo`.
- Website: `Vamshavardhan50/crawlingo-docs`, a separate repository.
- Product: a Rust-powered dataset generator for developers.
- Primary workflow: URL file → explicit schema → validation → deduplication →
  resumable JSONL/CSV shards.

Use the reviewed description in `repository.json` and topics in `topics.json`.
The current engine source supports Rust, Python and Node.js; do not advertise Go
or a hosted REST API. Keep package and website claims aligned with released artifacts.

## Prepared in source

| File | Purpose |
| --- | --- |
| `README.md` | Product purpose, use cases, first run, SDK examples and evidence |
| `CONTRIBUTING.md` | Correct build paths, checks and release process |
| `SUPPORT.md` | Entry points and useful bug-report information |
| `SECURITY.md` | Private reporting guidance and current alpha scope |
| `CODE_OF_CONDUCT.md` | Community standards and reporting guidance |
| `.github/ISSUE_TEMPLATE/` | Bug, feature and documentation forms |
| `.github/PULL_REQUEST_TEMPLATE.md` | Reviewable implementation and verification summary |
| `.github/CODEOWNERS` | Default maintainer review ownership |
| `.github/dependabot.yml` | Weekly Cargo, Python, npm and Actions update PRs |
| `.github/release.yml` | Generated release-note categories |
| `.github/workflows/ci.yml` | Source checks, native SDK checks and strict `CI Pass` summary |
| `.github/workflows/release.yml` | Artifact builds and gated registry publication |

Files take effect after they reach the relevant GitHub branch. Preparing them
locally does not change GitHub settings, publish packages or run hosted CI.

## Apply the About description and topics

After reviewing the exact JSON and authenticating a GitHub CLI with access to the
repository, run these commands from the engine checkout:

```sh
gh api --method PATCH repos/Vamshavardhan50/crawlingo --input .github/repository.json
gh api --method PUT repos/Vamshavardhan50/crawlingo/topics --input .github/topics.json
```

The first command updates only the description and homepage. The second replaces
the repository's topic list with the reviewed list. Neither changes visibility.
The description, homepage and topics were applied through the GitHub API on
2026-10-19 using the existing repository authentication. The commands above remain
available for future updates. Repository protection and release environments are
separate settings and are not implied by this metadata update.

## Review and merge settings

1. Push the prepared work on a review branch and open a pull request. Keep the
   engine and website changes in their respective repositories.
2. Run the new CI workflow and inspect its individual jobs. The gate requires
   successful results; skipped or cancelled dependencies do not count as success.
3. After the check has appeared on GitHub, require `CI Pass` on the default branch.
   Check for any old required status named `✅ CI Pass` when changing the rule.
4. Require pull requests and resolution of review conversations. Block force pushes
   and branch deletion on the default branch. Add required approvals and code-owner
   review when a second active reviewer is available; a sole author cannot approve
   their own PR.
5. Prefer squash merge and automatic deletion of merged feature branches if that
   matches the maintainers' working style.

These are intended settings, not a statement that protection has been enabled.
See [GitHub's protected-branch guide](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches).

## Security and contribution settings

- Enable private vulnerability reporting before advertising the private form as
  available. `SECURITY.md` itself does not enable the feature.
- Keep Actions permissions read-only by default. Registry publishing jobs request
  their own narrowly scoped permissions and use protected environments.
- Enable dependency alerts and review Dependabot pull requests with CI; do not
  automatically merge native dependency upgrades solely because a build passes.
- Keep secrets out of repository files, issue reports and example datasets.

Reference: [private vulnerability reporting](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository)
and [issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms).

## Release settings

Configure the `npm`, `pypi` and `crates-io` environments used by the release workflow.
Set npm/PyPI trusted publishers to the exact repository and workflow, and supply the
crate token through the crates-io environment. Follow [publishing.md](../guides/publishing.md)
for the artifact and registry checks. Review the broad wheel/native matrix before
creating a published GitHub Release; manual dispatch builds without uploading.
The next prepared version is `1.0.0-alpha.2` / Python `1.0.0a2`. All packages and CLI
archives now pass one artifact gate before publication, with SHA-256 checksums.
See [the release audit](../guides/RELEASE_STATUS.md) and [draft notes](../releases/v1.0.0-alpha.2.md).
The npm trusted publisher must allow direct `npm publish`, which the workflow uses.

The current source's cross-platform CI and revised release workflow still need a
hosted run. The standalone Python API suite can expose incomplete APIs; do not hide
those failures or call the current alpha production-ready before resolving them.

## Website repository

The website needs its own CI, deployment connection and domain configuration.
Use the included Docs CI workflow there and the [hosting guide](../guides/docs-hosting.md).
Publishing the engine README will not deploy the website. Verify the landing page,
`/guides`, `/api-reference`, direct guide URLs and a missing-route response after deployment.
