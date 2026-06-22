const fs = require('fs');
const path = require('path');

function copyFolderRecursiveSync(source, target) {
  if (!fs.existsSync(target)) {
    fs.mkdirSync(target, { recursive: true });
  }

  if (fs.lstatSync(source).isDirectory()) {
    const files = fs.readdirSync(source);
    files.forEach((file) => {
      const curSource = path.join(source, file);
      const curTarget = path.join(target, file);
      if (fs.lstatSync(curSource).isDirectory()) {
        copyFolderRecursiveSync(curSource, curTarget);
      } else {
        fs.copyFileSync(curSource, curTarget);
      }
    });
  }
}

// Ensure clean dist directory
if (fs.existsSync('dist')) {
  fs.rmSync('dist', { recursive: true, force: true });
}
fs.mkdirSync('dist');

// Copy waitlist build to root dist
if (fs.existsSync('waitlist/dist')) {
  console.log('Copying waitlist build to root dist...');
  copyFolderRecursiveSync('waitlist/dist', 'dist');
} else {
  console.error('Waitlist build not found!');
  process.exit(1);
}

// Copy docs build to dist/docs
if (fs.existsSync('docs/out')) {
  console.log('Copying Next.js docs static export to dist/docs...');
  
  // 1. Create dist/docs folder
  if (!fs.existsSync('dist/docs')) {
    fs.mkdirSync('dist/docs', { recursive: true });
  }

  // 2. Copy all files/folders from docs/out to dist/docs, but flatting the docs/out/docs/ folder
  const docsOutDir = 'docs/out';
  const items = fs.readdirSync(docsOutDir);
  
  items.forEach(item => {
    const srcPath = path.join(docsOutDir, item);
    const destPath = path.join('dist/docs', item);
    
    if (item === 'docs') {
      // Pull contents of docs/out/docs/ up one level directly into dist/docs/
      copyFolderRecursiveSync(srcPath, 'dist/docs');
    } else {
      // Copy other assets (_next, api, logo.svg, etc) into dist/docs/
      if (fs.lstatSync(srcPath).isDirectory()) {
        copyFolderRecursiveSync(srcPath, destPath);
      } else {
        fs.copyFileSync(srcPath, destPath);
      }
    }
  });

  // 3. Make sure visiting /docs works by copying docs.html to dist/docs/index.html
  if (fs.existsSync('docs/out/docs.html')) {
    fs.copyFileSync('docs/out/docs.html', 'dist/docs/index.html');
  }

} else {
  console.error('Docs build not found!');
  process.exit(1);
}


// Copy waitlist/api to root api for Vercel serverless functions
if (fs.existsSync('waitlist/api')) {
  console.log('Copying waitlist serverless functions to root api...');
  if (fs.existsSync('api')) {
    fs.rmSync('api', { recursive: true, force: true });
  }
  copyFolderRecursiveSync('waitlist/api', 'api');
}

console.log('Successfully merged builds into root dist/ directory!');
