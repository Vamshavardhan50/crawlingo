const { Page, Session } = require('../dist');

async function run(runner, base) {
  runner.section('Page APIs');

  runner.subsection('Page Fetching & Basics');
  try {
    const p = await Page.create(base);
    runner.check('Page.create() returns Page instance', p instanceof Page);
    runner.check('Page url matches requested url', p.url === base);
    runner.check('Page status matches expected code 200', p.status === 200);
    runner.check('Page html content loaded successfully', typeof p.html === 'string' && p.html.includes('OK'));
    runner.check('Page title matches expected value', p.title() === 'Test Page');
  } catch (e) {
    runner.check('Page creation and basics', false, e.message);
  }

  runner.subsection('Browser Navigation Features');
  runner.testFeature('Page.prototype.navigate()', Page.prototype, 'navigate', () => {});
  runner.testFeature('Page.prototype.reload()', Page.prototype, 'reload', () => {});
  runner.testFeature('Page.prototype.back()', Page.prototype, 'back', () => {});
  runner.testFeature('Page.prototype.forward()', Page.prototype, 'forward', () => {});
  runner.testFeature('Page.prototype.close()', Page.prototype, 'close', () => {});
}

module.exports = { run };
