import json
import urllib.parse
import threading
import queue
from http.server import BaseHTTPRequestHandler, HTTPServer
from socketserver import ThreadingMixIn
from .page import Page
from .dataset import Dataset
from .crawl import Crawl

# Thread-safe queue of client SSE connections
sse_clients = []
sse_clients_lock = threading.Lock()

class ThreadingHTTPServer(ThreadingMixIn, HTTPServer):
    """Multiple threads handling HTTP requests asynchronously."""
    daemon_threads = True

class MCPRequestHandler(BaseHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        super().end_headers()

    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()

    def do_GET(self):
        parsed_path = urllib.parse.urlparse(self.path)
        if parsed_path.path == "/sse":
            self.handle_sse_connect()
        else:
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b"Not Found")

    def do_POST(self):
        parsed_path = urllib.parse.urlparse(self.path)
        if parsed_path.path == "/message":
            self.handle_mcp_message()
        else:
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b"Not Found")

    def handle_sse_connect(self):
        """Register client connection and stream Server-Sent Events."""
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Cache-Control", "no-cache")
        self.send_header("Connection", "keep-alive")
        self.end_headers()

        # Write the SSE endpoint initialization event
        endpoint_event = "event: endpoint\ndata: /message\n\n"
        try:
            self.wfile.write(endpoint_event.encode("utf-8"))
            self.wfile.flush()
        except Exception:
            return

        client_queue = queue.Queue()
        with sse_clients_lock:
            sse_clients.append(client_queue)

        # Loop until client disconnects, sending events from the queue
        try:
            while True:
                try:
                    event_data = client_queue.get(timeout=10)
                    self.wfile.write(event_data.encode("utf-8"))
                    self.wfile.flush()
                except queue.Empty:
                    # Keep-alive ping
                    self.wfile.write(b": ping\n\n")
                    self.wfile.flush()
        except Exception:
            # Client disconnected
            pass
        finally:
            with sse_clients_lock:
                if client_queue in sse_clients:
                    sse_clients.remove(client_queue)

    def handle_mcp_message(self):
        """Reads JSON-RPC request and executes the requested tool."""
        content_length = int(self.headers.get("Content-Length", 0))
        post_data = self.rfile.read(content_length)
        
        try:
            request = json.loads(post_data.decode("utf-8"))
        except Exception as e:
            self.send_response(400)
            self.end_headers()
            self.wfile.write(f"Invalid JSON: {e}".encode("utf-8"))
            return

        # Respond HTTP 202 Accepted to the POST request immediately
        self.send_response(202)
        self.end_headers()
        self.wfile.write(b"Accepted")

        # Process the JSON-RPC request and push the response to SSE
        response = self.process_rpc(request)
        if response:
            sse_message = f"event: message\ndata: {json.dumps(response)}\n\n"
            with sse_clients_lock:
                for client in sse_clients:
                    client.put(sse_message)

    def process_rpc(self, req):
        """Processes JSON-RPC request and returns JSON-RPC response."""
        req_id = req.get("id")
        method = req.get("method")
        params = req.get("params", {})

        if not method:
            return None

        # Standard initialize handshake
        if method == "initialize":
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "crawlingo-mcp",
                        "version": "0.1.0"
                    }
                }
            }

        # List tools
        elif method == "tools/list":
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {
                    "tools": [
                        {
                            "name": "fetch_page",
                            "description": "Fetch a web page and get its text, title, and load status.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "url": {"type": "string", "description": "The URL of the page to fetch"},
                                    "auto_match": {"type": "boolean", "description": "Enable auto-matching self-healing selector recovery"},
                                    "timeout": {"type": "integer", "description": "Fetch timeout in seconds"}
                                },
                                "required": ["url"]
                            }
                        },
                        {
                            "name": "extract_data",
                            "description": "Extract structured fields from a URL using CSS/XPath selectors.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "url": {"type": "string", "description": "URL to extract from"},
                                    "fields": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "name": {"type": "string", "description": "Field key name"},
                                                "selector": {"type": "string", "description": "CSS or XPath selector string"},
                                                "selector_type": {"type": "string", "enum": ["css", "xpath"], "default": "css"}
                                            },
                                            "required": ["name", "selector"]
                                        }
                                    },
                                    "auto_match": {"type": "boolean", "description": "Enable auto-matching selector recovery"}
                                },
                                "required": ["url", "fields"]
                            }
                        },
                        {
                            "name": "crawl_site",
                            "description": "Crawl pages from a start URL following discovered links and extract custom fields.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "start_url": {"type": "string", "description": "The seed URL to begin crawl"},
                                    "follow_selector": {"type": "string", "description": "CSS selector for follow link elements"},
                                    "fields": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "name": {"type": "string"},
                                                "selector": {"type": "string"},
                                                "selector_type": {"type": "string", "enum": ["css", "xpath"], "default": "css"}
                                            },
                                            "required": ["name", "selector"]
                                        }
                                    },
                                    "limit": {"type": "integer", "description": "Max pages limit"},
                                    "depth": {"type": "integer", "description": "Max hops depth limit"}
                                },
                                "required": ["start_url", "follow_selector", "fields"]
                            }
                        }
                    ]
                }
            }

        # Call tools
        elif method == "tools/call":
            tool_name = params.get("name")
            arguments = params.get("arguments", {})
            return self.execute_tool(req_id, tool_name, arguments)

        # Unsupported method
        return {
            "jsonrpc": "2.0",
            "id": req_id,
            "error": {
                "code": -32601,
                "message": f"Method not found: {method}"
            }
        }

    def execute_tool(self, req_id, name, args):
        try:
            if name == "fetch_page":
                url = args.get("url")
                auto_match = args.get("auto_match", False)
                timeout = args.get("timeout", 30)
                page = Page(url, auto_match=auto_match, timeout=timeout)
                result_text = f"URL: {page.url}\nStatus: {page.status}\nTitle: {page.title()}\n\nText Content:\n{page.css('body').text()}"
                return {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {
                        "content": [{"type": "text", "text": result_text}]
                    }
                }

            elif name == "extract_data":
                url = args.get("url")
                fields = args.get("fields", [])
                auto_match = args.get("auto_match", False)

                dataset = Dataset(url)
                dataset.auto_match(auto_match)
                for f in fields:
                    dataset.field(
                        f["name"],
                        f["selector"],
                        selector_type=f.get("selector_type", "css")
                    )
                res = dataset.build()
                return {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {
                        "content": [{"type": "text", "text": json.dumps(res.to_dict(), indent=2)}]
                    }
                }

            elif name == "crawl_site":
                start_url = args.get("start_url")
                follow_selector = args.get("follow_selector")
                fields = args.get("fields", [])
                limit = args.get("limit", 10)
                depth = args.get("depth", 2)

                crawl = Crawl(start_url)
                crawl.follow(follow_selector)
                crawl.limit(limit)
                crawl.depth(depth)
                for f in fields:
                    crawl.field(
                        f["name"],
                        f["selector"],
                        selector_type=f.get("selector_type", "css")
                    )
                res = crawl.build()
                data = [item.to_dict() for item in res]
                return {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {
                        "content": [{"type": "text", "text": json.dumps(data, indent=2)}]
                    }
                }

            else:
                return {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "error": {
                        "code": -32602,
                        "message": f"Unknown tool: {name}"
                    }
                }

        except Exception as e:
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {
                    "isError": True,
                    "content": [{"type": "text", "text": f"Error running tool: {e}"}]
                }
            }

def run_mcp_server(host="127.0.0.1", port=8000):
    server = ThreadingHTTPServer((host, port), MCPRequestHandler)
    print(f"[*] MCP SSE Server running on http://{host}:{port}/sse")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n[*] Shutting down MCP server...")
        server.server_close()
