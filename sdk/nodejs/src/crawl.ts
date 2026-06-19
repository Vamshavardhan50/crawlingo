import { JsCrawl } from './native.js';
import { Session } from './session';
import { DatasetResult } from './dataset';

export class Crawl {
  private readonly inner: JsCrawl;

  private readonly session: Session;

  constructor(startUrl: string, session?: Session) {
    this.session = session ?? new Session();
    this.inner = new JsCrawl(startUrl, this.session.inner);
  }

  public autoMatch(enabled: boolean): this {
    this.session.autoMatch(enabled);
    return this;
  }

  public follow(selector: string): this {
    this.inner.follow(selector);
    return this;
  }

  public limit(limitNum: number): this {
    this.inner.limit(limitNum);
    return this;
  }

  public depth(depthNum: number): this {
    this.inner.depth(depthNum);
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

  public field(
    name: string,
    selector: string,
    options?: {
      selectorType?: 'css' | 'xpath';
    }
  ): this {
    this.inner.field(name, selector, options?.selectorType ?? 'css');
    return this;
  }

  public webhook(url: string): this {
    this.inner.webhook(url);
    return this;
  }

  public schedule(intervalSeconds: number): void {
    this.inner.schedule(intervalSeconds);
  }

  public async run(): Promise<DatasetResult[]> {
    const raw = await this.inner.run();
    return raw.map(r => new DatasetResult(r));
  }
}
