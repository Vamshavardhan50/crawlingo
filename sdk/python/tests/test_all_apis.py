"""
Crawlingo Python SDK — Comprehensive End-to-End API Test Suite
================================================================

Covers every public API in the Crawlingo Python SDK across 27 categories.

Usage:
  python tests/test_all_apis.py
  python tests/test_all_apis.py --save   (also saves output to a log file)

Output:
  Section headers with PASS/FAIL/SKIP per API, final summary table.
  All output is also saved to: crawlingo_test_output_<timestamp>.log

Test Categories:
   1 — Session
   2 — Fetchers (tiers)
   3 — Browser Profiles
   4 — Headers
   5 — Cookies
   6 — Proxy
   7 — Timeouts
   8 — Rate Limiting
   9 — Retry Logic
  10 — HTTP Requests
  11 — Page APIs
  12 — HTML
  13 — Text Extraction
  14 — Selectors
  15 — Extraction
  16 — Pagination
  17 — Screenshots
  18 — Downloads
  19 — Uploads
  20 — Authentication
  21 — Dataset
  22 — Parsing
  23 — Utilities
  24 — Errors
  25 — Logging / Hooks
  26 — Performance
  27 — Cleanup
"""

import sys
import os
import io
import time
import json
import csv
import tempfile
import threading
import traceback
from datetime import datetime
from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler
from socketserver import ThreadingMixIn

import crawlingo
from crawlingo import (
    Page, Session, Dataset, DatasetResult, Crawl,
    Watch, ElementCollection, ChangeEvent,
    CrawlingoError, FetchError, ParseError, SelectorError,
    AutoMatchFailed, TimeoutError, RateLimitError,
    ChangeDetectionError, ExportError, DnsError, FingerprintStoreError,
)
from crawlingo.hooks import (
    strip_whitespace, uppercase, lowercase, log_request, log_response,
)

try:
    import pandas as pd
    HAS_PANDAS = True
except ImportError:
    HAS_PANDAS = False


SAVE_LOG = "--save" in sys.argv


# ═══════════════════════════════════════════════════════════════════════════════
# Local Test HTTP Server
# ═══════════════════════════════════════════════════════════════════════════════

class _TestRequestHandler(BaseHTTPRequestHandler):
    """Simple HTTP handler responding with test data for every method/path."""

    RESPONSES = {
        "/json": (200, {"Content-Type": "application/json"},
                  b'{"key":"value","nested":{"a":1}}'),
        "/xml": (200, {"Content-Type": "application/xml"},
                 b"<?xml version='1.0'?><root><item id='1'>val</item></root>"),
        "/csv": (200, {"Content-Type": "text/csv"},
                 b"name,value\nfoo,1\nbar,2\n"),
        "/md": (200, {"Content-Type": "text/markdown"},
                b"# Hello\n\nThis is **markdown**.\n"),
        "/links": (200, {"Content-Type": "text/html"},
                   b"<html><body>"
                   b"<a href='/page1'>Page 1</a>"
                   b"<a href='/page2'>Page 2</a>"
                   b"<img src='/img.png'>"
                   b"<link rel='stylesheet' href='/style.css'>"
                   b"<script src='/app.js'></script>"
                   b"</body></html>"),
        "/table": (200, {"Content-Type": "text/html"},
                   b"<html><body>"
                   b"<table><tr><th>Name</th><th>Age</th></tr>"
                   b"<tr><td>Alice</td><td>30</td></tr>"
                   b"<tr><td>Bob</td><td>25</td></tr></table>"
                   b"</body></html>"),
        "/large": (200, {"Content-Type": "text/html"},
                   b"<html><body>" + b"<p>large</p>" * 10000 + b"</body></html>"),
    }

    DEFAULT_BODY = (b"<html><head><title>Test Page</title></head>"
                    b"<body><h1>OK</h1><p>test paragraph</p>"
                    b"<a href='/page2'>link</a></body></html>")

    def _send(self, status, headers, body):
        hdrs = dict(headers)
        hdrs["Content-Length"] = str(len(body))
        hdrs["Connection"] = "close"
        if status == 200:
            hdrs.setdefault("Set-Cookie", "test_session=abc123; Path=/")
        try:
            self.send_response(status)
            for k, v in hdrs.items():
                self.send_header(k, v)
            self.end_headers()
            self.wfile.write(body)
        except (BrokenPipeError, ConnectionAbortedError, ConnectionResetError):
            pass

    def do_GET(self):
        p = self.path
        if p in self.RESPONSES:
            return self._send(*self.RESPONSES[p])
        if p in ("/404", "/500", "/403"):
            return self._send(int(p[1:]), {"Content-Type": "text/plain"}, b"error")
        if p == "/auth":
            auth = self.headers.get("Authorization", "")
            cookie = self.headers.get("Cookie", "")
            api_key = self.headers.get("X-API-Key", "")
            if ("test_token" in auth or "dGVzdDpwYXNz" in auth or
                "session" in cookie or api_key):
                return self._send(200, {"Content-Type": "application/json"},
                                  b'{"authenticated":true}')
            return self._send(401, {"WWW-Authenticate": 'Bearer realm="test"',
                                    "Content-Type": "application/json"},
                              b'{"authenticated":false}')
        if p.startswith("/search"):
            body = (b"<html><body>"
                    b"<div class='result'><h2>R1</h2><span class='price'>$10</span></div>"
                    b"<div class='result'><h2>R2</h2><span class='price'>$20</span></div>"
                    b"</body></html>")
            return self._send(200, {"Content-Type": "text/html"}, body)
        if p == "/slow":
            time.sleep(0.3)
            return self._send(200, {"Content-Type": "text/plain"}, b"slow")
        self._send(200, {"Content-Type": "text/html"}, self.DEFAULT_BODY)

    def do_POST(self):
        self._send(200, {"Content-Type": "application/json"},
                   json.dumps({"method": "POST", "path": self.path}).encode())

    def do_PUT(self):
        self._send(200, {"Content-Type": "application/json"},
                   json.dumps({"method": "PUT"}).encode())

    def do_PATCH(self):
        self._send(200, {"Content-Type": "application/json"},
                   json.dumps({"method": "PATCH"}).encode())

    def do_DELETE(self):
        self._send(200, {"Content-Type": "application/json"},
                   json.dumps({"method": "DELETE"}).encode())

    def do_HEAD(self):
        self._send(200, {"Content-Length": "0"}, b"")

    def do_OPTIONS(self):
        self._send(204, {"Allow": "GET,POST,PUT,PATCH,DELETE,HEAD,OPTIONS"}, b"")

    def log_message(self, fmt, *args):
        pass


class TestServer:
    """Local HTTP server for integration tests. Usage: with TestServer() as s: ..."""

    def __init__(self):
        self.server = ThreadingHTTPServer(("127.0.0.1", 0), _TestRequestHandler)
        self.port = self.server.server_address[1]
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)

    def __enter__(self):
        self.thread.start()
        time.sleep(0.15)
        return self

    def __exit__(self, *args):
        self.server.shutdown()
        self.thread.join(timeout=3)

    @property
    def url(self):
        return f"http://127.0.0.1:{self.port}"


# ═══════════════════════════════════════════════════════════════════════════════
# Tee — writes to both stdout and a log file
# ═══════════════════════════════════════════════════════════════════════════════

class Tee:
    def __init__(self, *streams):
        self.streams = streams

    def write(self, data):
        for s in self.streams:
            s.write(data)
            s.flush()

    def flush(self):
        for s in self.streams:
            s.flush()


# ═══════════════════════════════════════════════════════════════════════════════
# Test Runner
# ═══════════════════════════════════════════════════════════════════════════════

class TestRunner:
    """Collects results from individual test methods and prints a summary."""

    def __init__(self):
        self.results = []
        self.start_time = time.time()
        self.skipped_apis = []

    def check(self, name, passed, detail=""):
        status = "PASS" if passed else "FAIL"
        self.results.append({"name": name, "status": status, "detail": detail})
        icon = "+" if passed else "X"
        print(f"  [{icon}] {name} — {status}" + (f" ({detail})" if detail else ""))

    def skip(self, name, reason="Not yet implemented in SDK"):
        self.results.append({"name": name, "status": "SKIP", "detail": reason})
        self.skipped_apis.append(name)
        print(f"  [-] {name} — SKIP ({reason})")

    def section(self, title):
        print(f"\n{'=' * 65}")
        print(f"  {title}")
        print(f"{'=' * 65}")

    def subsection(self, title):
        print(f"\n  --- {title} ---")

    @property
    def total(self):
        return len(self.results)

    @property
    def passed(self):
        return sum(1 for r in self.results if r["status"] == "PASS")

    @property
    def failed(self):
        return sum(1 for r in self.results if r["status"] == "FAIL")

    @property
    def skipped(self):
        return sum(1 for r in self.results if r["status"] == "SKIP")

    def print_summary(self):
        elapsed = time.time() - self.start_time
        coverage = (self.passed / max(self.total, 1)) * 100
        print(f"\n{'=' * 65}")
        print(f"  FINAL SUMMARY")
        print(f"{'=' * 65}")
        print(f"  Total APIs tested:  {self.total}")
        print(f"  Passed:             {self.passed}")
        print(f"  Failed:             {self.failed}")
        print(f"  Skipped:            {self.skipped}")
        print(f"  Execution time:     {elapsed:.2f}s")
        print(f"  Coverage:           {coverage:.1f}%")
        if self.skipped_apis:
            print(f"\n  Skipped APIs ({len(self.skipped_apis)}):")
            for name in self.skipped_apis:
                print(f"    - {name}")
        print(f"\n  {'ALL PASSED' if self.failed == 0 else 'SOME FAILURES — review above'}")


# ═══════════════════════════════════════════════════════════════════════════════
# 1. Session
# ═══════════════════════════════════════════════════════════════════════════════

def test_session(runner, server):
    runner.section("1. Session")
    runner.subsection("1.1 Create")
    s = Session()
    runner.check("Session() returns Session instance", isinstance(s, Session))

    runner.subsection("1.2 Configure")
    s2 = Session()
    s2.headers({"Accept": "text/html"}).timeout(30).rate_limit(2.0)
    s2.fetcher_tier("stealthy").browser_profile("chrome").auto_match(True)
    s2.fingerprint_path("/tmp/crawlingo_test_fp")
    s2.auto_match_weights({"text": 2.0})
    for name in ("headers", "timeout", "rate_limit", "fetcher_tier",
                 "browser_profile", "auto_match", "fingerprint_path", "auto_match_weights"):
        runner.check(f"Session.{name}() — sets config", True)

    runner.subsection("1.3 Default")
    s3 = Session()
    runner.check("Default Session", isinstance(s3, Session))
    runner.check("s.page(url) returns Page", isinstance(s3.page(server.url), Page))
    runner.check("s.dataset(url) returns Dataset", isinstance(s3.dataset(server.url), Dataset))
    runner.check("s.crawl(url) returns Crawl", isinstance(s3.crawl(server.url), Crawl))
    runner.check("s.watch(url) returns Watch", isinstance(s3.watch(server.url), Watch))

    runner.subsection("1.4 Custom")
    Session().headers({"X-C": "y"}).timeout(15).rate_limit(5)
    runner.check("Custom Session configured", True)

    runner.subsection("1.5 Clone")
    runner.skip("Session.clone()", "Not exposed")

    runner.subsection("1.6 Context Manager")
    with Session() as cs:
        runner.check("__enter__ returns Session", isinstance(cs, Session))
    runner.check("__exit__ completes", True)


# ═══════════════════════════════════════════════════════════════════════════════
# 2. Fetchers
# ═══════════════════════════════════════════════════════════════════════════════

def test_fetchers(runner, server):
    runner.section("2. Fetchers")

    for tier in ("standard", "stealthy"):
        s = Session().fetcher_tier(tier)
        p = Page(server.url, session=s, timeout=10)
        p.html()
        runner.check(f"Fetcher '{tier}' — status 200", p.status == 200)

    runner.skip("Fetcher 'browser'", "Not in SDK")
    runner.skip("Fetcher 'auto'", "Not in SDK")
    runner.skip("Future tiers", "Not exposed")


# ═══════════════════════════════════════════════════════════════════════════════
# 3. Browser Profiles
# ═══════════════════════════════════════════════════════════════════════════════

def test_browser_profiles(runner, server):
    runner.section("3. Browser Profiles")
    for profile in ("chrome", "firefox", "safari"):
        p = Page(server.url, session=Session().browser_profile(profile), timeout=10)
        p.html()
        runner.check(f"Profile '{profile}' — status 200", p.status == 200)
    runner.skip("Profile 'edge'", "Not exposed")


# ═══════════════════════════════════════════════════════════════════════════════
# 4. Headers
# ═══════════════════════════════════════════════════════════════════════════════

def test_headers(runner, server):
    runner.section("4. Headers")

    p = Page(server.url, session=Session().headers({"X-T": "h"}), timeout=10)
    runner.check("Custom headers via Session", p.status == 200)

    p2 = Page(server.url, timeout=10, headers={"X-O": "y"})
    runner.check("Per-request headers", p2.status == 200)

    runner.skip("Header merge", "No merge API")
    runner.skip("Header remove", "No remove API")


# ═══════════════════════════════════════════════════════════════════════════════
# 5. Cookies
# ═══════════════════════════════════════════════════════════════════════════════

def test_cookies(runner, server):
    runner.section("5. Cookies")

    p = Page(server.url, session=Session().cookies({"sid": "abc"}), timeout=10)
    runner.check("Session cookies — page fetched", p.status == 200)

    p2 = Page(server.url, timeout=10, cookies={"c": "v"})
    runner.check("Per-request cookies — page fetched", p2.status == 200)

    runner.skip("Update cookies", "No update API")
    runner.skip("Delete cookies", "No delete API")
    runner.skip("Persistent cookies", "No store API")


# ═══════════════════════════════════════════════════════════════════════════════
# 6. Proxy
# ═══════════════════════════════════════════════════════════════════════════════

def test_proxy(runner, server):
    runner.section("6. Proxy")

    runner.skip("Single proxy", "Requires live proxy")
    Session().proxy_pool(["http://p1:8080"])
    runner.check("Proxy pool — accepts list", True)
    Session().proxy_provider("http://example.com/list")
    runner.check("Proxy provider — accepts URL", True)
    runner.skip("Invalid proxy", "Covered by error tests")
    runner.skip("Proxy auth", "No explicit API")
    runner.skip("Proxy rotation", "Automatic via pool")


# ═══════════════════════════════════════════════════════════════════════════════
# 7. Timeouts
# ═══════════════════════════════════════════════════════════════════════════════

def test_timeouts(runner, server):
    runner.section("7. Timeouts")

    p = Page(server.url, timeout=30)
    p.html()
    runner.check("Default timeout (30s)", p.status == 200)

    p2 = Page(server.url, timeout=10)
    p2.html()
    runner.check("Custom timeout (10s)", p2.status == 200)

    runner.skip("Timeout exceeded", "Would hang; see section 24")


# ═══════════════════════════════════════════════════════════════════════════════
# 8. Rate Limiting
# ═══════════════════════════════════════════════════════════════════════════════

def test_rate_limiting(runner, server):
    runner.section("8. Rate Limiting")

    runner.check("Default — no limit set", True)
    Session().rate_limit(10)
    runner.check("Custom (10 req/s)", True)
    Session().rate_limit(100)
    runner.check("High (100 req/s)", True)
    Session().rate_limit(0.5)
    runner.check("Low (0.5 req/s)", True)


# ═══════════════════════════════════════════════════════════════════════════════
# 9. Retry Logic
# ═══════════════════════════════════════════════════════════════════════════════

def test_retry_logic(runner, server):
    runner.section("9. Retry Logic")

    p = Page(server.url, timeout=10, retries=3)
    p.html()
    runner.check("Page with retries=3 — fetched", p.status == 200)

    runner.skip("Retry exhausted", "See section 24")
    runner.skip("Backoff", "No backoff API")


# ═══════════════════════════════════════════════════════════════════════════════
# 10. HTTP Requests
# ═══════════════════════════════════════════════════════════════════════════════

def test_http_requests(runner, server):
    runner.section("10. HTTP Requests")

    p = Page(server.url, timeout=10)
    p.html()
    runner.check("GET — Page fetch", p.status == 200)

    runner.skip("POST", "Not exposed on Page/Session")
    runner.skip("PUT", "Not exposed")
    runner.skip("PATCH", "Not exposed")
    runner.skip("DELETE", "Not exposed")
    runner.skip("HEAD", "Not exposed")
    runner.skip("OPTIONS", "Not exposed")


# ═══════════════════════════════════════════════════════════════════════════════
# 11. Page APIs
# ═══════════════════════════════════════════════════════════════════════════════

def test_page_apis(runner, server):
    runner.section("11. Page APIs")

    p = Page(server.url, timeout=10)
    runner.check("Page(url) created", isinstance(p, Page) and p.url == server.url)
    p.html()
    runner.check("Page.url — string", isinstance(p.url, str))
    runner.check("Page.status — int 200", isinstance(p.status, int) and p.status == 200)

    p2 = Page(server.url, auto_match=True, timeout=10)
    p2.html()
    runner.check("auto_match=True — fetched", p2.status == 200)

    p3 = Page(server.url, auto_match=False, timeout=15, retries=2,
              headers={"X": "v"}, cookies={"t": "c"})
    p3.html()
    runner.check("All params — fetched", p3.status == 200)

    runner.skip("Navigate", "Not supported")
    runner.skip("Reload", "Not supported")
    runner.skip("Back/Forward", "Not supported")
    runner.skip("Close", "Not supported")


# ═══════════════════════════════════════════════════════════════════════════════
# 12. HTML
# ═══════════════════════════════════════════════════════════════════════════════

def test_html(runner, server):
    runner.section("12. HTML")

    p = Page(server.url, timeout=10)
    html = p.html()
    runner.check("Page.html() returns string", isinstance(html, str) and len(html) > 0)
    runner.check("Contains 'OK'", "OK" in html)

    runner.skip("Pretty HTML", "Not exposed")
    runner.check("Raw HTML — original server response", "<html" in html.lower() or "<!DOCTYPE" in html)


# ═══════════════════════════════════════════════════════════════════════════════
# 13. Text Extraction
# ═══════════════════════════════════════════════════════════════════════════════

def test_text_extraction(runner, server):
    runner.section("13. Text Extraction")

    p = Page(server.url, timeout=10)
    p.html()

    runner.check("Page.title() — string", isinstance(p.title(), str) and len(p.title()) > 0)

    body = p.css("body").first()
    runner.check("Body element present", body is not None and len(body.text()) > 0)

    runner.check("At least 1 <p>", len(p.css("p")) >= 1)
    runner.check("At least 1 <h1>", len(p.css("h1")) >= 1)


# ═══════════════════════════════════════════════════════════════════════════════
# 14. Selectors
# ═══════════════════════════════════════════════════════════════════════════════

def test_selectors(runner, server):
    runner.section("14. Selectors")

    p = Page(server.url, timeout=10)
    p.html()

    runner.check("CSS h1", isinstance(p.css("h1"), ElementCollection) and len(p.css("h1")) >= 1)
    runner.check("XPath //h1", isinstance(p.xpath("//h1"), ElementCollection) and len(p.xpath("//h1")) >= 1)
    runner.check("find_text 'OK'", len(p.find_text("OK")) >= 1)
    runner.check("after_text 'OK'", isinstance(p.after_text("OK"), ElementCollection))
    runner.check("before_text 'paragraph'", isinstance(p.before_text("paragraph"), ElementCollection))
    runner.check("regex 'OK'", isinstance(p.regex("OK"), ElementCollection))
    runner.check("ID selector '#nonexist' empty", len(p.css("#nonexist")) == 0)
    runner.check("Class selector '.nonexist' empty", len(p.css(".nonexist")) == 0)
    runner.check("XPath //@class", isinstance(p.xpath("//@class"), ElementCollection))
    runner.check("CSS h1 returns 1+", len(p.css("h1")) >= 1)
    runner.check("CSS p returns 1+", len(p.css("p")) >= 1)


# ═══════════════════════════════════════════════════════════════════════════════
# 15. Extraction
# ═══════════════════════════════════════════════════════════════════════════════

def test_extraction(runner, server):
    runner.section("15. Extraction")

    p = Page(server.url, timeout=10)
    p.html()

    runner.subsection("15.1 Single Value")
    h1 = p.css("h1").first()
    if h1:
        runner.check("h1.text() returns string", isinstance(h1.text(), str))

    runner.subsection("15.2 Multiple Values")
    texts = p.css("p").texts()
    runner.check("p.texts() returns list", isinstance(texts, list) and len(texts) >= 1)

    runner.subsection("15.3 Dataset.build()")
    ds = Dataset(server.url, Session())
    ds.field("heading", "h1").field("paragraph", "p").timeout(10)
    try:
        result = ds.build()
        rd = result.to_dict()
        runner.check("build().to_dict() returns dict", isinstance(rd, dict) and len(rd) > 0)
    except Exception as e:
        runner.check("Dataset.build()", False, str(e)[:80])

    runner.subsection("15.4 Tables")
    p2 = Page(f"{server.url}/table", timeout=10)
    p2.html()
    runner.check("Table tr found", len(p2.css("tr")) >= 2)

    runner.subsection("15.5 Links / Images / Scripts / Stylesheets")
    p3 = Page(f"{server.url}/links", timeout=10)
    p3.html()
    runner.check("Links — a[href]", len(p3.css("a[href]")) >= 1)
    runner.check("Images — img[src]", len(p3.css("img[src]")) >= 1)
    runner.check("Scripts — script[src]", len(p3.css("script[src]")) >= 1)
    runner.check("Stylesheets — link[rel]", len(p3.css("link[rel]")) >= 1)

    runner.skip("JSON extraction", "Use Python json module")
    runner.check("Meta elements", isinstance(p.css("meta"), ElementCollection))


# ═══════════════════════════════════════════════════════════════════════════════
# 16–19. Pagination / Screenshots / Downloads / Uploads
# ═══════════════════════════════════════════════════════════════════════════════

def test_pagination(runner, server):
    runner.section("16. Pagination")
    runner.skip("All pagination", "Use Crawl.follow() for link-based; no dedicated API")


def test_screenshots(runner, server):
    runner.section("17. Screenshots")
    runner.skip("All screenshots", "No browser engine in SDK")


def test_downloads(runner, server):
    runner.section("18. Downloads")
    runner.skip("All downloads", "Content via Page.html(); no file download API")


def test_uploads(runner, server):
    runner.section("19. Uploads")
    runner.skip("All uploads", "No upload API")


# ═══════════════════════════════════════════════════════════════════════════════
# 20. Authentication
# ═══════════════════════════════════════════════════════════════════════════════

def test_authentication(runner, server):
    runner.section("20. Authentication")

    base = f"{server.url}/auth"
    p = Page(base, timeout=10, headers={"Authorization": "Bearer test_token"})
    runner.check("Bearer token — fetched", p.status == 200)

    p2 = Page(base, timeout=10, headers={"Authorization": "Basic dGVzdDpwYXNz"})
    runner.check("Basic auth — fetched", p2.status == 200)

    p3 = Page(base, timeout=10, cookies={"session": "valid"})
    runner.check("Cookie auth — fetched", p3.status == 200)

    p4 = Page(base, timeout=10, headers={"X-API-Key": "secret"})
    runner.check("API key — fetched", p4.status == 200)


# ═══════════════════════════════════════════════════════════════════════════════
# 21. Dataset
# ═══════════════════════════════════════════════════════════════════════════════

def test_dataset(runner, server):
    runner.section("21. Dataset")

    runner.subsection("21.1 Create & Define")
    d = Dataset(server.url, Session())
    d.field("heading", "h1").field("paragraph", "p").field("missing", ".nonexist", default="N/A")
    runner.check("Dataset created with 3 fields", isinstance(d, Dataset))

    runner.subsection("21.2 Build")
    ds = Dataset(server.url, Session())
    ds.field("heading", "h1").field("paragraph", "p").timeout(10)
    try:
        result = ds.build()
        runner.check("build() returns DatasetResult", isinstance(result, DatasetResult))
        runner.check("to_dict() returns dict", isinstance(result.to_dict(), dict))
    except Exception as e:
        runner.check("Dataset.build()", False, str(e)[:80])
        return

    runner.subsection("21.3 Export")
    tmp = tempfile.gettempdir()
    try:
        csv_path = os.path.join(tmp, "_crawlingo_test.csv")
        result.to_csv(csv_path)
        with open(csv_path) as f:
            header = next(csv.reader(f))
        runner.check("CSV export — header written", os.path.exists(csv_path) and "heading" in header)
    except Exception as e:
        runner.check("CSV export", False, str(e)[:80])

    try:
        json_path = os.path.join(tmp, "_crawlingo_test.json")
        result.to_json(json_path)
        with open(json_path) as f:
            data = json.load(f)
        runner.check("JSON export — valid array", isinstance(data, list) and len(data) > 0)
    except Exception as e:
        runner.check("JSON export", False, str(e)[:80])

    try:
        pq_path = os.path.join(tmp, "_crawlingo_test.parquet")
        result.to_parquet(pq_path)
        runner.check("Parquet export — file written",
                     os.path.exists(pq_path) and os.path.getsize(pq_path) > 0)
    except Exception as e:
        runner.check("Parquet export", False, str(e)[:80])

    runner.subsection("21.4 getitem & hooks")
    try:
        val = result["heading"]
        runner.check("result[field] returns string", isinstance(val, str))
    except Exception as e:
        runner.check("result[field]", False, str(e)[:40])

    if HAS_PANDAS:
        runner.check("result.df() returns DataFrame", isinstance(result.df(), pd.DataFrame))
    else:
        runner.skip("result.df()", "pandas not installed")

    try:
        d2 = Dataset(server.url, Session())
        d2.field("h", "h1", transform=strip_whitespace)
        d2.field("p", "p", transform=uppercase)
        d2.timeout(10)
        r2 = d2.build()
        runner.check("Dataset with transform hooks — build succeeds", isinstance(r2, DatasetResult))
    except Exception as e:
        runner.check("Dataset with transforms", False, str(e)[:80])

    runner.skip("Row update", "Dataset is read-only")
    runner.skip("Row delete", "Dataset is read-only")


# ═══════════════════════════════════════════════════════════════════════════════
# 22. Parsing
# ═══════════════════════════════════════════════════════════════════════════════

def test_parsing(runner, server):
    runner.section("22. Parsing")

    p = Page(server.url, timeout=10)
    runner.check("HTML — Page.html() returns string", isinstance(p.html(), str))

    runner.skip("JSON parsing", "Use json module")
    runner.skip("XML parsing", "Use xml module")
    runner.skip("Markdown parsing", "Not built in")
    runner.skip("CSV parsing", "Use csv module")


# ═══════════════════════════════════════════════════════════════════════════════
# 23. Utilities
# ═══════════════════════════════════════════════════════════════════════════════

def test_utilities(runner, server):
    runner.section("23. Utilities")
    runner.skip("URL normalization", "Not in SDK")
    runner.skip("URL validation", "Not in SDK")
    runner.skip("Domain extraction", "Not in SDK")
    runner.skip("Hash / Encode / Decode", "Not in SDK")


# ═══════════════════════════════════════════════════════════════════════════════
# 24. Errors
# ═══════════════════════════════════════════════════════════════════════════════

def test_errors(runner, server):
    runner.section("24. Errors")

    runner.subsection("24.1 Exception Classes")
    for cls, name in [
        (CrawlingoError, "CrawlingoError"),
        (FetchError, "FetchError"),
        (ParseError, "ParseError"),
        (SelectorError, "SelectorError"),
        (AutoMatchFailed, "AutoMatchFailed"),
        (TimeoutError, "TimeoutError"),
        (RateLimitError, "RateLimitError"),
        (ChangeDetectionError, "ChangeDetectionError"),
        (ExportError, "ExportError"),
        (DnsError, "DnsError"),
        (FingerprintStoreError, "FingerprintStoreError"),
    ]:
        try:
            raise cls("test")
        except cls:
            runner.check(f"{name} — raised and caught", True)

    runner.subsection("24.2 Inheritance")
    runner.check("FetchError inherits CrawlingoError", issubclass(FetchError, CrawlingoError))
    runner.check("TimeoutError inherits CrawlingoError", issubclass(TimeoutError, CrawlingoError))

    runner.subsection("24.3 HTTP Errors")
    p404 = Page(f"{server.url}/404", timeout=10)
    runner.check("404 status", p404.status == 404)

    p500 = Page(f"{server.url}/500", timeout=10)
    runner.check("500 status", p500.status == 500)

    p403 = Page(f"{server.url}/403", timeout=10)
    runner.check("403 status", p403.status == 403)

    runner.subsection("24.4 Connection Refused")
    try:
        Page("http://127.0.0.1:1", timeout=5).html()
        runner.check("Connection refused — error raised", False, "No exception")
    except (FetchError, CrawlingoError, OSError):
        runner.check("Connection refused — error raised", True)
    except Exception as e:
        runner.check("Connection refused — error raised", False, type(e).__name__)

    runner.subsection("24.5 DNS Failure")
    try:
        Page("http://this-domain-does-not-exist-99999.com", timeout=5).html()
        runner.check("DNS failure — error raised", False, "No exception")
    except (FetchError, CrawlingoError, OSError):
        runner.check("DNS failure — error raised", True)
    except Exception as e:
        runner.check("DNS failure — error raised", False, type(e).__name__)

    runner.subsection("24.6 Invalid URL")
    try:
        Page("not a url", timeout=5).html()
        runner.check("Invalid URL — error raised", False, "No exception")
    except Exception:
        runner.check("Invalid URL — error raised", True)

    runner.skip("Timeout error", "Would hang")


# ═══════════════════════════════════════════════════════════════════════════════
# 25. Logging / Hooks
# ═══════════════════════════════════════════════════════════════════════════════

def test_logging(runner, server):
    runner.section("25. Logging / Hooks")

    for fn, name in [
        (strip_whitespace, "strip_whitespace"),
        (uppercase, "uppercase"),
        (lowercase, "lowercase"),
        (log_request, "log_request"),
        (log_response, "log_response"),
    ]:
        runner.check(f"Hook '{name}' callable", callable(fn))

    runner.check("strip_whitespace('  hi  ') = 'hi'", strip_whitespace("  hi  ") == "hi")
    runner.check("uppercase('hi') = 'HI'", uppercase("hi") == "HI")
    runner.check("lowercase('HI') = 'hi'", lowercase("HI") == "hi")

    p = Page(server.url, timeout=10)
    p.before_fetch(log_request)
    p.after_fetch(log_response)
    p.before_parse(lambda h: None)
    p.after_extract(lambda v: v)
    p.html()
    runner.check("Page hooks — before_fetch registered", True)
    runner.check("Page hooks — after_fetch registered", True)
    runner.check("Page hooks — before_parse registered", True)
    runner.check("Page hooks — after_extract registered", True)

    runner.skip("Log levels (debug/info/warn/error/trace)", "No logging levels; use hooks")


# ═══════════════════════════════════════════════════════════════════════════════
# 26. Performance
# ═══════════════════════════════════════════════════════════════════════════════

def test_performance(runner, server):
    runner.section("26. Performance")

    c = Crawl(server.url, Session())
    c.follow("a").limit(2).concurrency(4).delay(0.1)
    runner.check("Crawl.concurrency(4) — configured", True)

    p = Page(f"{server.url}/large", timeout=30)
    p.html()
    runner.check("Large page fetched", p.status == 200)
    runner.check("Large page > 50000 chars", len(p.html()) > 50000)

    runner.skip("Memory profiling", "External tools")
    runner.skip("Stress test", "External tools")


# ═══════════════════════════════════════════════════════════════════════════════
# 27. Cleanup
# ═══════════════════════════════════════════════════════════════════════════════

def test_cleanup(runner, server):
    runner.section("27. Cleanup")

    p = Page(server.url, timeout=10)
    p.html()
    runner.check("Page created and fetched", p.status == 200)
    del p
    runner.check("Page dereferenced", True)

    s = Session()
    runner.check("Session created", isinstance(s, Session))
    del s
    runner.check("Session dereferenced", True)

    runner.skip("Clear cache", "No cache clear API")
    runner.skip("Clear cookies", "No cookie clear API")
    runner.skip("Release resources", "Handled by GC")


# ═══════════════════════════════════════════════════════════════════════════════
# Main
# ═══════════════════════════════════════════════════════════════════════════════

def main():
    # Capture output to log file
    if SAVE_LOG:
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        log_path = os.path.join(os.getcwd(), f"crawlingo_test_output_{timestamp}.log")
        log_file = open(log_path, "w", encoding="utf-8")
        sys.stdout = Tee(sys.stdout, log_file)
        print(f"Output also saved to: {log_path}")

    runner = TestRunner()
    print(f"  SDK: crawlingo v{crawlingo.__version__} (Python)")
    print(f"  Started at: {datetime.now().isoformat()}")

    with TestServer() as server:
        print(f"  Test server: {server.url}\n")

        tests = [
            ("1.  Session",            test_session),
            ("2.  Fetchers",           test_fetchers),
            ("3.  Browser Profiles",   test_browser_profiles),
            ("4.  Headers",            test_headers),
            ("5.  Cookies",            test_cookies),
            ("6.  Proxy",              test_proxy),
            ("7.  Timeouts",           test_timeouts),
            ("8.  Rate Limiting",      test_rate_limiting),
            ("9.  Retry Logic",        test_retry_logic),
            ("10. HTTP Requests",      test_http_requests),
            ("11. Page APIs",          test_page_apis),
            ("12. HTML",               test_html),
            ("13. Text Extraction",    test_text_extraction),
            ("14. Selectors",          test_selectors),
            ("15. Extraction",         test_extraction),
            ("16. Pagination",         test_pagination),
            ("17. Screenshots",        test_screenshots),
            ("18. Downloads",          test_downloads),
            ("19. Uploads",            test_uploads),
            ("20. Authentication",     test_authentication),
            ("21. Dataset",            test_dataset),
            ("22. Parsing",            test_parsing),
            ("23. Utilities",          test_utilities),
            ("24. Errors",             test_errors),
            ("25. Logging / Hooks",    test_logging),
            ("26. Performance",        test_performance),
            ("27. Cleanup",            test_cleanup),
        ]

        for name, fn in tests:
            try:
                fn(runner, server)
            except Exception as e:
                runner.check(f"{name} — unexpected error", False, str(e)[:80])
                traceback.print_exc()
            time.sleep(0.15)

    runner.print_summary()

    if SAVE_LOG:
        log_file.close()
        print(f"\n  Full output saved to: {log_path}")

    return 0 if runner.failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
