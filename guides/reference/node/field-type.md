# FieldType · Node.js

Select string, integer, float or boolean validation.

<a id="overview"></a>

## Purpose and lifecycle

Node enum values are String=0, Integer=1, Float=2, Boolean=3. Python enum members wrap the native enum.

Validation does not turn string-valued job exports into typed JSON numbers or booleans.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export enum FieldType {
  String = 0,
  Integer = 1,
  Float = 2,
  Boolean = 3,
}
```

- [sdk/nodejs/src/schema.ts:4](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L4)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| String | variant | FieldType.String | Present in source |
| Integer | variant | FieldType.Integer | Present in source |
| Float | variant | FieldType.Float | Present in source |
| Boolean | variant | FieldType.Boolean | Present in source |

- [String](field-type.md#string)
- [Integer](field-type.md#integer)
- [Float](field-type.md#float)
- [Boolean](field-type.md#boolean)

<a id="string"></a>

## String

Named enum member; use the symbol rather than a numeric literal.

Returns / value: FieldType.String.

### Signature

```typescript
String = 0
```

- [Source · sdk/nodejs/src/schema.ts:5](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L5)

<a id="integer"></a>

## Integer

Named enum member; use the symbol rather than a numeric literal.

Returns / value: FieldType.Integer.

### Signature

```typescript
Integer = 1
```

- [Source · sdk/nodejs/src/schema.ts:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L6)

<a id="float"></a>

## Float

Named enum member; use the symbol rather than a numeric literal.

Returns / value: FieldType.Float.

### Signature

```typescript
Float = 2
```

- [Source · sdk/nodejs/src/schema.ts:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L7)

<a id="boolean"></a>

## Boolean

Named enum member; use the symbol rather than a numeric literal.

Returns / value: FieldType.Boolean.

### Signature

```typescript
Boolean = 3
```

- [Source · sdk/nodejs/src/schema.ts:8](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/schema.ts#L8)

<a id="errors"></a>

## Errors and operational limits

Use the named enum members; arbitrary numeric values are not a portable contract.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
