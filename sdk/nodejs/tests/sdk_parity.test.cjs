const fs = require('fs');
const path = require('path');
const NodeModule = require('../dist');

const PYTHON_SDK_DIR = path.join(__dirname, '../../python/crawlingo');

async function run(runner, base) {
  runner.section('Cross-Language SDK Parity');

  runner.subsection('Python SDK Source Audit');
  const pythonExists = fs.existsSync(PYTHON_SDK_DIR);
  runner.check('Python SDK directory exists for auditing', pythonExists);

  if (!pythonExists) {
    runner.skipped('Python SDK audit', 'Directory not found');
    return;
  }

  // Helper to check if Python class file defines a method
  function pythonDefinesMethod(file, methodName) {
    const filePath = path.join(PYTHON_SDK_DIR, file);
    if (!fs.existsSync(filePath)) return false;
    const content = fs.readFileSync(filePath, 'utf-8');
    
    // Match def method_name(
    const regex = new RegExp(`def\\s+${methodName}\\s*\\(`, 'g');
    return regex.test(content);
  }

  const expectedParityMatrix = [
    {
      className: 'Session',
      methods: [
        { js: 'headers', py: 'headers' },
        { js: 'cookies', py: 'cookies' },
        { js: 'proxy', py: 'proxy' },
        { js: 'rateLimit', py: 'rate_limit' },
        { js: 'autoMatch', py: 'auto_match' },
        { js: 'timeout', py: 'timeout' },
        { js: 'fingerprintPath', py: 'fingerprint_path' },
        { js: 'fetcherTier', py: 'fetcher_tier' },
        { js: 'browserProfile', py: 'browser_profile' },
        { js: 'autoMatchWeights', py: 'auto_match_weights' },
        { js: 'proxyPool', py: 'proxy_pool' },
        { js: 'proxyProvider', py: 'proxy_provider' },
        { js: 'clone', py: 'clone' },
        { js: 'destroy', py: 'destroy' },
      ],
      pyFile: 'session.py'
    },
    {
      className: 'Page',
      methods: [
        { js: 'create', py: 'create', static: true },
        { js: 'url', py: 'url', getter: true },
        { js: 'status', py: 'status', getter: true },
        { js: 'html', py: 'html', getter: true },
        { js: 'title', py: 'title' },
        { js: 'css', py: 'css' },
        { js: 'xpath', py: 'xpath' },
        { js: 'findText', py: 'find_text' },
        { js: 'afterText', py: 'after_text' },
        { js: 'beforeText', py: 'before_text' },
        { js: 'regex', py: 'regex' },
      ],
      pyFile: 'page.py'
    },
    {
      className: 'Dataset',
      methods: [
        { js: 'field', py: 'field' },
        { js: 'autoMatch', py: 'auto_match' },
        { js: 'timeout', py: 'timeout' },
        { js: 'headers', py: 'headers' },
        { js: 'build', py: 'build' },
        { js: 'extractStructured', py: 'extract_structured' },
        { js: 'buildStructured', py: 'build_structured' },
        { js: 'saveJson', py: 'save_json', static: true },
        { js: 'saveCsv', py: 'save_csv', static: true },
      ],
      pyFile: 'dataset.py'
    },
    {
      className: 'Downloader',
      methods: [
        { js: 'allowResume', py: 'allow_resume' },
        { js: 'maxBytes', py: 'max_bytes' },
        { js: 'download', py: 'download' },
        { js: 'downloadToMemory', py: 'download_to_memory' },
      ],
      pyFile: 'download.py'
    },
    {
      className: 'Sitemap',
      methods: [
        { js: 'maxDepth', py: 'max_depth' },
        { js: 'follow', py: 'follow' },
        { js: 'limit', py: 'limit' },
        { js: 'depth', py: 'depth' },
        { js: 'concurrency', py: 'concurrency' },
        { js: 'delay', py: 'delay' },
        { js: 'field', py: 'field' },
        { js: 'webhook', py: 'webhook' },
        { js: 'listUrls', py: 'list_urls' },
        { js: 'build', py: 'build' },
      ],
      pyFile: 'sitemap.py'
    }
  ];

  for (const item of expectedParityMatrix) {
    runner.subsection(`Parity Check: ${item.className}`);
    
    // JS Class Check
    const jsClass = NodeModule[item.className];
    runner.check(`JS Class ${item.className} exists`, typeof jsClass === 'function');

    if (typeof jsClass === 'function') {
      for (const m of item.methods) {
        let jsImplemented = false;
        
        if (m.static) {
          jsImplemented = typeof jsClass[m.js] === 'function';
        } else if (m.getter) {
          const descriptor = Object.getOwnPropertyDescriptor(jsClass.prototype, m.js);
          jsImplemented = descriptor && (typeof descriptor.get === 'function' || typeof descriptor.value === 'function');
        } else {
          jsImplemented = typeof jsClass.prototype[m.js] === 'function';
        }

        const pyImplemented = pythonDefinesMethod(item.pyFile, m.py);

        // Record parity status
        if (jsImplemented && pyImplemented) {
          runner.check(`Method parity: ${item.className}.${m.js} (JS) <-> ${m.py} (Python)`, true, 'Synchronized');
        } else {
          const missing = [];
          if (!jsImplemented) missing.push('JS');
          if (!pyImplemented) missing.push('Python');
          runner.partial(`Method parity: ${item.className}.${m.js}`, `Missing in bindings: ${missing.join(', ')}`);
        }
      }
    }
  }
}

module.exports = { run };
