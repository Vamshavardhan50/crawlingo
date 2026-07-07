/**
 * Crawlingo: Multi-page web crawler with field extraction.
 *
 * Usage:
 *   npm install crawlingo
 *   node 03_crawler.js
 */

const { Crawl } = require('crawlingo');

async function main() {
  console.log('=== Crawlingo Node.js Crawler Example ===\n');

  const startUrl = 'https://httpbin.org/links/5/0';
  console.log(`Starting crawl from ${startUrl}...`);

  try {
    const results = await new Crawl(startUrl)
      .follow('a')
      .limit(3)
      .depth(2)
      .concurrency(2)
      .delay(0.5)
      .field('title', 'h1')
      .run();

    console.log(`\nCrawled ${results.length} pages:`);
    for (let i = 0; i < results.length; i++) {
      const dict = results[i].toDict();
      console.log(`  Page ${i + 1}: ${dict.url} — title: '${dict.title || 'N/A'}'`);
    }
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
