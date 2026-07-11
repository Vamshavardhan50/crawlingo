const { Page } = require('../dist');

async function run(runner, base) {
  runner.section('Selectors & DOM Extraction');

  let page;
  try {
    page = await Page.create(`${base}/table`);
  } catch (e) {
    runner.check('Page load for selectors testing', false, e.message);
    return;
  }

  runner.subsection('CSS Selectors');
  try {
    const table = page.css('table');
    runner.check('CSS table selector matched', table.length === 1);
    
    const rows = page.css('tr');
    runner.check('CSS table rows matched count', rows.length === 3);

    const firstRowText = rows.first().text;
    runner.check('CSS first row text matched headers', firstRowText.includes('N') && firstRowText.includes('A'));

    const attrs = page.css('table').attr('border');
    runner.check('CSS attr check returns array', Array.isArray(attrs));
  } catch (e) {
    runner.check('CSS selector suite', false, e.message);
  }

  runner.subsection('XPath Selectors');
  try {
    const headings = page.xpath('//th');
    runner.check('XPath th selector matched count', headings.length === 2);
    runner.check('XPath th texts extracted correctly', headings.text[0] === 'N' && headings.text[1] === 'A');
  } catch (e) {
    runner.check('XPath selector suite', false, e.message);
  }

  runner.subsection('Text Boundary Selectors');
  try {
    const okPage = await Page.create(base);
    
    const matched = okPage.findText('OK');
    runner.check('findText matches element', matched.length >= 1 && matched.text[0] === 'OK');

    const afterMatched = okPage.afterText('OK');
    runner.check('afterText matches correctly', afterMatched.length >= 1 && afterMatched.text[0].includes('test paragraph'));

    const beforeMatched = okPage.beforeText('test paragraph');
    runner.check('beforeText matches correctly', beforeMatched.length >= 1 && beforeMatched.text[0].includes('OK'));

    const regexMatched = okPage.regex('p[a-z]+a[a-z]+h');
    runner.check('regex matches correctly', regexMatched.length >= 1 && regexMatched.text[0] === 'test paragraph');
  } catch (e) {
    runner.check('Text boundaries selector suite', false, e.message);
  }

  runner.subsection('Element Indexing');
  try {
    const rows = page.css('tr');
    const secondRow = rows.at(1);
    runner.check('ElementCollection.at(1) returns Element instance', secondRow !== null);
    runner.check('Element text extracted successfully', secondRow.text.includes('Alice'));
    runner.check('Element html extracted successfully', secondRow.html.includes('Alice'));
  } catch (e) {
    runner.check('Element indexing suite', false, e.message);
  }
}

module.exports = { run };
