# Headers and authentication

Configure credentials in the calling application and pass request headers through a session.

<a id="headers"></a>

## Bearer headers

### Python

```python
import os
from crawlingo import Session

session = Session().headers({
    "Authorization": "Bearer " + os.environ["DATA_SOURCE_TOKEN"]
})
```

### Node.js

```typescript
import { Session } from "crawlingo";

const token = process.env.DATA_SOURCE_TOKEN;
if (!token) throw new Error("DATA_SOURCE_TOKEN is required");
const session = new Session().headers({
  Authorization: `Bearer ${token}`
});
```

<a id="scope"></a>

## Keep credentials scoped to the source

A session’s headers are shared across its requests. Use a separate session and input list for each credential scope. Keep secrets out of committed job files and examples. Redirect and destination restrictions must be considered by the application.

<a id="refresh"></a>

## Token refresh

Obtain or refresh tokens in your application. The examples here use the public SDK header methods; they do not imply an automatic OAuth refresh workflow.
