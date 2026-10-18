const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '..');
const dist = path.join(root, 'dist');
assert.ok(fs.existsSync(path.join(root, 'LICENSE')), 'Missing MIT license; run prepack');
for (const name of ['index.js', 'index.d.ts', 'dataset-job.js', 'dataset-job.d.ts', 'native.js', 'native.d.ts']) {
  assert.ok(fs.existsSync(path.join(dist, name)), `Missing distribution file: ${name}`);
}
assert.match(fs.readFileSync(path.join(dist, 'index.d.ts'), 'utf8'), /dataset-job/);
assert.equal(typeof require(dist).DatasetJob, 'function', 'DatasetJob missing from installed exports');
if (process.argv.includes('--all-platforms')) {
  for (const target of ['linux-x64-gnu', 'darwin-x64', 'darwin-arm64', 'win32-x64-msvc']) {
    assert.ok(fs.existsSync(path.join(dist, `crawlingo-native.${target}.node`)), `Missing addon: ${target}`);
  }
}
console.log('Package files and DatasetJob exports verified.');
