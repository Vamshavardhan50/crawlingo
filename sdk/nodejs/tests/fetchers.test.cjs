const { Page, Session } = require('../dist');

async function run(runner, base) {
  runner.section('Fetcher Tiers');

  runner.subsection('Fetcher Tier: standard');
  try {
    const session = new Session().fetcherTier('standard');
    const p = await Page.create(base, { session });
    runner.check('Standard tier page fetch success', p.status === 200);
    runner.check('Standard tier page content match', p.html.includes('OK'));
  } catch (e) {
    runner.check('Standard tier fetch', false, e.message);
  }

  runner.subsection('Fetcher Tier: stealthy');
  try {
    const session = new Session().fetcherTier('stealthy');
    const p = await Page.create(base, { session });
    runner.check('Stealthy tier page fetch success', p.status === 200);
    runner.check('Stealthy tier page content match', p.html.includes('OK'));
  } catch (e) {
    runner.check('Stealthy tier fetch', false, e.message);
  }

  runner.subsection('Fetcher Tier: browser');
  // Feature detection by trying to construct/set browser tier. If the FFI does not support it, it should throw or error.
  try {
    const session = new Session().fetcherTier('browser');
    const p = await Page.create(base, { session });
    runner.check('Browser tier page fetch success', p.status === 200);
  } catch (e) {
    if (e.message.toLowerCase().includes('tier') || e.message.toLowerCase().includes('not implemented') || e.message.toLowerCase().includes('unsupported')) {
      runner.missing('Fetcher Tier: browser', 'Browser engine not implemented in backend');
    } else {
      runner.check('Browser tier fetch', false, e.message);
    }
  }

  runner.subsection('Fetcher Tier: auto');
  try {
    const session = new Session().fetcherTier('auto');
    const p = await Page.create(base, { session });
    runner.check('Auto tier page fetch success', p.status === 200);
  } catch (e) {
    if (e.message.toLowerCase().includes('tier') || e.message.toLowerCase().includes('not implemented') || e.message.toLowerCase().includes('unsupported')) {
      runner.missing('Fetcher Tier: auto', 'Auto tier not implemented in backend');
    } else {
      runner.check('Auto tier fetch', false, e.message);
    }
  }
}

module.exports = { run };
