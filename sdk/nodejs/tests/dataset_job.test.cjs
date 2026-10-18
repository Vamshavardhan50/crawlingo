const { test } = require('node:test');
const assert = require('node:assert/strict');
const http = require('node:http');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { DatasetJob, Watch } = require('../dist/index.js');

test('native dataset job exports, rejects invalid rows, and resumes', async () => {
  let calls = 0;
  const server = http.createServer((_req, res) => {
    calls++;
    res.setHeader('Content-Type', 'text/html; charset=utf-8');
    res.end('<article><h1>café 🚀</h1></article><article><p>missing title</p></article>');
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  try {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'crawlingo-job-'));
    const urls_file = path.join(root, 'urls.txt');
    fs.writeFileSync(urls_file, `http://127.0.0.1:${server.address().port}/one\n`);
    const config = { urls_file, output_dir: path.join(root, 'output'), row_selector: 'article', respect_robots: false, fields: [{ name: 'title', selector: 'h1', required: true }] };
    const result = await new DatasetJob(config).run();
    assert.equal(result.records_written, 1);
    assert.equal(result.records_rejected, 1);
    assert.equal(result.complete, true);
    const row = JSON.parse(fs.readFileSync(path.join(config.output_dir, 'part-00000000.jsonl'), 'utf8'));
    assert.equal(row.title, 'café 🚀');
    const resumed = await new DatasetJob(config).run();
    assert.equal(resumed.records_written, 1);
    assert.equal(calls, 1);
  } finally {
    server.closeAllConnections();
    await new Promise(resolve => server.close(resolve));
  }
});

test('watch emits a real content change and stops', async () => {
  let calls = 0;
  const server = http.createServer((_req, res) => {
    calls++;
    res.setHeader('Content-Type', 'text/html');
    res.end(`<h1>${calls === 1 ? 'Original title' : 'Updated title'}</h1>`);
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const watcher = new Watch(`http://127.0.0.1:${server.address().port}/`).field('title', 'h1').interval(1);
  let deadline;
  try {
    const event = await new Promise((resolve, reject) => {
      deadline = setTimeout(() => reject(new Error('Watch did not emit a change')), 10000);
      watcher.run((error, change) => error ? reject(error) : resolve(change));
    });
    assert.equal(event.field, 'title');
    assert.equal(event.oldValue, 'Original title');
    assert.equal(event.newValue, 'Updated title');
  } finally {
    clearTimeout(deadline);
    watcher.stop();
    server.closeAllConnections();
    await new Promise(resolve => server.close(resolve));
  }
});
