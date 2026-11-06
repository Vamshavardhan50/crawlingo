import { test } from 'node:test';
import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { mkdtemp, readFile, mkdir, readdir } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { chromium } from 'playwright';
import { saveSelection, savePreview, validateSelection } from '../export.mjs';
import { installLauncher, validatePageUrl } from '../launcher.mjs';

const fixture = `<!doctype html><html><head><title>Research directory fixture</title><style>
body{margin:0;background:#f2f5f2;color:#20362f;font:17px system-ui;padding:70px 410px 100px 70px;min-height:2200px}
h1{font-size:44px;letter-spacing:-2px}small{letter-spacing:2px;color:#56766b}article{background:white;border:1px solid #c9d7ce;border-radius:12px;padding:28px;margin:24px 0}h2{font-size:23px}a{color:#087c65}.space{height:500px}
</style></head><body><small>LOCAL TEST FIXTURE · NOT PUBLIC DATA</small><h1>Research directory</h1><p>Select a record, choose its fields, and export a dataset job.</p>
<article class="record"><h2>Open methods collection</h2><p class="summary">A fixture for testing research selectors.</p><a href="/next">Read document</a></article>
<article class="record"><h2>Public data handbook</h2><p class="summary">A second independent record.</p><a href="/next">Read document</a></article>
<div class="space"></div><article class="record"><h2>Long-page reference</h2><a href="/next">Read document</a></article>
</body></html>`;

test('export rejects unsafe paths, invalid fields and unsupported CSS', () => {
  const payload = { fields: [{ name: 'title', selector: 'h2', extract_type: 'text', required: true }] };
  assert.equal(validateSelection(payload, 'https://example.com/#anchor').url, 'https://example.com/');
  for (const url of ['file:///etc/passwd', 'https://user:password@example.com/']) assert.throws(() => validateSelection(payload, url));
  assert.throws(() => validateSelection({ fields: [...payload.fields, ...payload.fields] }, 'https://example.com'));
  assert.throws(() => validateSelection({ fields: [{ ...payload.fields[0], selector: 'div > h2' }] }, 'https://example.com'));
  assert.throws(() => validateSelection({ fields: [{ ...payload.fields[0], name: '../escape' }] }, 'https://example.com'));
  for (const value of ['file:///C:/private', 'javascript:alert(1)', 'https://user:pass@example.com']) assert.throws(() => validatePageUrl(value));
});

test('real browser: pick, row scope, scroll, export, browse and navigation', async () => {
  const server = createServer((req, res) => { res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' }); res.end(fixture); });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const base = `http://127.0.0.1:${server.address().port}`;
  const output = await mkdtemp(join(tmpdir(), 'crawlingo-inspector-'));
  let browser;
  try {
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1360, height: 940 } });
    const panel = page.locator('#crawlingo-selector-studio');
    let previewDirectory;
    await page.exposeBinding('__crawlingoSavePreview', async ({ frame }, records) => {
      previewDirectory = await savePreview(output, records, frame.url());
      return 'Saved collected preview.';
    });
    await page.exposeBinding('__crawlingoExport', async ({ frame }, config) => {
      assert.equal(frame, page.mainFrame());
      await saveSelection(output, config, frame.url());
      return 'Saved job.json and urls.txt.';
    });
    await page.addInitScript({ path: fileURLToPath(new URL('../overlay.js', import.meta.url)) });
    const home = await installLauncher(page);
    await page.goto(home);
    assert.equal(await panel.count(), 0);
    await mkdir('test-results', { recursive: true });
    await page.screenshot({ path: 'test-results/studio-start.png' });
    await page.setViewportSize({ width: 390, height: 844 });
    assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth));
    await page.screenshot({ path: 'test-results/studio-start-mobile.png', fullPage: true });
    await page.setViewportSize({ width: 1360, height: 940 });
    await page.getByRole('button', { name: 'Python PEP directory' }).click();
    assert.equal(await page.locator('#url').inputValue(), 'https://peps.python.org/numerical/');
    await page.locator('#url').fill('file:///C:/private');
    await page.getByRole('button', { name: 'Open in Studio' }).click();
    assert.match(await page.locator('#status').textContent(), /HTTP or HTTPS/);
    await page.locator('#url').fill(base);
    await page.getByRole('button', { name: 'Open in Studio' }).click();
    await page.waitForURL(base + '/');
    // Remote sites cannot use the start-screen binding to navigate this session.
    assert.match(await page.evaluate(async () => { try { await window.__crawlingoOpenPage('https://example.com'); } catch (error) { return error.message; } }), /start screen/);
    await panel.locator('#manual > summary').click();
    await page.locator('article h2').first().click();
    assert.equal(await panel.locator('.spider').evaluate(el => getComputedStyle(el).backgroundColor), 'rgba(0, 0, 0, 0)');
    assert.ok(Math.abs(await panel.locator('.spider').evaluate(el => el.getBoundingClientRect().width) - 144) < 0.01);
    await page.waitForFunction(() => {
      const spider = document.querySelector('#crawlingo-selector-studio').shadowRoot.querySelector('.spider').getBoundingClientRect();
      return Math.abs((spider.left + spider.width / 2) - innerWidth / 2) < 1 && Math.abs((spider.top + spider.height / 2) - innerHeight / 2) < 1;
    });
    const firstSelector = await panel.locator('#selector').inputValue();
    assert.equal(await page.locator(firstSelector).count(), 1);
    assert.equal(await page.locator(firstSelector).textContent(), 'Open methods collection');
    // Parent the selected heading into a reusable record container.
    await panel.getByRole('button', { name: 'Select parent' }).click();
    await page.waitForFunction(() => {
      const shadow = document.querySelector('#crawlingo-selector-studio').shadowRoot;
      const reach = shadow.querySelector('.reach');
      return !reach.classList.contains('hidden') && shadow.querySelector('#phase').textContent === 'Gripping';
    });
    assert.equal(await panel.locator('.reach path').count(), 1);
    assert.equal(await panel.locator('.spider .leg.active-reach').count(), 1);
    assert.equal(await panel.locator('.spider .leg').evaluateAll(legs => legs.filter(leg => getComputedStyle(leg).visibility === 'visible').length), 7);
    // The second left leg is closest to this record's bottom boundary.
    assert.equal(await panel.locator('.spider .leg').evaluateAll(legs => legs.findIndex(leg => leg.classList.contains('active-reach'))), 1);
    await mkdir('test-results', { recursive: true });
    await page.screenshot({ path: 'test-results/spider-reaching.png' });
    await panel.getByRole('button', { name: 'Use as rows' }).click();
    assert.equal(await panel.locator('#row').inputValue(), 'article.record');
    assert.match(await panel.locator('#row-status').textContent(), /^3 rows/);
    await page.locator('article h2').first().click();
    assert.equal(await panel.locator('#selector').inputValue(), 'h2');
    assert.match(await panel.locator('#preview').textContent(), /Public data handbook/);
    await panel.getByRole('button', { name: 'Add field' }).click();
    // Link selection must not navigate while picking.
    await page.locator('article a').first().click();
    assert.equal(page.url(), base + '/');
    await panel.locator('#name').fill('link');
    await panel.locator('#extract').selectOption('attr:href');
    assert.match(await panel.locator('#preview').textContent(), /\/next/);
    await panel.getByRole('button', { name: 'Add field' }).click();
    // Changing scope with already saved fields must not silently corrupt their meaning.
    await panel.getByRole('button', { name: 'Clear rows' }).click();
    assert.match(await panel.locator('#status').textContent(), /Remove saved fields/);
    await panel.getByRole('button', { name: 'Browse page' }).click();
    await page.evaluate(() => window.scrollTo(0, 120));
    await page.waitForFunction(() => {
      const shadow = document.querySelector('#crawlingo-selector-studio').shadowRoot;
      const target = document.querySelector('article a').getBoundingClientRect();
      const outline = shadow.querySelector('.outline').getBoundingClientRect();
      const spider = shadow.querySelector('.spider').getBoundingClientRect();
      return Math.abs(target.top - outline.top) < 1 && Math.abs(spider.left + spider.width / 2 - innerWidth / 2) < 1 && Math.abs(spider.top + spider.height / 2 - innerHeight / 2) < 1;
    });
    await panel.getByRole('button', { name: 'Export files' }).click();
    await page.waitForFunction(() => document.querySelector('#crawlingo-selector-studio').shadowRoot.querySelector('#status').textContent.startsWith('Saved'));
    const [directory] = await readdir(output);
    const job = JSON.parse(await readFile(join(output, directory, 'job.json'), 'utf8'));
    assert.equal(job.row_selector, 'article.record');
    assert.deepEqual(job.fields.map(field => [field.name, field.selector, field.extract_type]), [['title', 'h2', 'text'], ['link', 'a', 'attr:href']]);
    assert.equal(await readFile(join(output, directory, 'urls.txt'), 'utf8'), base + '/\n');
    // The export URL and filesystem paths cannot be supplied by the page payload.
    const second = await saveSelection(output, { ...job, output_dir: '../../escape', urls_file: '/private' }, base);
    assert.equal(JSON.parse(await readFile(join(second, 'job.json'), 'utf8')).output_dir, 'datasets/selected');
    // Optional end-to-end check against an already-built Crawlingo CLI.
    if (process.env.CRAWLINGO_CLI) {
      const { stdout } = await promisify(execFile)(process.env.CRAWLINGO_CLI, ['--rate-limit-rps', '10', 'dataset', 'job.json'], { cwd: join(output, directory), timeout: 30000 });
      const report = JSON.parse(stdout);
      assert.equal(report.pages_failed, 0);
      assert.equal(report.records_rejected, 0);
      const records = (await readFile(join(output, directory, 'datasets/selected/part-00000000.jsonl'), 'utf8')).trim().split('\n').map(line => JSON.parse(line));
      assert.deepEqual(records.map(record => record.title), ['Open methods collection', 'Public data handbook', 'Long-page reference']);
      assert.ok(records.every(record => record.link === '/next'));
    }
    await page.evaluate(() => window.scrollTo(0, 0));
    await mkdir('test-results', { recursive: true });
    await page.screenshot({ path: 'test-results/inspector.png' });
    await panel.getByRole('button', { name: 'Remove title', exact: true }).click();
    await panel.getByRole('button', { name: 'Remove link', exact: true }).click();
    await panel.getByRole('button', { name: 'Clear rows', exact: true }).click();
    await panel.locator('#manual > summary').click();
    await panel.locator('#pace').selectOption('quick');
    await panel.locator('#auto').click();
    await page.waitForFunction(() => document.querySelector('#crawlingo-selector-studio').shadowRoot.querySelector('#phase').textContent === 'Gripping');
    await page.screenshot({ path: 'test-results/centered-spider.png' });
    await page.waitForFunction(() => document.querySelector('#crawlingo-selector-studio').shadowRoot.querySelector('#auto-progress').textContent === 'Saved collected preview.');
    const previewData = JSON.parse(await readFile(join(previewDirectory, 'records.json'), 'utf8'));
    assert.deepEqual(previewData.records.map(item => item.title), ['Open methods collection', 'Public data handbook', 'Long-page reference']);
    assert.equal(previewData.method, 'browser-heuristic-preview');
    assert.ok(await page.evaluate(() => scrollY > 0));
    assert.equal(await panel.locator('#record-count').textContent(), '3');
    assert.equal(await panel.locator('#collection-progress').getAttribute('value'), '3');
    await page.screenshot({ path: 'test-results/auto-demo.png' });
    await panel.locator('#toggle-spider').click();
    await page.waitForFunction(() => document.querySelector('#crawlingo-selector-studio').shadowRoot.querySelector('.spider').classList.contains('hidden'));
    await panel.locator('#toggle-spider').click();
    await panel.locator('#auto').click();
    await panel.locator('#stop-auto').click();
    assert.match(await panel.locator('#auto-progress').textContent(), /Stopped/);
    assert.equal(await panel.locator('#auto').isEnabled(), true);
    await page.locator('article a').first().click();
    await page.waitForURL(base + '/next');
    await panel.locator('#manual > summary').click();
    await panel.getByRole('button', { name: 'Pick element' }).waitFor();
    assert.equal(await panel.locator('#fields li').count(), 0);
    assert.equal(await panel.locator('#pick').getAttribute('aria-pressed'), 'true');
    await page.keyboard.press('Escape');
    assert.equal(await panel.locator('#browse').getAttribute('aria-pressed'), 'true');
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await panel.getByRole('button', { name: 'Pick element' }).click();
    await page.locator('article h2').first().click();
    assert.equal(await panel.locator('.reach').isVisible(), false);
    await page.setViewportSize({ width: 390, height: 844 });
    const bounds = await panel.locator('.panel').boundingBox();
    assert.ok(bounds.x >= 0 && bounds.x + bounds.width <= 390);
    const centered = await panel.locator('.spider').boundingBox();
    assert.ok(Math.abs(centered.x + centered.width / 2 - 195) < 1 && Math.abs(centered.y + centered.height / 2 - 422) < 1);
    await panel.getByRole('button', { name: 'Collapse inspector' }).click();
    assert.equal(await panel.locator('.body').isVisible(), false);
    await page.screenshot({ path: 'test-results/inspector-mobile.png' });
  } finally {
    await browser?.close();
    await new Promise(resolve => server.close(resolve));
  }
});
