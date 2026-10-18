# Deploy the documentation

Build and host this Next.js website with working routes, shared content and a repeatable preview.

> Current development source. New dataset APIs require a matching source build; published alpha packages may differ.

## Build a production preview

Run inside crawlingo-docs/. Open http://localhost:3020 and check a guide by entering its URL directly. This workspace has a separate Git repository for the site; committing the Rust repository alone does not deploy these files.

### Website repository

```shell
npm ci
npm run lint
npm run build
npm start -- --port 3020
```

## Vercel

- Import the website repository. Use its root as the project directory; use crawlingo-docs only if you intentionally track the site inside a monorepo.
- Choose Next.js, Node 22, npm ci and npm run build. Let the framework choose its output directory.
- Preview the change, then check /quick-start, /examples and /publishing by direct navigation.
- Use the supplied vercel.json. The old index.html catch-all has been removed.

- [Next.js deployment options](https://nextjs.org/docs/pages/getting-started/deploying)

## Netlify

Connect the website repository, use npm run build and .next as the publish directory. The supplied netlify.toml sets Node 22. Netlify applies its Next.js adapter; do not add an SPA redirect to index.html.

- [Netlify Next.js guide](https://docs.netlify.com/build/frameworks/framework-setup-guides/nextjs/overview/)

## A Node server

Run npm ci, npm run build and npm start under a process supervisor. Keep package.json, production dependencies, .next and public available. Put your usual HTTPS reverse proxy in front of the Node process.

This site uses Next.js routing, including a server redirect from /docs. It is not currently configured as a static export for GitHub Pages.

- [Next.js self hosting](https://nextjs.org/docs/pages/guides/self-hosting)

## Keep content and metadata together

src/content/docs.ts owns the API guides and navigation. src/content/operations.ts owns runnable examples and deployment guides. Pages are thin route components using documentation-page.tsx.

npm run check:docs checks routes, section links and JSON snippets. npm run build regenerates the sitemap from the content registry. If the public domain changes, update src/content/site.json: layout metadata, robots, sitemap and exported guide links use that value.

- [Website source](https://github.com/Vamshavardhan50/crawlingo-docs)
