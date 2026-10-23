# CrawlingoError · Node.js

Base class for failures reported by the Crawlingo engine.

<a id="overview"></a>

## Purpose and lifecycle

CrawlingoError is exported from the Node package. Specialized errors inherit CrawlingoError; the base inherits JavaScript Error.

Construct with new ErrorClass(message, options?). Inherited properties include name, message, stack, cause, code and optional diagnostic context. SDK boundaries convert structured native failures to this hierarchy.

Page returns HTTP statuses, including exhausted 429/500 responses. A missing selector can return an empty collection. DatasetJob records individual failures in reports/error shards. These conditions do not automatically throw every exported error class.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class CrawlingoError extends Error
```

- [sdk/nodejs/src/errors.ts:13](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L13)

<a id="catch"></a>

## Construct and catch

### TypeScript

```typescript
import { CrawlingoError } from 'crawlingo';
const error = new CrawlingoError('Operation failed');
console.log(error instanceof CrawlingoError, error.message, error.code);
```



<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| defaultCode | static property | string | Present in source |
| code | property | string | Present in source |
| url | property | string \| undefined | Present in source |
| stage | property | string \| undefined | Present in source |
| recoverable | property | boolean | Present in source |
| suggestion | property | string \| undefined | Present in source |
| seconds | property | number \| undefined | Present in source |
| host | property | string \| undefined | Present in source |
| constructor | method | CrawlingoError | Present in source |

- [defaultCode](crawlingo-error.md#default-code)
- [code](crawlingo-error.md#code)
- [url](crawlingo-error.md#url)
- [stage](crawlingo-error.md#stage)
- [recoverable](crawlingo-error.md#recoverable)
- [suggestion](crawlingo-error.md#suggestion)
- [seconds](crawlingo-error.md#seconds)
- [host](crawlingo-error.md#host)
- [constructor](crawlingo-error.md#constructor)

<a id="default-code"></a>

## defaultCode

Default code inherited by instances of this error class.

Returns / value: string.

### Signature

```typescript
static readonly defaultCode: string = 'GENERIC_FAILURE';
```

- [Source · sdk/nodejs/src/errors.ts:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L14)

<a id="code"></a>

## code

Stable Node error category code.

Returns / value: string.

### Signature

```typescript
readonly code: string;
```

- [Source · sdk/nodejs/src/errors.ts:15](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L15)

<a id="url"></a>

## url

Source/final URL according to the operation; check redirects and provenance needs.

Returns / value: string | undefined.

### Signature

```typescript
readonly url?: string;
```

- [Source · sdk/nodejs/src/errors.ts:16](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L16)

<a id="stage"></a>

## stage

Operation stage supplied by the native bridge.

Returns / value: string | undefined.

### Signature

```typescript
readonly stage?: string;
```

- [Source · sdk/nodejs/src/errors.ts:17](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L17)

<a id="recoverable"></a>

## recoverable

Advisory indication, not a successful-retry guarantee.

Returns / value: boolean.

### Signature

```typescript
readonly recoverable: boolean;
```

- [Source · sdk/nodejs/src/errors.ts:18](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L18)

<a id="suggestion"></a>

## suggestion

Human-readable recovery guidance.

Returns / value: string | undefined.

### Signature

```typescript
readonly suggestion?: string;
```

- [Source · sdk/nodejs/src/errors.ts:19](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L19)

<a id="seconds"></a>

## seconds

Timeout duration when supplied.

Returns / value: number | undefined.

### Signature

```typescript
readonly seconds?: number;
```

- [Source · sdk/nodejs/src/errors.ts:20](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L20)

<a id="host"></a>

## host

Host context when supplied.

Returns / value: string | undefined.

### Signature

```typescript
readonly host?: string;
```

- [Source · sdk/nodejs/src/errors.ts:21](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L21)

<a id="constructor"></a>

## constructor()

Create an error with a message and optional cause/diagnostic context. Construction does not perform an operation or retry it.

Returns / value: CrawlingoError.

### Signature

```typescript
constructor(message?: string, options?: CrawlingoErrorOptions): CrawlingoError
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| message | string | Default: 'Crawlingo operation failed' | Human-readable error description; do not parse it as a stable error code. |
| options | CrawlingoErrorOptions | Default: {} | Optional settings object. Its inline signature lists the accepted keys and types. |

- [Source · sdk/nodejs/src/errors.ts:23](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L23)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
