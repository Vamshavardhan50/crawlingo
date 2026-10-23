# ChangeEvent · Python

Python callback event with snake_case fields. change_type is content, price, stock, added or removed.

<a id="overview"></a>

## Purpose and lifecycle

Python callback event with snake_case fields. change_type is content, price, stock, added or removed.

- [Python API index](../python.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](../../watch.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### Python declaration

```python
class ChangeEvent
```

- [src/change/detector.rs:148](https://github.com/Vamshavardhan50/crawlingo/blob/main/src/change/detector.rs#L148)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| url | property | str | Present in source |
| field | property | str | Present in source |
| change_type | property | str | Present in source |
| old_value | property | str | Present in source |
| new_value | property | str | Present in source |
| diff | property | str | Present in source |
| detected_at | property | str | Present in source |
| similarity_score | property | float | Present in source |

- [url](change-event.md#url)
- [field](change-event.md#field)
- [change_type](change-event.md#change-type)
- [old_value](change-event.md#old-value)
- [new_value](change-event.md#new-value)
- [diff](change-event.md#diff)
- [detected_at](change-event.md#detected-at)
- [similarity_score](change-event.md#similarity-score)

<a id="url"></a>

## url

Read-only callback event field.

Returns / value: str.

### Signature

```python
url: str
```

- [Source · src/change/detector.rs:150](https://github.com/Vamshavardhan50/crawlingo/blob/main/src/change/detector.rs#L150)

<a id="field"></a>

## field

Read-only callback event field.

Returns / value: str.

### Signature

```python
field: str
```

- [Source · src/change/detector.rs:152](https://github.com/Vamshavardhan50/crawlingo/blob/main/src/change/detector.rs#L152)

<a id="change-type"></a>

## change_type

Read-only callback event field.

Returns / value: str.

### Signature

```python
change_type: str
```

- [Source · src/change/detector.rs:154](https://github.com/Vamshavardhan50/crawlingo/blob/main/src/change/detector.rs#L154)

<a id="old-value"></a>

## old_value

Read-only callback event field.

Returns / value: str.

### Signature

```python
old_value: str
```

- [Source · src/change/detector.rs:156](https://github.com/Vamshavardhan50/crawlingo/blob/main/src/change/detector.rs#L156)

<a id="new-value"></a>

## new_value

Read-only callback event field.

Returns / value: str.

### Signature

```python
new_value: str
```

- [Source · src/change/detector.rs:158](https://github.com/Vamshavardhan50/crawlingo/blob/main/src/change/detector.rs#L158)

<a id="diff"></a>

## diff

Read-only callback event field.

Returns / value: str.

### Signature

```python
diff: str
```

- [Source · src/change/detector.rs:160](https://github.com/Vamshavardhan50/crawlingo/blob/main/src/change/detector.rs#L160)

<a id="detected-at"></a>

## detected_at

Read-only callback event field.

Returns / value: str.

### Signature

```python
detected_at: str
```

- [Source · src/change/detector.rs:162](https://github.com/Vamshavardhan50/crawlingo/blob/main/src/change/detector.rs#L162)

<a id="similarity-score"></a>

## similarity_score

Read-only callback event field.

Returns / value: float.

### Signature

```python
similarity_score: float
```

- [Source · src/change/detector.rs:164](https://github.com/Vamshavardhan50/crawlingo/blob/main/src/change/detector.rs#L164)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
