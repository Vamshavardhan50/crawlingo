const fs = require('fs');
const path = require('path');

const GOLDEN_DIR = path.join(__dirname, '../golden');

class TestRunner {
  constructor() {
    this.results = [];
    this.startTime = Date.now();
  }

  section(title) {
    console.log(`\n\x1b[36m=================================================================\x1b[0m`);
    console.log(`\x1b[36m  ${title}\x1b[0m`);
    console.log(`\x1b[36m=================================================================\x1b[0m`);
  }

  subsection(title) {
    console.log(`\n\x1b[33m  --- ${title} ---\x1b[0m`);
  }

  check(name, passed, detail = '') {
    const status = passed ? 'PASS' : 'FAIL';
    this.results.push({ name, status, detail });
    if (passed) {
      console.log(`  \x1b[32m✔ PASS\x1b[0m — ${name}${detail ? ' (' + detail + ')' : ''}`);
    } else {
      console.log(`  \x1b[31m✘ FAIL\x1b[0m — ${name}${detail ? ' (' + detail + ')' : ''}`);
    }
  }

  missing(name, reason = 'API missing') {
    this.results.push({ name, status: 'NOT_IMPLEMENTED', detail: reason });
    console.log(`  \x1b[35m🚧 NOT IMPLEMENTED\x1b[0m — ${name} (${reason})`);
  }

  partial(name, reason = 'Partially implemented') {
    this.results.push({ name, status: 'PARTIAL', detail: reason });
    console.log(`  \x1b[33m⚠ PARTIAL\x1b[0m — ${name} (${reason})`);
  }

  skipped(name, reason = 'Skipped') {
    this.results.push({ name, status: 'SKIPPED', detail: reason });
    console.log(`  \x1b[37m⏭ SKIPPED\x1b[0m — ${name} (${reason})`);
  }

  experimental(name, passed, detail = '') {
    const status = passed ? 'PASS' : 'FAIL';
    this.results.push({ name, status: 'EXPERIMENTAL', detail });
    if (passed) {
      console.log(`  \x1b[35m🧪 EXPERIMENTAL (PASS)\x1b[0m — ${name}${detail ? ' (' + detail + ')' : ''}`);
    } else {
      console.log(`  \x1b[31m🧪 EXPERIMENTAL (FAIL)\x1b[0m — ${name}${detail ? ' (' + detail + ')' : ''}`);
    }
  }

  toMatchSnapshot(name, actualString) {
    const fileName = `${name.replace(/[^a-z0-9]/gi, '_').toLowerCase()}.txt`;
    const snapshotPath = path.join(GOLDEN_DIR, fileName);

    if (!fs.existsSync(GOLDEN_DIR)) {
      fs.mkdirSync(GOLDEN_DIR, { recursive: true });
    }

    if (!fs.existsSync(snapshotPath)) {
      fs.writeFileSync(snapshotPath, actualString, 'utf-8');
      this.check(`Snapshot Match: ${name}`, true, 'New snapshot generated');
      return;
    }

    const expected = fs.readFileSync(snapshotPath, 'utf-8');
    const matches = expected === actualString;
    this.check(`Snapshot Match: ${name}`, matches, matches ? '' : `Mismatch on snapshot file: ${fileName}`);
  }

  // Feature detection convenience method
  testFeature(name, obj, prop, testFn) {
    if (obj && typeof obj[prop] === 'function') {
      try {
        testFn();
      } catch (e) {
        this.check(name, false, e.message);
      }
    } else {
      this.missing(name, `${prop} method is missing`);
    }
  }

  async testFeatureAsync(name, obj, prop, testFn) {
    if (obj && typeof obj[prop] === 'function') {
      try {
        await testFn();
      } catch (e) {
        this.check(name, false, e.message);
      }
    } else {
      this.missing(name, `${prop} method is missing`);
    }
  }

  get stats() {
    const total = this.results.length;
    const passed = this.results.filter(r => r.status === 'PASS' || r.status === 'EXPERIMENTAL').length;
    const failed = this.results.filter(r => r.status === 'FAIL').length;
    const skipped = this.results.filter(r => r.status === 'SKIPPED').length;
    const notImplemented = this.results.filter(r => r.status === 'NOT_IMPLEMENTED').length;
    const partial = this.results.filter(r => r.status === 'PARTIAL').length;

    return { total, passed, failed, skipped, notImplemented, partial };
  }
}

module.exports = { TestRunner };
