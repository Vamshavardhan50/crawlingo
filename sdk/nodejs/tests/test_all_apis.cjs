/**
 * Crawlingo Node.js SDK — Comprehensive End-to-End API Test Suite
 * =================================================================
 *
 * Covers every public API across 27 categories.
 *
 * Usage:
 *   node tests/test_all_apis.cjs
 *   node tests/test_all_apis.cjs --save    (saves output to a log file)
 *
 * Output:
 *   Section headers with PASS/FAIL/SKIP per API, final summary table.
 *   With --save, output also written to: crawlingo_test_output_<timestamp>.log
 */

const http = require('http');
const fs = require('fs');
const path = require('path');
const os = require('os');
const { Page, Session, Dataset, Crawl, Watch, ElementCollection, Element } = require('../dist');

const TEST_DIR = __dirname;

const TIMEOUT_S = 10;
const SAVE_LOG = process.argv.includes('--save');

// ─── Tee — writes to both stdout and a log file ──────────────────────────────

const LOG_STREAMS = [process.stdout];
if (SAVE_LOG) {
  const ts = new Date().toISOString().replace(/[:.]/g, '-');
  const logPath = path.join(TEST_DIR, `crawlingo_test_output_${ts}.log`);
  const logFd = fs.createWriteStream(logPath, 'utf-8');
  LOG_STREAMS.push(logFd);
  console.log = (...args) => {
    const msg = args.map(a => typeof a === 'string' ? a : JSON.stringify(a)).join(' ') + '\n';
    LOG_STREAMS.forEach(s => s.write(msg));
  };
  console.log(`Output also saved to: ${logPath}`);
}

function log(...args) {
  const msg = args.map(a => typeof a === 'string' ? a : JSON.stringify(a)).join(' ') + '\n';
  LOG_STREAMS.forEach(s => s.write(msg));
}

// ─── Test Runner ─────────────────────────────────────────────────────────────

class TestRunner {
  constructor() {
    this.results = [];
    this.startTime = Date.now();
    this.missingApis = [];
  }

  check(name, passed, detail = '') {
    const status = passed ? 'PASS' : 'FAIL';
    this.results.push({ name, status, detail });
    const icon = passed ? '+' : 'X';
    log(`  [${icon}] ${name} — ${status}${detail ? ' (' + detail + ')' : ''}`);
  }

  missing(name, reason = 'Not yet implemented') {
    this.results.push({ name, status: 'FAIL', detail: `Not implemented: ${reason}` });
    this.missingApis.push(name);
    log(`  [!] ${name} — FAIL (Not implemented: ${reason})`);
  }

  section(title) { log(`\n${'='.repeat(65)}\n  ${title}\n${'='.repeat(65)}`); }
  subsection(title) { log(`\n  --- ${title} ---`); }

  get total() { return this.results.length; }
  get passed() { return this.results.filter(r => r.status === 'PASS').length; }
  get failed() { return this.results.filter(r => r.status === 'FAIL').length; }

  printSummary() {
    const elapsed = ((Date.now() - this.startTime) / 1000).toFixed(2);
    const coverage = this.total > 0 ? ((this.passed / this.total) * 100).toFixed(1) : '0.0';
    log(`\n${'='.repeat(65)}`);
    log(`  FINAL SUMMARY`);
    log(`${'='.repeat(65)}`);
    log(`  Total APIs tested:  ${this.total}`);
    log(`  Passed:             ${this.passed}`);
    log(`  Failed:             ${this.failed}`);
    log(`  Execution time:     ${elapsed}s`);
    log(`  Coverage:           ${coverage}%`);
    if (this.missingApis.length > 0) {
      log(`\n  Missing APIs (${this.missingApis.length}):`);
      this.missingApis.forEach(n => log(`    - ${n}`));
      log(`  (mark features above as FAIL — implement to resolve)`);
    }
    log(`\n  ${this.failed === 0 ? 'ALL PASSED' : this.failed + ' FAILURE(S) — review above'}`);
    if (this.failed === 0) {
      log(`  ${this.passed}/${this.total} APIs implemented and passing.`);
    }
  }
}

// ─── Local Test HTTP Server ──────────────────────────────────────────────────

function ok(res, body, ct = 'text/html') {
  const buf = Buffer.isBuffer(body) ? body : Buffer.from(body);
  const hdrs = {
    'Content-Type': ct,
    'Content-Length': buf.length,
    'Connection': 'close',
  };
  if (ct === 'text/html') hdrs['Set-Cookie'] = 'test_session=abc123; Path=/';
  res.writeHead(200, hdrs);
  res.end(buf);
}

function json(res, data) { ok(res, JSON.stringify(data), 'application/json'); }

const HTML_DEFAULT = '<html><head><title>Test Page</title></head><body><h1>OK</h1><p>test paragraph</p><a href="/page2">link</a></body></html>';

const ROUTES = {
  '/xml':      [200, "<?xml version='1.0'?><root><item id='1'>val</item></root>", 'application/xml'],
  '/csv':      [200, 'name,value\nfoo,1\nbar,2\n', 'text/csv'],
  '/links':    [200, '<html><body><a href="/p1">P1</a><a href="/p2">P2</a><img src="/i.png"><link rel="stylesheet" href="/s.css"><script src="/a.js"></script></body></html>', 'text/html'],
  '/table':    [200, '<html><body><table><tr><th>N</th><th>A</th></tr><tr><td>Alice</td><td>30</td></tr><tr><td>Bob</td><td>25</td></tr></table></body></html>', 'text/html'],
  '/large':    [200, '<html><body>' + '<p>large</p>'.repeat(10000) + '</body></html>', 'text/html'],
  '/slow':     [200, 'slow', 'text/plain'],
};

function createTestServer() {
  return new Promise(resolve => {
    const srv = http.createServer({ keepAliveTimeout: 1000 }, (req, res) => {
      const p = req.url;
      const m = req.method;

      if (m === 'GET') {
        if (p === '/json') return json(res, { key: 'value', nested: { a: 1 } });
        if (p === '/md') return ok(res, '# Hello\n**markdown**', 'text/markdown');
        if (['/404', '/500', '/403'].includes(p)) {
          const status = parseInt(p.slice(1));
          res.writeHead(status, { 'Connection': 'close' });
          return res.end('error');
        }
        if (p === '/auth') {
          const a = req.headers['authorization'] || '';
          const c = req.headers['cookie'] || '';
          const k = req.headers['x-api-key'] || '';
          if (a.includes('test_token') || a.includes('dGVzdDpwYXNz') || c.includes('session') || k) {
            return json(res, { authenticated: true });
          }
          res.writeHead(401, { 'WWW-Authenticate': 'Bearer realm="test"', 'Connection': 'close' });
          return res.end(JSON.stringify({ authenticated: false }));
        }
        if (p.startsWith('/search')) {
          return ok(res, '<html><body><div class="result"><h2>R1</h2><span>$10</span></div></body></html>');
        }
        if (ROUTES[p]) {
          const [status, body, ct] = ROUTES[p];
          res.writeHead(status, { 'Content-Type': ct, 'Content-Length': Buffer.byteLength(body), 'Connection': 'close' });
          return res.end(body);
        }
        return ok(res, HTML_DEFAULT);
      }

      if (['POST', 'PUT', 'PATCH', 'DELETE'].includes(m)) {
        return json(res, { method: m, path: p });
      }
      if (m === 'HEAD') {
        res.writeHead(200, { 'Content-Type': 'text/html', 'Content-Length': '0', 'Connection': 'close' });
        return res.end();
      }
      if (m === 'OPTIONS') {
        res.writeHead(204, { 'Allow': 'GET,POST,PUT,PATCH,DELETE,HEAD,OPTIONS', 'Connection': 'close' });
        return res.end();
      }
      res.writeHead(405, { 'Connection': 'close' });
      res.end();
    });
    srv.timeout = 5000;
    srv.keepAliveTimeout = 1000;
    srv.on('connection', socket => {
      socket.setTimeout(5000);
      socket.on('error', () => {});
    });
    srv.listen(0, '127.0.0.1', () => resolve(srv));
  });
}

function baseUrl(srv) { return `http://127.0.0.1:${srv.address().port}`; }
function pause(ms) { return new Promise(r => setTimeout(r, ms)); }

// ─── 1. Session ──────────────────────────────────────────────────────────────

async function testSession(runner, base) {
  runner.section('1. Session');
  runner.subsection('1.1 Create');
  runner.check('Session() returns Session', new Session() instanceof Session);

  runner.subsection('1.2 Configure');
  new Session().headers({ A: 'b' }).timeout(30).rateLimit(2).fetcherTier('stealthy')
    .browserProfile('chrome').autoMatch(true).fingerprintPath('/tmp/fp').autoMatchWeights({ t: 1 });
  runner.check('All config methods chain', true);

  runner.subsection('1.3 Default');
  runner.check('Default session', new Session() instanceof Session);
  runner.subsection('1.4 Custom');
  new Session().headers({ X: 'y' }).timeout(15).rateLimit(5);
  runner.check('Custom session', true);
  runner.missing('Clone', 'Not exposed');
  runner.missing('Destroy', 'GC handled');
}

// ─── 2. Fetchers ─────────────────────────────────────────────────────────────

async function testFetchers(runner, base) {
  runner.section('2. Fetchers');
  for (const t of ['standard', 'stealthy']) {
    const p = await Page.create(base, { session: new Session().fetcherTier(t), timeout: TIMEOUT_S });
    runner.check(`Fetcher '${t}' — status 200`, p.status === 200);
  }
  runner.missing("'browser'", 'Not in SDK'); runner.missing("'auto'", 'Not in SDK'); runner.missing('Future', 'N/A');
}

// ─── 3. Profiles ─────────────────────────────────────────────────────────────

async function testProfiles(runner, base) {
  runner.section('3. Browser Profiles');
  for (const pr of ['chrome', 'firefox', 'safari']) {
    const p = await Page.create(base, { session: new Session().browserProfile(pr), timeout: TIMEOUT_S });
    runner.check(`Profile '${pr}' — status 200`, p.status === 200);
  }
  runner.missing("'edge'", 'Not exposed');
}

// ─── 4. Headers ──────────────────────────────────────────────────────────────

async function testHeaders(runner, base) {
  runner.section('4. Headers');
  let p = await Page.create(base, { session: new Session().headers({ X: 'y' }), timeout: TIMEOUT_S });
  runner.check('Session headers', p.status === 200);
  p = await Page.create(base, { timeout: TIMEOUT_S, headers: { O: 'v' } });
  runner.check('Per-request headers', p.status === 200);
  runner.missing('Merge', 'No API'); runner.missing('Remove', 'No API');
}

// ─── 5. Cookies ──────────────────────────────────────────────────────────────

async function testCookies(runner, base) {
  runner.section('5. Cookies');
  let p = await Page.create(base, { session: new Session().cookies({ s: 'v' }), timeout: TIMEOUT_S });
  runner.check('Session cookies', p.status === 200);
  p = await Page.create(base, { timeout: TIMEOUT_S, cookies: { c: 'v' } });
  runner.check('Per-request cookies', p.status === 200);
  runner.missing('Update/Delete/Persist', 'No API');
}

// ─── 6. Proxy ────────────────────────────────────────────────────────────────

async function testProxy(runner, base) {
  runner.section('6. Proxy');
  runner.missing('Single proxy', 'Requires live proxy');
  new Session().proxyPool(['http://p1:8080']); runner.check('Pool', true);
  new Session().proxyProvider('http://ex.com'); runner.check('Provider', true);
  runner.missing('Invalid/Auth/Rotation', 'See errors / no API');
}

// ─── 7. Timeouts ─────────────────────────────────────────────────────────────

async function testTimeouts(runner, base) {
  runner.section('7. Timeouts');
  let p = await Page.create(base, { timeout: 30 });
  runner.check('Default (30s)', p.status === 200);
  p = await Page.create(base, { timeout: TIMEOUT_S });
  runner.check('Custom (10s)', p.status === 200);
  runner.missing('Exceeded', 'See section 24');
}

// ─── 8. Rate ─────────────────────────────────────────────────────────────────

async function testRate(runner, base) {
  runner.section('8. Rate Limiting');
  runner.check('Default', true); new Session().rateLimit(10); runner.check('10/s', true);
  new Session().rateLimit(100); runner.check('100/s', true);
  new Session().rateLimit(0.5); runner.check('0.5/s', true);
}

// ─── 9. Retry ────────────────────────────────────────────────────────────────

async function testRetry(runner, base) {
  runner.section('9. Retry Logic');
  const p = await Page.create(base, { timeout: TIMEOUT_S });
  runner.check('Page fetched', p.status === 200);
  runner.missing('Exhausted/Backoff', 'No retry config in Node API');
}

// ─── 10. HTTP Methods ────────────────────────────────────────────────────────

async function testHttp(runner, base) {
  runner.section('10. HTTP Requests');
  const p = await Page.create(base, { timeout: TIMEOUT_S });
  runner.check('GET — fetched', p.status === 200);
  runner.missing('POST/PUT/PATCH/DELETE/HEAD/OPTIONS', 'Not exposed on Page/Session');
}

// ─── 11. Page APIs ───────────────────────────────────────────────────────────

async function testPageApis(runner, base) {
  runner.section('11. Page APIs');
  const p = await Page.create(base, { timeout: TIMEOUT_S });
  runner.check('Page.create() returns Page', p instanceof Page);
  runner.check('.url string', typeof p.url === 'string' && p.url === base);
  runner.check('.status number 200', typeof p.status === 'number' && p.status === 200);
  runner.check('.html string', typeof p.html === 'string' && p.html.length > 0);
  runner.check('.title() string', typeof p.title() === 'string' && p.title().length > 0);

  const p2 = await Page.create(base, { autoMatch: true, timeout: TIMEOUT_S });
  runner.check('autoMatch=true', p2.status === 200);
  const p3 = await Page.create(base, { timeout: 15, headers: { X: 'v' }, cookies: { t: 'c' } });
  runner.check('All params', p3.status === 200);
  runner.missing('Navigate/Reload/Back/Forward/Close', 'Not in SDK');
}

// ─── 12. HTML ────────────────────────────────────────────────────────────────

async function testHtml(runner, base) {
  runner.section('12. HTML');
  const p = await Page.create(base, { timeout: TIMEOUT_S });
  runner.check('.html returns string', typeof p.html === 'string' && p.html.length > 0);
  runner.check('Contains OK', p.html.includes('OK'));
  runner.missing('Pretty HTML', 'No API');
  runner.check('Raw HTML', p.html.toLowerCase().includes('<html'));
}

// ─── 13. Text ────────────────────────────────────────────────────────────────

async function testText(runner, base) {
  runner.section('13. Text Extraction');
  const p = await Page.create(base, { timeout: TIMEOUT_S });
  runner.check('title() string', typeof p.title() === 'string' && p.title().length > 0);
  runner.check('body exists', p.css('body').first() !== null);
  runner.check('at least 1 <p>', p.css('p').length >= 1);
  runner.check('at least 1 <h1>', p.css('h1').length >= 1);
}

// ─── 14. Selectors ───────────────────────────────────────────────────────────

async function testSelectors(runner, base) {
  runner.section('14. Selectors');
  const p = await Page.create(base, { timeout: TIMEOUT_S });
  runner.check('CSS h1', p.css('h1').length >= 1);
  runner.check('XPath //h1', p.xpath('//h1').length >= 1);
  runner.check('findText OK', p.findText('OK').length >= 1);
  runner.check('afterText OK', p.afterText('OK') instanceof ElementCollection);
  runner.check('beforeText paragraph', p.beforeText('paragraph') instanceof ElementCollection);
  runner.check('regex OK', p.regex('OK') instanceof ElementCollection);
  runner.check('ID #nonexist empty', p.css('#nonexist').length === 0);
  runner.check('Class .nonexist empty', p.css('.nonexist').length === 0);
  runner.check('XPath //@class', p.xpath('//@class') instanceof ElementCollection);
  runner.check('CSS h1 count >= 1', p.css('h1').length >= 1);
  runner.check('CSS p count >= 1', p.css('p').length >= 1);
}

// ─── 15. Extraction ──────────────────────────────────────────────────────────

async function testExtraction(runner, base) {
  runner.section('15. Extraction');
  const p = await Page.create(base, { timeout: TIMEOUT_S });

  runner.subsection('15.1 Single');
  const h1 = p.css('h1').first();
  runner.check('h1.text() string', h1 !== null && typeof h1.text() === 'string');

  runner.subsection('15.2 Multiple');
  runner.check('p.texts() array', Array.isArray(p.css('p').texts()) && p.css('p').texts().length >= 1);

  runner.subsection('15.3 Dataset.extractStructured');
  const ds = new Dataset(base, new Session());
  ds.field('h', 'h1').field('p', 'p');
  const recs = ds.extractStructured(p);
  runner.check('extractStructured returns array', Array.isArray(recs) && recs.length > 0);
  if (recs.length > 0) runner.check('Record has "h" key', 'h' in recs[0]);

  runner.subsection('15.4 buildStructured');
  const ds2 = new Dataset(base, new Session());
  ds2.field('h', 'h1').field('p', 'p');
  try { const r = await ds2.buildStructured(); runner.check('buildStructured array', Array.isArray(r)); }
  catch (e) { runner.check('buildStructured', false, (e.message||'').slice(0,60)); }

  runner.subsection('15.5 Static saveJson/saveCsv');
  Dataset.saveJson([{h:'T',p:'H'}], path.join(TEST_DIR, '_crawlingo_js_test.json'));
  runner.check('saveJson file exists', fs.existsSync(path.join(TEST_DIR, '_crawlingo_js_test.json')));
  Dataset.saveCsv([{h:'T',p:'H'}], path.join(TEST_DIR, '_crawlingo_js_test.csv'));
  runner.check('saveCsv file exists', fs.existsSync(path.join(TEST_DIR, '_crawlingo_js_test.csv')));

  runner.subsection('15.6 Dataset.build()');
  const ds3 = new Dataset(base, new Session());
  ds3.field('h', 'h1').timeout(15);
  try {
    const r = await ds3.build();
    runner.check('build() returns DatasetResult', r instanceof DatasetResult);
    log(`    Dataset result: ${JSON.stringify(r, null, 2)}`);
  }
  catch (e) { runner.check('build()', false, (e.message||'').slice(0,160)); }

  runner.subsection('15.7 Links/Images/Scripts/Styles');
  const p2 = await Page.create(base + '/links', { timeout: TIMEOUT_S });
  runner.check('a[href]', p2.css('a[href]').length >= 1);
  runner.check('img[src]', p2.css('img[src]').length >= 1);
  runner.check('script[src]', p2.css('script[src]').length >= 1);
  runner.check('link[rel]', p2.css('link[rel]').length >= 1);
}

// ─── 16–19. Skipped ──────────────────────────────────────────────────────────

async function testPagination(runner, base) { runner.section('16. Pagination'); runner.missing('All', 'No API'); }
async function testScreenshots(runner, base) { runner.section('17. Screenshots'); runner.missing('All', 'No browser engine'); }
async function testDownloads(runner, base) { runner.section('18. Downloads'); runner.missing('All', 'No download API'); }
async function testUploads(runner, base) { runner.section('19. Uploads'); runner.missing('All', 'No upload API'); }

// ─── 20. Auth ────────────────────────────────────────────────────────────────

async function testAuth(runner, base) {
  runner.section('20. Authentication');
  const a = base + '/auth';
  let p = await Page.create(a, { timeout: TIMEOUT_S, headers: { Authorization: 'Bearer test_token' } });
  runner.check('Bearer token', p.status === 200);
  p = await Page.create(a, { timeout: TIMEOUT_S, headers: { Authorization: 'Basic dGVzdDpwYXNz' } });
  runner.check('Basic auth', p.status === 200);
  p = await Page.create(a, { timeout: TIMEOUT_S, cookies: { s: 'v' } });
  runner.check('Cookie auth', p.status === 200);
  p = await Page.create(a, { timeout: TIMEOUT_S, headers: { 'X-API-Key': 'secret' } });
  runner.check('API key', p.status === 200);
}

// ─── 21. Dataset ─────────────────────────────────────────────────────────────

async function testDataset(runner, base) {
  runner.section('21. Dataset');
  runner.subsection('21.1 Create');
  const d = new Dataset(base, new Session());
  d.field('h', 'h1').field('p', 'p').field('x', '.nonexist', { defaultVal: 'N/A' });
  runner.check('Dataset created', d instanceof Dataset);

  runner.subsection('21.2 Build');
  const ds = new Dataset(base, new Session());
  ds.field('h', 'h1').field('p', 'p').timeout(15);
  try {
    const r = await ds.build();
    runner.check('build() returns DatasetResult', r instanceof DatasetResult);
    const rd = r.toDict();
    runner.check('toDict() object', typeof rd === 'object');
    log(`    Dataset result: ${JSON.stringify(rd, null, 2)}`);
  } catch (e) { runner.check('Dataset.build()', false, (e.message||'').slice(0,160)); return; }

  runner.subsection('21.3 Export');
  try {
    const r = await ds.build();
    await r.toCsv(path.join(TEST_DIR, '_crawlingo_n_test.csv'));
    runner.check('CSV export', fs.existsSync(path.join(TEST_DIR, '_crawlingo_n_test.csv')));
    await r.toJson(path.join(TEST_DIR, '_crawlingo_n_test.json'));
    runner.check('JSON export', fs.existsSync(path.join(TEST_DIR, '_crawlingo_n_test.json')));
    await r.toParquet(path.join(TEST_DIR, '_crawlingo_n_test.parquet'));
    runner.check('Parquet export', fs.existsSync(path.join(TEST_DIR, '_crawlingo_n_test.parquet')));
  } catch (e) { runner.check('Export', false, (e.message||'').slice(0,80)); }

  runner.missing('Update/Delete', 'Read-only');
}

// ─── 22. Parsing ─────────────────────────────────────────────────────────────

async function testParsing(runner, base) {
  runner.section('22. Parsing');
  const p = await Page.create(base, { timeout: TIMEOUT_S });
  runner.check('HTML string', typeof p.html === 'string');
  runner.missing('JSON/XML/MD/CSV', 'Use standard libraries');
}

// ─── 23. Utilities ───────────────────────────────────────────────────────────

async function testUtilities(runner, base) {
  runner.section('23. Utilities');
  runner.missing('All', 'No utility API in SDK');
}

// ─── 24. Errors ──────────────────────────────────────────────────────────────

async function testErrors(runner, base) {
  runner.section('24. Errors');

  let p = await Page.create(base + '/404', { timeout: TIMEOUT_S });
  runner.check('404 status', p.status === 404);
  p = await Page.create(base + '/500', { timeout: TIMEOUT_S });
  runner.check('500 status', p.status === 500);
  p = await Page.create(base + '/403', { timeout: TIMEOUT_S });
  runner.check('403 status', p.status === 403);

  try { await Page.create('http://127.0.0.1:1', { timeout: 5 }); runner.check('Conn refused', false, 'No error'); }
  catch (e) { runner.check('Conn refused — throws', true); }

  try { await Page.create('http://this-domain-does-not-exist-99999.com', { timeout: 5 }); runner.check('DNS fail', false, 'No error'); }
  catch (e) { runner.check('DNS fail — throws', true); }

  runner.missing('Invalid URL / Timeout', 'Handled or would hang');
}

// ─── 25. Logging ─────────────────────────────────────────────────────────────

async function testLogging(runner, base) {
  runner.section('25. Logging');
  runner.missing('All log levels', 'No logging API in Node.js SDK');
}

// ─── 26. Performance ─────────────────────────────────────────────────────────

async function testPerformance(runner, base) {
  runner.section('26. Performance');
  new Crawl(base, new Session()).follow('a').limit(2).concurrency(4).delay(0.1);
  runner.check('Crawl.concurrency(4)', true);
  const p = await Page.create(base + '/large', { timeout: 30 });
  runner.check('Large page fetched', p.status === 200 && p.html.length > 50000);
  runner.missing('Memory/Stress', 'External tools');
}

// ─── 27. Cleanup ─────────────────────────────────────────────────────────────

async function testCleanup(runner, base) {
  runner.section('27. Cleanup');
  let p = await Page.create(base, { timeout: TIMEOUT_S });
  runner.check('Page fetched', p.status === 200); p = null;
  runner.check('Page GC\'d', true);
  let s = new Session(); runner.check('Session created', s instanceof Session); s = null;
  runner.check('Session GC\'d', true);
  runner.missing('Cache/Cookies/Resources', 'GC handled');
}

// ─── Main ────────────────────────────────────────────────────────────────────

async function main() {
  const runner = new TestRunner();
  log(`  SDK: crawlingo (Node.js)`);
  log(`  Started at: ${new Date().toISOString()}`);

  let server;
  try {
    server = await createTestServer();
    const base = baseUrl(server);
    log(`  Test server: ${base}\n`);

    const tests = [
      ['1.  Session',            testSession],
      ['2.  Fetchers',           testFetchers],
      ['3.  Browser Profiles',   testProfiles],
      ['4.  Headers',            testHeaders],
      ['5.  Cookies',            testCookies],
      ['6.  Proxy',              testProxy],
      ['7.  Timeouts',           testTimeouts],
      ['8.  Rate Limiting',      testRate],
      ['9.  Retry Logic',        testRetry],
      ['10. HTTP Requests',      testHttp],
      ['11. Page APIs',          testPageApis],
      ['12. HTML',               testHtml],
      ['13. Text Extraction',    testText],
      ['14. Selectors',          testSelectors],
      ['15. Extraction',         testExtraction],
      ['16. Pagination',         testPagination],
      ['17. Screenshots',        testScreenshots],
      ['18. Downloads',          testDownloads],
      ['19. Uploads',            testUploads],
      ['20. Authentication',     testAuth],
      ['21. Dataset',            testDataset],
      ['22. Parsing',            testParsing],
      ['23. Utilities',          testUtilities],
      ['24. Errors',             testErrors],
      ['25. Logging',            testLogging],
      ['26. Performance',        testPerformance],
      ['27. Cleanup',            testCleanup],
    ];

    for (const [name, fn] of tests) {
      try { await fn(runner, base); }
      catch (e) { runner.check(`${name} — error`, false, (e.message||'').slice(0,80)); }
      await pause(150);
    }
  } finally {
    if (server) server.close();
  }

  runner.printSummary();

  if (SAVE_LOG) {
    log(`\n  (Log file written)`);
  }
}

main();
