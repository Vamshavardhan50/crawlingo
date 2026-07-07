/**
 * Crawlingo: Web page change monitoring with Watch.
 *
 * Usage:
 *   npm install crawlingo
 *   node 04_monitor.js
 */

const { Watch } = require('crawlingo');

async function main() {
  console.log('=== Crawlingo Node.js Monitor Example ===\n');

  const url = 'https://www.rust-lang.org/';
  console.log(`Starting monitor for ${url}...`);

  const watcher = new Watch(url)
    .field('title', 'h1')
    .interval(2);

  watcher.run((err, event) => {
    if (err) {
      console.error('Watcher error:', err);
      return;
    }
    console.log(`\n[EVENT] Field '${event.field}' changed!`);
    console.log(`  Old: '${event.oldValue}'`);
    console.log(`  New: '${event.newValue}'`);
    console.log(`  Type: ${event.changeType}`);
  });

  console.log('Watcher running for 6 seconds...');
  await new Promise(resolve => setTimeout(resolve, 6000));

  console.log('\nStopping watcher...');
  watcher.stop();
  console.log('Monitor stopped.');
}

main();
