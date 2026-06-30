# 11_SECURITY_REVIEW.md

This document presents a security review of Crawlingo's core network transport, fingerprint caching, and FFI wrappers.

---

## 1. Network Transport & SSL/TLS Security

- **Library Underlying:** Networking runs on `wreq` (which wraps `rquest` / `hyper` and `rustls` or native-tls).
- **SSL Verification:** SSL/TLS certificate verification is enabled by default. Users cannot inadvertently run in insecure mode unless they pass custom settings.
- **ALPN & JA3 Fingerprinting:** The `Stealthy` tier configures ALPN and JA3 TLS signatures to impersonate browser request styles. Care must be taken that these profiles match modern user-agent headers to avoid triggering fraud detection filters.

---

## 2. Proxy Authentication & Credentials Exposure

- **Proxy Configuration:** Session-level proxies support authentication format: `http://user:pass@host:port`.
- **Environment Exposure:** The Python and Node.js wrappers must secure memory boundaries to prevent credentials from leaking into debug stack traces or console logging hooks.
- **Recommendation:** Intercept proxy strings and scrub credential fields before writing log outputs.

---

## 3. Fingerprint Cache (Sled) Access Control

- **File Permission Security:** Sled stores fingerprint keys inside a local database directory (`.crawlingo/fingerprints`).
- **File System Locking:** Sled uses lockfiles to prevent concurrent processes from corrupting files.
- **Data Leakage Risks:** The fingerprint DB stores page structures and element tags. Although no full page body HTML is stored, cached paths might reveal sensitive URLs or internal DOM details.
- **Recommendation:** Encrypt or restrict permissions on the `.crawlingo` directory to the running process owner.

---

## 4. FFI Memory Boundaries & Sandbox Safety

- **Memory Leak Risks:** PyO3 and NAPI-RS bridge reference-counted pointers. If Javascript or Python processes drop a `Page` but Rust retains reference cycles, memory leaks can occur.
- **Threading Safety:** The Python GIL is explicitly released via `py.allow_threads` during long-running network requests to avoid blocking the Python thread. Thread-safety boundaries are enforced by wrapping internal structs in `Arc<RwLock<T>>`.

---

## 5. Dependency Audit

Regular scans should be performed using:
- **Rust:** `cargo audit` to inspect dependencies against the Advisory Database.
- **Python:** `pip-audit` or Dependabot.
- **Node.js:** `npm audit`.
- **Warning:** Direct dependency on experimental release-candidate libraries (`wreq = "6.0.0-rc.29"`) requires careful monitoring to quickly apply upstream security fixes.
