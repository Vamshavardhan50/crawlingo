/**
 * Crawlingo: Session configuration with headers, proxy, and rate limits.
 *
 * Usage:
 *   npm install crawlingo
 *   node 05_session.js
 */

const { Session, Page, Dataset } = require('crawlingo');

async function main() {
  console.log('=== Crawlingo Node.js Session Example ===\n');

  const session = new Session();
  session.headers({ 'User-Agent': 'CrawlingoExample/1.0' });
  session.rateLimit(5.0);
  session.timeout(30);
  session.autoMatch(true);

  console.log('Fetching page with session...');
  const page = await Page.create('https://httpbin.org/html', session);

  console.log(`Status: ${page.status}`);
  console.log(`Title: ${page.title()}`);

  console.log('\nSession configured and working.');
}

main();
