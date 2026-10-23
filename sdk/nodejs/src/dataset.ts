import { JsDataset, JsDatasetResult, JsPage, saveStructuredJson, saveStructuredCsv } from './native.js';
import { Session } from './session';
import type { Page } from './page';
import { DatasetSchema } from './schema';
import { callNative, callNativeAsync, ExportError } from './errors';

export class DatasetResult {
  constructor(private readonly inner: JsDatasetResult) {}

  public toDict(): Record<string, string> {
    return this.inner.toDict();
  }

  public async toJson(path: string): Promise<void> {
    await callNativeAsync(() => this.inner.toJson(path), ExportError);
  }

  public async toCsv(path: string): Promise<void> {
    await callNativeAsync(() => this.inner.toCsv(path), ExportError);
  }

  public async toParquet(path: string): Promise<void> {
    await callNativeAsync(() => this.inner.toParquet(path), ExportError);
  }
}

export interface DatasetFieldConfig {
  name: string;
  selector: string;
  selectorType: string;
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
      selectorType?: 'css' | 'xpath' | 'regex' | 'text' | 'after_text' | 'before_text';
      defaultVal?: string;
      extractType?: string;
    }
  ): this {
    const selType = options?.selectorType ?? 'css';
    this.inner.field(name, selector, selType, options?.defaultVal, options?.extractType);
    return this;
  }

  public withSchema(schema: DatasetSchema): this {
    this.inner.withSchema(schema.inner);
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
    const raw = await callNativeAsync(() => this.inner.build());
    return new DatasetResult(raw);
  }

  /**
   * Extracts clean, structured multi-row records from a parsed Page object.
   * Implemented entirely in Rust: zips selector results by element index.
   */
  public extractStructured(page: Page | JsPage): Array<Record<string, string>> {
    const jsPage = (page as any).nativePage ?? page;
    return callNative(() => this.inner.extractStructured(jsPage)) as Array<Record<string, string>>;
  }

  /**
   * Fetch the page URL and extract structured multi-row records entirely in Rust.
   */
  public async buildStructured(): Promise<Array<Record<string, string>>> {
    return callNativeAsync(() => this.inner.buildStructured()) as Promise<Array<Record<string, string>>>;
  }

  /**
   * Write structured records to a pretty-printed JSON file.
   * Implemented entirely in Rust via native FFI.
   */
  public static saveJson(records: Array<Record<string, string>>, path: string): void {
    callNative(() => saveStructuredJson(records as any, path), ExportError);
  }

  /**
   * Write structured records to a clean CSV file with header row.
   * Implemented entirely in Rust via native FFI.
   */
  public static saveCsv(records: Array<Record<string, string>>, path: string): void {
    callNative(() => saveStructuredCsv(records as any, path), ExportError);
  }
}
