const { Session } = require('../dist');

async function run(runner, base) {
  runner.section('Session Configurations');

  runner.subsection('Instantiation');
  const session = new Session();
  runner.check('Session instance created successfully', session instanceof Session);

  runner.subsection('Configuration Chaining');
  try {
    const chained = session
      .headers({ 'X-Test-Header': 'val' })
      .cookies({ 'session_id': 'xyz' })
      .proxy('http://127.0.0.1:8080')
      .rateLimit(5)
      .autoMatch(true)
      .timeout(20)
      .fingerprintPath('./fingerprints.json')
      .fetcherTier('stealthy')
      .browserProfile('chrome')
      .autoMatchWeights({ heading: 0.8 })
      .proxyPool(['http://proxy1:8080'])
      .proxyProvider(null);

    runner.check('All config methods exist and chain correctly', chained === session);
  } catch (e) {
    runner.check('Config methods chaining', false, e.message);
  }

  runner.subsection('Feature Detection');
  runner.testFeature('Session.prototype.clone()', Session.prototype, 'clone', () => {
    const cloned = session.clone();
    runner.check('Session successfully cloned', cloned instanceof Session);
  });

  runner.testFeature('Session.prototype.destroy()', Session.prototype, 'destroy', () => {
    session.destroy();
    runner.check('Session successfully destroyed', true);
  });
}

module.exports = { run };
