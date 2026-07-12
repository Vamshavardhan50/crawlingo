# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Yes |
| < 0.1   | ❌ No  |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities via public GitHub issues.**

Instead, use one of these private channels:

### GitHub Security Advisories (Preferred)

1. Go to [Security → Report a Vulnerability](https://github.com/Vamshavardhan50/crawlingo/security/advisories/new)
2. Describe the vulnerability and include:
   - Affected version(s)
   - Steps to reproduce
   - Potential impact assessment
   - Suggested fix (optional)

### Response Timeline

| Stage | Target Timeline |
|-------|----------------|
| Initial response | 48 hours |
| Triage & validation | 5 business days |
| Fix & private patch | 30 days |
| Public disclosure | After patch is released |

We follow [Coordinated Vulnerability Disclosure](https://vuls.cert.org/confluence/display/CVD/What+is+CVD) and will credit reporters in our release notes unless they prefer to remain anonymous.

## Scope

**In scope:**
- Credential exposure or leakage in logs/output
- Arbitrary code execution via malformed HTML or selectors
- SSRF via proxy configuration
- Dependency vulnerabilities with exploitable vectors

**Out of scope:**
- Rate limiting or abuse by users of the library
- Bypassing bot detection (that's a feature, not a bug)
- Denial of service of third-party sites using Crawlingo
