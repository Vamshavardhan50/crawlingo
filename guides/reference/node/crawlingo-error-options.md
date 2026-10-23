# CrawlingoErrorOptions · Node.js

Optional Node error context, including inherited ErrorOptions.cause. Native context is optional.

<a id="overview"></a>

## Purpose and lifecycle

Optional Node error context, including inherited ErrorOptions.cause. Native context is optional.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](crawlingo-error.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export interface CrawlingoErrorOptions extends ErrorOptions {
  code?: string;
  url?: string;
  stage?: string;
  recoverable?: boolean;
  suggestion?: string;
  seconds?: number;
  host?: string;
}
```

- [sdk/nodejs/src/errors.ts:2](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L2)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| code | property | string \| undefined | Present in source |
| url | property | string \| undefined | Present in source |
| stage | property | string \| undefined | Present in source |
| recoverable | property | boolean \| undefined | Present in source |
| suggestion | property | string \| undefined | Present in source |
| seconds | property | number \| undefined | Present in source |
| host | property | string \| undefined | Present in source |

- [code](crawlingo-error-options.md#code)
- [url](crawlingo-error-options.md#url)
- [stage](crawlingo-error-options.md#stage)
- [recoverable](crawlingo-error-options.md#recoverable)
- [suggestion](crawlingo-error-options.md#suggestion)
- [seconds](crawlingo-error-options.md#seconds)
- [host](crawlingo-error-options.md#host)

<a id="code"></a>

## code

Stable Node error category code.

Returns / value: string | undefined.

### Signature

```typescript
code?: string;
```

- [Source · sdk/nodejs/src/errors.ts:3](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L3)

<a id="url"></a>

## url

Source/final URL according to the operation; check redirects and provenance needs.

Returns / value: string | undefined.

### Signature

```typescript
url?: string;
```

- [Source · sdk/nodejs/src/errors.ts:4](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L4)

<a id="stage"></a>

## stage

Operation stage supplied by the native bridge.

Returns / value: string | undefined.

### Signature

```typescript
stage?: string;
```

- [Source · sdk/nodejs/src/errors.ts:5](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L5)

<a id="recoverable"></a>

## recoverable

Advisory indication, not a successful-retry guarantee.

Returns / value: boolean | undefined.

### Signature

```typescript
recoverable?: boolean;
```

- [Source · sdk/nodejs/src/errors.ts:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L6)

<a id="suggestion"></a>

## suggestion

Human-readable recovery guidance.

Returns / value: string | undefined.

### Signature

```typescript
suggestion?: string;
```

- [Source · sdk/nodejs/src/errors.ts:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L7)

<a id="seconds"></a>

## seconds

Timeout duration when supplied.

Returns / value: number | undefined.

### Signature

```typescript
seconds?: number;
```

- [Source · sdk/nodejs/src/errors.ts:8](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L8)

<a id="host"></a>

## host

Host context when supplied.

Returns / value: string | undefined.

### Signature

```typescript
host?: string;
```

- [Source · sdk/nodejs/src/errors.ts:9](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L9)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
