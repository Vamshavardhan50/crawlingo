#!/usr/bin/env node
import { chromium } from 'playwright';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import { saveSelection, savePreview } from './export.mjs';
import { installLauncher, validatePageUrl } from './launcher.mjs';

const args = process.argv.slice(2);
if (args.includes('--help')) {
  console.log('Usage: npm start [-- https://example.com [--output ./exports]]\nWithout a URL, opens the Studio start screen. Escape toggles browse mode.');
  process.exit(0);
}
const url = args[0] ? validatePageUrl(args[0]) : null;
if (args.length > 1 && (args[1] !== '--output' || !args[2] || args.length !== 3)) throw new Error('Expected --output DIRECTORY');
const output = resolve(args[2] || 'exports');
const browser = await chromium.launch({ headless: false });
const context = await browser.newContext({ viewport: null });
const page = await context.newPage();
const home = await installLauncher(page);
let closing = false;
const close = async () => { if (!closing) { closing = true; await browser.close(); } };
process.once('SIGINT', close);
process.once('SIGTERM', close);
page.once('close', close);
let saving = false;
let lastSave = 0;
await page.exposeBinding('__crawlingoExport', async ({ frame }, payload) => {
  if (frame !== page.mainFrame()) throw new Error('Only the main page can export.');
  if (saving || Date.now() - lastSave < 1500) throw new Error('Please wait before exporting again.');
  saving = true;
  lastSave = Date.now();
  try {
    const directory = await saveSelection(output, payload, frame.url());
    console.log(`Saved job.json and urls.txt to ${directory}`);
    return 'Saved job.json and urls.txt. See the terminal for the folder.';
  } finally { saving = false; }
});
await page.addInitScript({ path: fileURLToPath(new URL('./overlay.js', import.meta.url)) });
await page.exposeBinding('__crawlingoSavePreview', async ({ frame }, payload) => {
  if (frame !== page.mainFrame() || saving || Date.now() - lastSave < 1500) throw new Error('Please wait before saving again.');
  saving = true; lastSave = Date.now();
  try {
    const directory = await savePreview(output, payload, frame.url());
    console.log(`Collected preview: ${directory}\\records.json`);
    return 'Saved records.json. See the terminal for its folder.';
  } finally { saving = false; }
});
console.log('Crawlingo Selector Studio — rendered-page preview. Escape toggles selection. Close the browser to exit.');
try {
  await page.goto(url || home, { waitUntil: 'domcontentloaded', timeout: 60000 });
}
catch (error) { console.error('Navigation did not complete:', error.message); }
