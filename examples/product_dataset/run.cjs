// Use the SDK built in this checkout. In your application: require('crawlingo').
const { readFileSync } = require('node:fs');
const { DatasetJob } = require('../../sdk/nodejs/dist');
process.chdir(__dirname);
const job = new DatasetJob(JSON.parse(readFileSync('job.json', 'utf8')));
process.once('SIGINT', () => job.cancel());
process.once('SIGTERM', () => job.cancel());
job.run().then(report => {
  console.log(JSON.stringify(report, null, 2));
  process.exitCode = report.pages_failed || report.records_rejected || report.cancelled ? 2 : 0;
}).catch(error => { console.error(error); process.exitCode = 1; });
