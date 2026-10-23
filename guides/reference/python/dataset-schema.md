# DatasetSchema · Python

Validate required fields and canonicalize string values in small Dataset results.

<a id="overview"></a>

## Purpose and lifecycle

Add named constraints, then attach the schema to a Dataset or call validate on a field map.

This class is separate from DatasetJob’s JobField validation options. JobField lives in job configuration and supports required, min_length and whitespace normalization.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class DatasetSchema
```

- [sdk/python/crawlingo/schema.py:31](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L31)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Python

```python
from crawlingo import DatasetSchema, FieldType
schema = DatasetSchema().add_field("count", FieldType.Integer, True)
print(schema.validate({"count": "42"}))
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| __init__ | method | Not annotated; see behavior notes | Present in source |
| add_field | method | 'DatasetSchema' | Present in source |
| validate | method | Dict[str, str] | Present in source |

- [__init__](dataset-schema.md#init)
- [add_field](dataset-schema.md#add-field)
- [validate](dataset-schema.md#validate)

<a id="init"></a>

## __init__()

Create an empty field schema, or wrap the optional native schema in Python. Add field constraints before validating records.

Returns / value: Not annotated; see behavior notes.

### Signature

```python
def __init__(self, _core_schema=None)
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| _core_schema | Not annotated | Default: None | Optional native schema handle; omit to create an empty schema. |

- [Source · sdk/python/crawlingo/schema.py:33](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L33)

<a id="add-field"></a>

## add_field()

Add a named type constraint; required determines whether an empty/missing value is rejected.

Returns / value: 'DatasetSchema'.

### Signature

```python
def add_field(self, name: str, field_type: FieldType, required: bool) -> 'DatasetSchema'
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | str | Required | Field, header or attribute name for this operation. |
| field_type | FieldType | Required | A FieldType enum member. |
| required | bool | Required | Whether an empty or missing field is rejected. |

- [Source · sdk/python/crawlingo/schema.py:36](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L36)

<a id="validate"></a>

## validate()

Validate the supplied map and return canonicalized string values for declared fields.

Returns / value: Dict[str, str].

### Signature

```python
def validate(self, record: Dict[str, str]) -> Dict[str, str]
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| record | Dict[str, str] | Required | String-valued field map to validate. |

- [Source · sdk/python/crawlingo/schema.py:41](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/python/crawlingo/schema.py#L41)

<a id="errors"></a>

## Errors and operational limits

Missing required values and invalid integer/float/boolean values fail validation. Node raises CrawlingoError; Python may expose native RuntimeError.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
