# Session

Configure request behavior before creating a job. Python uses snake_case methods; Node.js uses camelCase.

<a id="create"></a>

## Create and pass a session

### Python

```python
import json
from crawlingo import DatasetJob, Session

with open("job.json", encoding="utf-8") as source:
    config = json.load(source)

session = Session().rate_limit(2).timeout(30)
session.headers({"User-Agent": "MyDatasetBot/1.0"})
report = DatasetJob(config, session=session).run()
print(report)
```

### Node.js

```javascript
import { readFileSync } from 'node:fs';
import { DatasetJob, Session } from 'crawlingo';

const config = JSON.parse(readFileSync('job.json', 'utf8'));
const session = new Session().rateLimit(2).timeout(30);
session.headers({ 'User-Agent': 'MyDatasetBot/1.0' });
const report = await new DatasetJob(config, session).run();
console.log(report);
```

- [Create job.json](api-job-config.md#minimal)

<a id="requests"></a>

## Request configuration methods

These setters return the session for chaining. Configure them before the first request; per-host rate limiters cache their configuration. timeout uses seconds. Session pacing is per host, while JobConfig.concurrency limits simultaneous page work.

| Python | Node.js | Argument | Purpose |
| --- | --- | --- | --- |
| headers(headers) | headers(headers) | dict / Record<string, string> | Request headers, including the User-Agent used for robots matching. |
| cookies(cookies) | cookies(cookies) | dict / Record<string, string> | Request cookies. |
| timeout(seconds) | timeout(seconds) | int / number | Request timeout in seconds. |
| rate_limit(requests_per_second) | rateLimit(requestsPerSecond) | float / number | Per-host request pacing. |
| proxy(proxy_url) | proxy(proxyUrl) | string | Configure an outbound proxy URL. |
| proxy_pool(proxies) | proxyPool(proxies) | list[str] / string[] | Configure a set of proxy URLs. |
| proxy_provider(url) | proxyProvider(url) | URL string; Node also accepts null | Configure a proxy-provider URL. |

<a id="transport"></a>

## Transport and selector recovery

A transport profile does not guarantee access to a target site. Configure browser rendering only through a build and API that expose it.

| Python | Node.js | Values / behavior |
| --- | --- | --- |
| fetcher_tier(tier) | fetcherTier(tier) | Node typing supports standard and stealthy. Chromium requires a separate browser-enabled Rust build. |
| browser_profile(profile) | browserProfile(profile) | Transport profile: chrome, firefox or safari in the Node type. |
| auto_match(enabled) | autoMatch(enabled) | Enable selector recovery for supported existing extraction APIs. Not used by DatasetJob row extraction. |
| fingerprint_path(path) | fingerprintPath(path) | Location for stored selector fingerprints. |
| auto_match_weights(weights) | autoMatchWeights(weights) | Dictionary / record of numeric matching weights. |

- [Selector recovery scope](auto-match.md)
- [Authentication examples](authentication.md)

<a id="ownership"></a>

## Ownership and binding differences

clone() returns another Session wrapper backed by the native clone operation. destroy() invokes native cleanup and returns None / void; avoid destroying a session while its jobs are running.

Both bindings load config: Python Session.from_config(path) and Node Session.fromConfig(path). Python additionally provides basic_auth, bearer_auth, metrics, page and dataset helpers. Node exposes retries, retryDelay, retryBackoff, getCookies, deleteCookie and clearCookies; the public Python wrapper does not yet expose those methods. Read the language-specific signatures.

- [Every Node Session member](reference/node/session.md)
- [Every Python Session member](reference/python/session.md)
- [Page methods](api-page.md)
