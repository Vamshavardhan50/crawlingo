import { pathToFileURL, fileURLToPath } from 'node:url';

export function validatePageUrl(value) {
  if (typeof value !== 'string' || value.length > 8192) throw new Error('Enter a valid page URL.');
  const url = new URL(value.trim());
  if (!['http:', 'https:'].includes(url.protocol) || url.username || url.password) throw new Error('Use an HTTP or HTTPS URL without embedded credentials.');
  return url.href;
}

export async function installLauncher(page) {
  const home = pathToFileURL(fileURLToPath(new URL('./studio.html', import.meta.url))).href;
  await page.exposeBinding('__crawlingoOpenPage', async ({ frame }, value) => {
    if (frame !== page.mainFrame() || frame.url().split('#')[0] !== home) throw new Error('Open pages from the Studio start screen.');
    const url = validatePageUrl(value);
    await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 60000 });
  });
  return home;
}
