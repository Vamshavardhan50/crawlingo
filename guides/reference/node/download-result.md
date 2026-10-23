# DownloadResult · Node.js

Response metadata and bytes actually retained/written. Check status and resumed explicitly.

<a id="overview"></a>

## Purpose and lifecycle

Response metadata and bytes actually retained/written. Check status and resumed explicitly.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](downloader.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export interface DownloadResult {
  /** The final URL after any redirects. */
  url: string;
  /** HTTP response status code. */
  status: number;
  /** Number of bytes written to the output. */
  bytesWritten: number;
  /** MIME type from `Content-Type`, or `application/octet-stream`. */
  contentType: string;
  /** Filename hint from `Content-Disposition` or the URL's last path segment. */
  suggestedFilename?: string;
  /** `true` if the server honored a `Range:` request and returned `206`. */
  resumed: boolean;
}
```

- [sdk/nodejs/src/download.ts:12](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L12)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| url | property | string | Present in source |
| status | property | number | Present in source |
| bytesWritten | property | number | Present in source |
| contentType | property | string | Present in source |
| suggestedFilename | property | string \| undefined | Present in source |
| resumed | property | boolean | Present in source |

- [url](download-result.md#url)
- [status](download-result.md#status)
- [bytesWritten](download-result.md#bytes-written)
- [contentType](download-result.md#content-type)
- [suggestedFilename](download-result.md#suggested-filename)
- [resumed](download-result.md#resumed)

<a id="url"></a>

## url

The final URL after any redirects.

Returns / value: string.

### Signature

```typescript
url: string;
```

- [Source · sdk/nodejs/src/download.ts:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L14)

<a id="status"></a>

## status

HTTP response status code.

Returns / value: number.

### Signature

```typescript
status: number;
```

- [Source · sdk/nodejs/src/download.ts:16](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L16)

<a id="bytes-written"></a>

## bytesWritten

Number of bytes written to the output.

Returns / value: number.

### Signature

```typescript
bytesWritten: number;
```

- [Source · sdk/nodejs/src/download.ts:18](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L18)

<a id="content-type"></a>

## contentType

MIME type from `Content-Type`, or `application/octet-stream`.

Returns / value: string.

### Signature

```typescript
contentType: string;
```

- [Source · sdk/nodejs/src/download.ts:20](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L20)

<a id="suggested-filename"></a>

## suggestedFilename

Filename hint from `Content-Disposition` or the URL's last path segment.

Returns / value: string | undefined.

### Signature

```typescript
suggestedFilename?: string;
```

- [Source · sdk/nodejs/src/download.ts:22](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L22)

<a id="resumed"></a>

## resumed

`true` if the server honored a `Range:` request and returned `206`.

Returns / value: boolean.

### Signature

```typescript
resumed: boolean;
```

- [Source · sdk/nodejs/src/download.ts:24](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L24)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
