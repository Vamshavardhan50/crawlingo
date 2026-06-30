# 20_PRODUCTION_CHECKLIST.md

This checklist outlines the verification steps required to deploy Crawlingo in a production environment.

---

## 1. Network & Resource Limits

- [ ] **Connection Limits:** Ensure `ConnectionPoolConfig` limits match the max file descriptor limits (`ulimit -n` on Linux) of your server.
- [ ] **DNS Caching Configuration:** Verify the DNS resolver cache lifetime in `DnsCacheResolver` matches your target domain TTLs to minimize lookups without using stale IPs.
- [ ] **Proxy Availability:** Validate proxy list strings match the format: `http://user:pass@host:port`.

---

## 2. Rate Limiting & Stealth Compliance

- [ ] **Host Rate Limits:** Configure conservative requests-per-second (RPS) limits to prevent triggering web application firewalls (WAF).
- [ ] **JA3 Profile Matches:** Test that the configured TLS JA3 headers match the active User-Agent string to prevent client fingerprint mismatches.

---

## 3. Database & Caching Setup

- [ ] **Sled DB Permissions:** Secure the `.crawlingo` database directory with process-exclusive write permissions.
- [ ] **Locking Safeties:** Confirm no two processes open the same `.crawlingo` database folder concurrently to prevent Sled lock crashes.

---

## 4. Diagnostics & Observability

- [ ] **Tracing Levels:** Configure `tracing-subscriber` to write error and warning logs to standard error.
- [ ] **Unhandled FFI Errors:** Verify all Rust `Result` types are caught at the FFI boundary to prevent segmentation faults in JavaScript or Python.
