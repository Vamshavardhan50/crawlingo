/**
 * Sitemap crawling helpers for the Crawlingo Node.js SDK.
 *
 * Pure-TypeScript implementation that parses sitemap XML and seeds a downstream
 * {@link Crawl} with the discovered URLs. No additional native bindings required.
 *
 * @example
 * ```ts
 * import { Session, Sitemap } from 'crawlingo';
 *
 * const session = new Session();
 * const results = await new Sitemap('https://example.com/sitemap.xml', { session })
 *   .field('title', 'h1')
 *   .limit(50)
 *   .build();
 * ```
 */

import { Page } from './page';
import { Session } from './session';
import { Crawl } from './crawl';
import { DatasetResult } from './dataset';

/** A single `<url>` entry from a leaf sitemap `<urlset>`. */
export interface SitemapEntry {
  loc: string;
  lastmod?: string;
  changefreq?: string;
  priority?: string;
}

/** A single `<sitemap>` entry from a `<sitemapindex>`. */
export interface SitemapIndexEntry {
  loc: string;
  lastmod?: string;
}

/** The parsed contents of a sitemap document. */
export type ParsedSitemap =
  | { type: 'urlset'; entries: SitemapEntry[] }
  | { type: 'index'; entries: SitemapIndexEntry[] };

/**
 * Parse raw XML sitemap text and return its entries.
 *
 * Does not perform any network access.
 */
export function parseSitemapXml(xml: string): ParsedSitemap {
  const isIndex = xml.includes('<sitemapindex');
  const isUrlset = xml.includes('<urlset');

  if (!isIndex && !isUrlset) {
    return { type: 'urlset', entries: [] };
  }

  if (isIndex) {
    const entries: SitemapIndexEntry[] = [];
    for (const chunk of splitTags(xml, 'sitemap')) {
      const loc = extractTag(chunk, 'loc');
      if (loc) {
        entries.push({ loc, lastmod: extractTag(chunk, 'lastmod') || undefined });
      }
    }
    return { type: 'index', entries };
  }

  const entries: SitemapEntry[] = [];
  for (const chunk of splitTags(xml, 'url')) {
    const loc = extractTag(chunk, 'loc');
    if (loc) {
      entries.push({
        loc,
        lastmod: extractTag(chunk, 'lastmod') || undefined,
        changefreq: extractTag(chunk, 'changefreq') || undefined,
        priority: extractTag(chunk, 'priority') || undefined,
      });
    }
  }
  return { type: 'urlset', entries };
}

/** Return the canonical `/sitemap.xml` URL for an origin. */
export function sitemapUrlForOrigin(origin: string): string {
  return origin.replace(/\/$/, '') + '/sitemap.xml';
}

/**
 * Fetches and parses a sitemap, then optionally runs a downstream {@link Crawl}.
 */
export class Sitemap {
  private readonly _sitemapUrl: string;
  private readonly _session: Session;
  private _maxDepth = 5;
  private _follow?: string;
  private _limit?: number;
  private _depth?: number;
  private _concurrency?: number;
  private _delay?: number;
  private _fields: Array<{ name: string; selector: string; selectorType: string; defaultValue?: string }> = [];
  private _webhook?: string;

  constructor(sitemapUrl: string, opts: { session?: Session } = {}) {
    this._sitemapUrl = sitemapUrl;
    this._session = opts.session ?? new Session();
  }

  public maxDepth(depth: number): this {
    this._maxDepth = depth;
    return this;
  }

  public follow(selector: string): this {
    this._follow = selector;
    return this;
  }

  public limit(pages: number): this {
    this._limit = pages;
    return this;
  }

  public depth(maxDepth: number): this {
    this._depth = maxDepth;
    return this;
  }

  public concurrency(n: number): this {
    this._concurrency = n;
    return this;
  }

  public delay(seconds: number): this {
    this._delay = seconds;
    return this;
  }

  /** Add a field extraction definition applied to each crawled page. */
  public field(name: string, selector: string, selectorType = 'css', defaultValue?: string): this {
    this._fields.push({ name, selector, selectorType, defaultValue });
    return this;
  }

  public webhook(url: string): this {
    this._webhook = url;
    return this;
  }

  /** Fetch the sitemap and return all discovered URL entries without crawling. */
  public async listUrls(): Promise<SitemapEntry[]> {
    const seen = new Set<string>();
    const entries: SitemapEntry[] = [];
    await this._collect(this._sitemapUrl, 0, seen, entries);
    return entries;
  }

  /**
   * Fetch the sitemap, seed discovered URLs into crawls, run them, and return
   * all collected `DatasetResult[]`.
   */
  public async build(): Promise<DatasetResult[]> {
    const seedEntries = await this.listUrls();
    if (seedEntries.length === 0) return [];

    const allResults: DatasetResult[] = [];

    for (const entry of seedEntries) {
      // Crawl constructor: (startUrl, session?)
      const crawl = new Crawl(entry.loc, this._session);
      if (this._follow) crawl.follow(this._follow);
      crawl.limit(1);
      if (this._depth !== undefined) crawl.depth(this._depth);
      if (this._concurrency !== undefined) crawl.concurrency(this._concurrency);
      if (this._delay !== undefined) crawl.delay(this._delay);
      for (const f of this._fields) {
        // field(name, selector, options?)
        crawl.field(f.name, f.selector, { selectorType: f.selectorType as 'css' | 'xpath', defaultVal: f.defaultValue });
      }
      if (this._webhook) crawl.webhook(this._webhook);
      try {
        const results = await crawl.run();
        allResults.push(...results);
      } catch {
        // Skip individual fetch failures silently.
      }
    }

    return allResults;
  }

  // ── private ────────────────────────────────────────────────────────────────

  private async _fetchXml(url: string): Promise<string> {
    try {
      const page = await Page.create(url, { session: this._session });
      return page.html; // html is a getter, not a method
    } catch {
      return '';
    }
  }

  private async _collect(
    url: string,
    depth: number,
    seen: Set<string>,
    entries: SitemapEntry[],
  ): Promise<void> {
    if (depth > this._maxDepth || seen.has(url)) return;
    seen.add(url);
    const xml = await this._fetchXml(url);
    if (!xml) return;
    const parsed = parseSitemapXml(xml);
    if (parsed.type === 'urlset') {
      entries.push(...parsed.entries);
    } else {
      for (const child of parsed.entries) {
        await this._collect(child.loc, depth + 1, seen, entries);
      }
    }
  }
}

// ── helpers ────────────────────────────────────────────────────────────────────

/** Yield the inner text of each `<tag>…</tag>` block in `text`. */
function* splitTags(text: string, tag: string): Generator<string> {
  const open = `<${tag}`;
  const close = `</${tag}>`;
  let rest = text;
  while (true) {
    const start = rest.indexOf(open);
    if (start === -1) break;
    const gt = rest.indexOf('>', start);
    if (gt === -1) break;
    const closeIdx = rest.indexOf(close, gt + 1);
    if (closeIdx === -1) break;
    yield rest.slice(gt + 1, closeIdx);
    rest = rest.slice(closeIdx + close.length);
  }
}

/** Return the trimmed text of the first `<tag>…</tag>` in `text`, or `''`. */
function extractTag(text: string, tag: string): string {
  const open = `<${tag}>`;
  const close = `</${tag}>`;
  const start = text.indexOf(open);
  if (start !== -1) {
    const end = text.indexOf(close, start + open.length);
    if (end !== -1) return text.slice(start + open.length, end).trim();
  }
  return '';
}
