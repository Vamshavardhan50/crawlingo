const { Session, Page } = require('../dist');

async function run(runner, base) {
  runner.section('Proxy Configurations');

  runner.subsection('Single Proxy');
  try {
    const session = new Session().proxy('http://127.0.0.1:8888');
    runner.check('Single proxy config set successfully', true);
  } catch (e) {
    runner.check('Single proxy config', false, e.message);
  }

  runner.subsection('Proxy Pool');
  try {
    const session = new Session().proxyPool(['http://p1:8080', 'http://p2:8080']);
    runner.check('Proxy pool set successfully', true);
  } catch (e) {
    runner.check('Proxy pool config', false, e.message);
  }

  runner.subsection('Proxy Provider');
  try {
    const session = new Session().proxyProvider('http://example.com/proxies.txt');
    runner.check('Proxy provider set successfully', true);
  } catch (e) {
    runner.check('Proxy provider config', false, e.message);
  }

  runner.subsection('Feature Detection');
  runner.testFeature('Session.prototype.proxyAuth()', Session.prototype, 'proxyAuth', () => {});
  runner.testFeature('Session.prototype.proxyRotate()', Session.prototype, 'proxyRotate', () => {});
}

module.exports = { run };
