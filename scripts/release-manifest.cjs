// Run after every release package has been built, before any registry upload.
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '..');
const directory = path.resolve(process.argv[2] || 'release-assets');
const version = JSON.parse(fs.readFileSync(path.join(root, 'sdk/nodejs/package.json'), 'utf8')).version;
const pythonVersion = fs.readFileSync(path.join(root, 'sdk/python/pyproject.toml'), 'utf8').match(/^version = "([^"]+)"/m)[1];
const targets = ['x86_64-unknown-linux-gnu', 'x86_64-apple-darwin', 'aarch64-apple-darwin', 'x86_64-pc-windows-msvc'];
const files = fs.readdirSync(directory).filter(name => !['release-manifest.json', 'SHA256SUMS'].includes(name)).sort();
const required = [
  `crawlingo-${version}.tgz`, `crawlingo-${version}.crate`,
  `crawlingo-${pythonVersion}.tar.gz`,
  `crawlingo-${version}-rustdoc.tar.gz`,
  ...targets.map(target => `crawlingo-${version}-${target}.tar.gz`),
];
for (const name of required) assert.ok(files.includes(name), `Missing release artifact: ${name}`);
const wheels = files.filter(name => name.startsWith(`crawlingo-${pythonVersion}-`) && name.endsWith('.whl'));
for (const platform of [/manylinux.*x86_64/, /manylinux.*aarch64/, /macosx.*x86_64/, /macosx.*arm64/, /win_amd64/]) {
  assert.ok(wheels.some(name => platform.test(name)), `Missing Python wheel platform: ${platform}`);
}
const allowed = new Set([...required, ...wheels]);
const artifacts = files.map(name => {
  assert.ok(allowed.has(name), `Unexpected release artifact: ${name}`);
  const bytes = fs.readFileSync(path.join(directory, name));
  assert.ok(bytes.length, `Empty artifact: ${name}`);
  return { file: name, bytes: bytes.length, sha256: crypto.createHash('sha256').update(bytes).digest('hex') };
});
const manifest = {
  version, pythonVersion, commit: process.env.GITHUB_SHA || null,
  workflowRun: process.env.GITHUB_RUN_ID || null,
  artifacts,
};
fs.writeFileSync(path.join(directory, 'release-manifest.json'), JSON.stringify(manifest, null, 2) + '\n');
fs.writeFileSync(path.join(directory, 'SHA256SUMS'), artifacts.map(item => `${item.sha256}  ${item.file}`).join('\n') + '\n');
console.log(`Prepared ${artifacts.length} artifacts for ${version}; all registry uploads may now proceed.`);
