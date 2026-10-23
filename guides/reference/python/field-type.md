# FieldType · Python

Select string, integer, float or boolean validation.

<a id="overview"></a>

## Purpose and lifecycle

Node enum values are String=0, Integer=1, Float=2, Boolean=3. Python enum members wrap the native enum.

Validation does not turn string-valued job exports into typed JSON numbers or booleans.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class FieldType(Enum)
```

- [sdk/python/crawlingo/schema.py:5](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L5)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| String | variant | FieldType | Present in source |
| Integer | variant | FieldType | Present in source |
| Float | variant | FieldType | Present in source |
| Boolean | variant | FieldType | Present in source |

- [String](field-type.md#string)
- [Integer](field-type.md#integer)
- [Float](field-type.md#float)
- [Boolean](field-type.md#boolean)

<a id="string"></a>

## String

Named enum member; use the symbol rather than a numeric literal.

Returns / value: FieldType.

### Signature

```python
String = _CoreFieldType.String
```

- [Source · sdk/python/crawlingo/schema.py:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L7)

<a id="integer"></a>

## Integer

Named enum member; use the symbol rather than a numeric literal.

Returns / value: FieldType.

### Signature

```python
Integer = _CoreFieldType.Integer
```

- [Source · sdk/python/crawlingo/schema.py:8](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L8)

<a id="float"></a>

## Float

Named enum member; use the symbol rather than a numeric literal.

Returns / value: FieldType.

### Signature

```python
Float = _CoreFieldType.Float
```

- [Source · sdk/python/crawlingo/schema.py:9](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L9)

<a id="boolean"></a>

## Boolean

Named enum member; use the symbol rather than a numeric literal.

Returns / value: FieldType.

### Signature

```python
Boolean = _CoreFieldType.Boolean
```

- [Source · sdk/python/crawlingo/schema.py:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L10)

<a id="errors"></a>

## Errors and operational limits

Use the named enum members; arbitrary numeric values are not a portable contract.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
