const { Watch } = require('../sdk/nodejs');

async function main() {
  console.log("=== Crawlingo Node.js Web Monitor Example ===");

  const url = "https://www.rust-lang.org/";
  console.log(`Starting monitor for ${url}...`);

  const watcher = new Watch(url)
    .field("title", "h1")
    .interval(2); // Check every 2 seconds

  watcher.run((err, event) => {
    if (err) {
      console.error("Watcher error:", err);
      return;
    }
    console.log(`\n[EVENT] Field '${event.field}' changed!`);
    console.log(`  Old value: '${event.oldValue}'`);
    console.log(`  New value: '${event.newValue}'`);
    console.log(`  Change Type: ${event.changeType}`);
  });

  console.log("Watcher is running in the background. We will stop it after 6 seconds...");
  await new Promise(resolve => setTimeout(resolve, 6000));

  console.log("Stopping watcher...");
  watcher.stop();
  console.log("Monitor stopped successfully.");
}

main();
