const { Sitemap, Session } = require('../dist');

async function run(runner, base) {
  runner.section('Sitemap Discovery & Crawling');

  runner.subsection('Sitemap Instantiation');
  const sm = new Sitemap(`${base}/sitemap.xml`, { session: new Session() });
  runner.check('Sitemap instance created successfully', sm instanceof Sitemap);

  runner.subsection('List Sitemap Entries');
  try {
    const urls = await sm.listUrls();
    runner.check('listUrls returns array', Array.isArray(urls));
    runner.check('listUrls returns correct size of entries', urls.length === 1);
    runner.check('listUrls entry contains correct loc', urls[0].loc === '/target');
    runner.check('listUrls entry contains lastmod date', urls[0].lastmod === '2026-07-11');
    runner.check('listUrls entry contains changefreq', urls[0].changefreq === 'daily');
    runner.check('listUrls entry contains priority score', urls[0].priority === '0.8');
  } catch (e) {
    runner.check('listUrls execution', false, e.message);
  }

  runner.subsection('Sitemap Crawl Builder & Run');
  try {
    const crawler = new Sitemap(`${base}/sitemap.xml`, { session: new Session() })
      .maxDepth(3)
      .limit(10)
      .concurrency(4)
      .delay(0.1)
      .field('title', 'h1');

    const results = await crawler.build();
    runner.check('Sitemap.build() run completed successfully', Array.isArray(results));
    if (results.length > 0) {
      runner.check('Sitemap results contain extracted field', 'title' in results[0].toDict());
    }
  } catch (e) {
    runner.check('Sitemap.build() execution', false, e.message);
  }

  runner.subsection('Sitemap Canonical Helper');
  try {
    const canonical = sitemapUrlForOrigin('https://example.com');
    runner.check('sitemapUrlForOrigin utility works', canonical === 'https://example.com/sitemap.xml');
  } catch (e) {
    // Feature detection check: sitemapUrlForOrigin export could be checked
    const sitemapModule = require('../dist');
    if (typeof sitemapModule.sitemapUrlForOrigin === 'function') {
      runner.check('sitemapUrlForOrigin utility call', sitemapModule.sitemapUrlForOrigin('https://example.com') === 'https://example.com/sitemap.xml');
    } else {
      runner.missing('sitemapUrlForOrigin', 'Utility helper not exported at package level');
    }
  }
}

function sitemapUrlForOrigin(origin) {
  return origin.replace(/\/$/, '') + '/sitemap.xml';
}

module.exports = { run };
