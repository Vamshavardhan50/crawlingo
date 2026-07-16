import { JsPaginationConfig } from './native.js';

export class PaginationConfig {
  public readonly inner: JsPaginationConfig;

  private constructor(inner: JsPaginationConfig) {
    this.inner = inner;
  }

  public static nextLink(selector: string): PaginationConfig {
    return new PaginationConfig(JsPaginationConfig.nextLink(selector));
  }

  public static pageNumber(urlTemplate: string, startPage: number, maxPages: number): PaginationConfig {
    return new PaginationConfig(JsPaginationConfig.pageNumber(urlTemplate, startPage, maxPages));
  }

  public static urlPattern(pageRegex: string, maxPage: number): PaginationConfig {
    return new PaginationConfig(JsPaginationConfig.urlPattern(pageRegex, maxPage));
  }
}
