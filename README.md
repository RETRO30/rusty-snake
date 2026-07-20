# Rusty Snake
A classic Snake game written in Rust with the [Bevy](https://bevyengine.org/) game engine
(0.19, ECS architecture).

# Author: [RETRO30](https://github.com/RETRO30)

## Features

- Playfield with a border frame and a score bar at the top-left that never overlaps the board.
- Menu flow: **Start** → play → pause (**Esc** or the on-screen pause button) →
  **Resume**/**Restart**.
- Game over when the snake hits the border or its own tail.
- On-screen control hints.
- Swipe controls on mobile devices.

## Controls

| Action           | Keys / gesture              |
| ---------------- | --------------------------- |
| Move             | Arrow keys or `W` `A` `S` `D` |
| Pause / resume   | `Esc` or the pause button   |
| Move (mobile)    | Swipe in the desired direction |

## Running locally

Requires [Rust](https://www.rust-lang.org/tools/install) (edition 2024) to be installed.

```sh
cargo run
```

## Building for the web (WebAssembly)

### Prerequisites

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

The `wasm-bindgen-cli` version must match the `wasm-bindgen` version Bevy uses — if the build
reports a version mismatch, install the version named in the error with
`cargo install wasm-bindgen-cli --version <version>`.

### Build

Requires [Python 3](https://www.python.org/) (the engine itself is built with `cargo`; Python is
only used for the build and local-server scripts, which are cross-platform and work on Windows
too).

```sh
python build_web.py
```

The script builds the release wasm binary without the `dev` feature, generates the JS bindings
with `wasm-bindgen`, and packages everything together with `web/index.html` and the `assets/`
folder into `dist/`.

### Preview locally

The project ships a small Python HTTP server:

```sh
python serve.py
```

It serves `dist/` at `http://127.0.0.1:8080`. The port and directory can be changed with flags:

```sh
python serve.py --port 3000 --dir dist
```

`Makefile` targets are also available: `make web` (build), `make serve` (server), `make preview`
(build and serve).

## Publishing to GitHub Pages

Publishing is automated with GitHub Actions
([`.github/workflows/deploy.yml`](.github/workflows/deploy.yml)): pushing a version tag
(`vX.Y.Z`) whose commit is on `main` builds the web version and deploys it to GitHub Pages.

### One-time setup

In the repository settings on GitHub: **Settings → Pages → Build and deployment → Source**, choose
**GitHub Actions**.

### Releasing a version

Make sure the desired commit is merged into `main`, then create a version tag on it:

```sh
git tag v0.0.1
git push origin v0.0.1
```

The workflow runs automatically, builds the web version, and publishes it. Progress is visible in
the **Actions** tab. Once it finishes, the game is available at a URL like
`https://<user>.github.io/<repository>/`.

> A tag pointing at a commit that isn't on `main` won't trigger a deploy — the workflow checks that
> the tag's commit is reachable from `main` and fails otherwise.

### Manual publishing (fallback)

Build locally (`python build_web.py`) and publish the contents of `dist/` to a `gh-pages` branch,
e.g. `git subtree push --prefix dist origin gh-pages`, selecting the `gh-pages` branch as the
source under **Settings → Pages**.

## Project structure

```
src/
  main.rs            — window, camera, entry point
  config.rs          — playfield, window, and cell sizes
  game/
    state.rs         — game states (menu/playing/paused/game over)
    grid/            — board, frame, cell coordinates
    snake/           — movement, input, swipes, collisions
    food/            — food
    ui/              — menu, pause, game over, score, hints
assets/
  fonts/             — UI font (arrows/Cyrillic)
build_web.py         — builds the web version into dist/
serve.py             — local HTTP server for previewing the web build
.github/workflows/
  deploy.yml         — auto-deploy to GitHub Pages on a version tag
```

## License

Released under the [MIT License](LICENSE).
