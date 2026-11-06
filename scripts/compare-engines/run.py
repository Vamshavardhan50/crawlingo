"""Cold process comparison on one loopback fixture. Use a Python env with Scrapy installed."""
import argparse
import hashlib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import platform
import subprocess
import sys
import threading
import time


class Fixture(BaseHTTPRequestHandler):
    protocol_version = 'HTTP/1.1'
    wbufsize = -1

    def do_GET(self):
        if not self.path[1:].isdigit():
            self.send_error(404)
            return
        body = f'<html><article><h1>Document {int(self.path[1:])}</h1></article></html>'.encode()
        self.send_response(200)
        self.send_header('Content-Type', 'text/html; charset=utf-8')
        self.send_header('Content-Length', str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *args):
        pass


class FixtureServer(ThreadingHTTPServer):
    request_queue_size = 128


def scrapy_worker(directory, concurrency):
    import scrapy
    from scrapy.crawler import CrawlerProcess

    class Documents(scrapy.Spider):
        name = 'documents'
        start_urls = (directory / 'urls.txt').read_text().splitlines()

        def parse(self, response):
            yield {'title': response.css('article h1::text').get(), '_source_url': response.url}

    process = CrawlerProcess(settings={
        'LOG_LEVEL': 'ERROR', 'CONCURRENT_REQUESTS': concurrency,
        'CONCURRENT_REQUESTS_PER_DOMAIN': concurrency, 'DOWNLOAD_DELAY': 0,
        'ROBOTSTXT_OBEY': False, 'HTTPPROXY_ENABLED': False, 'RETRY_ENABLED': False,
        'TELNETCONSOLE_ENABLED': False, 'FEEDS': {(directory / 'scrapy.jsonl').as_uri(): {'format': 'jsonlines'}},
    })
    process.crawl(Documents)
    process.start()


def read_rows(files):
    for path in files:
        with path.open(encoding='utf-8') as stream:
            for line in stream:
                row = json.loads(line)
                yield row['_source_url'], row['title']


def run(directory, engine, urls, concurrency, cli):
    directory.mkdir(parents=True)
    (directory / 'urls.txt').write_text('\n'.join(urls) + '\n')
    if engine == 'crawlingo':
        job = {'urls_file': 'urls.txt', 'output_dir': 'dataset', 'row_selector': 'article',
               'fields': [{'name': 'title', 'selector': 'h1', 'required': True}],
               'concurrency': concurrency, 'respect_robots': False}
        (directory / 'job.json').write_text(json.dumps(job))
        command = [str(cli), 'dataset', 'job.json']
    elif engine == 'scrapy':
        command = [sys.executable, __file__, '--scrapy-worker', str(directory), str(concurrency)]
    else:
        command = ['node', str(Path(__file__).with_name('playwright.cjs')), str(concurrency)]
    started = time.perf_counter()
    subprocess.run(command, check=True, cwd=directory, stdout=subprocess.DEVNULL, timeout=300)
    seconds = time.perf_counter() - started
    files = sorted((directory / 'dataset').glob('part-' + '[0-9]' * 8 + '.jsonl')) if engine == 'crawlingo' else [directory / f'{engine}.jsonl']
    expected = [(url, f'Document {index}') for index, url in enumerate(urls)]
    if sorted(read_rows(files)) != sorted(expected):
        raise ValueError(f'{engine}: output mismatch')
    return {'engine': engine, 'seconds': seconds, 'verified_rows': len(urls), 'pages_per_second': len(urls) / seconds}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory', type=Path)
    parser.add_argument('--cli', type=Path, required=True)
    parser.add_argument('--pages', type=int, default=200)
    parser.add_argument('--concurrency', type=int, default=4)
    args = parser.parse_args()
    root = args.directory.resolve()
    if root.exists() or args.pages < 1 or not 1 <= args.concurrency <= 50:
        parser.error('Choose a fresh directory, positive page count and concurrency 1..50.')
    import scrapy
    cli = args.cli.resolve(strict=True)
    evidence = {'platform': platform.platform(), 'python': sys.version, 'scrapy': scrapy.__version__,
                'cli_sha256': hashlib.sha256(cli.read_bytes()).hexdigest(), 'pages': args.pages,
                'concurrency': args.concurrency, 'scope': 'Process startup through JSONL export. Crawlingo includes durable checkpoints and deduplication. Playwright includes Chromium startup and rendering.', 'runs': []}
    engines = ['crawlingo', 'scrapy', 'playwright']
    with FixtureServer(('127.0.0.1', 0), Fixture) as server:
        threading.Thread(target=server.serve_forever, daemon=True).start()
        urls = [f'http://127.0.0.1:{server.server_port}/{i}' for i in range(args.pages)]
        try:
            for repeat in range(3):
                for engine in engines[repeat:] + engines[:repeat]:
                    sample = root / f'{repeat + 1}-{engine}'
                    result = run(sample, engine, urls, args.concurrency, cli)
                    if engine == 'playwright':
                        evidence.update(json.loads((sample / 'versions.json').read_text()))
                    evidence['runs'].append(result)
                    (root / 'evidence.json').write_text(json.dumps(evidence, indent=2))
                    print(result, flush=True)
        finally:
            server.shutdown()


if __name__ == '__main__':
    if sys.argv[1:2] == ['--scrapy-worker']:
        scrapy_worker(Path(sys.argv[2]), int(sys.argv[3]))
    else:
        main()
