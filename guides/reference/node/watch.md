# Watch · Node.js

Poll selected fields and deliver detected changes to application callbacks.

<a id="overview"></a>

## Purpose and lifecycle

The first successful sample establishes a baseline; later samples are compared with it. Stop the watcher explicitly when the worker shuts down.

Node run takes an error-first callback. Python registers callbacks with on_change and typed event helpers; run blocks and run_async uses an executor.

Set a positive interval in whole seconds. Zero is not a supported polling interval. Callback delivery and baseline state are not a durable event log.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
export class Watch
```

- [sdk/nodejs/src/watch.ts:13](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L13)

<a id="example"></a>

## Usage example

Use a matching source build. Where a snippet uses page, obtain it with the Page example first. Example URLs illustrate API shape; they are not promised datasets.

### Node.js / TypeScript

```typescript
import { Watch } from 'crawlingo';
const watch = new Watch('https://example.com/').field('title', 'h1').interval(60);
watch.run((error, event) => {
  if (error) { console.error(error); return; }
  console.log(event.field, event.oldValue, event.newValue);
});
process.once('SIGINT', () => watch.stop());
```

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| constructor | method | Watch | Present in source |
| field | method | this | Present in source |
| interval | method | this | Present in source |
| run | method | void | Present in source |
| stop | method | void | Present in source |

- [constructor](watch.md#constructor)
- [field](watch.md#field)
- [interval](watch.md#interval)
- [run](watch.md#run)
- [stop](watch.md#stop)

<a id="constructor"></a>

## constructor()

Create a monitor with a URL and optional session. Add fields and callbacks before starting the polling loop with run.

Returns / value: Watch.

### Signature

```typescript
constructor(url: string, session?: Session): Watch
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| url | string | Required | HTTP(S) source URL; verify the target is appropriate for your application. |
| session | Session \| undefined | Optional | Shared request state. Omit only where the signature provides a default. |

- [Source · sdk/nodejs/src/watch.ts:16](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L16)

<a id="field"></a>

## field()

Add a named field to compare between polls. Configure all fields before starting.

Returns / value: this.

### Signature

```typescript
field(name: string, selector: string, options?: { selectorType?: "css" | "xpath"; defaultVal?: string; }): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| name | string | Required | Field, header or attribute name for this operation. |
| selector | string | Required | Selector evaluated against the fetched DOM. |
| options | { selectorType?: "css" \| "xpath"; defaultVal?: string; } \| undefined | Optional | Optional settings object. Its inline signature lists the accepted keys and types. |

- [Source · sdk/nodejs/src/watch.ts:21](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L21)

<a id="interval"></a>

## interval()

Set a positive poll interval in seconds. This is not a request timeout.

Returns / value: this.

### Signature

```typescript
interval(seconds: number): this
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| seconds | number | Required | Time in seconds; see the method-specific range and timing semantics. |

- [Source · sdk/nodejs/src/watch.ts:33](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L33)

<a id="run"></a>

## run()

Start polling. Node returns immediately and delivers callbacks; Python blocks.

Returns / value: void.

### Signature

```typescript
run(callback: (err: Error | null, event: WatchChangeEvent) => void): void
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| callback | (err: Error \| null, event: WatchChangeEvent) => void | Required | Application callback; check errors before consuming results. |

- [Source · sdk/nodejs/src/watch.ts:38](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L38)

<a id="stop"></a>

## stop()

Request cancellation of the polling loop. It does not erase a separately owned dataset directory.

Returns / value: void.

### Signature

```typescript
stop(): void
```

- [Source · sdk/nodejs/src/watch.ts:54](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/watch.ts#L54)

<a id="errors"></a>

## Errors and operational limits

Node reports poll failures to the callback. Handle err before reading the event. Python native polling logs some failures; it is not equivalent to a rejected DatasetJob.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
