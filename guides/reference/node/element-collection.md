# ElementCollection · Node.js

Read, iterate and select from multiple DOM matches.

<a id="overview"></a>

## Purpose and lifecycle

Node text/html properties return arrays. Python text() concatenates text, while texts() returns a list. Node attr(name) returns one value per match; Python attr(name) reads the first match.

Node indices are zero-based. first()/at() return null when no element exists; Python first()/last()/nth() return None.

Python filter/map run application callbacks in Python. Known issue: iteration over a materialized filtered collection currently exits without yielding its stored items; use texts() or explicit nth() access until corrected.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class ElementCollection implements Iterable<Element>
```

- [sdk/nodejs/src/element.ts:19](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L19)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
const links = page.css('a');
for (const link of links) console.log(link.attr('href'));
console.log(links.length, links.text);
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | ElementCollection | Present in source |
| length | property | number | Present in source |
| text | property | string[] | Present in source |
| html | property | string[] | Present in source |
| attr | method | (string \| null)[] | Present in source |
| first | method | Element \| null | Present in source |
| at | method | Element \| null | Present in source |
| [Symbol.iterator] | method | Iterator<Element, any, any> | Present in source |

- [constructor](element-collection.md#constructor)
- [length](element-collection.md#length)
- [text](element-collection.md#text)
- [html](element-collection.md#html)
- [attr](element-collection.md#attr)
- [first](element-collection.md#first)
- [at](element-collection.md#at)
- [[Symbol.iterator]](element-collection.md#symbol-iterator)

<a id="constructor"></a>

## constructor()

Native interop entry point. Applications normally obtain this wrapper from Page.create, a selector or Dataset.build; do not pass a URL or an arbitrary object as a native handle.

Returns / value: ElementCollection.

### Signature

```typescript
constructor(inner: JsElementCollection): ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| inner | JsElementCollection | Required | Existing native handle for wrapper interop; not a URL or plain data object. |

- [Source · sdk/nodejs/src/element.ts:20](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L20)

<a id="length"></a>

## length

Read the declared value. Its exact type is shown in the signature.

Returns / value: number.

### Signature

```typescript
length: number
```

- [Source · sdk/nodejs/src/element.ts:22](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L22)

<a id="text"></a>

## text

Node: array of each match’s text. Python: concatenated text for the collection.

Returns / value: string[].

### Signature

```typescript
text: string[]
```

- [Source · sdk/nodejs/src/element.ts:26](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L26)

<a id="html"></a>

## html

Return reconstructed HTML for every match as a Node array.

Returns / value: string[].

### Signature

```typescript
html: string[]
```

- [Source · sdk/nodejs/src/element.ts:30](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L30)

<a id="attr"></a>

## attr()

Node: array of attribute values or null. Python: attribute of the first match, or None.

Returns / value: (string | null)[].

### Signature

```typescript
attr(name: string): (string | null)[]
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | string | Required | Field, header or attribute name for this operation. |

- [Source · sdk/nodejs/src/element.ts:34](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L34)

<a id="first"></a>

## first()

Return the first match, or null/None.

Returns / value: Element | null.

### Signature

```typescript
first(): Element | null
```

- [Source · sdk/nodejs/src/element.ts:38](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L38)

<a id="at"></a>

## at()

Return a zero-based match, or null if out of bounds.

Returns / value: Element | null.

### Signature

```typescript
at(index: number): Element | null
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| index | number | Required | Zero-based element index. |

- [Source · sdk/nodejs/src/element.ts:43](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L43)

<a id="symbol-iterator"></a>

## [Symbol.iterator]()

Iterate over matched Element wrappers in collection order. See any known limitation above before using a materialized Python collection.

Returns / value: Iterator<Element, any, any>.

### Signature

```typescript
[Symbol.iterator](): Iterator<Element>
```

- [Source · sdk/nodejs/src/element.ts:48](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/element.ts#L48)

<a id="errors"></a>

## Errors and operational limits

An empty match is valid. Reject negative indices in your Node caller; the native index type is unsigned. Callback exceptions can propagate from Python map/filter.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
