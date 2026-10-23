# WatchChangeEvent · Node.js

Node callback event with camelCase names and change types such as ContentChange and PriceChange.

<a id="overview"></a>

## Purpose and lifecycle

Node callback event with camelCase names and change types such as ContentChange and PriceChange.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](../../watch.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export interface WatchChangeEvent {
  url: string;
  field: string;
  changeType: string;
  oldValue?: string;
  newValue?: string;
}
```

- [sdk/nodejs/src/watch.ts:5](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L5)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| url | property | string | Present in source |
| field | property | string | Present in source |
| changeType | property | string | Present in source |
| oldValue | property | string \| undefined | Present in source |
| newValue | property | string \| undefined | Present in source |

- [url](watch-change-event.md#url)
- [field](watch-change-event.md#field)
- [changeType](watch-change-event.md#change-type)
- [oldValue](watch-change-event.md#old-value)
- [newValue](watch-change-event.md#new-value)

<a id="url"></a>

## url

Source/final URL according to the operation; check redirects and provenance needs.

Returns / value: string.

### Signature

```typescript
url: string;
```

- [Source · sdk/nodejs/src/watch.ts:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L6)

<a id="field"></a>

## field

Name of the monitored field.

Returns / value: string.

### Signature

```typescript
field: string;
```

- [Source · sdk/nodejs/src/watch.ts:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L7)

<a id="change-type"></a>

## changeType

Binding-specific change category.

Returns / value: string.

### Signature

```typescript
changeType: string;
```

- [Source · sdk/nodejs/src/watch.ts:8](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L8)

<a id="old-value"></a>

## oldValue

Previously observed string value, when supplied.

Returns / value: string | undefined.

### Signature

```typescript
oldValue?: string;
```

- [Source · sdk/nodejs/src/watch.ts:9](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L9)

<a id="new-value"></a>

## newValue

Newly observed string value, when supplied.

Returns / value: string | undefined.

### Signature

```typescript
newValue?: string;
```

- [Source · sdk/nodejs/src/watch.ts:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L10)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
