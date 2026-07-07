/**
 * Crawlingo: Dataset export with streaming and schema.
 *
 * Usage:
 *   npm install crawlingo
 *   node 06_dataset_advanced.js
 */

const { Dataset } = require('crawlingo');

async function main() {
  console.log('=== Crawlingo Node.js Advanced Dataset Example ===\n');

  const urls = [
    'https://www.rust-lang.org/',
    'https://httpbin.org/html',
    'https://example.com/',
  ];

  // Extract from each URL
  for (const url of urls) {
    console.log(`Processing ${url}...`);
    try {
      const result = await new Dataset(url)
        .autoMatch(true)
        .field('title', 'h1')
        .field('description', 'p')
        .build();

      const dict = result.toDict();
      console.log(`  Title: ${dict.title?.trim().slice(0, 60)}`);
    } catch (err) {
      console.log(`  Failed: ${err.message}`);
    }
  }
}

main();
