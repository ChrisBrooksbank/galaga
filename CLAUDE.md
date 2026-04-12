# CLAUDE.md

## Project Overview

A faithful recreation of the classic 1981 Namco Galaga arcade game, built in Rust with the Bevy game engine. Features full formation AI, boss tractor beam captures, dual fighter mechanic, challenging bonus stages, and authentic arcade scoring.

## Tech Stack

- **Rust** (2024 Edition)
- **Bevy** 0.18 — ECS game engine

## Development Commands

```bash
cargo run          # Build and run the game
cargo build        # Build without running
cargo test         # Run tests
cargo clippy       # Lint
```

## Architecture

Bevy ECS pattern — entities, components, systems, resources.

```
src/
  main.rs         # App setup, state machine, system registration
  states.rs       # Game states (Menu, Playing, Paused, etc.)
  components.rs   # ECS components
  constants.rs    # Game tuning constants
  resources.rs    # Shared game resources
  player/         # Ship movement, shooting, death, dual fighter
  enemies/        # Formation, AI, dive paths, tractor beam
  collision/      # Bullet/enemy/player collision systems
  waves/          # Wave controller, difficulty scaling
  scoring/        # Score events and extra life logic
  effects/        # Starfield, explosions, animations
  ui/             # Menu, HUD, pause, game over screens
  audio/          # Music and SFX event handling
assets/
  sprites/        # Pixel art sprite sheets
```

## Game Tuning

All gameplay constants (speeds, timings, scores, probabilities) are in `src/constants.rs`.
