# Sessions and fetching

Share request configuration and connection pools across a job.

<a id="configure"></a>

## Configure a session

These fragments extend the config loaded in the quick start. Per-host rate limits and job concurrency control different things: the rate limits request frequency; concurrency caps simultaneous work.

### Python

```python
from crawlingo import DatasetJob, Session

session = Session().rate_limit(2).timeout(30)
session.headers({"User-Agent": "MyDatasetBot/1.0"})
job = DatasetJob(config, session=session)
```

### Node.js

```typescript
import { DatasetJob, Session } from "crawlingo";

const session = new Session().rateLimit(2).timeout(30);
session.headers({ "User-Agent": "MyDatasetBot/1.0" });
const job = new DatasetJob(config, session);
```

<a id="transport"></a>

## Choose a fetch tier

standard fetches HTTP without rendering JavaScript. stealthy configures browser-like TLS/HTTP profiles through wreq; this is transport emulation, not a guarantee of website access.

The optional Rust browser feature runs Chromium. Current Node.js Session typing exposes standard and stealthy only; do not assume all SDK packages enable browser execution.

<a id="robots"></a>

## Robots rules and pacing

Dataset jobs check robots.txt by default using the configured User-Agent. A missing robots file with HTTP 404 or 410 permits fetching; a transport error or other unsuccessful status denies it. Crawl delay is respected. Retry attempts also go through session pacing.

Session rate limiters cache their configuration per host. Configure the session before starting a run.
