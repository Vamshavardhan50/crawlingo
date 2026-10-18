const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '..');
const read = file => fs.readFileSync(path.join(root, file), 'utf8').replace(/^\uFEFF/, '');
const version = file => {
  const found = read(file).match(/^version\s*=\s*"([^"]+)"/m);
  if (!found) throw new Error(`No version in ${file}`);
  return found[1];
};
const normalize = value => value.replace(/(\d)(a|b|rc)(\d+)$/, (_, digit, kind, count) => `${digit}-${{ a: 'alpha', b: 'beta', rc: 'rc' }[kind]}.${count}`);
const lockedVersion = (file, name) => {
  const block = read(file).split('[[package]]').find(entry => entry.includes(`name = "${name}"`));
  const found = block?.match(/^version\s*=\s*"([^"]+)"/m);
  if (!found) throw new Error(`Missing ${name} in ${file}`);
  return found[1];
};
const versions = {
  rust: version('Cargo.toml'), native: version('sdk/nodejs/native/Cargo.toml'),
  rustLock: lockedVersion('Cargo.lock', 'crawlingo'),
  nativeCoreLock: lockedVersion('sdk/nodejs/native/Cargo.lock', 'crawlingo'),
  nativeLock: lockedVersion('sdk/nodejs/native/Cargo.lock', 'crawlingo-native'),
  python: normalize(version('sdk/python/pyproject.toml')),
  pythonRuntime: normalize(read('sdk/python/crawlingo/__init__.py').match(/^__version__\s*=\s*"([^"]+)"/m)?.[1] || ''),
  node: JSON.parse(read('sdk/nodejs/package.json')).version,
  lock: JSON.parse(read('sdk/nodejs/package-lock.json')).version,
  lockRoot: JSON.parse(read('sdk/nodejs/package-lock.json')).packages[''].version,
};
if (new Set(Object.values(versions)).size !== 1) throw new Error(`Version mismatch: ${JSON.stringify(versions)}`);
const tag = process.argv[2];
if (tag && tag !== `v${versions.rust}`) throw new Error(`Release tag must be v${versions.rust}, received ${tag}`);
if (tag && process.env.RELEASE_PRERELEASE !== undefined) {
  const expected = versions.rust.includes('-');
  if ((process.env.RELEASE_PRERELEASE === 'true') !== expected) {
    throw new Error(`GitHub prerelease must be ${expected} for ${versions.rust}`);
  }
}
console.log(`Release versions aligned: ${versions.rust}${tag ? ` (${tag})` : ''}`);
