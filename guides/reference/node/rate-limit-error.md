# RateLimitError · Node.js

class declaration and members.

<a id="overview"></a>

## Purpose and lifecycle

RateLimitError is exported from the Node package. Specialized errors inherit CrawlingoError; the base inherits JavaScript Error.

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
export class RateLimitError extends CrawlingoError
```

- [sdk/nodejs/src/errors.ts:41](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L41)

<a id="catch"></a>

## Construct and catch

### TypeScript

```typescript
import { RateLimitError, CrawlingoError } from 'crawlingo';
const error = new RateLimitError('Operation failed');
console.log(error instanceof CrawlingoError, error.message, error.code);
```

- [Base exception contract](crawlingo-error.md)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| defaultCode | static property | "RATE_LIMIT" | Present in source |

- [defaultCode](rate-limit-error.md#default-code)

<a id="default-code"></a>

## defaultCode

Default code inherited by instances of this error class.

Returns / value: "RATE_LIMIT".

### Signature

```typescript
static readonly defaultCode = 'RATE_LIMIT';
```

- [Source · sdk/nodejs/src/errors.ts:41](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/errors.ts#L41)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
