import { fetchPage as nativeFetchPage, JsPage } from './native.js';
import { ElementCollection } from './element';
import { Session } from './session';

export class Page {
  constructor(public readonly nativePage: JsPage) {}

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
      options?.session?.inner
    );
    return new Page(raw);
  }

  public get url(): string {
    return this.nativePage.url;
  }

  public get status(): number {
    return this.nativePage.status;
  }

  public get html(): string {
    return this.nativePage.html;
  }

  public title(): string {
    return this.nativePage.title();
  }

  public css(selector: string): ElementCollection {
    return new ElementCollection(this.nativePage.css(selector));
  }

  public xpath(query: string): ElementCollection {
    return new ElementCollection(this.nativePage.xpath(query));
  }

  public findText(text: string): ElementCollection {
    return new ElementCollection(this.nativePage.findText(text));
  }

  public afterText(text: string): ElementCollection {
    return new ElementCollection(this.nativePage.afterText(text));
  }

  public beforeText(text: string): ElementCollection {
    return new ElementCollection(this.nativePage.beforeText(text));
  }

  public regex(pattern: string): ElementCollection {
    return new ElementCollection(this.nativePage.regex(pattern));
  }
}
