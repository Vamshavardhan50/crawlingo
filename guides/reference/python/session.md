# Session · Python

Own shared request defaults, connection pools, per-host pacing and retry settings.

<a id="overview"></a>

## Purpose and lifecycle

Create one session per site or authentication identity and pass it to related operations. Configuration setters return the same session. Settings are held in the Rust engine.

Set authentication and response-cache middleware before the first fetch. A clone copies request configuration but starts independent pools, metrics and middleware. It does not inherit installed auth/cache layers.

Cookies are explicitly configured request defaults, not a browser cookie jar. They have no domain, path or expiry attributes and Set-Cookie responses are not automatically persisted.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class Session
```

- [sdk/python/crawlingo/session.py:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L10)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
from crawlingo import Session, Dataset

session = Session().rate_limit(2).timeout(30)
session.headers({"User-Agent": "ResearchDataset/1.0"})
result = Dataset("https://example.com/", session=session).field("title", "h1").build()
print(result.to_dict())
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| headers | method | 'Session' | Present in source |
| cookies | method | 'Session' | Present in source |
| proxy | method | 'Session' | Present in source |
| rate_limit | method | 'Session' | Present in source |
| auto_match | method | 'Session' | Present in source |
| timeout | method | 'Session' | Present in source |
| fingerprint_path | method | 'Session' | Present in source |
| fetcher_tier | method | 'Session' | Present in source |
| browser_profile | method | 'Session' | Present in source |
| auto_match_weights | method | 'Session' | Present in source |
| proxy_pool | method | 'Session' | Present in source |
| proxy_provider | method | 'Session' | Present in source |
| enable_response_cache | method | 'Session' | Present in source |
| basic_auth | method | 'Session' | Present in source |
| bearer_auth | method | 'Session' | Present in source |
| header_auth | method | 'Session' | Present in source |
| api_key_auth | method | 'Session' | Present in source |
| from_config | static method | 'Session' | Present in source |
| metrics | method | dict | Present in source |
| page | method | 'Page' | Present in source |
| dataset | method | 'Dataset' | Present in source |
| watch | method | 'Watch' | Present in source |
| crawl | method | 'Crawl' | Present in source |
| clone | method | 'Session' | Present in source |
| destroy | method | None | Present in source |
| __enter__ | method | 'Session' | Present in source |
| __exit__ | method | Not annotated; see behavior notes | Present in source |

- [__init__](session.md#init)
- [headers](session.md#headers)
- [cookies](session.md#cookies)
- [proxy](session.md#proxy)
- [rate_limit](session.md#rate-limit)
- [auto_match](session.md#auto-match)
- [timeout](session.md#timeout)
- [fingerprint_path](session.md#fingerprint-path)
- [fetcher_tier](session.md#fetcher-tier)
- [browser_profile](session.md#browser-profile)
- [auto_match_weights](session.md#auto-match-weights)
- [proxy_pool](session.md#proxy-pool)
- [proxy_provider](session.md#proxy-provider)
- [enable_response_cache](session.md#enable-response-cache)
- [basic_auth](session.md#basic-auth)
- [bearer_auth](session.md#bearer-auth)
- [header_auth](session.md#header-auth)
- [api_key_auth](session.md#api-key-auth)
- [from_config](session.md#from-config)
- [metrics](session.md#metrics)
- [page](session.md#page)
- [dataset](session.md#dataset)
- [watch](session.md#watch)
- [crawl](session.md#crawl)
- [clone](session.md#clone)
- [destroy](session.md#destroy)
- [__enter__](session.md#enter)
- [__exit__](session.md#exit)

<a id="init"></a>

## __init__()

Create an empty session with a 30-second request timeout, standard HTTP transport and no pacing limit. No request is sent by construction.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self)
```

- [Source · sdk/python/crawlingo/session.py:15](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L15)

<a id="headers"></a>

## headers()

Replace the entire default header map. Node Page.create merges its request headers over this map. Authentication headers apply to every request using this session.

Returns / value: 'Session'.

### Signature

```python
def headers(self, headers: dict) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| headers | dict | Required | Header names and values; avoid sharing credentials across unrelated sites. |

- [Source · sdk/python/crawlingo/session.py:18](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L18)

<a id="cookies"></a>

## cookies()

Replace the entire explicit cookie map. Use a different session for another authentication identity.

Returns / value: 'Session'.

### Signature

```python
def cookies(self, cookies: dict) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| cookies | dict | Required | Explicit cookie-name/value defaults. |

- [Source · sdk/python/crawlingo/session.py:23](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L23)

<a id="proxy"></a>

## proxy()

Set a static proxy URL. A static proxy takes priority over the rotation pool. HTTP(S) and supported SOCKS proxy behavior depends on the compiled HTTP client.

Returns / value: 'Session'.

### Signature

```python
def proxy(self, proxy_url: str) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| proxy_url | str | Required | Proxy endpoint URL accepted by the compiled HTTP transport. |

- [Source · sdk/python/crawlingo/session.py:28](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L28)

<a id="rate-limit"></a>

## rate_limit()

Set the shared per-host request rate in requests per second. 0 disables configured pacing. This is not a global limit across separate sessions or machines.

Returns / value: 'Session'.

### Signature

```python
def rate_limit(self, requests_per_second: float) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| requests_per_second | float | Required | Per-host rate for the shared session. |

- [Source · sdk/python/crawlingo/session.py:33](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L33)

<a id="auto-match"></a>

## auto_match()

Enable heuristic CSS fingerprint recovery for the Dataset builder. It does not enable selector recovery in DatasetJob or execute JavaScript.

Returns / value: 'Session'.

### Signature

```python
def auto_match(self, enabled: bool) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | bool | Required | Enable or disable this behavior. |

- [Source · sdk/python/crawlingo/session.py:38](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L38)

<a id="timeout"></a>

## timeout()

Set the per-attempt timeout in seconds. Total time can exceed it because of retries, backoff and rate-limit waits.

Returns / value: 'Session'.

### Signature

```python
def timeout(self, seconds: int) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | int | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/python/crawlingo/session.py:43](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L43)

<a id="fingerprint-path"></a>

## fingerprint_path()

Select the local directory used for lazily opened fingerprint storage. Keep one coherent identity/history for a selector; a new path has no recovery history.

Returns / value: 'Session'.

### Signature

```python
def fingerprint_path(self, path: str) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | str | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/session.py:48](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L48)

<a id="fetcher-tier"></a>

## fetcher_tier()

Node exposes standard and stealthy HTTP tiers. Python can request browser only when its native build includes the browser feature and local Chromium is available. Stealth emulation does not guarantee access.

Returns / value: 'Session'.

### Signature

```python
def fetcher_tier(self, tier: str) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| tier | str | Required | Transport mode supported by this binding and native build. |

- [Source · sdk/python/crawlingo/session.py:53](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L53)

<a id="browser-profile"></a>

## browser_profile()

Choose a supported HTTP impersonation profile: chrome, firefox or safari. A profile changes transport emulation; it does not launch a browser.

Returns / value: 'Session'.

### Signature

```python
def browser_profile(self, profile: str) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| profile | str | Required | HTTP impersonation profile; this does not launch a browser. |

- [Source · sdk/python/crawlingo/session.py:58](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L58)

<a id="auto-match-weights"></a>

## auto_match_weights()

Set scoring weights used by the heuristic matcher. Similarity scores are not calibrated probabilities; validate recovered field values.

Returns / value: 'Session'.

### Signature

```python
def auto_match_weights(self, weights: dict) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| weights | dict | Required | Named heuristic matching weights. Review extracted values after changing scores. |

- [Source · sdk/python/crawlingo/session.py:63](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L63)

<a id="proxy-pool"></a>

## proxy_pool()

Replace the pool of proxy URLs used in round-robin order when no static proxy is set. This is configuration, not a proxy health-check service.

Returns / value: 'Session'.

### Signature

```python
def proxy_pool(self, proxies: list) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| proxies | list | Required | Proxy URL list used when no static proxy is configured. |

- [Source · sdk/python/crawlingo/session.py:68](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L68)

<a id="proxy-provider"></a>

## proxy_provider()

Fetch proxy entries from the configured provider endpoint. The Node binding currently discards a provider-fetch error; do not treat this setter returning as proof that proxies were loaded.

Returns / value: 'Session'.

### Signature

```python
def proxy_provider(self, url: str) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/python/crawlingo/session.py:73](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L73)

<a id="enable-response-cache"></a>

## enable_response_cache()

Install in-memory response caching before first fetch. max_entries bounds entry count; default_ttl_seconds is the fallback freshness lifetime. Cache headers and validators still apply.

Returns / value: 'Session'.

### Signature

```python
def enable_response_cache(self, max_entries: int=1024, default_ttl_seconds: int=300) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| max_entries | int | Default: 1024 | Maximum number of cached responses, not a total byte limit. |
| default_ttl_seconds | int | Default: 300 | Fallback cache lifetime in seconds. |

- [Source · sdk/python/crawlingo/session.py:78](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L78)

<a id="basic-auth"></a>

## basic_auth()

Install HTTP Basic Authorization middleware before the first request. Pass credentials through application configuration; do not embed them in example source.

Returns / value: 'Session'.

### Signature

```python
def basic_auth(self, username: str, password: str) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| username | str | Required | HTTP Basic username from application configuration. |
| password | str | Required | HTTP Basic password; obtain it from protected application configuration. |

- [Source · sdk/python/crawlingo/session.py:85](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L85)

<a id="bearer-auth"></a>

## bearer_auth()

Install a fixed Bearer token header before first fetch. Refresh or rotation is owned by your application.

Returns / value: 'Session'.

### Signature

```python
def bearer_auth(self, token: str) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| token | str | Required | Bearer credential managed by the application. |

- [Source · sdk/python/crawlingo/session.py:90](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L90)

<a id="header-auth"></a>

## header_auth()

Install a named authentication header before the first fetch.

Returns / value: 'Session'.

### Signature

```python
def header_auth(self, name: str, value: str) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |
| value | str | Required | Header or query value for the configured authentication method. |

- [Source · sdk/python/crawlingo/session.py:96](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L96)

<a id="api-key-auth"></a>

## api_key_auth()

Append an API-key query parameter to outgoing request URLs. Query values may appear in access logs; use a header where the upstream supports it.

Returns / value: 'Session'.

### Signature

```python
def api_key_auth(self, name: str, value: str) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |
| value | str | Required | Header or query value for the configured authentication method. |

- [Source · sdk/python/crawlingo/session.py:102](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L102)

<a id="from-config"></a>

## from_config()

Load TOML or JSON, apply CRAWLINGO_* environment overrides and validate configuration. Omit the path to load defaults plus environment only. Explicit setters applied afterward override loaded settings.

Returns / value: 'Session'.

### Signature

```python
def from_config(cls, path: Optional[str]=None) -> 'Session'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | Optional[str] | Default: None | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/python/crawlingo/session.py:109](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L109)

<a id="metrics"></a>

## metrics()

Return a snapshot of fetch counts, statuses, per-host activity and latency. These are session metrics, not the DatasetJob record-quality report.

Returns / value: dict.

### Signature

```python
def metrics(self) -> dict
```

- [Source · sdk/python/crawlingo/session.py:117](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L117)

<a id="page"></a>

## page()

Create a lazy Python Page attached to the session. Python Page currently supplies its own default timeout=30 and retries=3; pass explicit Page options when those values matter.

Returns / value: 'Page'.

### Signature

```python
def page(self, url: str) -> 'Page'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/python/crawlingo/session.py:121](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L121)

<a id="dataset"></a>

## dataset()

Create a Dataset builder attached to this session. Add fields and call build to fetch.

Returns / value: 'Dataset'.

### Signature

```python
def dataset(self, url: str) -> 'Dataset'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/python/crawlingo/session.py:126](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L126)

<a id="watch"></a>

## watch()

Create a field-monitoring Watch handle. Register fields and callbacks before starting it.

Returns / value: 'Watch'.

### Signature

```python
def watch(self, url: str) -> 'Watch'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/python/crawlingo/session.py:131](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L131)

<a id="crawl"></a>

## crawl()

Create a Python Crawl wrapper. Review its documented native-binding gaps before use.

Returns / value: 'Crawl'.

### Signature

```python
def crawl(self, url: str) -> 'Crawl'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | str | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/python/crawlingo/session.py:136](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L136)

<a id="clone"></a>

## clone()

Return an independent session with copied defaults and retry settings. Cookie changes on the clone do not affect the original. Middleware is not cloned.

Returns / value: 'Session'.

### Signature

```python
def clone(self) -> 'Session'
```

- [Source · sdk/python/crawlingo/session.py:141](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L141)

<a id="destroy"></a>

## destroy()

Clear selected native state and drop cached managers when no operation retains them. This is cleanup, not cancellation of running jobs. Await owned work before cleanup.

Returns / value: None.

### Signature

```python
def destroy(self) -> None
```

- [Source · sdk/python/crawlingo/session.py:147](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L147)

<a id="enter"></a>

## __enter__()

Return the same Python session for a with block.

Returns / value: 'Session'.

### Signature

```python
def __enter__(self) -> 'Session'
```

- [Source · sdk/python/crawlingo/session.py:151](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L151)

<a id="exit"></a>

## __exit__()

The current Python context-manager exit is a no-op. It does not call destroy; manage cleanup explicitly.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __exit__(self, exc_type, exc_val, exc_tb)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| exc_type | Not annotated | Required | Exception class supplied by the Python with statement, or None. |
| exc_val | Not annotated | Required | Exception instance supplied by the Python with statement, or None. |
| exc_tb | Not annotated | Required | Traceback supplied by the Python with statement, or None. |

- [Source · sdk/python/crawlingo/session.py:154](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/session.py#L154)

<a id="errors"></a>

## Errors and operational limits

Node retry setters reject invalid numbers with RangeError. Native configuration failures use CrawlingoError; Python session methods can propagate native RuntimeError or ValueError directly. Do not assume identical exception translation across bindings.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
