const fs = require('fs');
const path = require('path');

function writeHtmlReport(runner, reportPath) {
  const dir = path.dirname(reportPath);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }

  const elapsed = ((Date.now() - runner.startTime) / 1000).toFixed(2);
  const stats = runner.stats;
  const coverage = stats.total > 0 ? ((stats.passed / stats.total) * 100).toFixed(1) : '0.0';

  const implementedCount = stats.total - stats.notImplemented;
  const healthScore = implementedCount > 0 
    ? Math.max(0, Math.min(100, Math.round(((stats.passed - stats.failed * 2) / stats.total) * 100)))
    : 0;

  let rowsHtml = '';
  for (const r of runner.results) {
    let statusClass = 'status-pass';
    let statusSymbol = '✔ PASS';
    if (r.status === 'FAIL') {
      statusClass = 'status-fail';
      statusSymbol = '✘ FAIL';
    } else if (r.status === 'NOT_IMPLEMENTED') {
      statusClass = 'status-not-implemented';
      statusSymbol = '🚧 NOT IMPLEMENTED';
    } else if (r.status === 'SKIPPED') {
      statusClass = 'status-skipped';
      statusSymbol = '⏭ SKIPPED';
    } else if (r.status === 'PARTIAL') {
      statusClass = 'status-partial';
      statusSymbol = '⚠ PARTIAL';
    } else if (r.status === 'EXPERIMENTAL') {
      statusClass = 'status-experimental';
      statusSymbol = '🧪 EXPERIMENTAL';
    }

    rowsHtml += `
      <div class="test-row">
        <div class="test-name">${r.name}</div>
        <div class="test-status ${statusClass}">${statusSymbol}</div>
        <div class="test-detail">${r.detail || ''}</div>
      </div>
    `;
  }

  const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Crawlingo Node.js SDK Test Report</title>
  <style>
    :root {
      --bg: #0f172a;
      --glass: rgba(30, 41, 59, 0.7);
      --border: rgba(255, 255, 255, 0.08);
      --text: #e2e8f0;
      --text-muted: #94a3b8;
      --primary: #38bdf8;
      --pass: #10b981;
      --fail: #ef4444;
      --skip: #64748b;
      --not-impl: #a855f7;
      --partial: #f59e0b;
      --experimental: #ec4899;
    }
    body {
      background: var(--bg);
      color: var(--text);
      font-family: 'Outfit', 'Inter', -apple-system, sans-serif;
      margin: 0;
      padding: 2rem;
    }
    .container {
      max-width: 1200px;
      margin: 0 auto;
    }
    header {
      margin-bottom: 2rem;
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    h1 {
      font-size: 2.2rem;
      margin: 0;
      background: linear-gradient(135deg, #38bdf8, #818cf8);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
    .meta-time {
      color: var(--text-muted);
      font-size: 0.9rem;
    }
    .dashboard {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
      gap: 1.5rem;
      margin-bottom: 2.5rem;
    }
    .card {
      background: var(--glass);
      backdrop-filter: blur(12px);
      border: 1px solid var(--border);
      border-radius: 16px;
      padding: 1.5rem;
      text-align: center;
      box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);
    }
    .card-title {
      font-size: 0.85rem;
      color: var(--text-muted);
      text-transform: uppercase;
      letter-spacing: 0.05em;
      margin-bottom: 0.5rem;
    }
    .card-value {
      font-size: 2.5rem;
      font-weight: 700;
    }
    .card-value.pass { color: var(--pass); }
    .card-value.fail { color: var(--fail); }
    .card-value.health { color: var(--primary); }
    .test-list {
      background: var(--glass);
      border: 1px solid var(--border);
      border-radius: 16px;
      padding: 1.5rem;
      box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);
    }
    .test-list-header {
      font-size: 1.2rem;
      font-weight: 600;
      margin-bottom: 1rem;
      border-bottom: 1px solid var(--border);
      padding-bottom: 0.5rem;
    }
    .test-row {
      display: grid;
      grid-template-columns: 2fr 1.2fr 2fr;
      padding: 0.8rem;
      border-bottom: 1px solid rgba(255, 255, 255, 0.03);
      align-items: center;
    }
    .test-row:hover {
      background: rgba(255, 255, 255, 0.02);
    }
    .test-name {
      font-weight: 500;
    }
    .test-status {
      font-size: 0.85rem;
      font-weight: 600;
      text-align: center;
      padding: 0.3rem 0.6rem;
      border-radius: 8px;
      max-width: fit-content;
    }
    .status-pass { background: rgba(16, 185, 129, 0.15); color: var(--pass); }
    .status-fail { background: rgba(239, 68, 68, 0.15); color: var(--fail); }
    .status-not-implemented { background: rgba(168, 85, 247, 0.15); color: var(--not-impl); }
    .status-skipped { background: rgba(100, 116, 139, 0.15); color: var(--skip); }
    .status-partial { background: rgba(245, 158, 11, 0.15); color: var(--partial); }
    .status-experimental { background: rgba(236, 72, 153, 0.15); color: var(--experimental); }
    .test-detail {
      font-size: 0.85rem;
      color: var(--text-muted);
    }
  </style>
</head>
<body>
  <div class="container">
    <header>
      <div>
        <h1>Crawlingo SDK Health Dashboard</h1>
        <div class="meta-time">Generated: ${new Date().toLocaleString()} | Duration: ${elapsed}s</div>
      </div>
    </header>

    <div class="dashboard">
      <div class="card">
        <div class="card-title">SDK Health Score</div>
        <div class="card-value health">${healthScore}%</div>
      </div>
      <div class="card">
        <div class="card-title">API Coverage</div>
        <div class="card-value">${coverage}%</div>
      </div>
      <div class="card">
        <div class="card-title">Total Tests</div>
        <div class="card-value">${stats.total}</div>
      </div>
      <div class="card">
        <div class="card-title">Passed</div>
        <div class="card-value pass">${stats.passed}</div>
      </div>
      <div class="card">
        <div class="card-title">Failed</div>
        <div class="card-value fail">${stats.failed}</div>
      </div>
    </div>

    <div class="test-list">
      <div class="test-list-header">API Capability Matrix</div>
      ${rowsHtml}
    </div>
  </div>
</body>
</html>`;

  fs.writeFileSync(reportPath, html, 'utf-8');
}

module.exports = { writeHtmlReport };
