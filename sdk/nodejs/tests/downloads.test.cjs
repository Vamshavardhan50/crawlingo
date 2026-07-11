const fs = require('fs');
const path = require('path');
const { Downloader, Session } = require('../dist');

const OUTPUT_DIR = path.join(__dirname, 'output');

async function run(runner, base) {
  runner.section('File & Binary Downloads');

  if (!fs.existsSync(OUTPUT_DIR)) {
    fs.mkdirSync(OUTPUT_DIR, { recursive: true });
  }

  runner.subsection('Downloader Instantiation');
  const dl = new Downloader(new Session());
  runner.check('Downloader instance created successfully', dl instanceof Downloader);

  runner.subsection('Download to Memory');
  try {
    const { result, data } = await dl.downloadToMemory(`${base}/download.bin`);
    runner.check('downloadToMemory response code matches', result.status === 200);
    runner.check('downloadToMemory bytes matches content length', result.bytesWritten === 33);
    runner.check('downloadToMemory data buffer contains payload', data.toString() === 'file-download-data-stream-content');
    runner.check('downloadToMemory content type is binary', result.contentType === 'application/octet-stream');
    runner.check('downloadToMemory suggested filename matched header hint', result.suggestedFilename === 'test_output.bin');
  } catch (e) {
    runner.check('downloadToMemory execution', false, e.message);
  }

  runner.subsection('Download to Disk');
  const destPath = path.join(OUTPUT_DIR, 'downloaded_test.bin');
  try {
    if (fs.existsSync(destPath)) {
      fs.unlinkSync(destPath);
    }
    const result = await dl.download(`${base}/download.bin`, destPath);
    runner.check('download to disk response code matches', result.status === 200);
    runner.check('download to disk file generated', fs.existsSync(destPath));
    const diskContent = fs.readFileSync(destPath, 'utf-8');
    runner.check('download to disk file payload matches', diskContent === 'file-download-data-stream-content');
  } catch (e) {
    runner.check('download to disk execution', false, e.message);
  }

  runner.subsection('Download Resumption (Partial Range Content)');
  try {
    const resumeDest = path.join(OUTPUT_DIR, 'resume_test.bin');
    // Pre-create half of the file
    fs.writeFileSync(resumeDest, 'file-download-');
    
    // Now download again with allowResume
    const resumedDl = new Downloader(new Session()).allowResume(true);
    const result = await resumedDl.download(`${base}/download.bin`, resumeDest);
    
    // Since mock server does not support range headers specifically, it might return 200 or 206 depending on FFI logic
    // Let's assert that the file still completes
    runner.check('resumed download completes successfully', result.status === 200 || result.status === 206);
  } catch (e) {
    runner.check('resumed download execution', false, e.message);
  }

  runner.subsection('Limit Download Max Bytes');
  try {
    const limitedDl = new Downloader(new Session()).maxBytes(13);
    const { result, data } = await limitedDl.downloadToMemory(`${base}/download.bin`);
    runner.check('maxBytes limits content length of buffer', data.length === 13);
    runner.check('maxBytes buffer content trimmed correctly', data.toString() === 'file-download');
  } catch (e) {
    runner.check('maxBytes download execution', false, e.message);
  }
}

module.exports = { run };
