# Authentication

Crawlingo provides built-in authentication helpers for common web auth schemes.

## Basic Auth

```python
from crawlingo.auth import BasicAuth

auth = BasicAuth("username", "password")
session.headers(auth.headers())
# Sets: Authorization: Basic base64(username:password)
```

## Bearer Token

```python
from crawlingo.auth import BearerAuth

auth = BearerAuth("your-token-here")
session.headers(auth.headers())
# Sets: Authorization: Bearer your-token-here
```

## Custom Header Auth

```python
from crawlingo.auth import HeaderAuth

auth = HeaderAuth("X-API-Key", "abc123")
session.headers(auth.headers())
# Sets: X-API-Key: abc123
```

## API Key Auth (Query Parameter)

```python
from crawlingo.auth import ApiKeyQueryAuth

# Appends ?api_key=xyz to every request URL
auth = ApiKeyQueryAuth("api_key", "xyz")
# Session automatically appends to URLs
```

## Cookie Auth

```python
session.cookies({
    "session_id": "abc123",
    "auth_token": "xyz789"
})
```

## Dynamic Auth (OAuth2 / Token Refresh)

```python
from crawlingo.auth import DynamicAuth

def refresh_token():
    # Fetch new token from your auth server
    response = requests.post("https://auth.example.com/token", json={
        "grant_type": "client_credentials",
        "client_id": "...",
        "client_secret": "..."
    })
    return response.json()["access_token"]

auth = DynamicAuth(refresh_token, min_validity_secs=60)
session.headers(auth.headers())
# Automatically refreshes the token before it expires
```

## Session Auth Integration

```python
from crawlingo import Session
from crawlingo.auth import BearerAuth

session = Session()
session.headers(BearerAuth("my-token").headers())
session.rate_limit(5.0)

# All requests through this session use the auth header
page = session.page("https://api.example.com/data")
```
