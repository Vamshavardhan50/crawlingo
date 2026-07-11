const fs = require('fs');
const path = require('path');

function escapeXml(unsafe) {
  if (!unsafe) return '';
  return unsafe.replace(/[<>&'"]/g, (c) => {
    switch (c) {
      case '<': return '&lt;';
      case '>': return '&gt;';
      case '&': return '&amp;';
      case '\'': return '&apos;';
      case '"': return '&quot;';
    }
  });
}

function writeJunitReport(runner, reportPath) {
  const dir = path.dirname(reportPath);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }

  const elapsed = ((Date.now() - runner.startTime) / 1000).toFixed(2);
  const stats = runner.stats;

  let xml = `<?xml version="1.0" encoding="UTF-8"?>\n`;
  xml += `<testsuites name="Crawlingo Node.js SDK Test Suite" tests="${stats.total}" failures="${stats.failed}" skipped="${stats.skipped + stats.notImplemented}" time="${elapsed}">\n`;
  xml += `  <testsuite name="SDK API Capabilities" tests="${stats.total}" failures="${stats.failed}" errors="0" skipped="${stats.skipped + stats.notImplemented}" time="${elapsed}">\n`;

  for (const r of runner.results) {
    const className = 'crawlingo.sdk.Node';
    const name = escapeXml(r.name);
    
    if (r.status === 'FAIL') {
      xml += `    <testcase classname="${className}" name="${name}">\n`;
      xml += `      <failure message="${escapeXml(r.detail || 'Test failed')}">${escapeXml(r.detail)}</failure>\n`;
      xml += `    </testcase>\n`;
    } else if (r.status === 'NOT_IMPLEMENTED') {
      xml += `    <testcase classname="${className}" name="${name}">\n`;
      xml += `      <skipped message="Not Implemented: ${escapeXml(r.detail || 'API missing')}" />\n`;
      xml += `    </testcase>\n`;
    } else if (r.status === 'SKIPPED') {
      xml += `    <testcase classname="${className}" name="${name}">\n`;
      xml += `      <skipped message="Skipped: ${escapeXml(r.detail || 'Test skipped')}" />\n`;
      xml += `    </testcase>\n`;
    } else {
      xml += `    <testcase classname="${className}" name="${name}" />\n`;
    }
  }

  xml += `  </testsuite>\n`;
  xml += `</testsuites>\n`;

  fs.writeFileSync(reportPath, xml, 'utf-8');
}

module.exports = { writeJunitReport };
