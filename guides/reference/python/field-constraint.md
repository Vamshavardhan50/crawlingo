# FieldConstraint · Python

Describe a field name, its type and whether it is required.

<a id="overview"></a>

## Purpose and lifecycle

Node stores readonly name, fieldType and required properties. Python wraps a native constraint and exposes properties.

DatasetSchema.addField/add_field accepts these three values directly; it does not accept a FieldConstraint object.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class FieldConstraint
```

- [sdk/python/crawlingo/schema.py:13](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L13)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| name | property | str | Present in source |
| field_type | property | FieldType | Present in source |
| required | property | bool | Present in source |

- [__init__](field-constraint.md#init)
- [name](field-constraint.md#name)
- [field_type](field-constraint.md#field-type)
- [required](field-constraint.md#required)

<a id="init"></a>

## __init__()

Store a field name, FieldType and required flag. Pass those values to DatasetSchema.addField/add_field to apply validation.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, name: str, field_type: FieldType, required: bool)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |
| field_type | FieldType | Required | A FieldType enum member. |
| required | bool | Required | Whether an empty or missing field is rejected. |

- [Source · sdk/python/crawlingo/schema.py:15](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L15)

<a id="name"></a>

## name

Unique field name or context-specific item name.

Returns / value: str.

### Signature

```python
def name(self) -> str
```

- [Source · sdk/python/crawlingo/schema.py:19](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L19)

<a id="field-type"></a>

## field_type

Read the declared value. Its exact type is shown in the signature.

Returns / value: FieldType.

### Signature

```python
def field_type(self) -> FieldType
```

- [Source · sdk/python/crawlingo/schema.py:23](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L23)

<a id="required"></a>

## required

Reject empty/whitespace-only required fields; default false for job fields.

Returns / value: bool.

### Signature

```python
def required(self) -> bool
```

- [Source · sdk/python/crawlingo/schema.py:27](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L27)

<a id="errors"></a>

## Errors and operational limits

Invalid native type arguments may raise a binding error.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
