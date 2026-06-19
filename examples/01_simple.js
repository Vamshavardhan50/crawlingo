const { Page } = require('../sdk/nodejs');

async function main() {
  console.log("=== Crawlingo Node.js Simple Extraction Example ===");

  const url = "https://httpbin.org/html";
  console.log(`Fetching ${url}...`);

  try {
    const page = await Page.create(url);

    console.log(`\nResponse Status: ${page.status}`);
    console.log(`Page Title: '${page.title()}'`);

    // CSS Selector
    console.log("\n--- CSS Selector (h1) ---");
    const h1 = page.css("h1");
    console.log(`Found ${h1.length} h1 elements. Text: '${h1.text.join(", ")}'`);

    // XPath Selector
    console.log("\n--- XPath Selector (//p) ---");
    const paragraphs = page.xpath("//p");
    for (let i = 0; i < paragraphs.length; i++) {
      console.log(`Paragraph ${i + 1}: '${paragraphs.at(i).text}'`);
    }

    // Text Anchor
    console.log("\n--- Text Anchors ---");
    const melvilleEl = page.findText("Herman Melville");
    console.log(`Found 'Herman Melville': ${melvilleEl.length > 0}`);
    if (melvilleEl.length > 0) {
      console.log(`Outer HTML: ${melvilleEl.first().html}`);
    }
  } catch (error) {
    console.error("An error occurred during page fetch:", error);
  }
}

main();
