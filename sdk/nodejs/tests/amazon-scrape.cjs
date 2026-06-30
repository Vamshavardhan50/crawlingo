const { Page, Session, Dataset, Crawl, Watch, ElementCollection, Element } = require('../dist');

const TIMEOUT = 30;
const AMAZON_SEARCH = 'https://www.amazon.com/s?k=laptop&ref=nb_sb_noss';

async function pause(ms) {
  return new Promise(r => setTimeout(r, ms));
}

async function sleep(seconds) {
  await pause(seconds * 1000);
}

function section(title) {
  console.log(`\n${'='.repeat(60)}`);
  console.log(`  ${title}`);
  console.log(`${'='.repeat(60)}`);
}

async function demoSession() {
  section('1. Session Configuration');
  const session = new Session()
    .headers({ 'Accept-Language': 'en-US,en;q=0.9' })
    .timeout(TIMEOUT)
    .rateLimit(2)
    .autoMatch(true)
    .fetcherTier('stealthy')
    .browserProfile('chrome');
  console.log('  Session created and configured with headers, rate limit, stealth tier');
  return session;
}

async function demoFetchPage(session) {
  section('2. Fetch Page (Amazon Search Results)');
  console.log(`  Fetching: ${AMAZON_SEARCH}`);

  const page = await Page.create(AMAZON_SEARCH, { session, timeout: TIMEOUT });

  console.log(`  URL:     ${page.url}`);
  console.log(`  Status:  ${page.status}`);
  console.log(`  Title:   ${page.title()}`);
  console.log(`  HTML:    ${page.html.length} chars`);

  return page;
}

async function demoCssSelectors(page) {
  section('3. CSS Selectors - Extract Products');

  const titles = page.css('span.a-size-medium');
  console.log(`  Found ${titles.length} product titles via CSS`);

  for (let i = 0; i < Math.min(titles.length, 5); i++) {
    const el = titles.at(i);
    console.log(`  [${i + 1}] ${el.text.trim().substring(0, 80)}`);
  }

  const prices = page.css('span.a-price-whole');
  console.log(`\n  Found ${prices.length} price elements`);
  for (let i = 0; i < Math.min(prices.length, 5); i++) {
    console.log(`  [${i + 1}] $${prices.text[i] || 'N/A'}`);
  }

  return { titles, prices };
}

async function demoXpathSelectors(page) {
  section('4. XPath Selectors');

  const results = page.xpath('//span[contains(@class, "a-price-whole")]/text()');
  console.log(`  Found ${results.length} prices via XPath`);
  for (let i = 0; i < Math.min(results.length, 3); i++) {
    console.log(`  [${i + 1}] $${results.text[i].trim()}`);
  }

  return results;
}

async function demoTextSelectors(page) {
  section('5. Text Search Selectors');

  const laptopTexts = page.findText('Laptop');
  console.log(`  Found ${laptopTexts.length} elements containing "Laptop"`);

  const ratingLabel = page.afterText('out of 5 stars');
  console.log(`  Found ${ratingLabel.length} rating labels via afterText`);

  return { laptopTexts, ratingLabel };
}

async function demoRegexSelectors(page) {
  section('6. Regex Selectors');

  const priceMatches = page.regex('\\$[0-9,]+\\.[0-9]{2}');
  console.log(`  Found ${priceMatches.length} price matches via regex`);
  for (let i = 0; i < Math.min(priceMatches.length, 5); i++) {
    console.log(`  [${i + 1}] ${priceMatches.text[i].trim()}`);
  }

  return priceMatches;
}

async function demoDataset(page, session) {
  section('7. Dataset - Structured Data Extraction');

  const dataset = new Dataset(AMAZON_SEARCH, session);
  dataset
    .field('title', 'span.a-size-medium')
    .field('price', 'span.a-price-whole')
    .field('rating', 'span.a-icon-alt')
    .field('url', 'a.a-link-normal.a-text-normal');

  const records = dataset.extractStructured(page);
  console.log(`  Extracted ${records.length} structured records`);

  for (let i = 0; i < Math.min(records.length, 3); i++) {
    console.log(`  Record ${i + 1}:`);
    for (const [key, val] of Object.entries(records[i])) {
      console.log(`    ${key}: ${val.substring(0, 100)}`);
    }
  }

  if (records.length > 0) {
    Dataset.saveJson(records, '/tmp/amazon-products.json');
    console.log('  Saved to /tmp/amazon-products.json');
    Dataset.saveCsv(records, '/tmp/amazon-products.csv');
    console.log('  Saved to /tmp/amazon-products.csv');
  }

  return records;
}

async function demoDatasetBuild(session) {
  section('8. Dataset.build() - Fetch + Extract in Rust');

  const dataset = new Dataset(AMAZON_SEARCH, session);
  dataset
    .field('title', 'span.a-size-medium')
    .field('price', 'span.a-price-whole')
    .field('rating', 'span.a-icon-alt');

  const result = await dataset.build();
  console.log('  Dataset.build() completed');
  const dict = result.toDict();
  console.log(`  Dict keys: ${Object.keys(dict).join(', ')}`);
  console.log(`  Build result: ${JSON.stringify(dict).substring(0, 200)}`);

  return result;
}

async function demoCrawl(session) {
  section('9. Crawl - Multi-page Crawling');

  const crawl = new Crawl(AMAZON_SEARCH, session);
  crawl
    .follow('a.a-link-normal.s-no-outline')
    .limit(3)
    .depth(1)
    .concurrency(2)
    .delay(2)
    .field('title', 'span.a-size-medium')
    .field('price', 'span.a-price-whole');

  console.log('  Crawl configured with follow, limit=3, depth=1, concurrency=2, delay=2s');

  crawl.schedule(3600);
  console.log('  Scheduled for hourly re-crawl');

  try {
    const results = await crawl.run();
    console.log(`\n  Crawl completed with ${results.length} result(s)`);
    for (let i = 0; i < results.length; i++) {
      console.log(`  Result ${i + 1}: ${JSON.stringify(results[i].toDict()).substring(0, 150)}`);
    }
    return results;
  } catch (err) {
    console.log(`  Crawl skipped (expected on restrictive sites): ${err.message}`);
    return [];
  }
}

async function demoWatch(session) {
  section('10. Watch - Change Monitoring');

  const watch = new Watch(AMAZON_SEARCH, session);
  watch
    .field('price', 'span.a-price-whole')
    .interval(300);

  console.log('  Watch configured for price monitoring every 300s');
  console.log('  (Stopping immediately without running to avoid hanging)');
  watch.stop();
  console.log('  Watch stopped');
}

async function demoElementIteration(page) {
  section('11. ElementCollection Iteration');

  const titles = page.css('span.a-size-medium');
  console.log(`  Iterating over ${titles.length} title elements:`);

  let count = 0;
  for (const el of titles) {
    if (count >= 3) break;
    console.log(`  [${count + 1}] ${el.text.trim().substring(0, 80)}`);
    count++;
  }
}

async function demoErrorHandling() {
  section('12. Error Handling');

  const session = new Session().timeout(5);

  try {
    await Page.create('https://this-domain-does-not-exist-12345.com', { session, timeout: 5 });
  } catch (err) {
    console.log(`  Expected error handled: ${err.message.substring(0, 100)}`);
  }
}

async function main() {
  console.log('Crawlingo Node.js SDK - Amazon Scraper Demo');
  console.log('============================================');
  console.log(`Started at: ${new Date().toISOString()}`);

  let session;
  let page;

  try {
    session = await demoSession();
    await sleep(1);

    page = await demoFetchPage(session);
    await sleep(1);

    await demoCssSelectors(page);
    await sleep(1);

    await demoXpathSelectors(page);
    await sleep(1);

    await demoTextSelectors(page);
    await sleep(1);

    await demoRegexSelectors(page);
    await sleep(1);

    await demoDataset(page, session);
    await sleep(1);

    await demoDatasetBuild(session);
    await sleep(1);

    await demoCrawl(session);
    await sleep(1);

    await demoWatch(session);
    await sleep(1);

    await demoElementIteration(page);
    await sleep(1);

    await demoErrorHandling();

    section('SUMMARY');
    console.log('  All demo functions completed successfully!');
    console.log(`Finished at: ${new Date().toISOString()}`);
  } catch (err) {
    console.error(`\nFATAL ERROR: ${err.message}`);
    if (err.stack) {
      console.error(err.stack.split('\n').slice(0, 4).join('\n'));
    }
    process.exit(1);
  }
}

main();
