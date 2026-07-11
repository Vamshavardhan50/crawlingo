const { Session, Page } = require('../dist');

async function run(runner, base) {
  runner.section('Request Cookies');

  runner.subsection('Session Cookies');
  try {
    const session = new Session().cookies({ 'test_session': 'abc123' });
    const p = await Page.create(`${base}/auth-cookie`, { session });
    runner.check('Page fetch with Session cookies successful', p.status === 200);
    runner.check('Session cookies authenticated successfully', p.html.includes('"authenticated":true'));
  } catch (e) {
    runner.check('Session cookies fetch', false, e.message);
  }

  runner.subsection('Per-Request Cookies');
  try {
    const p = await Page.create(`${base}/auth-cookie`, {
      cookies: { 'test_session': 'abc123' }
    });
    runner.check('Page fetch with request cookies successful', p.status === 200);
    runner.check('Request cookies authenticated successfully', p.html.includes('"authenticated":true'));
  } catch (e) {
    runner.check('Per-request cookies fetch', false, e.message);
  }

  runner.subsection('Feature Detection');
  runner.testFeature('Session.prototype.deleteCookie()', Session.prototype, 'deleteCookie', () => {});
  runner.testFeature('Session.prototype.clearCookies()', Session.prototype, 'clearCookies', () => {});
  runner.testFeature('Session.prototype.getCookies()', Session.prototype, 'getCookies', () => {});
}

module.exports = { run };
