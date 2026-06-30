const assert = require('assert');
const crawlingo = require('../dist');

assert.strictEqual(typeof crawlingo.Page, 'function');
assert.strictEqual(typeof crawlingo.Session, 'function');
assert.strictEqual(typeof crawlingo.Dataset, 'function');
assert.strictEqual(typeof crawlingo.Crawl, 'function');
assert.strictEqual(typeof crawlingo.Watch, 'function');

console.log('Node.js SDK smoke test passed');
