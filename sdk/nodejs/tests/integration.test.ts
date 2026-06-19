import { Page, Dataset, Crawl, Session } from '../dist/index';

async function runTests() {
  console.log("Starting Node.js Integration Tests...");

  // Test 1: Page fetching
  console.log("\n1. Testing Page fetching...");
  const page = await Page.create("https://httpbin.org/html");
  console.log(`Page Title: ${page.title()}`);
  if (page.status !== 200 && page.status !== 503) {
    throw new Error(`Unexpected status code: ${page.status}`);
  }

  // Test 2: Selectors
  console.log("\n2. Testing Selectors...");
  const titleEl = page.css("h1");
  console.log(`h1 text: ${titleEl.text}`);

  // Test 3: Dataset Builder
  console.log("\n3. Testing Dataset builder...");
  const dataset = new Dataset("https://httpbin.org/html")
    .field("title", "h1");
  const result = await dataset.build();
  console.log("Dataset result:", result.toDict());

  // Test 4: Crawl
  console.log("\n4. Testing Crawl...");
  const crawl = new Crawl("https://httpbin.org/links/2/0")
    .follow("a")
    .limit(2)
    .field("title", "h1");
  const crawlRes = await crawl.run();
  console.log(`Crawled ${crawlRes.length} pages.`);

  console.log("\nNode.js integration tests completed successfully!");
}

runTests().catch(err => {
  console.error("Test failure:", err);
  process.exit(1);
});
