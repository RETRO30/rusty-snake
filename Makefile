# Task runner for rusty_snake.
#
# The web build and dev server are plain Python scripts (build_web.py / serve.py)
# so they run anywhere without extra tooling. `make` itself is optional — every
# target is a one-liner you can run by hand.
#
# Override the interpreter if needed, e.g. `make web PYTHON=python3`.
PYTHON ?= uv run python

.PHONY: run web serve preview check fmt clean

## run: launch the native desktop game
run:
	cargo run

## web: build the WebAssembly bundle into dist/
web:
	$(PYTHON) build_web.py

## serve: serve the built dist/ at http://127.0.0.1:8080
serve:
	$(PYTHON) serve.py

## preview: build for web, then serve it locally
preview: web serve

## check: fast type-check
check:
	cargo check

## fmt: format all sources
fmt:
	cargo fmt

## clean: remove cargo build artifacts (dist/ is recreated by `make web`)
clean:
	cargo clean
