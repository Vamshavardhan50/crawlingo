const fs = require('node:fs/promises');
const { chromium } = require('playwright');

async function main() {
  const urls = (await fs.readFile('urls.txt', 'utf8')).trim().split(/\r?\n/);
  const browser = await chromium.launch();
  try {
    const context = await browser.newContext();
    const rows = [];
    let next = 0;
    async function worker() {
      const page = await context.newPage();
      while (next < urls.length) {
        const url = urls[next++];
        const response = await page.goto(url, { waitUntil: 'load' });
        if (response.status() !== 200) throw Error(`Unexpected status ${response.status()}`);
        const title = await page.locator('article h1').textContent();
        rows.push(JSON.stringify({ _source_url: page.url(), title }));
      }
    }
    await Promise.all(Array.from({ length: Number(process.argv[2]) }, worker));
    await fs.writeFile('playwright.jsonl', rows.join('\n') + '\n');
    await fs.writeFile('versions.json', JSON.stringify({ chromium: browser.version(), playwright: require('playwright/package.json').version }));
  } finally { await browser.close(); }
}
main().catch(error => { console.error(error); process.exitCode = 1; });
