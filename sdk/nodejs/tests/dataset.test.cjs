const fs = require('fs');
const path = require('path');
const { Dataset, Page, Session } = require('../dist');

const OUTPUT_DIR = path.join(__dirname, 'output');

async function run(runner, base) {
  runner.section('Dataset & Structured Extraction');

  if (!fs.existsSync(OUTPUT_DIR)) {
    fs.mkdirSync(OUTPUT_DIR, { recursive: true });
  }

  let page;
  try {
    page = await Page.create(`${base}/table`);
  } catch (e) {
    runner.check('Page load for dataset testing', false, e.message);
    return;
  }

  runner.subsection('Structured Extraction (Rust-FFI)');
  try {
    const ds = new Dataset(`${base}/table`);
    ds.field('name', 'td.name');
    ds.field('age', 'td.age');

    const records = ds.extractStructured(page);
    runner.check('extractStructured returns expected array format', Array.isArray(records));
    runner.check('extractStructured correct rows length', records.length === 2);
    runner.check('extractStructured field matching', records[0].name === 'Alice' && records[0].age === '30');
    runner.check('extractStructured second row matching', records[1].name === 'Bob' && records[1].age === '25');
  } catch (e) {
    runner.check('Structured extraction', false, e.message);
  }

  runner.subsection('Static Exporters (saveJson / saveCsv)');
  const testData = [{ name: 'Alice', age: '30' }, { name: 'Bob', age: '25' }];
  const jsonPath = path.join(OUTPUT_DIR, 'test_output.json');
  const csvPath = path.join(OUTPUT_DIR, 'test_output.csv');

  try {
    Dataset.saveJson(testData, jsonPath);
    runner.check('Dataset.saveJson() file generated', fs.existsSync(jsonPath));
    const rawJson = fs.readFileSync(jsonPath, 'utf-8');
    runner.check('Dataset.saveJson() output matches format', rawJson.includes('"name": "Alice"'));
  } catch (e) {
    runner.check('Static saveJson exporter', false, e.message);
  }

  try {
    Dataset.saveCsv(testData, csvPath);
    runner.check('Dataset.saveCsv() file generated', fs.existsSync(csvPath));
    const rawCsv = fs.readFileSync(csvPath, 'utf-8');
    runner.check('Dataset.saveCsv() output matches format', rawCsv.includes('name') && rawCsv.includes('age') && rawCsv.includes('Alice') && rawCsv.includes('30'));
  } catch (e) {
    runner.check('Static saveCsv exporter', false, e.message);
  }

  runner.subsection('Dataset Build & File Export');
  try {
    const ds = new Dataset(`${base}/`, new Session());
    ds.field('title', 'h1');
    ds.field('desc', 'p');

    const result = await ds.build();
    runner.check('Dataset.build() returns DatasetResult instance', result !== null);
    
    const dict = result.toDict();
    runner.check('toDict() maps single row correctly', dict.title === 'OK' && dict.desc === 'test paragraph');

    const resultJson = path.join(OUTPUT_DIR, 'result_out.json');
    const resultCsv = path.join(OUTPUT_DIR, 'result_out.csv');
    const resultParquet = path.join(OUTPUT_DIR, 'result_out.parquet');

    await result.toJson(resultJson);
    runner.check('DatasetResult.toJson() outputs file', fs.existsSync(resultJson));

    await result.toCsv(resultCsv);
    runner.check('DatasetResult.toCsv() outputs file', fs.existsSync(resultCsv));

    await result.toParquet(resultParquet);
    runner.check('DatasetResult.toParquet() outputs file', fs.existsSync(resultParquet));
  } catch (e) {
    runner.check('Dataset build and export pipeline', false, e.message);
  }

  runner.subsection('Feature Detection');
  runner.testFeature('Dataset.prototype.toArrow()', Dataset.prototype, 'toArrow', () => {});
  runner.testFeature('Dataset.prototype.toDataFrame()', Dataset.prototype, 'toDataFrame', () => {});
  runner.testFeature('Dataset.prototype.df()', Dataset.prototype, 'df', () => {});
  runner.testFeature('Dataset.prototype.stream()', Dataset.prototype, 'stream', () => {});
  runner.testFeature('Dataset.prototype.update()', Dataset.prototype, 'update', () => {});
  runner.testFeature('Dataset.prototype.delete()', Dataset.prototype, 'delete', () => {});
}

module.exports = { run };
