# Advanced integration

Extend the Rust pipeline while keeping backpressure and failure reporting explicit.

<a id="transport"></a>

## Custom transports

Rust Session supports a custom Transport, including MockTransport for deterministic fixtures. Custom transports must enforce their own streaming body limit. The job’s byte check happens after a response is available.

<a id="stream"></a>

## Record streams

Use DatasetStream when you already have a Rust record producer. Await handle.send to apply backpressure. Start the export consumer while the producer runs, and drop all producer handles to signal completion.

- [Rust SDK](sdk/rust.md#stream)

<a id="service"></a>

## Embedding in a service

Do not expose arbitrary URL submission without a separate destination policy. The current URL validator is not an SSRF boundary for an untrusted public service. Assign each job its own directory and enforce disk quotas and lifecycle ownership in the service.
