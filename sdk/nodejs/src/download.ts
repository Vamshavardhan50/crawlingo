/**
 * File and binary download helpers for the Crawlingo Node.js SDK.
 *
 * @example
 * ```ts
 * import { Session, Downloader } from 'crawlingo';
 *
 * const session = new Session();
 * const dl = new Downloader(session);
 * const { result } = await dl.downloadToMemory('https://example.com/data.json');
 * console.log(result.contentType, result.bytesWritten);
 * ```
 */

import * as fs from 'fs';
import { Page } from './page';
import { Session } from './session';

/** The result of a completed download. */
export interface DownloadResult {
  /** The final URL after any redirects. */
  url: string;
  /** HTTP response status code. */
  status: number;
  /** Number of bytes written to the output. */
  bytesWritten: number;
  /** MIME type from `Content-Type`, or `application/octet-stream`. */
  contentType: string;
  /** Filename hint from `Content-Disposition` or the URL's last path segment. */
  suggestedFilename?: string;
  /** `true` if the server honored a `Range:` request and returned `206`. */
  resumed: boolean;
}

/**
 * Downloads files via the session's HTTP stack, sharing rate limiting, retry,
 * caching, and auth configuration with the rest of the engine.
 */
export class Downloader {
  private readonly _session: Session;
  private _allowResume = true;
  private _maxBytes?: number;

  constructor(session?: Session) {
    this._session = session ?? new Session();
  }

  public allowResume(enabled: boolean): this {
    this._allowResume = enabled;
    return this;
  }

  public maxBytes(n: number): this {
    this._maxBytes = n;
    return this;
  }

  /**
   * Download `url` to a local file at `dest`.
   *
   * If `dest` already exists and resume is enabled, a `Range: bytes=<size>-` request
   * is sent. A `206` response causes the file to be appended; a `200` (no Range support)
   * causes the file to be overwritten from the beginning.
   */
  public async download(url: string, dest: string): Promise<DownloadResult> {
    let offset = 0;
    if (this._allowResume && fs.existsSync(dest)) {
      offset = fs.statSync(dest).size;
    }

    const extraHeaders: Record<string, string> = {};
    if (this._allowResume && offset > 0) {
      extraHeaders['Range'] = `bytes=${offset}-`;
    }

    const { body, status, finalUrl } = await this._fetchRaw(url, extraHeaders);
    const resumed = status === 206;
    const contentType = 'application/octet-stream';
    const suggestedFilename = extractFilename(finalUrl);

    let bytes = body;
    if (this._maxBytes !== undefined) bytes = bytes.slice(0, this._maxBytes);

    const flag = resumed ? 'a' : 'w';
    fs.writeFileSync(dest, bytes, { flag });

    return {
      url: finalUrl,
      status,
      bytesWritten: bytes.length,
      contentType,
      suggestedFilename,
      resumed,
    };
  }

  /**
   * Download `url` into memory and return `{ result, data }`.
   */
  public async downloadToMemory(url: string): Promise<{ result: DownloadResult; data: Buffer }> {
    const { body, status, finalUrl } = await this._fetchRaw(url, {});
    let bytes = body;
    if (this._maxBytes !== undefined) bytes = bytes.slice(0, this._maxBytes);

    return {
      result: {
        url: finalUrl,
        status,
        bytesWritten: bytes.length,
        contentType: 'application/octet-stream',
        suggestedFilename: extractFilename(finalUrl),
        resumed: false,
      },
      data: bytes,
    };
  }

  // ── private ────────────────────────────────────────────────────────────────

  private async _fetchRaw(
    url: string,
    extraHeaders: Record<string, string>,
  ): Promise<{ body: Buffer; status: number; finalUrl: string }> {
    // Apply extra headers (e.g. Range) to the session temporarily.
    if (Object.keys(extraHeaders).length > 0) {
      this._session.headers(extraHeaders);
    }

    let html = '';
    let status = 200;
    let finalUrl = url;

    try {
      // Page.create is the static async factory; html is a getter.
      const page = await Page.create(url, { session: this._session });
      html = page.html;  // getter, not a method call
      status = page.status;
      finalUrl = page.url;
    } finally {
      if (Object.keys(extraHeaders).length > 0) {
        // Restore by clearing the extra headers.
        this._session.headers({});
      }
    }

    return {
      body: Buffer.from(html, 'utf-8'),
      status,
      finalUrl,
    };
  }
}

// ── helpers ────────────────────────────────────────────────────────────────────

function extractFilename(url: string): string | undefined {
  try {
    const segments = new URL(url).pathname.split('/').filter(Boolean);
    return segments.at(-1);
  } catch {
    return undefined;
  }
}
