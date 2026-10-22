"""Deterministic research directory for container acceptance; no public network needed."""
import json
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/health":
            body = b"ok"
        elif self.path == "/robots.txt":
            body = b"User-agent: *\nAllow: /\n"
        elif self.path.startswith("/slow/"):
            time.sleep(1)
            body = f'<article><h2 data-id="{self.path}">Slow document</h2><a href="/paper">Paper</a></article>'.encode()
        elif self.path in ("/directory/1", "/directory/2"):
            ids = [1, 2] if self.path.endswith("1") else [2, 3]
            body = "".join(f'<article><h2 data-id="{i}">Research paper {i}</h2><a href="/papers/{i}">Read</a></article>' for i in ids).encode()
        else:
            self.send_error(404)
            return
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        try:
            self.wfile.write(body)
        except (BrokenPipeError, ConnectionResetError):
            pass

    def log_message(self, format, *args):
        print(json.dumps({"path": self.path, "message": format % args}), flush=True)

if __name__ == "__main__":
    ThreadingHTTPServer(("0.0.0.0", 8080), Handler).serve_forever()
