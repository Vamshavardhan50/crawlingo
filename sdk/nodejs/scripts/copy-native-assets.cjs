const fs = require('fs');
const path = require('path');

const packageRoot = path.resolve(__dirname, '..');
const distDir = path.join(packageRoot, 'dist');

fs.mkdirSync(distDir, { recursive: true });
fs.copyFileSync(path.join(packageRoot, '..', '..', 'LICENSE'), path.join(packageRoot, 'LICENSE'));

for (const file of ['native.js', 'native.d.ts']) {
  fs.copyFileSync(path.join(packageRoot, 'src', file), path.join(distDir, file));
}

const binaries = new Map();
for (const dir of [path.join(packageRoot, 'src'), packageRoot]) {
  if (!fs.existsSync(dir)) continue;

  for (const file of fs.readdirSync(dir)) {
    if (file.endsWith('.node')) {
      binaries.set(file, path.join(dir, file));
    }
  }
}
if (!binaries.size) throw new Error('No native addons found. Run npm run build:native before packaging.');
// Drop only stale generated addons inside dist; freshly built root files win.
for (const file of fs.readdirSync(distDir)) {
  if (file.endsWith('.node') && !binaries.has(file)) fs.unlinkSync(path.join(distDir, file));
}
for (const [file, source] of binaries) fs.copyFileSync(source, path.join(distDir, file));
