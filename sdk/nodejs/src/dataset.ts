import { JsDataset, JsDatasetResult } from './native.js';
import { Session } from './session';

export class DatasetResult {
  constructor(private readonly inner: JsDatasetResult) {}

  public toDict(): Record<string, string> {
    return this.inner.toDict();
  }

  public async toJson(path: string): Promise<void> {
    await this.inner.toJson(path);
  }

  public async toCsv(path: string): Promise<void> {
    await this.inner.toCsv(path);
  }

  public async toParquet(path: string): Promise<void> {
    await this.inner.toParquet(path);
  }
}

export class Dataset {
  private readonly inner: JsDataset;
  private readonly session: Session;

  constructor(url: string, session?: Session) {
    this.session = session ?? new Session();
    this.inner = new JsDataset(url, this.session.inner);
  }

  public field(
    name: string,
    selector: string,
    options?: {
      selectorType?: 'css' | 'xpath';
      defaultVal?: string;
    }
  ): this {
    this.inner.field(name, selector, options?.selectorType ?? 'css', options?.defaultVal);
    return this;
  }

  public autoMatch(enabled: boolean): this {
    this.session.autoMatch(enabled);
    return this;
  }

  public timeout(seconds: number): this {
    this.session.timeout(seconds);
    return this;
  }

  public headers(headers: Record<string, string>): this {
    this.session.headers(headers);
    return this;
  }

  public async build(): Promise<DatasetResult> {
    const raw = await this.inner.build();
    return new DatasetResult(raw);
  }
}
