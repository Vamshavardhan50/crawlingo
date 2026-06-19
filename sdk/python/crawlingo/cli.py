import argparse
import sys
import code
import json
from .page import Page
from .dataset import Dataset
from .crawl import Crawl
from .watch import Watch
from .session import Session

def run_shell(args):
    """Launches an interactive Python shell with crawlingo imports preloaded."""
    local_vars = {
        "Page": Page,
        "Dataset": Dataset,
        "Crawl": Crawl,
        "Watch": Watch,
        "Session": Session,
    }
    
    welcome = (
        "\n=========================================\n"
        "      Crawlingo Interactive Shell      \n"
        "=========================================\n"
        "Exposed: Page, Dataset, Crawl, Watch, Session\n"
    )
    
    if args.url:
        print(f"[*] Fetching {args.url} ...")
        try:
            page = Page(args.url)
            local_vars["page"] = page
            welcome += f"Loaded: `page` for {args.url}\n"
            welcome += f"Try: page.css('title').text() or page.title()\n"
        except Exception as e:
            print(f"[!] Error loading page: {e}", file=sys.stderr)
            
    welcome += "=========================================\n"
    code.interact(banner=welcome, local=local_vars)

def run_extract(args):
    """Directly extracts data from a URL using specified selectors."""
    try:
        page = Page(args.url, auto_match=args.auto_match)
    except Exception as e:
        print(f"[!] Fetch failed: {e}", file=sys.stderr)
        sys.exit(1)

    results = {}
    if args.css:
        results["css"] = page.css(args.css).texts()
    if args.xpath:
        results["xpath"] = page.xpath(args.xpath).texts()
    if args.text:
        results["text"] = page.find_text(args.text).texts()
    if args.regex:
        results["regex"] = page.regex(args.regex).texts()

    if not results:
        # If no selector given, output page title
        results["title"] = page.title()

    if args.json:
        print(json.dumps(results, indent=2))
    else:
        for selector_type, items in results.items():
            print(f"[{selector_type.upper()} MATCHES]")
            if isinstance(items, list):
                for item in items:
                    print(f"  - {item}")
            else:
                print(f"  {items}")

def run_mcp(args):
    """Starts the Model Context Protocol (MCP) server."""
    try:
        from .mcp import run_mcp_server
        print(f"[*] Starting MCP server on {args.host}:{args.port} ...")
        run_mcp_server(host=args.host, port=args.port)
    except ImportError as e:
        print(f"[!] Failed to import MCP server dependencies: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"[!] MCP server error: {e}", file=sys.stderr)
        sys.exit(1)

def main():
    parser = argparse.ArgumentParser(
        description="Crawlingo CLI — Next-generation scraping & monitoring framework"
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    # Subcommand: shell
    shell_parser = subparsers.add_parser("shell", help="Launch interactive Python REPL")
    shell_parser.add_argument("url", nargs="?", help="Optional URL to fetch and preload into shell context")

    # Subcommand: extract
    extract_parser = subparsers.add_parser("extract", help="Directly extract selector matches from a URL")
    extract_parser.add_argument("url", help="Target URL to fetch and extract from")
    extract_parser.add_argument("--css", help="CSS selector query")
    extract_parser.add_argument("--xpath", help="XPath selector query")
    extract_parser.add_argument("--text", help="Fuzzy or exact text anchor query")
    extract_parser.add_argument("--regex", help="Regex match query")
    extract_parser.add_argument("--auto-match", action="store_true", help="Enable auto-selector self-healing")
    extract_parser.add_argument("--json", action="store_true", help="Format outputs as JSON object")

    # Subcommand: mcp
    mcp_parser = subparsers.add_parser("mcp", help="Run the Model Context Protocol (MCP) server")
    mcp_parser.add_argument("--host", default="127.0.0.1", help="Host address to bind to")
    mcp_parser.add_argument("--port", type=int, default=8000, help="Port to bind the SSE server to")

    args = parser.parse_args()

    if args.command == "shell":
        run_shell(args)
    elif args.command == "extract":
        run_extract(args)
    elif args.command == "mcp":
        run_mcp(args)

if __name__ == "__main__":
    main()
