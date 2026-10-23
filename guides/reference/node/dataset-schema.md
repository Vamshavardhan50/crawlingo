# DatasetSchema · Node.js

Validate required fields and canonicalize string values in small Dataset results.

<a id="overview"></a>

## Purpose and lifecycle

Add named constraints, then attach the schema to a Dataset or call validate on a field map.

This class is separate from DatasetJob’s JobField validation options. JobField lives in job configuration and supports required, min_length and whitespace normalization.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class DatasetSchema
```

- [sdk/nodejs/src/schema.ts:23](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L23)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { DatasetSchema, FieldType } from 'crawlingo';
const schema = new DatasetSchema().addField('count', FieldType.Integer, true);
console.log(schema.validate({ count: '42' }));
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| inner | property | JsDatasetSchema | Present in source |
| constructor | method | DatasetSchema | Present in source |
| addField | method | this | Present in source |
| validate | method | Record<string, string> | Present in source |

- [inner](dataset-schema.md#inner)
- [constructor](dataset-schema.md#constructor)
- [addField](dataset-schema.md#add-field)
- [validate](dataset-schema.md#validate)

<a id="inner"></a>

## inner

Native interop handle. Prefer the documented wrapper methods; bypassing them can bypass validation and typed error conversion.

Returns / value: JsDatasetSchema.

### Signature

```typescript
public readonly inner: JsDatasetSchema;
```

- [Source · sdk/nodejs/src/schema.ts:24](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L24)

<a id="constructor"></a>

## constructor()

Create an empty field schema, or wrap the optional native schema in Python. Add field constraints before validating records.

Returns / value: DatasetSchema.

### Signature

```typescript
constructor(): DatasetSchema
```

- [Source · sdk/nodejs/src/schema.ts:26](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L26)

<a id="add-field"></a>

## addField()

Add a named type constraint; required determines whether an empty/missing value is rejected.

Returns / value: this.

### Signature

```typescript
addField(name: string, fieldType: FieldType, required: boolean): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | string | Required | Field, header or attribute name for this operation. |
| fieldType | FieldType | Required | A FieldType enum member. |
| required | boolean | Required | Whether an empty or missing field is rejected. |

- [Source · sdk/nodejs/src/schema.ts:30](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L30)

<a id="validate"></a>

## validate()

Validate the supplied map and return canonicalized string values for declared fields.

Returns / value: Record<string, string>.

### Signature

```typescript
validate(record: Record<string, string>): Record<string, string>
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| record | Record<string, string> | Required | String-valued field map to validate. |

- [Source · sdk/nodejs/src/schema.ts:35](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L35)

<a id="errors"></a>

## Errors and operational limits

Missing required values and invalid integer/float/boolean values fail validation. Node raises CrawlingoError; Python may expose native RuntimeError.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
