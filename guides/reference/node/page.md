# Page · Node.js

Fetch one page and inspect its parsed HTML before defining a dataset schema.

<a id="overview"></a>

## Purpose and lifecycle

Python Page(url) is lazy; reading status, html or selectors triggers loading. Python Page.create fetches immediately. Node Page.create returns a promise; the Node constructor accepts an existing native page.

A page retains HTML and a DOM tree. Use DatasetJob for large inputs rather than keeping a list of Page objects.

HTTP statuses such as 404, 429 and 500 are returned through status, including after retries are exhausted. They do not automatically become typed HTTP exceptions.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class Page
```

- [sdk/nodejs/src/page.ts:6](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L6)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { Page, Session, TimeoutError } from 'crawlingo';
try {
  const page = await Page.create('https://example.com/', { session: new Session().retries(1) });
  console.log(page.status, page.css('h1').text);
} catch (error) {
  if (error instanceof TimeoutError) console.error(error.seconds);
  else throw error;
}
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | Page | Present in source |
| create | static method | Promise<Page> | Present in source |
| url | property | string | Present in source |
| status | property | number | Present in source |
| html | property | string | Present in source |
| title | method | string | Present in source |
| css | method | ElementCollection | Present in source |
| xpath | method | ElementCollection | Present in source |
| findText | method | ElementCollection | Present in source |
| afterText | method | ElementCollection | Present in source |
| beforeText | method | ElementCollection | Present in source |
| regex | method | ElementCollection | Present in source |
| nativePage | property | JsPage | Present in source |

- [constructor](page.md#constructor)
- [create](page.md#create)
- [url](page.md#url)
- [status](page.md#status)
- [html](page.md#html)
- [title](page.md#title)
- [css](page.md#css)
- [xpath](page.md#xpath)
- [findText](page.md#find-text)
- [afterText](page.md#after-text)
- [beforeText](page.md#before-text)
- [regex](page.md#regex)
- [nativePage](page.md#native-page)

<a id="constructor"></a>

## constructor()

Native interop entry point. Applications normally obtain this wrapper from Page.create, a selector or Dataset.build; do not pass a URL or an arbitrary object as a native handle.

Returns / value: Page.

### Signature

```typescript
constructor(nativePage: JsPage): Page
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| nativePage | JsPage | Required | Existing native page handle; applications should normally call Page.create. |

- [Source · sdk/nodejs/src/page.ts:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L7)

<a id="create"></a>

## create()

Fetch and parse the URL. Node request headers/cookies merge over the session defaults; explicit timeout, proxy and profile take precedence. Python passes its Page defaults to native code, including 30 seconds and 3 retries.

Returns / value: Promise<Page>.

### Signature

```typescript
create(url: string, options?: { autoMatch?: boolean; timeout?: number; headers?: Record<string, string>; cookies?: Record<string, string>; proxy?: string; browserProfile?: string; session?: Session; }): Promise<Page>
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | string | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| options | { autoMatch?: boolean; timeout?: number; headers?: Record<string, string>; cookies?: Record<string, string>; proxy?: string; browserProfile?: string; session?: Session; } \| undefined | Optional | Optional settings object. Its inline signature lists the accepted keys and types. |

- [Source · sdk/nodejs/src/page.ts:12](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L12)

<a id="url"></a>

## url

Source/final URL according to the operation; check redirects and provenance needs.

Returns / value: string.

### Signature

```typescript
url: string
```

- [Source · sdk/nodejs/src/page.ts:37](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L37)

<a id="status"></a>

## status

HTTP response status code.

Returns / value: number.

### Signature

```typescript
status: number
```

- [Source · sdk/nodejs/src/page.ts:41](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L41)

<a id="html"></a>

## html

Load the page if necessary and return its HTML string. Inspect status separately; an HTTP error response can still have a body.

Returns / value: string.

### Signature

```typescript
html: string
```

- [Source · sdk/nodejs/src/page.ts:45](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L45)

<a id="title"></a>

## title()

Return the text of the title element, or an empty string when absent.

Returns / value: string.

### Signature

```typescript
title(): string
```

- [Source · sdk/nodejs/src/page.ts:49](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L49)

<a id="css"></a>

## css()

Evaluate the supported CSS subset and return a collection in document order. This is not a browser querySelectorAll implementation.

Returns / value: ElementCollection.

### Signature

```typescript
css(selector: string): ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| selector | string | Required | Selector evaluated against the fetched DOM. |

- [Source · sdk/nodejs/src/page.ts:53](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L53)

<a id="xpath"></a>

## xpath()

Evaluate the supported basic XPath subset; advanced XPath axes/functions are not implemented.

Returns / value: ElementCollection.

### Signature

```typescript
xpath(query: string): ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| query | string | Required | Supported XPath expression. |

- [Source · sdk/nodejs/src/page.ts:57](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L57)

<a id="find-text"></a>

## findText()

Find DOM nodes by text content using the engine text selector.

Returns / value: ElementCollection.

### Signature

```typescript
findText(text: string): ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| text | string | Required | Text anchor used for matching. |

- [Source · sdk/nodejs/src/page.ts:61](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L61)

<a id="after-text"></a>

## afterText()

Select the element after a matching text anchor; inspect sample output before relying on a repeated label.

Returns / value: ElementCollection.

### Signature

```typescript
afterText(text: string): ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| text | string | Required | Text anchor used for matching. |

- [Source · sdk/nodejs/src/page.ts:65](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L65)

<a id="before-text"></a>

## beforeText()

Select the element before a matching text anchor.

Returns / value: ElementCollection.

### Signature

```typescript
beforeText(text: string): ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| text | string | Required | Text anchor used for matching. |

- [Source · sdk/nodejs/src/page.ts:69](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L69)

<a id="regex"></a>

## regex()

Compile the Rust regular expression and match text. Invalid syntax produces SelectorError. Rust regex does not support look-around or backreferences.

Returns / value: ElementCollection.

### Signature

```typescript
regex(pattern: string): ElementCollection
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| pattern | string | Required | Rust regular expression. |

- [Source · sdk/nodejs/src/page.ts:73](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L73)

<a id="native-page"></a>

## nativePage

Native interop entry point. Applications normally obtain this wrapper from Page.create, a selector or Dataset.build; do not pass a URL or an arbitrary object as a native handle.

Returns / value: JsPage.

### Signature

```typescript
public readonly nativePage: JsPage
```

- [Source · sdk/nodejs/src/page.ts:7](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/page.ts#L7)

<a id="errors"></a>

## Errors and operational limits

Network failures reject/raise. Node maps native timeout, DNS and regex selector failures to typed errors. Missing selector matches return an empty collection. Python exception conversion is message-based.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
