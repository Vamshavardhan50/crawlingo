# JobField · Node.js

One extraction rule and its validation requirements. Exported values remain strings.

<a id="overview"></a>

## Purpose and lifecycle

One extraction rule and its validation requirements. Exported values remain strings.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)
- [Full field contract](../../schema.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export interface JobField {
  name: string;
  selector: string;
  selector_type?: 'css' | 'xpath' | 'regex' | 'text' | 'after_text' | 'before_text';
  extract_type?: 'text' | 'price' | 'datetime' | 'url' | `attr:${string}`;
  required?: boolean;
  default?: string;
  data_type?: 'string' | 'integer' | 'float' | 'boolean';
  normalize_whitespace?: boolean;
  min_length?: number;
}
```

- [sdk/nodejs/src/dataset-job.ts:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L6)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| name | property | string | Present in source |
| selector | property | string | Present in source |
| selector_type | property | "css" \| "xpath" \| "regex" \| "text" \| "after_text" \| "before_text" \| undefined | Present in source |
| extract_type | property | "text" \| "price" \| "datetime" \| "url" \| `attr:${string}` \| undefined | Present in source |
| required | property | boolean \| undefined | Present in source |
| default | property | string \| undefined | Present in source |
| data_type | property | "string" \| "boolean" \| "integer" \| "float" \| undefined | Present in source |
| normalize_whitespace | property | boolean \| undefined | Present in source |
| min_length | property | number \| undefined | Present in source |

- [name](job-field.md#name)
- [selector](job-field.md#selector)
- [selector_type](job-field.md#selector-type)
- [extract_type](job-field.md#extract-type)
- [required](job-field.md#required)
- [default](job-field.md#default)
- [data_type](job-field.md#data-type)
- [normalize_whitespace](job-field.md#normalize-whitespace)
- [min_length](job-field.md#min-length)

<a id="name"></a>

## name

Unique field name or context-specific item name.

Returns / value: string.

### Signature

```typescript
name: string;
```

- [Source · sdk/nodejs/src/dataset-job.ts:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L7)

<a id="selector"></a>

## selector

Field selector. With row_selector, matches are assigned to the row container.

Returns / value: string.

### Signature

```typescript
selector: string;
```

- [Source · sdk/nodejs/src/dataset-job.ts:8](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L8)

<a id="selector-type"></a>

## selector_type

css (default), xpath, regex, text, after_text or before_text.

Returns / value: "css" | "xpath" | "regex" | "text" | "after_text" | "before_text" | undefined.

### Signature

```typescript
selector_type?: 'css' | 'xpath' | 'regex' | 'text' | 'after_text' | 'before_text';
```

- [Source · sdk/nodejs/src/dataset-job.ts:9](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L9)

<a id="extract-type"></a>

## extract_type

text (default), price, datetime, url or attr:name.

Returns / value: "text" | "price" | "datetime" | "url" | `attr:${string}` | undefined.

### Signature

```typescript
extract_type?: 'text' | 'price' | 'datetime' | 'url' | `attr:${string}`;
```

- [Source · sdk/nodejs/src/dataset-job.ts:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L10)

<a id="required"></a>

## required

Reject empty/whitespace-only required fields; default false for job fields.

Returns / value: boolean | undefined.

### Signature

```typescript
required?: boolean;
```

- [Source · sdk/nodejs/src/dataset-job.ts:11](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L11)

<a id="default"></a>

## default

Fallback when extraction is empty.

Returns / value: string | undefined.

### Signature

```typescript
default?: string;
```

- [Source · sdk/nodejs/src/dataset-job.ts:12](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L12)

<a id="data-type"></a>

## data_type

string (default), integer, float or boolean validation; output remains string-valued.

Returns / value: "string" | "boolean" | "integer" | "float" | undefined.

### Signature

```typescript
data_type?: 'string' | 'integer' | 'float' | 'boolean';
```

- [Source · sdk/nodejs/src/dataset-job.ts:13](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L13)

<a id="normalize-whitespace"></a>

## normalize_whitespace

Collapse whitespace runs; default true.

Returns / value: boolean | undefined.

### Signature

```typescript
normalize_whitespace?: boolean;
```

- [Source · sdk/nodejs/src/dataset-job.ts:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L14)

<a id="min-length"></a>

## min_length

Minimum character count after normalization; default 0.

Returns / value: number | undefined.

### Signature

```typescript
min_length?: number;
```

- [Source · sdk/nodejs/src/dataset-job.ts:15](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/dataset-job.ts#L15)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
