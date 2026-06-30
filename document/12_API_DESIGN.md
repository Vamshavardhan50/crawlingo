# 12_API_DESIGN.md

This document details the public API signatures, parameters, and return types for the Python SDK, Node.js SDK, CLI, and the Model Context Protocol (MCP) server.

---

## 1. Python SDK API

### `Session` Class
Coordinates shared request attributes, cookie jars, header maps, and proxy pools.
```python
class Session:
    def __init__(self): ...
    
    def headers(self, headers: dict) -> Session: ...
    def cookies(self, cookies: dict) -> Session: ...
    def proxy(self, proxy_url: str) -> Session: ...
    def rate_limit(self, requests_per_second: float) -> Session: ...
    def auto_match(self, enabled: bool) -> Session: ...
    def timeout(self, seconds: int) -> Session: ...
    def fingerprint_path(self, path: str) -> Session: ...
    def fetcher_tier(self, tier: str) -> Session: ...
    def browser_profile(self, profile: str) -> Session: ...
    def auto_match_weights(self, weights: dict) -> Session: ...
    def proxy_pool(self, proxies: list) -> Session: ...
    def proxy_provider(self, url: str) -> Session: ...
    def page(self, url: str) -> Page: ...
    def dataset(self, url: str) -> Dataset: ...
    def crawl(self, url: str) -> Crawl: ...
    def watch(self, url: str) -> Watch: ...
```

### `Page` Class
The representation of a downloaded web page and its parsed DOM structure.
```python
class Page:
    def __init__(
        self,
        url: str,
        auto_match: bool = False,
        timeout: int = 30,
        retries: int = 3,
        headers: dict[str, str] = None,
        cookies: dict[str, str] = None,
        proxy: str = None,
        session: Session = None,
    ): ...

    def css(self, selector: str) -> ElementCollection: ...
    def xpath(self, query: str) -> ElementCollection: ...
    def find_text(self, text: str) -> ElementCollection: ...
    def before_text(self, text: str) -> ElementCollection: ...
    def after_text(self, text: str) -> ElementCollection: ...
    def regex(self, pattern: str) -> ElementCollection: ...
    
    def html(self) -> str: ...
    def title(self) -> str: ...
    
    # Hook lifecycles
    def before_fetch(self, fn) -> Page: ...
    def after_fetch(self, fn) -> Page: ...
    def before_parse(self, fn) -> Page: ...
    def after_extract(self, fn) -> Page: ...
```

### `Dataset` Class
Extracts and serializes structured data based on defined fields.
```python
class Dataset:
    def __init__(self, url: str, session: Session = None): ...
    def field(self, name: str, selector: str, selector_type: str = "css", transform: Callable = None, default: str = None) -> Dataset: ...
    def auto_match(self, enabled: bool) -> Dataset: ...
    def timeout(self, seconds: int) -> Dataset: ...
    def headers(self, headers: dict) -> Dataset: ...
    def build(self) -> DatasetResult: ...
    async def build_async(self) -> DatasetResult: ...
```

### `DatasetResult` Class
```python
class DatasetResult:
    def to_json(self, path: str): ...
    def to_csv(self, path: str): ...
    def to_parquet(self, path: str): ...
    def to_dict(self) -> dict: ...
    def df(self) -> DataFrame: ...
    def __getitem__(self, key: str) -> str: ...
```

### `Crawl` Class
Performs parallel crawls starting from a seed URL.
```python
class Crawl:
    def __init__(self, start_url: str, session: Session = None): ...
    def follow(self, selector: str) -> Crawl: ...
    def limit(self, pages: int) -> Crawl: ...
    def depth(self, max_depth: int) -> Crawl: ...
    def field(self, name: str, selector: str, selector_type: str = "css", default: str = None) -> Crawl: ...
    def auto_match(self, enabled: bool) -> Crawl: ...
    def concurrency(self, n: int) -> Crawl: ...
    def delay(self, seconds: float) -> Crawl: ...
    def webhook(self, url: str) -> Crawl: ...
    def schedule(self, interval_seconds: int): ...
    def build(self) -> CrawlResults: ...
```

### `Watch` Class
Polling change monitor for websites.
```python
class Watch:
    def __init__(self, url: str, session: Session = None): ...
    def field(self, name: str, selector: str, selector_type: str = "css", default: str = None) -> Watch: ...
    def interval(self, seconds: int) -> Watch: ...
    def auto_match(self, enabled: bool) -> Watch: ...
    def on_change(self, fn: Callable) -> Watch: ...
    def on_price_change(self, fn: Callable) -> Watch: ...
    def on_stock_change(self, fn: Callable) -> Watch: ...
    def on_element_added(self, fn: Callable) -> Watch: ...
    def on_element_removed(self, fn: Callable) -> Watch: ...
    def run(self): ...
    async def run_async(self): ...
    def stop(self): ...
```

---

## 2. Node.js SDK API

Exposed through native bindings (`crawlingo-native`).

### `Session` Class
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

### `Page` Class
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

### `Element` & `ElementCollection` Classes
```typescript
class Element {
  readonly text: string;
  readonly html: string;
  attr(name: string): string | null;
}

class ElementCollection implements Iterable<Element> {
  readonly length: number;
  readonly text: string[];
  readonly html: string[];
  attr(name: string): (string | null)[];
  first(): Element | null;
  at(index: number): Element | null;
}
```

### `Dataset` Class
```typescript
class Dataset {
  constructor(url: string, session?: Session);
  field(name: string, selector: string, options?: { selectorType?: 'css' | 'xpath'; defaultVal?: string }): this;
  autoMatch(enabled: boolean): this;
  timeout(seconds: number): this;
  headers(headers: Record<string, string>): this;
  build(): Promise<DatasetResult>;
  extractStructured(page: Page | JsPage): Record<string, string>[];
  buildStructured(): Promise<Record<string, string>[]>;
  static saveJson(records: Record<string, string>[], path: string): void;
  static saveCsv(records: Record<string, string>[], path: string): void;
}
```

### `Crawl` Class
```typescript
class Crawl {
  constructor(startUrl: string, session?: Session);
  follow(selector: string): this;
  limit(limitNum: number): this;
  depth(depthNum: number): this;
  concurrency(n: number): this;
  delay(seconds: number): this;
  field(name: string, selector: string, options?: { selectorType?: 'css' | 'xpath'; defaultVal?: string }): this;
  webhook(url: string): this;
  schedule(intervalSeconds: number): void;
  run(): Promise<DatasetResult[]>;
}
```

### `Watch` Class
```typescript
class Watch {
  constructor(url: string, session?: Session);
  field(name: string, selector: string, options?: { selectorType?: 'css' | 'xpath'; defaultVal?: string }): this;
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

---

## 3. Command Line Interface (CLI)

Accessible via the `crawlingo` entrypoint in the Python installation:

```bash
# 1. Start an interactive Python shell with preloaded Crawlingo classes
crawlingo shell

# 2. Extract structured data from a URL using a JSON schema file
crawlingo extract --url <url> --schema <schema_json_file> --output <output_file>

# 3. Start the Model Context Protocol SSE server
crawlingo mcp --host 127.0.0.1 --port 8000
```

---

## 4. Model Context Protocol (MCP) Tools

Exposes a JSON-RPC 2.0 API over SSE (default `127.0.0.1:8000`) supporting the following tools for AI Agents:

1. **`fetch_page`**
   - *Arguments:* `url` (string), `auto_match` (boolean), `timeout` (number)
   - *Returns:* Normalized text content, HTML snippet, and Page metadata.
2. **`extract_data`**
   - *Arguments:* `url` (string), `fields` (array of `{name, selector, selector_type}`), `auto_match` (boolean)
   - *Returns:* Structured JSON key-value extraction matching the schema rules.
3. **`crawl_site`**
   - *Arguments:* `url` (string), `follow_selector` (string), `fields` (array), `max_pages` (number), `max_depth` (number)
   - *Returns:* List of discovered URLs and page data.
