const { Dataset } = require('../sdk/nodejs');

async function main() {
  console.log("=== Crawlingo Node.js Structured Dataset Example ===");

  const url = "https://www.rust-lang.org/";
  console.log(`Building dataset query for ${url}...`);

  try {
    // Fluent API definition
    const dataset = new Dataset(url)
      .autoMatch(true) // Learns element DOM fingerprint for self-healing
      .field("title", "h1")
      .field("tagline", "header p");

    const result = await dataset.build();

    console.log("\nExtracted Fields:");
    const resultDict = result.toDict();
    for (const [field, value] of Object.entries(resultDict)) {
      console.log(`  ${field}: ${value.trim().slice(0, 80)}...`);
    }

    // Export results
    console.log("\nExporting results...");
    await result.toJson("dataset_result.json");
    await result.toCsv("dataset_result.csv");
    console.log("Created 'dataset_result.json' and 'dataset_result.csv'.");
  } catch (error) {
    console.error("An error occurred during dataset build:", error);
  }
}

main();
