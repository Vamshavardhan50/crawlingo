import { createServer } from 'node:http';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import { chromium } from 'playwright';
import { savePreview, saveSelection } from './export.mjs';

const titles = ['Open research methods', 'Public data handbook', 'Climate observations', 'Digital library index', 'Research software catalog', 'Reproducibility checklist'];
const html = `<!doctype html><html lang="en"><meta charset="utf-8"><title>Crawlingo — interactive spider demo</title>
<style>body{margin:0;padding:64px 410px 120px 64px;background:#f2f6f4;color:#18352b;font:18px/1.6 system-ui}main{max-width:850px}small{font-size:12px;letter-spacing:2px;color:#55766b}h1{font-size:48px;line-height:1.1;letter-spacing:-2px}article{background:white;border:1px solid #cadbd3;border-radius:18px;padding:38px;margin:35px 0;min-height:220px}h2{font-size:27px}a{color:#167b5f}@media(max-width:800px){body{padding:40px 24px}}</style>
<main><small>CRAWLINGO · SYNTHETIC DEMO DATA</small><h1>Watch the spider collect.</h1><p>The browser will scroll through six sample records. Watch the spider reach, catch, and highlight each one. Use Stop to take control.</p>
${titles.map((title, index) => `<article class="record"><small>RECORD ${String(index + 1).padStart(2, '0')}</small><h2>${title}</h2><p>This is example record ${index + 1} for demonstrating browser collection. It is synthetic data, not a real research publication.</p><a href="/record/${index + 1}">View sample record</a></article>`).join('')}</main></html>`;
const server = createServer((request, response) => { response.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' }); response.end(html); });
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
let browser;
try {
  browser = await chromium.launch({ headless: false });
  const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });
  const output = resolve('exports/demo');
  let saving = false;
  const bind = async (name, save) => page.exposeBinding(name, async ({ frame }, payload) => {
    if (frame !== page.mainFrame() || saving) throw new Error('Please wait.');
    saving = true;
    try {
      const directory = await save(output, payload, frame.url());
      console.log('Demo saved:', directory);
      return 'Saved. See the terminal for the folder. Click Watch spider collect to replay.';
    } finally { saving = false; }
  });
  await bind('__crawlingoSavePreview', savePreview);
  await bind('__crawlingoExport', saveSelection);
  await page.addInitScript({ path: fileURLToPath(new URL('./overlay.js', import.meta.url)) });
  await page.goto(`http://127.0.0.1:${server.address().port}`);
  console.log('Demo open. Six synthetic records; automatic collection starts now. Close the browser to exit.');
  await page.locator('#crawlingo-selector-studio').locator('#auto').click();
  page.once('close', () => { void browser.close(); });
  process.once('SIGINT', () => { void browser.close(); });
  process.once('SIGTERM', () => { void browser.close(); });
  await new Promise(resolve => browser.once('disconnected', resolve));
} finally {
  await browser?.close();
  server.close();
}
