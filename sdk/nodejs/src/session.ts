import { JsSession } from './native.js';
import { callNative } from './errors';

export class Session {
  public readonly inner: JsSession;

  constructor(inner?: JsSession) {
    this.inner = inner ?? new JsSession();
  }

  /** Load TOML/JSON and CRAWLINGO_* environment overrides with Rust validation. */
  public static fromConfig(path?: string): Session {
    return new Session(callNative(() => JsSession.fromConfig(path)));
  }

  public headers(headers: Record<string, string>): this {
    callNative(() => this.inner.headers(headers));
    return this;
  }

  public cookies(cookies: Record<string, string>): this {
    callNative(() => this.inner.cookies(cookies));
    return this;
  }

  /** Snapshot of explicit session defaults; this is not a browser cookie jar. */
  public getCookies(): Record<string, string> {
    return callNative(() => this.inner.getCookies());
  }

  /** Remove a default cookie. A missing name is a no-op. */
  public deleteCookie(name: string): this {
    callNative(() => this.inner.deleteCookie(name));
    return this;
  }

  /** Remove all default cookies. */
  public clearCookies(): this {
    callNative(() => this.inner.clearCookies());
    return this;
  }

  /** Additional attempts after the initial request. Zero disables retries. */
  public retries(count: number): this {
    if (!Number.isInteger(count) || count < 0 || count > 0xffffffff) {
      throw new RangeError('retries must be an integer between 0 and 4294967295');
    }
    callNative(() => this.inner.retries(count));
    return this;
  }

  /** Exponential delay multiplier, finite and >= 1. */
  public retryBackoff(factor: number): this {
    if (!Number.isFinite(factor) || factor < 1) {
      throw new RangeError('retryBackoff must be finite and >= 1');
    }
    callNative(() => this.inner.retryBackoff(factor));
    return this;
  }

  /** Initial delay in whole seconds (0–86400); Retry-After takes precedence. */
  public retryDelay(seconds: number): this {
    if (!Number.isInteger(seconds) || seconds < 0 || seconds > 86400) {
      throw new RangeError('retryDelay must be an integer between 0 and 86400 seconds');
    }
    callNative(() => this.inner.retryDelay(seconds));
    return this;
  }

  public proxy(proxyUrl: string): this {
    callNative(() => this.inner.proxy(proxyUrl));
    return this;
  }

  public rateLimit(requestsPerSecond: number): this {
    callNative(() => this.inner.rateLimit(requestsPerSecond));
    return this;
  }

  public autoMatch(enabled: boolean): this {
    callNative(() => this.inner.autoMatch(enabled));
    return this;
  }

  public timeout(seconds: number): this {
    callNative(() => this.inner.timeout(seconds));
    return this;
  }

  public fingerprintPath(path: string): this {
    callNative(() => this.inner.fingerprintPath(path));
    return this;
  }

  public fetcherTier(tier: 'standard' | 'stealthy'): this {
    callNative(() => this.inner.fetcherTier(tier));
    return this;
  }

  public browserProfile(profile: 'chrome' | 'firefox' | 'safari'): this {
    callNative(() => this.inner.browserProfile(profile));
    return this;
  }

  public autoMatchWeights(weights: Record<string, number>): this {
    callNative(() => this.inner.autoMatchWeights(weights));
    return this;
  }

  public proxyPool(proxies: string[]): this {
    callNative(() => this.inner.proxyPool(proxies));
    return this;
  }

  public proxyProvider(url: string | null): this {
    callNative(() => this.inner.proxyProvider(url));
    return this;
  }

  public clone(): Session {
    return new Session(callNative(() => this.inner.clone()));
  }

  public destroy(): void {
    callNative(() => this.inner.destroy());
  }
}
