# sitemapUrlForOrigin · Node.js

Return the canonical `/sitemap.xml` URL for an origin.

<a id="overview"></a>

## Purpose and lifecycle

Return the canonical `/sitemap.xml` URL for an origin.

- [Node.js API index](../node.md)
- [Implementation status and known gaps](../implementation-status.md)

<a id="definition"></a>

## Definition and availability

Source snapshot: 1.0.0-alpha.2. Package export. Declared in source; this is not a runtime test result.

Signatures and source locations are generated from the SDK wrappers. Clean snapshots link to the recorded commit. Modified snapshots link to main, which may differ; file hashes identify the local inputs.

### TypeScript declaration

```typescript
sitemapUrlForOrigin(origin: string): string
```

- [sdk/nodejs/src/sitemap.ts:21](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L21)

<a id="members"></a>

## Members

| Member | Kind | Return / value type | Source status |
| --- | --- | --- | --- |
| sitemapUrlForOrigin | method | string | Present in source |

- [sitemapUrlForOrigin](sitemap-url-for-origin.md#sitemap-url-for-origin)

<a id="sitemap-url-for-origin"></a>

## sitemapUrlForOrigin()

Return the canonical `/sitemap.xml` URL for an origin.

Returns / value: string.

### Signature

```typescript
sitemapUrlForOrigin(origin: string): string
```

| Parameter | Type | Required / default | Meaning |
| --- | --- | --- | --- |
| origin | string | Required | Site origin used to construct the canonical /sitemap.xml URL. |

- [Source · sdk/nodejs/src/sitemap.ts:21](https://github.com/Vamshavardhan50/crawlingo/blob/main/sdk/nodejs/src/sitemap.ts#L21)

<a id="errors"></a>

## Errors and operational limits

See the consuming API for validation and failure behavior. Types and enum declarations do not perform network requests by themselves.

Source availability, package publication and runtime validation are separate facts. These pages describe the source snapshot, not a claim that all platforms or every API path have passed production acceptance.

- [Build and install the matching SDK](../../installation.md)
- [DatasetJob workflow](../../api-dataset-job.md)
