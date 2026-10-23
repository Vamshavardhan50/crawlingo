# Selector recovery

Understand the scope and limits of heuristic CSS selector recovery.

<a id="how"></a>

## How it works

The Dataset builder can save a DOM fingerprint when a CSS selector succeeds. If that selector later fails, the matcher scores candidate nodes using the saved fingerprint and returns a candidate above its threshold.

Similarity is not a calibrated probability of correctness. Recovery can select the wrong element, particularly on pages with many similar records. Validate recovered values before using them downstream.

<a id="enable"></a>

## Enable it on a session

Pass the session to the existing Dataset builder. The first successful selection establishes the fingerprint; recovery cannot infer a trustworthy target without that history.

### Python

```python
from crawlingo import Session

session = Session().auto_match(True)
```

### Node.js

```typescript
import { Session } from "crawlingo";

const session = new Session().autoMatch(true);
```

<a id="jobs"></a>

## Bulk job behavior

DatasetJob uses explicit selectors and validation. Auto-match is not applied to its row extraction. Review error shards, update the extraction configuration and start a new job directory when layout changes.
