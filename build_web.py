#!/usr/bin/env python3
"""Build the rusty_snake WebAssembly bundle into dist/.

Cross-platform (Windows / macOS / Linux). Run with `python build_web.py`
or `make web`.

Prerequisites:
    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli --version <matching bevy's wasm-bindgen>
"""

import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
OUT_DIR = ROOT / "dist"
WASM = ROOT / "target" / "wasm32-unknown-unknown" / "release" / "rusty_snake.wasm"


def run(*cmd: str) -> None:
    """Run an external command from the crate root; abort on failure."""
    print("$", " ".join(cmd))
    try:
        subprocess.run(cmd, cwd=ROOT, check=True)
    except FileNotFoundError:
        sys.exit(f"error: `{cmd[0]}` not found on PATH")
    except subprocess.CalledProcessError as err:
        sys.exit(f"error: `{cmd[0]}` exited with {err.returncode}")


def main() -> None:
    # 1. Compile the game to wasm (release, without the dev/dynamic_linking
    #    feature — wasm doesn't support dynamic linking).
    run(
        "cargo", "build", "--release",
        "--no-default-features",
        "--target", "wasm32-unknown-unknown",
        "--bin", "rusty_snake",
    )

    # 2. Reset the output directory.
    if OUT_DIR.exists():
        shutil.rmtree(OUT_DIR)
    OUT_DIR.mkdir(parents=True)

    # 3. Generate the JS glue + packaged wasm.
    run(
        "wasm-bindgen",
        "--no-typescript",
        "--target", "web",
        "--out-dir", str(OUT_DIR),
        "--out-name", "rusty_snake",
        str(WASM),
    )

    # 4. Copy the HTML shell and runtime assets (fonts, etc.) so asset loads
    #    resolve at runtime.
    shutil.copy(ROOT / "web" / "index.html", OUT_DIR / "index.html")
    assets = ROOT / "assets"
    if assets.exists():
        shutil.copytree(assets, OUT_DIR / "assets")

    print(f"\nBuild complete: {OUT_DIR}")
    print("Preview locally with: python serve.py")


if __name__ == "__main__":
    main()
