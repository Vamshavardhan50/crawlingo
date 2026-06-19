const { Crawl } = require('../sdk/nodejs');

async function main() {
  console.log("=== Crawlingo Node.js Web Crawler Example ===");

  const startUrl = "https://httpbin.org/links/5/0";
  console.log(`Starting crawl from ${startUrl}...`);

  try {
    const crawlJob = new Crawl(startUrl)
      .follow("a")
      .limit(3)
      .depth(2)
      .concurrency(2)
      .delay(0.5)
      .field("title", "h1");

    const results = await crawlJob.run();

    console.log(`\nCrawled ${results.length} pages:`);
    for (let i = 0; i < results.length; i++) {
      const res = results[i].toDict();
      console.log(`  Page ${i + 1} title: '${res.title || "No Title"}'`);
    }
  } catch (error) {
    console.error("An error occurred during crawler execution:", error);
  }
}

main();
