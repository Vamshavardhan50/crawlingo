import json
import tempfile
import threading
import unittest
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from crawlingo import DatasetJob


class DatasetJobTest(unittest.TestCase):
    def test_native_pipeline_and_resume(self):
        calls = []

        class Handler(BaseHTTPRequestHandler):
            def do_GET(self):
                calls.append(self.path)
                body = '<article><h1>café 🚀</h1></article><article></article>'.encode()
                self.send_response(200)
                self.send_header('Content-Type', 'text/html; charset=utf-8')
                self.send_header('Content-Length', str(len(body)))
                self.end_headers()
                self.wfile.write(body)

            def log_message(self, *args):
                pass

        server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        worker = threading.Thread(target=server.serve_forever, daemon=True)
        worker.start()
        try:
            with tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                source = root / 'urls.txt'
                source.write_text(f'http://127.0.0.1:{server.server_port}/one\n', encoding='utf-8')
                config = dict(urls_file=str(source), output_dir=str(root / 'output'), row_selector='article', respect_robots=False, fields=[dict(name='title', selector='h1', required=True)])
                report = DatasetJob(config).run()
                self.assertEqual(report['records_written'], 1)
                self.assertEqual(report['records_rejected'], 1)
                self.assertTrue(report['complete'])
                row = json.loads((root / 'output' / 'part-00000000.jsonl').read_text(encoding='utf-8'))
                self.assertEqual(row['title'], 'café 🚀')
                self.assertEqual(DatasetJob(config).run()['records_written'], 1)
                self.assertEqual(len(calls), 1)
        finally:
            server.shutdown()
            server.server_close()
            worker.join()


if __name__ == '__main__':
    unittest.main()
