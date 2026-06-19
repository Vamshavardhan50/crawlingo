import { JsSession } from './native.js';

export class Session {
  public readonly inner: JsSession;

  constructor() {
    this.inner = new JsSession();
  }

  public headers(headers: Record<string, string>): this {
    this.inner.headers(headers);
    return this;
  }

  public cookies(cookies: Record<string, string>): this {
    this.inner.cookies(cookies);
    return this;
  }

  public proxy(proxyUrl: string): this {
    this.inner.proxy(proxyUrl);
    return this;
  }

  public rateLimit(requestsPerSecond: number): this {
    this.inner.rateLimit(requestsPerSecond);
    return this;
  }

  public autoMatch(enabled: boolean): this {
    this.inner.autoMatch(enabled);
    return this;
  }

  public timeout(seconds: number): this {
    this.inner.timeout(seconds);
    return this;
  }

  public fingerprintPath(path: string): this {
    this.inner.fingerprintPath(path);
    return this;
  }

  public fetcherTier(tier: 'standard' | 'stealthy'): this {
    this.inner.fetcherTier(tier);
    return this;
  }

  public browserProfile(profile: 'chrome' | 'firefox' | 'safari'): this {
    this.inner.browserProfile(profile);
    return this;
  }

  public autoMatchWeights(weights: Record<string, number>): this {
    this.inner.autoMatchWeights(weights);
    return this;
  }

  public proxyPool(proxies: string[]): this {
    this.inner.proxyPool(proxies);
    return this;
  }

  public proxyProvider(url: string | null): this {
    this.inner.proxyProvider(url);
    return this;
  }
}
