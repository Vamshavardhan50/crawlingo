# Downloader · Node.js

Save a response to a local file or receive its bytes in memory.

<a id="overview"></a>

## Purpose and lifecycle

allowResume uses HTTP Range when resuming an existing file. Inspect resumed and status rather than assuming the server honored the range.

The underlying transport buffers the response and applies its response-size cap. This is not an unlimited streaming downloader; chunk_size controls file writes, not network memory.

maxBytes/max_bytes limits the bytes retained/written after fetching. It does not make the upstream transfer stop at that exact byte count.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class Downloader
```

- [sdk/nodejs/src/download.ts:30](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L30)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { Downloader, Session } from 'crawlingo';
const result = await new Downloader(new Session().retries(2))
  .allowResume(true).maxBytes(1048576)
  .download('https://example.com/', 'page.html');
console.log(result.status, result.bytesWritten, result.resumed);
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | Downloader | Present in source |
| allowResume | method | this | Present in source |
| maxBytes | method | this | Present in source |
| download | method | Promise<DownloadResult> | Present in source |
| downloadToMemory | method | Promise<{ result: DownloadResult; data: Buffer; }> | Present in source |

- [constructor](downloader.md#constructor)
- [allowResume](downloader.md#allow-resume)
- [maxBytes](downloader.md#max-bytes)
- [download](downloader.md#download)
- [downloadToMemory](downloader.md#download-to-memory)

<a id="constructor"></a>

## constructor()

Create a downloader using a supplied session or a fresh default session. Configure limits and call download or download_to_memory.

Returns / value: Downloader.

### Signature

```typescript
constructor(session?: Session): Downloader
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| session | Session \| undefined | Optional | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/nodejs/src/download.ts:34](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L34)

<a id="allow-resume"></a>

## allowResume()

Enable or disable range-based continuation of an existing destination file.

Returns / value: this.

### Signature

```typescript
allowResume(enabled: boolean): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | boolean | Required | Enable or disable this behavior. |

- [Source · sdk/nodejs/src/download.ts:39](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L39)

<a id="max-bytes"></a>

## maxBytes()

Limit the returned/written body bytes. Choose a positive integer.

Returns / value: this.

### Signature

```typescript
maxBytes(n: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| n | number | Required | Count or index; see the method description. |

- [Source · sdk/nodejs/src/download.ts:44](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L44)

<a id="download"></a>

## download()

Fetch into the destination and return DownloadResult. This can append or replace depending on resume/server response.

Returns / value: Promise<DownloadResult>.

### Signature

```typescript
download(url: string, dest: string): Promise<DownloadResult>
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | string | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| dest | string | Required | Destination file path; existing content may be replaced or resumed. |

- [Source · sdk/nodejs/src/download.ts:52](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L52)

<a id="download-to-memory"></a>

## downloadToMemory()

Return result metadata and the response bytes; retain only small downloads this way.

Returns / value: Promise<{ result: DownloadResult; data: Buffer; }>.

### Signature

```typescript
downloadToMemory(url: string): Promise<{ result: DownloadResult; data: Buffer; }>
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | string | Required | HTTP(S) source URL; verify the target is appropriate for your application. |

- [Source · sdk/nodejs/src/download.ts:67](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/download.ts#L67)

<a id="errors"></a>

## Errors and operational limits

Network, timeout and filesystem failures can propagate. Node uses typed native errors; Python downloader methods may propagate native exceptions directly.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
