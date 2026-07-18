# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`rusty_snake` — a Snake game built with Bevy 0.19 (ECS), edition 2024. Early-stage: no commits yet, no README.

Code layout: `src/main.rs` sets up the App/window/camera and mounts `game::plugins::plugin`. Game logic lives under `src/game/`, split into `grid` (GridPosition component + Transform sync), `snake` (movement, input, segments), and `food` (component + spawn helper, not yet wired into `game/plugins.rs`).

## Known gotchas

- **`src/game/systems/food.rs` is currently broken** — it has a syntax error (incomplete `GridPosition::new(...)` call, dangling `for` loop with no body) that fails `cargo build`. Don't assume the whole project compiles; check this file's state before relying on a clean build.
- The `dynamic_linking` Bevy feature (enabled in `Cargo.toml`) speeds up `cargo run`/`cargo build` for dev but must be dropped for release/distribution builds.

## Conventions

- Commit messages follow Conventional Commits (`feat:`, `fix:`, `refactor:`, etc.).
- No test suite exists yet.
