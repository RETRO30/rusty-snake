# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`rusty_snake` — a Snake game built with Bevy 0.19 (ECS), edition 2024. Ships both as a native
desktop build and a WebAssembly build deployable to GitHub Pages. Russian-language `README.md`
at the repo root is the user-facing doc; this file is for Claude Code only.

Code layout: `src/main.rs` sets up the App/window/camera and mounts `game::plugins::plugin`. The
window is `resizable: true` with `fit_canvas_to_parent: true` on web; the canvas tracks the size
of its parent `#game-wrap` (see `web/index.html`), which caps the max size, centers it, and locks
the canvas to the game's aspect ratio (`WINDOW_WIDTH:WINDOW_HEIGHT`) so there's no internal
letterbox and window-percentage UI lines up with the world. The `Camera2d`'s
`OrthographicProjection` uses `ScalingMode::AutoMin { min_width: WINDOW_WIDTH, min_height:
WINDOW_HEIGHT }` (full playfield + top bar always visible without cropping on any aspect ratio),
and the camera is shifted up by `TOP_BAR_HEIGHT / 2` so the reserved bar strip sits above the
frame. Game logic lives under `src/game/`:

- `state` — the `GameState` state machine (`Menu` → `Playing` → `Paused` → `GameOver`) and the
  shared `start_new_game` reset helper used by the menu's Start/Restart buttons. Esc toggles
  Playing/Paused from here regardless of what else is gated.
- `grid` — `GridPosition` component + Transform sync, checkerboard board, and the border frame
  drawn around the playfield.
- `snake` — movement, keyboard input, mobile swipe input (`systems/touch.rs`, via Bevy's
  `Touches`), border/self-collision detection (`systems/collision.rs`), and segments. Gameplay
  systems (`handle_input`, `handle_swipe`, `move_snake`, `check_collisions`,
  `sync_scale_by_role`) are chained and gated with `run_if(in_state(GameState::Playing))`.
  Spawning is not a `Startup` system — a fresh snake is only created by
  `state::start_new_game`, so pausing/resuming never respawns it. The `Snake` entity (which owns
  the `segments: VecDeque<Entity>`) is a *separate* entity from the individual `SnakeSegment`
  entities it references, so `start_new_game` must despawn both the `Snake` and its
  `SnakeSegment`s on reset — otherwise a stale `Snake` lingers and `move_snake` spams
  "Head doesn't have position" every tick.
  - Input is double-buffered: `handle_input`/`handle_swipe` only ever write `Snake::next_direction`
    and validate it against `Snake::direction` (the direction actually committed to the last
    tick) — never against each other's writes. `move_snake` commits `next_direction ->
    direction` exactly once, at the start of the tick it fires on. This is what stops the classic
    Snake bug where two direction changes queued between two movement ticks (e.g. a fast
    Right-then-Down swipe) compound into a same-tick 180° reversal into the snake's own neck.
  - `handle_swipe` reads `Touches` directly (not `iter_just_released`) with a per-touch-id
    rolling origin (`Local<HashMap<u64, Vec2>>`, reset every time a swipe threshold fires): turns
    land the instant a drag crosses `MIN_SWIPE_DISTANCE`, and one continuous drag can chain
    several turns (e.g. an L-shaped swipe) instead of needing a separate touch per turn.
- `food` — component + spawn helper; eat/spawn systems are likewise gated to `Playing`.
- `ui` — score (top-left bar, sized in canvas percentages derived from the `config` constants so
  it hugs the playfield frame's width and never exceeds it — see `ui/systems/spawn.rs`), an
  in-bar pause button (`MenuButtonAction::Pause`, pinned to the bar's right edge for touch users
  with no Esc key; `handle_menu_buttons` only pauses it while `Playing`), the main menu, pause
  overlay, game-over overlay, key-cap style control hints, and the generic button interaction
  system driving state transitions. Note the score bar is spawned at `Startup` and always present,
  so it sits (dimmed) behind the menu/pause/game-over overlays, which are drawn on top via
  `GlobalZIndex(10)`. Outside the canvas, `web/index.html` also renders a static HTML control
  legend (arrow/Esc key-caps + a "swipe on touch screens" caption) below the game. The pause screen's Resume/Restart button
  row wraps (`FlexWrap::Wrap` + `max_width: Val::Vw(90.0)`) instead of overflowing on narrow
  phone widths, where two 180px buttons + gap don't fit side by side.

## Commands

`rusty_snake` is the only cargo binary, so plain `cargo run` works. The web build and dev server
are Python scripts (`build_web.py` / `serve.py`) — chosen over Rust bins so they run cross-
platform (Windows included) without extra deps. A `Makefile` wraps the common tasks (`make run` /
`web` / `serve` / `preview` / `check` / `fmt`); it runs Python via `uv run python` by default
(override with `make web PYTHON="python3"` if `uv` isn't available), but `make` itself is
optional.

- Run the game: `cargo run` (or `make run`).
- Build for web → `dist/`: `python build_web.py` (or `make web`).
- Preview the web build: `python serve.py` (or `make preview` to build+serve); serves `dist/` at
  `http://127.0.0.1:8080`, `--port` / `--dir` to override.
- Fast type-check: `cargo check`. Format: `rustfmt --edition 2024` (a PostToolUse hook already
  runs it on every edited `.rs` file). No test suite exists yet.

## Known gotchas

- The `dynamic_linking` Bevy feature is behind the `dev` Cargo feature (on by default) for fast
  local `cargo run`/`cargo build`. It must NOT be enabled for release or wasm builds — wasm
  doesn't support dynamic linking at all. Wasm builds pass `--no-default-features`.
- Bevy 0.19's text stack is Parley-based: `TextFont.font_size` is a `FontSize` enum
  (`FontSize::Px(24.0)`, not a bare `f32`), `TextFont.font` is a `FontSource` enum (wrap a
  handle as `FontSource::Handle(handle.clone())`, not a bare `Handle<Font>`), and `BorderColor`
  has four independent `top`/`right`/`bottom`/`left` fields (use `BorderColor::all(color)` for a
  uniform border).
- Bevy's embedded default font (`FiraMono-subset`) only covers basic Latin — arrow glyphs
  (↑↓←→) and Cyrillic render blank with it. `DejaVuSans.ttf` is bundled at `assets/fonts/` and
  loaded once into the `ui::resources::UiFont` resource (`FromWorld` via `AssetServer`, wired in
  `ui::plugins`); any `Text` that needs those glyphs must set `TextFont.font` to that handle. The
  control hints (`ui/systems/hints.rs`) already do. Russian UI text is still kept out of spawned
  `Text` (in `README.md` only) by convention, but DejaVu Sans does cover Cyrillic if needed.

## Web deploy

- `build_web.py` (run via `python build_web.py` / `make web`) builds the wasm release binary
  (`--no-default-features --target wasm32-unknown-unknown --bin rusty_snake`), runs
  `wasm-bindgen`, and packages the result with `web/index.html` and the `assets/` dir (copied to
  `dist/assets/`, so runtime asset loads like the font resolve) into `dist/`. Requires `rustup
  target add wasm32-unknown-unknown` and a matching `wasm-bindgen-cli` install.
- Preview the built `dist/` locally with `serve.py` (`python serve.py`, defaults to
  `http://127.0.0.1:8080`). It sets the `application/wasm` MIME type and `Cache-Control: no-store`
  so the browser stream-compiles the wasm and doesn't serve stale builds during iteration.
- GitHub Pages publishing is automated by `.github/workflows/deploy.yml`: pushing a version tag
  (`v*`) whose commit is on `main` builds the web bundle (via `build_web.py`) and deploys it with
  the official `upload-pages-artifact`/`deploy-pages` actions. `wasm-bindgen-cli` is pinned there
  to the `Cargo.lock` version (currently `0.2.126`) — bump it when the lockfile changes. The
  manual `gh-pages` branch fallback is documented in `README.md`.

## Conventions

- Commit messages follow Conventional Commits (`feat:`, `fix:`, `refactor:`, etc.).
- No test suite exists yet.
