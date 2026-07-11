const { Page, Session } = require('../dist');

async function run(runner, base) {
  runner.section('Browser Profiles');

  const profiles = ['chrome', 'firefox', 'safari'];
  for (const pr of profiles) {
    runner.subsection(`Profile: ${pr}`);
    try {
      const session = new Session().browserProfile(pr);
      const p = await Page.create(base, { session });
      runner.check(`Profile '${pr}' loaded successfully`, p.status === 200);
    } catch (e) {
      runner.check(`Profile '${pr}'`, false, e.message);
    }
  }

  runner.subsection('Profile: edge');
  try {
    const session = new Session().browserProfile('edge');
    const p = await Page.create(base, { session });
    runner.check(`Profile 'edge' loaded successfully`, p.status === 200);
  } catch (e) {
    if (e.message.toLowerCase().includes('profile') || e.message.toLowerCase().includes('not implemented') || e.message.toLowerCase().includes('unsupported')) {
      runner.missing("Profile: edge", "Edge profile not supported by native backend");
    } else {
      runner.check("Profile 'edge' fetch", false, e.message);
    }
  }
}

module.exports = { run };
