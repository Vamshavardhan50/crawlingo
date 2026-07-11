/**
 * File and binary download helpers for the Crawlingo Node.js SDK.
 *
 * Uses the native Rust Downloader under the hood.
 */

import { Session } from './session';
import { JsDownloader } from './native.js';

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
 * Downloads files via the session's HTTP stack.
 */
export class Downloader {
  private readonly inner: JsDownloader;
  private readonly session: Session;

  constructor(session?: Session) {
    this.session = session ?? new Session();
    this.inner = new JsDownloader(this.session.inner);
  }

  public allowResume(enabled: boolean): this {
    this.inner.allowResume(enabled);
    return this;
  }

  public maxBytes(n: number): this {
    this.inner.maxBytes(n);
    return this;
  }

  /**
   * Download `url` to a local file at `dest`.
   */
  public async download(url: string, dest: string): Promise<DownloadResult> {
    const res = await this.inner.download(url, dest);
    return {
      url: res.url,
      status: res.status,
      bytesWritten: res.bytesWritten,
      contentType: res.contentType,
      suggestedFilename: res.suggestedFilename ?? undefined,
      resumed: res.resumed,
    };
  }

  /**
   * Download `url` into memory and return `{ result, data }`.
   */
  public async downloadToMemory(url: string): Promise<{ result: DownloadResult; data: Buffer }> {
    const { result, data } = await this.inner.downloadToMemory(url);
    return {
      result: {
        url: result.url,
        status: result.status,
        bytesWritten: result.bytesWritten,
        contentType: result.contentType,
        suggestedFilename: result.suggestedFilename ?? undefined,
        resumed: result.resumed,
      },
      data,
    };
  }
}
