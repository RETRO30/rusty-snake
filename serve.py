#!/usr/bin/env python3
"""Serve the built dist/ locally for previewing the web build.

Cross-platform (Windows / macOS / Linux). Run with `python serve.py`
or `make serve`.

Usage: python serve.py [--port 8080] [--dir dist]
"""

import argparse
import mimetypes
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

ROOT = Path(__file__).resolve().parent

# Python's default table may not map these; wasm in particular must be served
# as application/wasm or the browser refuses to stream-compile it.
mimetypes.add_type("application/wasm", ".wasm")
mimetypes.add_type("application/javascript", ".js")


class Handler(SimpleHTTPRequestHandler):
    def end_headers(self) -> None:
        # Avoid stale wasm/js while iterating on the build locally.
        self.send_header("Cache-Control", "no-store")
        super().end_headers()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--port", type=int, default=8080)
    parser.add_argument("--dir", default="dist")
    args = parser.parse_args()

    directory = str((ROOT / args.dir).resolve())
    handler = partial(Handler, directory=directory)
    server = ThreadingHTTPServer(("0.0.0.0", args.port), handler)

    print(f"Serving {directory} at http://0.0.0.0:{args.port}")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nStopping server")
        server.shutdown()


if __name__ == "__main__":
    main()
