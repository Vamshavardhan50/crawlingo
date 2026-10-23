# Session · Node.js

Own shared request defaults, connection pools, per-host pacing and retry settings.

<a id="overview"></a>

## Purpose and lifecycle

Create one session per site or authentication identity and pass it to related operations. Configuration setters return the same session. Settings are held in the Rust engine.

Set authentication and response-cache middleware before the first fetch. A clone copies request configuration but starts independent pools, metrics and middleware. It does not inherit installed auth/cache layers.

Cookies are explicitly configured request defaults, not a browser cookie jar. They have no domain, path or expiry attributes and Set-Cookie responses are not automatically persisted.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class Session
```

- [sdk/nodejs/src/session.ts:4](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L4)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { Session, Page } from 'crawlingo';

const session = new Session().rateLimit(2).timeout(30)
  .retries(3).retryDelay(1).retryBackoff(2)
  .cookies({ locale: 'en' });
const page = await Page.create('https://example.com/', { session });
console.log(page.status, session.getCookies());
session.deleteCookie('locale').clearCookies();
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| inner | property | JsSession | Present in source |
| constructor | method | Session | Present in source |
| fromConfig | static method | Session | Present in source |
| headers | method | this | Present in source |
| cookies | method | this | Present in source |
| getCookies | method | Record<string, string> | Present in source |
| deleteCookie | method | this | Present in source |
| clearCookies | method | this | Present in source |
| retries | method | this | Present in source |
| retryBackoff | method | this | Present in source |
| retryDelay | method | this | Present in source |
| proxy | method | this | Present in source |
| rateLimit | method | this | Present in source |
| autoMatch | method | this | Present in source |
| timeout | method | this | Present in source |
| fingerprintPath | method | this | Present in source |
| fetcherTier | method | this | Present in source |
| browserProfile | method | this | Present in source |
| autoMatchWeights | method | this | Present in source |
| proxyPool | method | this | Present in source |
| proxyProvider | method | this | Present in source |
| clone | method | Session | Present in source |
| destroy | method | void | Present in source |

- [inner](session.md#inner)
- [constructor](session.md#constructor)
- [fromConfig](session.md#from-config)
- [headers](session.md#headers)
- [cookies](session.md#cookies)
- [getCookies](session.md#get-cookies)
- [deleteCookie](session.md#delete-cookie)
- [clearCookies](session.md#clear-cookies)
- [retries](session.md#retries)
- [retryBackoff](session.md#retry-backoff)
- [retryDelay](session.md#retry-delay)
- [proxy](session.md#proxy)
- [rateLimit](session.md#rate-limit)
- [autoMatch](session.md#auto-match)
- [timeout](session.md#timeout)
- [fingerprintPath](session.md#fingerprint-path)
- [fetcherTier](session.md#fetcher-tier)
- [browserProfile](session.md#browser-profile)
- [autoMatchWeights](session.md#auto-match-weights)
- [proxyPool](session.md#proxy-pool)
- [proxyProvider](session.md#proxy-provider)
- [clone](session.md#clone)
- [destroy](session.md#destroy)

<a id="inner"></a>

## inner

Native interop handle. Prefer the documented wrapper methods; bypassing them can bypass validation and typed error conversion.

Returns / value: JsSession.

### Signature

```typescript
public readonly inner: JsSession;
```

- [Source · sdk/nodejs/src/session.ts:5](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L5)

<a id="constructor"></a>

## constructor()

Create an empty session with a 30-second request timeout, standard HTTP transport and no pacing limit. No request is sent by construction.

Returns / value: Session.

### Signature

```typescript
constructor(inner?: JsSession): Session
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| inner | JsSession \| undefined | Optional | Existing native handle for wrapper interop; not a URL or plain data object. |

- [Source · sdk/nodejs/src/session.ts:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L7)

<a id="from-config"></a>

## fromConfig()

Load TOML or JSON, apply CRAWLINGO_* environment overrides and validate configuration. Omit the path to load defaults plus environment only. Explicit setters applied afterward override loaded settings.

Returns / value: Session.

### Signature

```typescript
fromConfig(path?: string): Session
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | string \| undefined | Optional | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/nodejs/src/session.ts:12](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L12)

<a id="headers"></a>

## headers()

Replace the entire default header map. Node Page.create merges its request headers over this map. Authentication headers apply to every request using this session.

Returns / value: this.

### Signature

```typescript
headers(headers: Record<string, string>): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| headers | Record<string, string> | Required | Header names and values; avoid sharing credentials across unrelated sites. |

- [Source · sdk/nodejs/src/session.ts:16](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L16)

<a id="cookies"></a>

## cookies()

Replace the entire explicit cookie map. Use a different session for another authentication identity.

Returns / value: this.

### Signature

```typescript
cookies(cookies: Record<string, string>): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| cookies | Record<string, string> | Required | Explicit cookie-name/value defaults. |

- [Source · sdk/nodejs/src/session.ts:21](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L21)

<a id="get-cookies"></a>

## getCookies()

Return a snapshot of configured cookies. Mutating the returned object does not mutate the session.

Returns / value: Record<string, string>.

### Signature

```typescript
getCookies(): Record<string, string>
```

- [Source · sdk/nodejs/src/session.ts:27](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L27)

<a id="delete-cookie"></a>

## deleteCookie()

Remove one cookie by its case-sensitive name. Deleting an absent name succeeds without changing anything.

Returns / value: this.

### Signature

```typescript
deleteCookie(name: string): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | string | Required | Field, header or attribute name for this operation. |

- [Source · sdk/nodejs/src/session.ts:32](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L32)

<a id="clear-cookies"></a>

## clearCookies()

Remove all explicit cookie defaults. This does not clear an independently supplied Cookie header.

Returns / value: this.

### Signature

```typescript
clearCookies(): this
```

- [Source · sdk/nodejs/src/session.ts:38](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L38)

<a id="retries"></a>

## retries()

Set additional attempts after the initial request. 0 means one attempt. Integers 0–4294967295 are accepted. Default Page/Dataset/Crawl/Job retry count is 3; downloader/sitemap defaults can differ until explicitly set.

Returns / value: this.

### Signature

```typescript
retries(count: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| count | number | Required | Count of additional retries after the initial attempt. |

- [Source · sdk/nodejs/src/session.ts:44](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L44)

<a id="retry-backoff"></a>

## retryBackoff()

Set a finite exponential multiplier >= 1. Delay for retry n starts at base × factor^n, capped by policy. Default factor is 2. Numeric Retry-After takes precedence when enabled. Changes affect subsequent dispatches without rebuilding pools.

Returns / value: this.

### Signature

```typescript
retryBackoff(factor: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| factor | number | Required | Finite multiplier, at least 1. |

- [Source · sdk/nodejs/src/session.ts:53](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L53)

<a id="retry-delay"></a>

## retryDelay()

Set initial backoff in whole seconds, from 0 to 86400. The default policy uses 500 ms; use shared configuration for fractional-second timing. Computed delay is capped at max(configured maximum, initial delay).

Returns / value: this.

### Signature

```typescript
retryDelay(seconds: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | number | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/nodejs/src/session.ts:62](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L62)

<a id="proxy"></a>

## proxy()

Set a static proxy URL. A static proxy takes priority over the rotation pool. HTTP(S) and supported SOCKS proxy behavior depends on the compiled HTTP client.

Returns / value: this.

### Signature

```typescript
proxy(proxyUrl: string): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| proxyUrl | string | Required | Proxy endpoint URL accepted by the compiled HTTP transport. |

- [Source · sdk/nodejs/src/session.ts:70](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L70)

<a id="rate-limit"></a>

## rateLimit()

Set the shared per-host request rate in requests per second. 0 disables configured pacing. This is not a global limit across separate sessions or machines.

Returns / value: this.

### Signature

```typescript
rateLimit(requestsPerSecond: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| requestsPerSecond | number | Required | Per-host rate for the shared session. |

- [Source · sdk/nodejs/src/session.ts:75](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L75)

<a id="auto-match"></a>

## autoMatch()

Enable heuristic CSS fingerprint recovery for the Dataset builder. It does not enable selector recovery in DatasetJob or execute JavaScript.

Returns / value: this.

### Signature

```typescript
autoMatch(enabled: boolean): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | boolean | Required | Enable or disable this behavior. |

- [Source · sdk/nodejs/src/session.ts:80](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L80)

<a id="timeout"></a>

## timeout()

Set the per-attempt timeout in seconds. Total time can exceed it because of retries, backoff and rate-limit waits.

Returns / value: this.

### Signature

```typescript
timeout(seconds: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | number | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/nodejs/src/session.ts:85](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L85)

<a id="fingerprint-path"></a>

## fingerprintPath()

Select the local directory used for lazily opened fingerprint storage. Keep one coherent identity/history for a selector; a new path has no recovery history.

Returns / value: this.

### Signature

```typescript
fingerprintPath(path: string): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | string | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/nodejs/src/session.ts:90](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L90)

<a id="fetcher-tier"></a>

## fetcherTier()

Node exposes standard and stealthy HTTP tiers. Python can request browser only when its native build includes the browser feature and local Chromium is available. Stealth emulation does not guarantee access.

Returns / value: this.

### Signature

```typescript
fetcherTier(tier: "standard" | "stealthy"): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| tier | "standard" \| "stealthy" | Required | Transport mode supported by this binding and native build. |

- [Source · sdk/nodejs/src/session.ts:95](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L95)

<a id="browser-profile"></a>

## browserProfile()

Choose a supported HTTP impersonation profile: chrome, firefox or safari. A profile changes transport emulation; it does not launch a browser.

Returns / value: this.

### Signature

```typescript
browserProfile(profile: "chrome" | "firefox" | "safari"): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| profile | "chrome" \| "firefox" \| "safari" | Required | HTTP impersonation profile; this does not launch a browser. |

- [Source · sdk/nodejs/src/session.ts:100](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L100)

<a id="auto-match-weights"></a>

## autoMatchWeights()

Set scoring weights used by the heuristic matcher. Similarity scores are not calibrated probabilities; validate recovered field values.

Returns / value: this.

### Signature

```typescript
autoMatchWeights(weights: Record<string, number>): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| weights | Record<string, number> | Required | Named heuristic matching weights. Review extracted values after changing scores. |

- [Source · sdk/nodejs/src/session.ts:105](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L105)

<a id="proxy-pool"></a>

## proxyPool()

Replace the pool of proxy URLs used in round-robin order when no static proxy is set. This is configuration, not a proxy health-check service.

Returns / value: this.

### Signature

```typescript
proxyPool(proxies: string[]): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| proxies | string[] | Required | Proxy URL list used when no static proxy is configured. |

- [Source · sdk/nodejs/src/session.ts:110](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L110)

<a id="proxy-provider"></a>

## proxyProvider()

Fetch proxy entries from the configured provider endpoint. The Node binding currently discards a provider-fetch error; do not treat this setter returning as proof that proxies were loaded.

Returns / value: this.

### Signature

```typescript
proxyProvider(url: string | null): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | string \| null | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/nodejs/src/session.ts:115](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L115)

<a id="clone"></a>

## clone()

Return an independent session with copied defaults and retry settings. Cookie changes on the clone do not affect the original. Middleware is not cloned.

Returns / value: Session.

### Signature

```typescript
clone(): Session
```

- [Source · sdk/nodejs/src/session.ts:120](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L120)

<a id="destroy"></a>

## destroy()

Clear selected native state and drop cached managers when no operation retains them. This is cleanup, not cancellation of running jobs. Await owned work before cleanup.

Returns / value: void.

### Signature

```typescript
destroy(): void
```

- [Source · sdk/nodejs/src/session.ts:124](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/session.ts#L124)

<a id="errors"></a>

## Errors and operational limits

Node retry setters reject invalid numbers with RangeError. Native configuration failures use CrawlingoError; Python session methods can propagate native RuntimeError or ValueError directly. Do not assume identical exception translation across bindings.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
