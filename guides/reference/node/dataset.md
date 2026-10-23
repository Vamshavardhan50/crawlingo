# Dataset · Node.js

Extract named fields from one page, with optional validation and small-result exports.

<a id="overview"></a>

## Purpose and lifecycle

field adds a named selector; build fetches once and returns DatasetResult. This API keeps results in memory.

buildStructured/extractStructured zip selector results by their positions. Missing values can change row alignment. Prefer DatasetJob with row_selector for reliable repeated records.

The Python wrapper exposes transform callbacks for ordinary build/build_async. The Node field options expose extractType. Do not infer parity from the underlying Rust field type.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class Dataset
```

- [sdk/nodejs/src/dataset.ts:33](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L33)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { Dataset } from 'crawlingo';
const result = await new Dataset('https://example.com/')
  .field('title', 'h1').build();
console.log(result.toDict());
await result.toJson('page.json');
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | Dataset | Present in source |
| field | method | this | Present in source |
| withSchema | method | this | Present in source |
| autoMatch | method | this | Present in source |
| timeout | method | this | Present in source |
| headers | method | this | Present in source |
| build | method | Promise<DatasetResult> | Present in source |
| extractStructured | method | Record<string, string>[] | Present in source |
| buildStructured | method | Promise<Record<string, string>[]> | Present in source |
| saveJson | static method | void | Present in source |
| saveCsv | static method | void | Present in source |

- [constructor](dataset.md#constructor)
- [field](dataset.md#field)
- [withSchema](dataset.md#with-schema)
- [autoMatch](dataset.md#auto-match)
- [timeout](dataset.md#timeout)
- [headers](dataset.md#headers)
- [build](dataset.md#build)
- [extractStructured](dataset.md#extract-structured)
- [buildStructured](dataset.md#build-structured)
- [saveJson](dataset.md#save-json)
- [saveCsv](dataset.md#save-csv)

<a id="constructor"></a>

## constructor()

Configure extraction for one URL with the supplied or default session. Add fields, then call build to fetch and produce a result.

Returns / value: Dataset.

### Signature

```typescript
constructor(url: string, session?: Session): Dataset
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | string | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| session | Session \| undefined | Optional | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/nodejs/src/dataset.ts:37](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L37)

<a id="field"></a>

## field()

Append a field definition. name is its output key; selector_type/selectorType chooses the selector family; default/defaultVal supplies a fallback when empty. Node extractType supports text, price, datetime, url and attr:name.

Returns / value: this.

### Signature

```typescript
field(name: string, selector: string, options?: { selectorType?: "css" | "xpath" | "regex" | "text" | "after_text" | "before_text"; defaultVal?: string; extractType?: string; }): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | string | Required | Field, header or attribute name for this operation. |
| selector | string | Required | Selector evaluated against the fetched DOM. |
| options | { selectorType?: "css" \| "xpath" \| "regex" \| "text" \| "after_text" \| "before_text"; defaultVal?: string; extractType?: string; } \| undefined | Optional | Optional settings object. Its inline signature lists the accepted keys and types. |

- [Source · sdk/nodejs/src/dataset.ts:42](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L42)

<a id="with-schema"></a>

## withSchema()

Attach DatasetSchema to validate/coerce the collected field map before returning it.

Returns / value: this.

### Signature

```typescript
withSchema(schema: DatasetSchema): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| schema | DatasetSchema | Required | DatasetSchema containing validation constraints. |

- [Source · sdk/nodejs/src/dataset.ts:56](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L56)

<a id="auto-match"></a>

## autoMatch()

Set heuristic recovery on the attached session for ordinary Dataset extraction.

Returns / value: this.

### Signature

```typescript
autoMatch(enabled: boolean): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| enabled | boolean | Required | Enable or disable this behavior. |

- [Source · sdk/nodejs/src/dataset.ts:61](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L61)

<a id="timeout"></a>

## timeout()

Set timeout on the attached session in seconds.

Returns / value: this.

### Signature

```typescript
timeout(seconds: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | number | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/nodejs/src/dataset.ts:66](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L66)

<a id="headers"></a>

## headers()

Replace headers on the attached session.

Returns / value: this.

### Signature

```typescript
headers(headers: Record<string, string>): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| headers | Record<string, string> | Required | Header names and values; avoid sharing credentials across unrelated sites. |

- [Source · sdk/nodejs/src/dataset.ts:71](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L71)

<a id="build"></a>

## build()

Fetch, parse and extract one field map. Node returns Promise<DatasetResult>; Python blocks and returns DatasetResult.

Returns / value: Promise<DatasetResult>.

### Signature

```typescript
build(): Promise<DatasetResult>
```

- [Source · sdk/nodejs/src/dataset.ts:76](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L76)

<a id="extract-structured"></a>

## extractStructured()

Extract positional rows from an already parsed Page without making another fetch.

Returns / value: Record<string, string>[].

### Signature

```typescript
extractStructured(page: Page | JsPage): Array<Record<string, string>>
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| page | Page \| JsPage | Required | Page object to extract from; distinct from a page-number pagination parameter. |

- [Source · sdk/nodejs/src/dataset.ts:85](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L85)

<a id="build-structured"></a>

## buildStructured()

Fetch the configured URL and return positional multi-row records in memory.

Returns / value: Promise<Record<string, string>[]>.

### Signature

```typescript
buildStructured(): Promise<Array<Record<string, string>>>
```

- [Source · sdk/nodejs/src/dataset.ts:93](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L93)

<a id="save-json"></a>

## saveJson()

Write the supplied records as a JSON array. The destination file is created or replaced.

Returns / value: void.

### Signature

```typescript
saveJson(records: Array<Record<string, string>>, path: string): void
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| records | Record<string, string>[] | Required | In-memory row maps to export. |
| path | string | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/nodejs/src/dataset.ts:101](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L101)

<a id="save-csv"></a>

## saveCsv()

Write a CSV header and the supplied records. The destination file is created or replaced. Use a consistent record schema.

Returns / value: void.

### Signature

```typescript
saveCsv(records: Array<Record<string, string>>, path: string): void
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| records | Record<string, string>[] | Required | In-memory row maps to export. |
| path | string | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/nodejs/src/dataset.ts:109](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L109)

<a id="errors"></a>

## Errors and operational limits

Non-success HTTP statuses abort a Dataset build. Required schema violations, regex syntax errors, fingerprint storage failures and export errors can fail the operation. Node uses the typed error bridge; Python uses its wrapper-specific conversion.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
