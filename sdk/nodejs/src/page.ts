import { fetchPage as nativeFetchPage, JsPage } from './native.js';
import { ElementCollection } from './element';
import { Session } from './session';

export class Page {
  private constructor(private readonly inner: JsPage) {}

  /**
   * Fetches a web page and returns a parsed `Page` object.
   */
  public static async create(
    url: string,
    options?: {
      autoMatch?: boolean;
      timeout?: number;
      headers?: Record<string, string>;
      cookies?: Record<string, string>;
      proxy?: string;
      browserProfile?: string;
      session?: Session;
    }
  ): Promise<Page> {
    const raw = await nativeFetchPage(
      url,
      options?.autoMatch ?? false,
      options?.timeout,
      options?.headers,
      options?.cookies,
      options?.proxy,
      options?.browserProfile,
      options?.session ? (options.session as any).inner : undefined
    );
    return new Page(raw);
  }

  public get url(): string {
    return this.inner.url;
  }

  public get status(): number {
    return this.inner.status;
  }

  public get html(): string {
    return this.inner.html;
  }

  public title(): string {
    return this.inner.title();
  }

  public css(selector: string): ElementCollection {
    return new ElementCollection(this.inner.css(selector));
  }

  public xpath(query: string): ElementCollection {
    return new ElementCollection(this.inner.xpath(query));
  }

  public findText(text: string): ElementCollection {
    return new ElementCollection(this.inner.findText(text));
  }

  public afterText(text: string): ElementCollection {
    return new ElementCollection(this.inner.afterText(text));
  }

  public beforeText(text: string): ElementCollection {
    return new ElementCollection(this.inner.beforeText(text));
  }

  public regex(pattern: string): ElementCollection {
    return new ElementCollection(this.inner.regex(pattern));
  }
}
