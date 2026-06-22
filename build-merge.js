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
if (fs.existsSync('docs/dist')) {
  console.log('Copying docs build to dist/docs...');
  copyFolderRecursiveSync('docs/dist', 'dist/docs');
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
