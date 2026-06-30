const { Page, Session, Dataset, Crawl, Watch, ElementCollection, Element } = require('../dist');
const path = require('path');

const TIMEOUT = 30;
const AMAZON_SEARCH = 'https://www.amazon.com/s?k=laptop&ref=nb_sb_noss';
const TEST_DIR = __dirname;

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

// ─── Session ──────────────────────────────────────────────────────────────────

async function demoSession() {
  section('1. Session Configuration');
  const session = new Session()
    .headers({ 'Accept-Language': 'en-US,en;q=0.9', 'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36' })
    .timeout(TIMEOUT)
    .rateLimit(2)
    .autoMatch(true)
    .fetcherTier('stealthy')
    .browserProfile('chrome');
  console.log('  Session created and configured');
  return session;
}

// ─── Fetch Page ───────────────────────────────────────────────────────────────

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

// ─── CSS Selectors ────────────────────────────────────────────────────────────

async function demoCssSelectors(page) {
  section('3. CSS Selectors - Extract Products');

  // Amazon rotates class names frequently, so we use resilient selectors
  // Product titles are typically inside h2 > a > span or h2 a
  const titleEls = page.css('h2 a');
  console.log(`  Found ${titleEls.length} product title links via "h2 a"`);

  if (titleEls.length > 0) {
    for (let i = 0; i < Math.min(titleEls.length, 5); i++) {
      const el = titleEls.at(i);
      const txt = el.text.trim().substring(0, 100);
      if (txt) console.log(`  [${i + 1}] ${txt}`);
    }
  } else {
    // Fallback: try broader selectors
    const fallback = page.css('a[href*="/dp/"]');
    console.log(`  Fallback: found ${fallback.length} product links via "a[href*='/dp/']"`);
    for (let i = 0; i < Math.min(fallback.length, 5); i++) {
      console.log(`  [${i + 1}] ${fallback.at(i).text.trim().substring(0, 100)}`);
    }
  }

  // Prices — Amazon uses .a-price with an offscreen span for the full text
  const priceEls = page.css('span.a-price');
  console.log(`\n  Found ${priceEls.length} price containers via "span.a-price"`);

  // Try offscreen price text (full formatted price)
  const offscreenPrices = page.css('span.a-offscreen');
  if (offscreenPrices.length > 0) {
    console.log(`  Found ${offscreenPrices.length} offscreen prices`);
    for (let i = 0; i < Math.min(offscreenPrices.length, 5); i++) {
      console.log(`  [${i + 1}] ${offscreenPrices.text[i] || 'N/A'}`);
    }
  }

  // Star ratings
  const ratings = page.css('i.a-icon-star');
  console.log(`\n  Found ${ratings.length} star rating elements`);

  return { titleEls, priceEls, offscreenPrices, ratings };
}

// ─── XPath Selectors ──────────────────────────────────────────────────────────

async function demoXpathSelectors(page) {
  section('4. XPath Selectors');

  const results = page.xpath('//h2/a');
  console.log(`  Found ${results.length} title links via XPath "//h2/a"`);

  const prices = page.xpath('//span[contains(@class,"a-price")]//text()');
  console.log(`  Found ${prices.length} price texts via XPath`);

  return { results, prices };
}

// ─── Text Selectors ───────────────────────────────────────────────────────────

async function demoTextSelectors(page) {
  section('5. Text Search Selectors');

  const laptopTexts = page.findText('Laptop');
  console.log(`  Found ${laptopTexts.length} elements containing "Laptop"`);

  const afterRatings = page.afterText('out of 5 stars');
  console.log(`  Found ${afterRatings.length} elements after "out of 5 stars"`);

  return { laptopTexts, afterRatings };
}

// ─── Regex Selectors ──────────────────────────────────────────────────────────

async function demoRegexSelectors(page) {
  section('6. Regex Selectors');

  const priceMatches = page.regex('\\$[0-9,]+\\.[0-9]{2}');
  console.log(`  Found ${priceMatches.length} price matches via regex`);
  for (let i = 0; i < Math.min(priceMatches.length, 5); i++) {
    console.log(`  [${i + 1}] ${priceMatches.text[i].trim()}`);
  }

  return priceMatches;
}

// ─── Dataset — Structured Data ────────────────────────────────────────────────

async function demoDataset(page, session) {
  section('7. Dataset — Structured Data Extraction');

  const dataset = new Dataset(AMAZON_SEARCH, session);
  dataset
    .field('title', 'h2 a')
    .field('price', 'span.a-price')
    .field('rating', 'i.a-icon-star')
    .field('url', 'a[href*="/dp/"]');

  const records = dataset.extractStructured(page);
  console.log(`  Extracted ${records.length} structured records`);

  for (let i = 0; i < Math.min(records.length, 3); i++) {
    console.log(`  Record ${i + 1}:`);
    for (const [key, val] of Object.entries(records[i])) {
      console.log(`    ${key}: ${String(val).substring(0, 100)}`);
    }
  }

  if (records.length > 0) {
    const jsonPath = path.join(TEST_DIR, '_amazon_products.json');
    const csvPath = path.join(TEST_DIR, '_amazon_products.csv');
    Dataset.saveJson(records, jsonPath);
    console.log(`  Saved to ${jsonPath}`);
    Dataset.saveCsv(records, csvPath);
    console.log(`  Saved to ${csvPath}`);
  }

  return records;
}

// ─── Dataset.build() — Full Rust Pipeline ─────────────────────────────────────

async function demoDatasetBuild(session) {
  section('8. Dataset.build() — Fetch + Extract in Rust');

  const dataset = new Dataset(AMAZON_SEARCH, session);
  dataset
    .field('title', 'h2 a')
    .field('price', 'span.a-price')
    .field('rating', 'i.a-icon-star');

  try {
    const result = await dataset.build();
    console.log('  Dataset.build() completed');
    const dict = result.toDict();
    console.log(`  Keys: ${Object.keys(dict).join(', ')}`);
    console.log(`  Sample: ${JSON.stringify(dict).substring(0, 200)}`);
    return result;
  } catch (err) {
    console.log(`  Dataset.build() error: ${err.message.substring(0, 100)}`);
  }
}

// ─── Crawl — Multi-page ───────────────────────────────────────────────────────

async function demoCrawl(session) {
  section('9. Crawl — Multi-page Crawling');

  const crawl = new Crawl(AMAZON_SEARCH, session);
  crawl
    .follow('a[href*="/dp/"]')
    .limit(3)
    .depth(1)
    .concurrency(2)
    .delay(2)
    .field('title', 'h2 a')
    .field('price', 'span.a-price');

  console.log('  Crawl configured: follow product links, limit=3, depth=1, concurrency=2');

  try {
    const results = await crawl.run();
    console.log(`  Crawl completed with ${results.length} result(s)`);
    for (let i = 0; i < results.length; i++) {
      console.log(`  Result ${i + 1}: ${JSON.stringify(results[i].toDict()).substring(0, 150)}`);
    }
    return results;
  } catch (err) {
    console.log(`  Crawl skipped: ${err.message.substring(0, 100)}`);
    return [];
  }
}

// ─── Watch — Change Monitoring ────────────────────────────────────────────────

async function demoWatch() {
  section('10. Watch — Change Monitoring');

  const watch = new Watch(AMAZON_SEARCH, new Session());
  watch
    .field('price', 'span.a-price')
    .interval(300);

  console.log('  Watch configured for price monitoring every 300s');
  console.log('  (Stopped immediately to avoid hanging)');
  watch.stop();
  console.log('  Watch stopped');
}

// ─── ElementCollection Iteration ──────────────────────────────────────────────

async function demoElementIteration(page) {
  section('11. ElementCollection Iteration');

  const links = page.css('h2 a');
  console.log(`  Iterating over ${links.length} product links:`);

  let count = 0;
  for (const el of links) {
    if (count >= 5) break;
    const txt = el.text.trim();
    if (txt) {
      console.log(`  [${count + 1}] ${txt.substring(0, 80)}`);
      count++;
    }
  }
}

// ─── Error Handling ───────────────────────────────────────────────────────────

async function demoErrorHandling() {
  section('12. Error Handling');

  try {
    await Page.create('https://this-domain-does-not-exist-12345.com', { timeout: 5 });
  } catch (err) {
    console.log(`  Expected error caught: ${err.message.substring(0, 100)}`);
  }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

async function main() {
  console.log('Crawlingo Node.js SDK — Amazon Scraper Demo');
  console.log('============================================');
  console.log(`Started at: ${new Date().toISOString()}`);

  let session, page;

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

    await demoWatch();
    await sleep(1);

    await demoElementIteration(page);
    await sleep(1);

    await demoErrorHandling();

    section('SUMMARY');
    console.log('  All demo functions completed successfully!');
    console.log(`Finished at: ${new Date().toISOString()}`);
  } catch (err) {
    console.error(`\nFATAL: ${err.message}`);
    if (err.stack) console.error(err.stack.split('\n').slice(0, 4).join('\n'));
    process.exit(1);
  }
}

main();
