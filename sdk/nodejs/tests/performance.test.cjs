const { Page, Session } = require('../dist');

async function run(runner, base) {
  runner.section('Performance & Concurrency');

  runner.subsection('Shared Session Concurrency (50 requests)');
  try {
    const session = new Session().rateLimit(200);
    const start = Date.now();
    
    // Fire 50 requests in parallel
    const reqs = Array.from({ length: 50 }, () => 
      Page.create(base, { session, timeout: 15 })
    );
    
    const responses = await Promise.all(reqs);
    const elapsed = Date.now() - start;
    
    runner.check('All 50 concurrent requests completed successfully', responses.every(p => p.status === 200));
    runner.check('Average request speed matches high throughput (> 15 req/sec)', elapsed < 3500); // 50 requests in 3.5s
  } catch (e) {
    runner.check('50 concurrent requests', false, e.message);
  }

  runner.subsection('Separate Session Concurrency');
  try {
    const start = Date.now();
    
    const reqs = Array.from({ length: 20 }, () => {
      const session = new Session().rateLimit(50);
      return Page.create(base, { session, timeout: 15 });
    });
    
    const responses = await Promise.all(reqs);
    runner.check('All separate session requests completed successfully', responses.every(p => p.status === 200));
  } catch (e) {
    runner.check('Separate session concurrent requests', false, e.message);
  }
}

module.exports = { run };
