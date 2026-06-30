# Node.js SDK API (crawlingo)

## Installation

```bash
npm install crawlingo
```

Requires Node.js 16+. Prebuilt `.node` binaries available for x86_64 and aarch64 (Windows, macOS, Linux).

## Classes

### Session

```typescript
class Session {
  constructor();
  headers(headers: Record<string, string>): this;
  cookies(cookies: Record<string, string>): this;
  proxy(proxyUrl: string): this;
  rateLimit(requestsPerSecond: number): this;
  autoMatch(enabled: boolean): this;
  timeout(seconds: number): this;
  fingerprintPath(path: string): this;
  fetcherTier(tier: 'standard' | 'stealthy'): this;
  browserProfile(profile: 'chrome' | 'firefox' | 'safari'): this;
  autoMatchWeights(weights: Record<string, number>): this;
  proxyPool(proxies: string[]): this;
  proxyProvider(url: string | null): this;
}
```

### Page

```typescript
class Page {
  static create(url: string, options?: PageOptions): Promise<Page>;
  
  readonly url: string;
  readonly status: number;
  readonly html: string;
  
  title(): string;
  css(selector: string): ElementCollection;
  xpath(query: string): ElementCollection;
  findText(text: string): ElementCollection;
  afterText(text: string): ElementCollection;
  beforeText(text: string): ElementCollection;
  regex(pattern: string): ElementCollection;
}

interface PageOptions {
  autoMatch?: boolean;
  timeout?: number;
  headers?: Record<string, string>;
  cookies?: Record<string, string>;
  proxy?: string;
  browserProfile?: string;
  session?: Session;
}
```

### Element

```typescript
class Element {
  readonly text: string;
  readonly html: string;
  attr(name: string): string | null;
}
```

### ElementCollection

```typescript
class ElementCollection implements Iterable<Element> {
  readonly length: number;
  readonly text: string[];
  readonly html: string[];
  attr(name: string): (string | null)[];
  first(): Element | null;
  at(index: number): Element | null;
}
```

### Dataset

```typescript
class Dataset {
  constructor(url: string, session?: Session);
  field(name: string, selector: string, options?: FieldOptions): this;
  autoMatch(enabled: boolean): this;
  timeout(seconds: number): this;
  headers(headers: Record<string, string>): this;
  build(): Promise<DatasetResult>;
  extractStructured(page: Page | JsPage): Record<string, string>[];
  buildStructured(): Promise<Record<string, string>[]>;
  static saveJson(records: Record<string, string>[], path: string): void;
  static saveCsv(records: Record<string, string>[], path: string): void;
}

interface FieldOptions {
  selectorType?: 'css' | 'xpath';
  defaultVal?: string;
}
```

### Crawl

```typescript
class Crawl {
  constructor(startUrl: string, session?: Session);
  follow(selector: string): this;
  limit(limitNum: number): this;
  depth(depthNum: number): this;
  concurrency(n: number): this;
  delay(seconds: number): this;
  field(name: string, selector: string, options?: FieldOptions): this;
  webhook(url: string): this;
  schedule(intervalSeconds: number): void;
  run(): Promise<DatasetResult[]>;
}
```

### Watch

```typescript
class Watch {
  constructor(url: string, session?: Session);
  field(name: string, selector: string, options?: FieldOptions): this;
  interval(seconds: number): this;
  run(callback: (err: Error | null, event: WatchChangeEvent) => void): void;
  stop(): void;
}

interface WatchChangeEvent {
  url: string;
  field: string;
  changeType: string;
  oldValue?: string;
  newValue?: string;
}
```

## Examples

```typescript
import { Session, Page } from 'crawlingo';

const session = new Session();
session.autoMatch(true);

const page = await Page.create("https://example.com", { session });
const headings = page.css("h1");
console.log("Title:", page.title());
```

## See Also

- [Python SDK](sdk_python.md): Equivalent API for Python
- [CLI reference](cli.md): Command-line interface
