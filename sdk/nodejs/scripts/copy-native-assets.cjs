const fs = require('fs');
const path = require('path');

const packageRoot = path.resolve(__dirname, '..');
const distDir = path.join(packageRoot, 'dist');

fs.mkdirSync(distDir, { recursive: true });

for (const file of ['native.js', 'native.d.ts']) {
  fs.copyFileSync(path.join(packageRoot, 'src', file), path.join(distDir, file));
}

for (const dir of [packageRoot, path.join(packageRoot, 'src')]) {
  if (!fs.existsSync(dir)) continue;

  for (const file of fs.readdirSync(dir)) {
    if (file.endsWith('.node')) {
      fs.copyFileSync(path.join(dir, file), path.join(distDir, file));
    }
  }
}
