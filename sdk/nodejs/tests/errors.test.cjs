const { Page } = require('../dist');

async function run(runner, base) {
  runner.section('Error Handling & Network Resiliency');

  runner.subsection('HTTP Error Status: 404');
  try {
    const p = await Page.create(`${base}/404`);
    runner.check('404 request returns status code 404', p.status === 404);
  } catch (e) {
    runner.check('404 request', false, e.message);
  }

  runner.subsection('HTTP Error Status: 500');
  try {
    const p = await Page.create(`${base}/500`);
    runner.check('500 request returns status code 500', p.status === 500);
  } catch (e) {
    runner.check('500 request', false, e.message);
  }

  runner.subsection('HTTP Error Status: 403');
  try {
    const p = await Page.create(`${base}/403`);
    runner.check('403 request returns status code 403', p.status === 403);
  } catch (e) {
    runner.check('403 request', false, e.message);
  }

  runner.subsection('Network Conn Refused');
  try {
    await Page.create('http://127.0.0.1:1', { timeout: 3 });
    runner.check('Conn refused should throw error', false, 'Successfully loaded a non-existent port');
  } catch (e) {
    runner.check('Conn refused throws error successfully', true);
  }

  runner.subsection('DNS Failure');
  try {
    await Page.create('http://non-existent-domain-name-crawlingo-12345.com', { timeout: 3 });
    runner.check('DNS failure should throw error', false, 'Loaded non-existent domain');
  } catch (e) {
    runner.check('DNS failure throws error successfully', true);
  }

  runner.subsection('Invalid URL Exception');
  try {
    await Page.create('invalid://url_format');
    runner.check('Invalid URL should throw error', false, 'Loaded invalid schema');
  } catch (e) {
    runner.check('Invalid URL throws error successfully', true);
  }

  runner.subsection('Custom Error Classes Feature Detection');
  const mainModule = require('../dist');
  const errorClasses = [
    'CrawlingoError',
    'FetchError',
    'ParseError',
    'SelectorError',
    'AutoMatchFailed',
    'TimeoutError',
    'RateLimitError',
    'ChangeDetectionError',
    'ExportError',
    'DnsError',
    'FingerprintStoreError'
  ];

  for (const errCls of errorClasses) {
    if (mainModule[errCls]) {
      runner.check(`Error class ${errCls} exported`, true);
    } else {
      runner.missing(errCls, `Custom error class is not exported in Node SDK`);
    }
  }
}

module.exports = { run };
