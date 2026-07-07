/**
 * Crawlingo: Structured data extraction with Dataset.
 *
 * Usage:
 *   npm install crawlingo
 *   node 02_dataset.js
 */

const { Dataset } = require('crawlingo');

async function main() {
  console.log('=== Crawlingo Node.js Dataset Example ===\n');

  const url = 'https://www.rust-lang.org/';
  console.log(`Building dataset query for ${url}...`);

  try {
    const result = await new Dataset(url)
      .autoMatch(true)
      .field('title', 'h1')
      .field('tagline', 'header p')
      .build();

    console.log('\nExtracted Fields:');
    const dict = result.toDict();
    for (const [field, value] of Object.entries(dict)) {
      console.log(`  ${field}: ${value.trim().slice(0, 80)}...`);
    }

    await result.toJson('dataset_result.json');
    await result.toCsv('dataset_result.csv');
    console.log('\nCreated dataset_result.json and dataset_result.csv');
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
