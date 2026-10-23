# DatasetResult · Node.js

Read and export a single extracted field map.

<a id="overview"></a>

## Purpose and lifecycle

Returned by Dataset.build. Constructing it manually requires the correct native result handle.

Export methods create or replace files. A small-result export is not a resumable dataset shard or checkpoint. Python df() needs pandas installed separately.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class DatasetResult
```

- [sdk/nodejs/src/dataset.ts:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L7)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | DatasetResult | Present in source |
| toDict | method | Record<string, string> | Present in source |
| toJson | method | Promise<void> | Present in source |
| toCsv | method | Promise<void> | Present in source |
| toParquet | method | Promise<void> | Present in source |

- [constructor](dataset-result.md#constructor)
- [toDict](dataset-result.md#to-dict)
- [toJson](dataset-result.md#to-json)
- [toCsv](dataset-result.md#to-csv)
- [toParquet](dataset-result.md#to-parquet)

<a id="constructor"></a>

## constructor()

Native interop entry point. Applications normally obtain this wrapper from Page.create, a selector or Dataset.build; do not pass a URL or an arbitrary object as a native handle.

Returns / value: DatasetResult.

### Signature

```typescript
constructor(inner: JsDatasetResult): DatasetResult
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| inner | JsDatasetResult | Required | Existing native handle for wrapper interop; not a URL or plain data object. |

- [Source · sdk/nodejs/src/dataset.ts:8](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L8)

<a id="to-dict"></a>

## toDict()

Return a copy of the extracted string-valued fields.

Returns / value: Record<string, string>.

### Signature

```typescript
toDict(): Record<string, string>
```

- [Source · sdk/nodejs/src/dataset.ts:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L10)

<a id="to-json"></a>

## toJson()

Serialize the field map to one JSON object file.

Returns / value: Promise<void>.

### Signature

```typescript
toJson(path: string): Promise<void>
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | string | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/nodejs/src/dataset.ts:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L14)

<a id="to-csv"></a>

## toCsv()

Write one header row and one data row.

Returns / value: Promise<void>.

### Signature

```typescript
toCsv(path: string): Promise<void>
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | string | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/nodejs/src/dataset.ts:18](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L18)

<a id="to-parquet"></a>

## toParquet()

Write the fields as a one-row Parquet dataset.

Returns / value: Promise<void>.

### Signature

```typescript
toParquet(path: string): Promise<void>
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| path | string | Required | Filesystem path relative to the process working directory unless absolute. |

- [Source · sdk/nodejs/src/dataset.ts:22](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset.ts#L22)

<a id="errors"></a>

## Errors and operational limits

Filesystem and serialization failures propagate. Node export methods reject with ExportError. Python can also expose OSError directly.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
