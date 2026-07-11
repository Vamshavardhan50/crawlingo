const path = require('path');
const { createMockServer } = require('./mocks/mock_server.cjs');
const { TestRunner } = require('./helpers/harness.cjs');
const { writeJsonReport } = require('./reporters/json.cjs');
const { writeJunitReport } = require('./reporters/junit.cjs');
const { writeHtmlReport } = require('./reporters/html.cjs');

// Test files list
const SUITES = [
  'session.test.cjs',
  'fetchers.test.cjs',
  'page.test.cjs',
  'selectors.test.cjs',
  'dataset.test.cjs',
  'browser_profiles.test.cjs',
  'proxy.test.cjs',
  'headers.test.cjs',
  'cookies.test.cjs',
  'retry.test.cjs',
  'downloads.test.cjs',
  'sitemap.test.cjs',
  'performance.test.cjs',
  'errors.test.cjs',
  'integration.test.cjs',
  'sdk_parity.test.cjs'
];

async function main() {
  const runner = new TestRunner();
  let server;

  try {
    // Start mock HTTP server
    server = await createMockServer();
    const port = server.address().port;
    const base = `http://127.0.0.1:${port}`;

    console.log(`\n\x1b[1m\x1b[36m=================================================================\x1b[0m`);
    console.log(`\x1b[1m\x1b[36m  Crawlingo SDK - Production Test Suite\x1b[0m`);
    console.log(`\x1b[36m  Mock Server Running: ${base}\x1b[0m`);
    console.log(`\x1b[36m=================================================================\x1b[0m`);

    // Run suites
    for (const file of SUITES) {
      const suitePath = path.join(__dirname, file);
      const suite = require(suitePath);
      await suite.run(runner, base);
    }

  } catch (err) {
    console.error('Test execution aborted due to unexpected error:', err);
    process.exit(1);
  } finally {
    if (server) {
      server.close();
    }
  }

  // Generate Reports
  const reportsDir = path.join(__dirname, 'reports');
  const jsonPath = path.join(reportsDir, 'report.json');
  const junitPath = path.join(reportsDir, 'junit.xml');
  const htmlPath = path.join(reportsDir, 'report.html');

  writeJsonReport(runner, jsonPath);
  writeJunitReport(runner, junitPath);
  writeHtmlReport(runner, htmlPath);

  // Classify API categories for Release Readiness Health Scoring
  function classifyResult(name) {
    const browserOnly = [
      'Page.prototype.navigate()',
      'Page.prototype.reload()',
      'Page.prototype.back()',
      'Page.prototype.forward()',
      'Page.prototype.close()'
    ];
    
    const experimentalPlanned = [
      'Dataset.prototype.toArrow()',
      'Dataset.prototype.toDataFrame()',
      'Dataset.prototype.df()',
      'Dataset.prototype.stream()',
      'Dataset.prototype.update()',
      'Dataset.prototype.delete()',
      'Session.prototype.proxyAuth()',
      'Session.prototype.proxyRotate()',
      'Session.prototype.removeHeader()',
      'Session.prototype.mergeHeaders()',
      'Session.prototype.deleteCookie()',
      'Session.prototype.clearCookies()',
      'Session.prototype.getCookies()',
      'Session.prototype.retries()',
      'Session.prototype.retryBackoff()',
      'Session.prototype.retryDelay()',
      'CrawlingoError',
      'FetchError',
      'ParseError',
      'SelectorError',
      'AutoMatchFailed',
      'TimeoutError',
      'RateLimitError',
      'ChangeDetectionError',
      'ExportError',
      'DnsError',
      'FingerprintStoreError'
    ];

    if (browserOnly.includes(name)) return 'browser';
    if (experimentalPlanned.includes(name)) return 'experimental';
    return 'stable';
  }

  let stableTotal = 0;
  let stablePassed = 0;
  let stableFailed = 0;
  let stableNotImplemented = 0;
  let stablePartial = 0;

  let browserCount = 0;
  let experimentalCount = 0;

  for (const r of runner.results) {
    const cat = classifyResult(r.name);
    if (cat === 'stable') {
      stableTotal++;
      if (r.status === 'PASS') stablePassed++;
      else if (r.status === 'FAIL') stableFailed++;
      else if (r.status === 'NOT_IMPLEMENTED') stableNotImplemented++;
      else if (r.status === 'PARTIAL') stablePartial++;
    } else if (cat === 'browser') {
      browserCount++;
    } else if (cat === 'experimental') {
      experimentalCount++;
    }
  }

  const elapsed = ((Date.now() - runner.startTime) / 1000).toFixed(2);
  
  // Only Core / Stable APIs affect API coverage and release readiness health score
  const coverage = stableTotal > 0 ? ((stablePassed / stableTotal) * 100).toFixed(1) : '0.0';
  const healthScore = stableTotal > 0
    ? Math.max(0, Math.min(100, Math.round(((stablePassed + stablePartial * 0.5 - stableFailed * 2) / stableTotal) * 100)))
    : 0;

  console.log(`\n\x1b[36m=================================================================\x1b[0m`);
  console.log(`\x1b[1m  FINAL SUMMARY (v1 Stable Release Readiness)\x1b[0m`);
  console.log(`\x1b[36m=================================================================\x1b[0m`);
  console.log(`  SDK Health Score (Stable Only):  \x1b[35m${healthScore}%\x1b[0m`);
  console.log(`  Stable API Coverage:             \x1b[36m${coverage}%\x1b[0m`);
  console.log(`  Total Stable Tests:              ${stableTotal}`);
  console.log(`    - Passed:                      \x1b[32m${stablePassed}\x1b[0m`);
  console.log(`    - Failed:                      \x1b[31m${stableFailed}\x1b[0m`);
  console.log(`    - Not Implemented:            \x1b[33m${stableNotImplemented}\x1b[0m`);
  console.log(`    - Partial Parity:              \x1b[34m${stablePartial}\x1b[0m`);
  console.log(`  Browser-Only Excluded:           \x1b[37m${browserCount}\x1b[0m`);
  console.log(`  Experimental/Planned Excluded:   \x1b[37m${experimentalCount}\x1b[0m`);
  console.log(`  Execution Time:                  ${elapsed}s`);
  console.log(`\x1b[36m=================================================================\x1b[0m`);

  console.log(`\n  Reports written to:`);
  console.log(`    - JSON:  ${jsonPath}`);
  console.log(`    - JUnit: ${junitPath}`);
  console.log(`    - HTML:  ${htmlPath}`);

  const stats = runner.stats;
  if (stats.failed > 0) {
    console.log(`\n  \x1b[31m✘ TEST RUN FAILED\x1b[0m\n`);
    process.exit(1);
  } else {
    console.log(`\n  \x1b[32m✔ TEST RUN SUCCESSFUL\x1b[0m\n`);
    process.exit(0);
  }
}

main();
