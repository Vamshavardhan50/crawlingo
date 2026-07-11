const { Session, Page } = require('../dist');

async function run(runner, base) {
  runner.section('Request Headers');

  runner.subsection('Session Headers');
  try {
    const session = new Session().headers({ 'X-Session-Header': 'session-val' });
    const p = await Page.create(base, { session });
    runner.check('Page fetch with Session headers successful', p.status === 200);
  } catch (e) {
    runner.check('Session headers fetch', false, e.message);
  }

  runner.subsection('Per-Request Headers');
  try {
    const p = await Page.create(base, {
      headers: { 'X-Request-Header': 'req-val' }
    });
    runner.check('Page fetch with request headers successful', p.status === 200);
  } catch (e) {
    runner.check('Per-request headers fetch', false, e.message);
  }

  runner.subsection('Feature Detection');
  runner.testFeature('Session.prototype.removeHeader()', Session.prototype, 'removeHeader', () => {});
  runner.testFeature('Session.prototype.mergeHeaders()', Session.prototype, 'mergeHeaders', () => {});
}

module.exports = { run };
