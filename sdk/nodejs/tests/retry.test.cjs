const { Session, Page } = require('../dist');

async function run(runner, base) {
  runner.section('Retry Logic & Backoff');

  runner.subsection('Default Retries');
  try {
    const p = await Page.create(base);
    runner.check('Page fetch with default retries successful', p.status === 200);
  } catch (e) {
    runner.check('Default retry page fetch', false, e.message);
  }

  runner.subsection('Feature Detection');
  runner.testFeature('Session.prototype.retries()', Session.prototype, 'retries', () => {});
  runner.testFeature('Session.prototype.retryBackoff()', Session.prototype, 'retryBackoff', () => {});
  runner.testFeature('Session.prototype.retryDelay()', Session.prototype, 'retryDelay', () => {});
}

module.exports = { run };
