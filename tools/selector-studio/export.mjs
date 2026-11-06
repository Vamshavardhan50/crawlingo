import { mkdir, writeFile } from 'node:fs/promises';
import { resolve, join } from 'node:path';
import { randomUUID } from 'node:crypto';

export function validateSelection(payload, sourceUrl) {
  const url = new URL(sourceUrl);
  if (!['http:', 'https:'].includes(url.protocol)) throw new Error('Open an HTTP or HTTPS page first.');
  url.hash = '';
  if (url.username || url.password) throw new Error('Credential-bearing URLs cannot be exported.');
  if (!payload || !Array.isArray(payload.fields) || payload.fields.length < 1 || payload.fields.length > 50) {
    throw new Error('Choose between 1 and 50 fields.');
  }
  const selector = value => {
    if (typeof value !== 'string' || !value.trim() || value.length > 2048) throw new Error('Invalid selector.');
    // Deliberately use only the conservative subset offered by the picker.
    const atom = /^(?:[a-zA-Z][\w-]*|\*)?(?:(?:[.#][a-zA-Z_][\w-]*)|(?::nth-child\([1-9][0-9]*\)))*$/;
    if (!value.trim().split(/\s+/).every(part => atom.test(part))) throw new Error('Use tags, classes, IDs, spaces and positive :nth-child(N).');
    return value.trim();
  };
  const names = new Set();
  const fields = payload.fields.map(field => {
    if (!field || typeof field.name !== 'string' || !/^[a-zA-Z][a-zA-Z0-9_]{0,63}$/.test(field.name) || names.has(field.name)) throw new Error('Field names must be unique identifiers starting with a letter.');
    names.add(field.name);
    if (field.extract_type !== 'text' && !/^attr:[a-zA-Z_][\w:.-]*$/.test(field.extract_type)) throw new Error('Invalid extraction type.');
    return { name: field.name, selector: selector(field.selector), extract_type: field.extract_type, required: field.required === true };
  });
  return {
    url: url.href,
    config: { urls_file: 'urls.txt', output_dir: 'datasets/selected', ...(payload.row_selector ? { row_selector: selector(payload.row_selector) } : {}), fields, concurrency: 2, respect_robots: true, format: 'jsonl' },
  };
}

export async function saveSelection(base, payload, sourceUrl) {
  const { url, config } = validateSelection(payload, sourceUrl);
  // Only these two fixed filenames are accepted; remote page input never chooses a path.
  const directory = resolve(base, 'selection-' + randomUUID());
  await mkdir(directory, { recursive: true });
  await writeFile(join(directory, 'job.json'), JSON.stringify(config, null, 2) + '\n', { flag: 'wx' });
  await writeFile(join(directory, 'urls.txt'), url + '\n', { flag: 'wx' });
  return directory;
}

export async function savePreview(base, payload, sourceUrl) {
  const url = new URL(sourceUrl);
  if (!['http:', 'https:'].includes(url.protocol) || url.username || url.password) throw new Error('Invalid page URL.');
  if (!Array.isArray(payload) || payload.length < 1 || payload.length > 30) throw new Error('Expected 1–30 preview records.');
  const records = payload.map(record => {
    const result = {};
    for (const key of ['title', 'text', 'url']) {
      if (typeof record?.[key] !== 'string' || record[key].length > 12000) throw new Error('Invalid preview field.');
      result[key] = record[key];
    }
    return result;
  });
  const directory = resolve(base, 'preview-' + randomUUID());
  await mkdir(directory, { recursive: true });
  await writeFile(join(directory, 'records.json'), JSON.stringify({ source_url: url.href, collected_at: new Date().toISOString(), method: 'browser-heuristic-preview', records }, null, 2) + '\n', { flag: 'wx' });
  return directory;
}
