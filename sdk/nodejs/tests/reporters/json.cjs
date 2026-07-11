const fs = require('fs');
const path = require('path');

function writeJsonReport(runner, reportPath) {
  const dir = path.dirname(reportPath);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }

  const elapsed = ((Date.now() - runner.startTime) / 1000).toFixed(2);
  const stats = runner.stats;
  const coverage = stats.total > 0 ? ((stats.passed / stats.total) * 100).toFixed(1) : '0.0';

  // Calculate health score: passed/implemented ratio, heavily penalizing failures
  // Let's count implemented as total - notImplemented
  const implementedCount = stats.total - stats.notImplemented;
  const healthScore = implementedCount > 0 
    ? Math.max(0, Math.min(100, Math.round(((stats.passed - stats.failed * 2) / stats.total) * 100)))
    : 0;

  const data = {
    summary: {
      total: stats.total,
      passed: stats.passed,
      failed: stats.failed,
      skipped: stats.skipped,
      notImplemented: stats.notImplemented,
      partial: stats.partial,
      coveragePercent: parseFloat(coverage),
      sdkHealthScore: healthScore,
      elapsedSeconds: parseFloat(elapsed),
      timestamp: new Date().toISOString()
    },
    results: runner.results
  };

  fs.writeFileSync(reportPath, JSON.stringify(data, null, 2), 'utf-8');
}

module.exports = { writeJsonReport };
