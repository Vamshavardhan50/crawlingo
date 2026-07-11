const http = require('http');
const zlib = require('zlib');
const fs = require('fs');
const path = require('path');

function createMockServer() {
  return new Promise((resolve) => {
    const srv = http.createServer((req, res) => {
      const p = req.url;
      const m = req.method;

      // Enable CORS
      res.setHeader('Access-Control-Allow-Origin', '*');
      res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS');
      res.setHeader('Access-Control-Allow-Headers', '*');

      if (m === 'OPTIONS') {
        res.writeHead(204);
        return res.end();
      }

      // Legacy routes needed by page, selectors, and datasets tests
      if (p === '/xml') {
        res.writeHead(200, { 'Content-Type': 'application/xml' });
        return res.end("<?xml version='1.0'?><root><item id='1'>val</item></root>");
      }

      if (p === '/csv') {
        res.writeHead(200, { 'Content-Type': 'text/csv' });
        return res.end('name,value\nfoo,1\nbar,2\n');
      }

      if (p === '/links') {
        res.writeHead(200, { 'Content-Type': 'text/html' });
        return res.end('<html><body><a href="/p1">P1</a><a href="/p2">P2</a><img src="/i.png"><link rel="stylesheet" href="/s.css"><script src="/a.js"></script></body></html>');
      }

      if (p === '/table') {
        res.writeHead(200, { 'Content-Type': 'text/html' });
        return res.end('<html><body><table><tr><th>N</th><th>A</th></tr><tr><td class="name">Alice</td><td class="age">30</td></tr><tr><td class="name">Bob</td><td class="age">25</td></tr></table></body></html>');
      }

      if (p === '/large') {
        res.writeHead(200, { 'Content-Type': 'text/html' });
        return res.end('<html><body>' + '<p>large</p>'.repeat(10000) + '</body></html>');
      }

      if (p === '/slow') {
        res.writeHead(200, { 'Content-Type': 'text/plain' });
        return res.end('slow');
      }

      // Compression routes
      if (p === '/gzip') {
        const body = 'gzip-compressed-content-data';
        zlib.gzip(body, (err, buffer) => {
          res.writeHead(200, {
            'Content-Encoding': 'gzip',
            'Content-Type': 'text/plain',
            'Content-Length': buffer.length,
          });
          res.end(buffer);
        });
        return;
      }

      if (p === '/brotli') {
        const body = 'brotli-compressed-content-data';
        zlib.brotliCompress(body, (err, buffer) => {
          res.writeHead(200, {
            'Content-Encoding': 'br',
            'Content-Type': 'text/plain',
            'Content-Length': buffer.length,
          });
          res.end(buffer);
        });
        return;
      }

      // Redirect routes
      if (p === '/redirect') {
        res.writeHead(302, { 'Location': '/target' });
        return res.end();
      }

      if (p === '/redirect-loop') {
        res.writeHead(302, { 'Location': '/redirect-loop' });
        return res.end();
      }

      if (p === '/target') {
        res.writeHead(200, { 'Content-Type': 'text/plain' });
        return res.end('redirected-target-content');
      }

      // Streaming routes
      if (p === '/slow-stream') {
        res.writeHead(200, { 'Content-Type': 'text/plain', 'Transfer-Encoding': 'chunked' });
        res.write('chunk-1\n');
        setTimeout(() => {
          res.write('chunk-2\n');
          setTimeout(() => {
            res.end('chunk-3\n');
          }, 100);
        }, 100);
        return;
      }

      if (p === '/chunked') {
        res.writeHead(200, { 'Content-Type': 'text/plain', 'Transfer-Encoding': 'chunked' });
        for (let i = 1; i <= 5; i++) {
          res.write(`chunk-${i}\n`);
        }
        res.end();
        return;
      }

      // Auth routes
      if (p === '/auth-basic') {
        const auth = req.headers['authorization'] || '';
        if (auth === 'Basic dGVzdDpwYXNz') { // test:pass
          res.writeHead(200, { 'Content-Type': 'application/json' });
          return res.end(JSON.stringify({ authenticated: true, type: 'basic' }));
        }
        res.writeHead(401, { 'WWW-Authenticate': 'Basic realm="test"' });
        return res.end('Unauthorized');
      }

      if (p === '/auth-bearer') {
        const auth = req.headers['authorization'] || '';
        if (auth === 'Bearer test_token') {
          res.writeHead(200, { 'Content-Type': 'application/json' });
          return res.end(JSON.stringify({ authenticated: true, type: 'bearer' }));
        }
        res.writeHead(401);
        return res.end('Unauthorized');
      }

      if (p === '/auth-cookie') {
        const cookie = req.headers['cookie'] || '';
        if (cookie.includes('test_session=abc123')) {
          res.writeHead(200, { 'Content-Type': 'application/json' });
          return res.end(JSON.stringify({ authenticated: true, type: 'cookie' }));
        }
        res.writeHead(401);
        return res.end('Unauthorized');
      }

      if (p === '/auth-apikey') {
        const key = req.headers['x-api-key'] || '';
        if (key === 'secret') {
          res.writeHead(200, { 'Content-Type': 'application/json' });
          return res.end(JSON.stringify({ authenticated: true, type: 'apikey' }));
        }
        res.writeHead(401);
        return res.end('Unauthorized');
      }

      if (p === '/csrf') {
        const token = req.headers['x-csrf-token'];
        if (token === 'csrf-secret-123') {
          res.writeHead(200, { 'Content-Type': 'application/json' });
          return res.end(JSON.stringify({ csrf: 'valid' }));
        }
        res.writeHead(403);
        return res.end('CSRF Token Invalid');
      }

      // Large and malformed payload routes
      if (p === '/large-json') {
        const data = {
          items: Array.from({ length: 1000 }, (_, i) => ({
            id: i,
            name: `Item ${i}`,
            value: Math.random(),
            tags: ['tag1', 'tag2', 'tag3'],
            nested: { active: true, score: 100 }
          }))
        };
        res.writeHead(200, { 'Content-Type': 'application/json' });
        return res.end(JSON.stringify(data));
      }

      if (p === '/large-html') {
        const data = '<html><body>' + '<div><p class="text">nested text content</p></div>'.repeat(1000) + '</body></html>';
        res.writeHead(200, { 'Content-Type': 'text/html' });
        return res.end(data);
      }

      if (p === '/large-xml') {
        const items = Array.from({ length: 100 }, (_, i) => `<item id="${i}"><name>Val ${i}</name></item>`).join('');
        const data = `<?xml version="1.0" encoding="UTF-8"?><root>${items}</root>`;
        res.writeHead(200, { 'Content-Type': 'application/xml' });
        return res.end(data);
      }

      if (p === '/malformed-html') {
        res.writeHead(200, { 'Content-Type': 'text/html' });
        return res.end('<html><head><title>Malformed</title><body><h1>Header without close <div>no close div</html>');
      }

      // ETag and Cache-Control
      if (p === '/cache-control') {
        res.writeHead(200, {
          'Cache-Control': 'public, max-age=3600',
          'ETag': 'etag-12345',
          'Content-Type': 'text/plain'
        });
        return res.end('cacheable-content');
      }

      // Robots.txt
      if (p === '/robots.txt') {
        res.writeHead(200, { 'Content-Type': 'text/plain' });
        return res.end('User-agent: *\nDisallow: /private\nAllow: /\nSitemap: http://127.0.0.1/sitemap.xml');
      }

      // Binary download route
      if (p === '/download.bin') {
        res.writeHead(200, {
          'Content-Type': 'application/octet-stream',
          'Content-Disposition': 'attachment; filename="test_output.bin"',
        });
        return res.end(Buffer.from('file-download-data-stream-content'));
      }

      // Sitemap XML
      if (p === '/sitemap.xml') {
        res.writeHead(200, { 'Content-Type': 'application/xml' });
        return res.end(`<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>/target</loc>
    <lastmod>2026-07-11</lastmod>
    <changefreq>daily</changefreq>
    <priority>0.8</priority>
  </url>
</urlset>`);
      }

      // Timeout simulation route
      if (p === '/timeout') {
        setTimeout(() => {
          res.writeHead(200);
          res.end('delayed-response');
        }, 15000); // 15 seconds delay
        return;
      }

      // 404, 500, 403 HTTP codes
      if (['/404', '/500', '/403'].includes(p)) {
        const code = parseInt(p.slice(1));
        res.writeHead(code, { 'Content-Type': 'text/plain' });
        return res.end(`Error ${code}`);
      }

      // Default index page
      res.writeHead(200, {
        'Content-Type': 'text/html',
        'Set-Cookie': 'test_session=abc123; Path=/',
      });
      res.end('<html><head><title>Test Page</title></head><body><h1>OK</h1><p>test paragraph</p><a href="/target">link</a></body></html>');
    });

    srv.listen(0, '127.0.0.1', () => {
      resolve(srv);
    });
  });
}

module.exports = { createMockServer };
