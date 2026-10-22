#!/usr/bin/env node
'use strict';
const fs = require('node:fs');
const { parseArgs } = require('node:util');
const { DatasetJob, Session } = require('crawlingo');

async function main() {
  const { values } = parseArgs({ options: {
    job: { type: 'string', default: '/config/job.json' },
    session: { type: 'string', default: '/config/session.toml' },
  } });
  const session = Session.fromConfig(values.session);
  const config = JSON.parse(fs.readFileSync(values.job, 'utf8'));
  const job = new DatasetJob(config, session);
  const stop = () => job.cancel();
  process.on('SIGINT', stop);
  process.on('SIGTERM', stop);
  try {
    const report = await job.run();
    console.log(JSON.stringify(report));
    process.exitCode = report.complete && !report.cancelled &&
      report.pages_failed === 0 && report.records_rejected === 0 ? 0 : 2;
  } finally {
    process.off('SIGINT', stop);
    process.off('SIGTERM', stop);
  }
}
main().catch((error) => {
  console.error('Dataset worker failed:', error.message);
  process.exitCode = 1;
});
