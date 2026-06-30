# crawlingo Fetch Layer

HTTP fetching subsystem configurable as a standard fetcher or stealthy browser-emulated fetcher.

## Fetcher Tiers

### `standard`

Standard HTTP client with connection pooling and automatic redirect following.

- HTTP/1.1 and HTTP/2
- Connection keep-alive
- Automatic gzip/deflate/brotli decompression
- Redirect following (max 10)
- Timeout configurable per request (default 30s)
- Retry on 5xx and transient network errors (default 3 retries)

### `stealthy`

Browser-emulated fetcher that uses TLS fingerprints and HTTP headers to mimic real browsers and avoid bot detection.

- TLS fingerprint spoofing (faked via `browser_profile`)
- Header ordering mimicry
- HTTP/2 pseudo-header ordering
- Brotli compression
- Browser-detectable feature parity (WebGL, canvas, fonts, etc.)

## Browser Profiles

| Profile | TLS Fingerprint | User-Agent |
|---------|----------------|------------|
| `chrome` | Chrome 120+ | Chrome on Windows NT 10.0 |
| `firefox` | Firefox 121+ | Firefox on Windows NT 10.0 |
| `safari` | Safari 17+ | Safari on macOS |

## Proxy Support

- HTTP/HTTPS/SOCKS5 proxies via `proxy` parameter
- Rotating proxy pool via `proxy_pool` (round-robin)
- Dynamic proxy provider via `proxy_provider` (fetches proxy list from API)

## Rate Limiting

Per-host request rate limiting to avoid overwhelming servers:

```python
session.rate_limit(2.0)  # Max 2 requests per second per host
```

## See Also

- [Session](session.md): Configures fetch settings
- [Page](page.md): Uses the fetch layer when loading URLs
- [Fingerprint](fingerprint.md): Browser fingerprint spoofing for stealth mode
