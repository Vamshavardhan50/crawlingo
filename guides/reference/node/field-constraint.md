# FieldConstraint · Node.js

Describe a field name, its type and whether it is required.

<a id="overview"></a>

## Purpose and lifecycle

Node stores readonly name, fieldType and required properties. Python wraps a native constraint and exposes properties.

DatasetSchema.addField/add_field accepts these three values directly; it does not accept a FieldConstraint object.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class FieldConstraint
```

- [sdk/nodejs/src/schema.ts:11](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L11)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| name | property | string | Present in source |
| fieldType | property | FieldType | Present in source |
| required | property | boolean | Present in source |
| constructor | method | FieldConstraint | Present in source |

- [name](field-constraint.md#name)
- [fieldType](field-constraint.md#field-type)
- [required](field-constraint.md#required)
- [constructor](field-constraint.md#constructor)

<a id="name"></a>

## name

Unique field name or context-specific item name.

Returns / value: string.

### Signature

```typescript
public readonly name: string;
```

- [Source · sdk/nodejs/src/schema.ts:12](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L12)

<a id="field-type"></a>

## fieldType

Read the declared value. Its exact type is shown in the signature.

Returns / value: FieldType.

### Signature

```typescript
public readonly fieldType: FieldType;
```

- [Source · sdk/nodejs/src/schema.ts:13](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L13)

<a id="required"></a>

## required

Reject empty/whitespace-only required fields; default false for job fields.

Returns / value: boolean.

### Signature

```typescript
public readonly required: boolean;
```

- [Source · sdk/nodejs/src/schema.ts:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L14)

<a id="constructor"></a>

## constructor()

Store a field name, FieldType and required flag. Pass those values to DatasetSchema.addField/add_field to apply validation.

Returns / value: FieldConstraint.

### Signature

```typescript
constructor(name: string, fieldType: FieldType, required: boolean): FieldConstraint
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | string | Required | Field, header or attribute name for this operation. |
| fieldType | FieldType | Required | A FieldType enum member. |
| required | boolean | Required | Whether an empty or missing field is rejected. |

- [Source · sdk/nodejs/src/schema.ts:16](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L16)

<a id="errors"></a>

## Errors and operational limits

Invalid native type arguments may raise a binding error.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
