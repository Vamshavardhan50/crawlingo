/**
 * Sitemap crawling helpers for the Crawlingo Node.js SDK.
 *
 * Uses the native Rust SitemapCrawler under the hood.
 */

import { Session } from './session';
import { DatasetResult } from './dataset';
import { JsSitemap, sitemapUrlForOrigin as _sitemapUrlForOrigin } from './native.js';

/** A single `<url>` entry from a leaf sitemap `<urlset>`. */
export interface SitemapEntry {
  loc: string;
  lastmod?: string;
  changefreq?: string;
  priority?: string;
}

/** Return the canonical `/sitemap.xml` URL for an origin. */
export function sitemapUrlForOrigin(origin: string): string {
  return _sitemapUrlForOrigin(origin);
}

/**
 * Fetches and parses a sitemap, then optionally runs a downstream crawl.
 */
export class Sitemap {
  private readonly inner: JsSitemap;
  private readonly session: Session;

  constructor(sitemapUrl: string, opts: { session?: Session } = {}) {
    this.session = opts.session ?? new Session();
    this.inner = new JsSitemap(sitemapUrl, this.session.inner);
  }

  public maxDepth(depth: number): this {
    this.inner.maxDepth(depth);
    return this;
  }

  public follow(selector: string): this {
    this.inner.follow(selector);
    return this;
  }

  public limit(pages: number): this {
    this.inner.limit(pages);
    return this;
  }

  public depth(maxDepth: number): this {
    this.inner.depth(maxDepth);
    return this;
  }

  public concurrency(n: number): this {
    this.inner.concurrency(n);
    return this;
  }

  public delay(seconds: number): this {
    this.inner.delay(seconds);
    return this;
  }

  /** Add a field extraction definition applied to each crawled page. */
  public field(name: string, selector: string, selectorType = 'css', defaultValue?: string): this {
    this.inner.field(name, selector, selectorType, defaultValue);
    return this;
  }

  public webhook(url: string): this {
    this.inner.webhook(url);
    return this;
  }

  /** Fetch the sitemap and return all discovered URL entries without crawling. */
  public async listUrls(): Promise<SitemapEntry[]> {
    const raw = await this.inner.listUrls();
    return raw.map(e => ({
      loc: e.loc,
      lastmod: e.lastmod ?? undefined,
      changefreq: e.changefreq ?? undefined,
      priority: e.priority ?? undefined,
    }));
  }

  /**
   * Fetch the sitemap, crawl, and return all collected DatasetResult[].
   */
  public async build(): Promise<DatasetResult[]> {
    const raw = await this.inner.run();
    return raw.map(r => new DatasetResult(r));
  }
}
