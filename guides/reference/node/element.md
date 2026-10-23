# Element · Node.js

Read one selected DOM element and its attributes.

<a id="overview"></a>

## Purpose and lifecycle

Obtain elements from a Page collection rather than constructing wrappers around native handles yourself.

Node uses text and html properties. Python uses text() and html() methods and exposes additional traversal helpers. Python Element is available through crawlingo.element and as a collection result, not as a top-level import.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class Element
```

- [sdk/nodejs/src/element.ts:3](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L3)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
const heading = page.css('h1').first();
if (heading) console.log(heading.text, heading.attr('id'));
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | Element | Present in source |
| text | property | string | Present in source |
| html | property | string | Present in source |
| attr | method | string \| null | Present in source |

- [constructor](element.md#constructor)
- [text](element.md#text)
- [html](element.md#html)
- [attr](element.md#attr)

<a id="constructor"></a>

## constructor()

Native interop entry point. Applications normally obtain this wrapper from Page.create, a selector or Dataset.build; do not pass a URL or an arbitrary object as a native handle.

Returns / value: Element.

### Signature

```typescript
constructor(inner: JsElement): Element
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| inner | JsElement | Required | Existing native handle for wrapper interop; not a URL or plain data object. |

- [Source · sdk/nodejs/src/element.ts:4](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L4)

<a id="text"></a>

## text

Read descendant text in document order. Python after-extract hooks apply to this value.

Returns / value: string.

### Signature

```typescript
text: string
```

- [Source · sdk/nodejs/src/element.ts:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L6)

<a id="html"></a>

## html

Read the reconstructed outer HTML for the selected node.

Returns / value: string.

### Signature

```typescript
html: string
```

- [Source · sdk/nodejs/src/element.ts:10](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L10)

<a id="attr"></a>

## attr()

Look up one attribute by name. Missing values return null in Node or None in Python.

Returns / value: string | null.

### Signature

```typescript
attr(name: string): string | null
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | string | Required | Field, header or attribute name for this operation. |

- [Source · sdk/nodejs/src/element.ts:14](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L14)

<a id="errors"></a>

## Errors and operational limits

Absent attributes and absent relatives may return null/None even where a Python annotation is narrower. Node exposes only text, html and attr; do not translate Python traversal calls directly.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
