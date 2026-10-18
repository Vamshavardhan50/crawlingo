# Security policy

## Supported source

Crawlingo is in alpha development. Reports against the current development branch
and `1.0.0-alpha.1` are assessed on a best-effort basis. Published packages may differ
from the current source. Include the exact package version or commit and build features.
There is no guaranteed response timeline or backport policy.

## Report privately

Use [GitHub private vulnerability reporting](https://github.com/Vamshavardhan50/crawlingo/security/advisories/new)
when it is enabled for this repository. If the private form is unavailable, ask the
maintainer for a private contact route without including exploit details, credentials
or affected private data in a public issue.

Include the affected component, environment, minimal reproduction, impact and any
suggested fix. Maintainers should coordinate a fix and disclosure with the reporter
and ask before publishing their identity. Do not open a public issue containing a
working exploit while private triage is pending.

Private reporting is a GitHub repository setting; adding this file does not enable it.
See [GitHub's configuration guide](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository).

## Scope and application boundaries

Reports may include parser failures on hostile input, credential leakage, unsafe
filesystem handling, unexpected network access or exploitable dependency behavior.
Describe the actual affected configuration and boundary.

Crawlingo makes network requests chosen by its caller. Applications accepting URLs,
selectors, output paths or credentials from other users must enforce their own access
policy and resource limits. The SDK is not an isolation boundary for untrusted jobs.
Run browser workloads with the protections appropriate to the host environment.

Do not include secrets in logs, issue attachments, example files or committed datasets.
For ordinary correctness bugs and feature requests, use the public issue forms.
