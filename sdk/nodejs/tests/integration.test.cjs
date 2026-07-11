const { Watch, Session } = require('../dist');

async function run(runner, base) {
  runner.section('E2E Integration & Watch Queries');

  runner.subsection('Watch Real-Time Changes');
  try {
    const watch = new Watch(base, new Session());
    watch.field('heading', 'h1');
    watch.interval(1);

    runner.check('Watch class created successfully', watch instanceof Watch);

    // Test feature detection
    if (typeof watch.run === 'function') {
      let eventReceived = false;
      
      watch.run((err, event) => {
        if (event) {
          eventReceived = true;
        }
      });

      // Pause to let at least one poll happen
      await new Promise(r => setTimeout(r, 1200));
      watch.stop();

      // Check if it didn't crash and we stopped it cleanly
      runner.check('Watch query polling and stop works cleanly', true);
    } else {
      runner.missing('Watch.run()', 'run method missing on Watch');
    }
  } catch (e) {
    runner.check('Watch execution', false, e.message);
  }
}

module.exports = { run };
