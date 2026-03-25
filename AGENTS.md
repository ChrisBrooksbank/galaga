# AGENTS.md - Operational Guide

Keep this file under 60 lines. It's loaded every iteration.

## Build Commands

```bash
cargo build              # Debug build
cargo build --release    # Release build (optimized)
cargo run                # Build and run (debug)
```

## Test Commands

```bash
cargo test               # Run all tests
cargo test -- --nocapture # Tests with stdout
```

## Validation (run before committing)

```bash
cargo build              # Must compile without errors
cargo test               # All tests must pass
cargo clippy             # Lint check (if clippy installed)
```

## Tech Stack

- **Language**: Rust (stable)
- **Engine**: Bevy 0.18
- **Collision**: bevy_rapier2d or hand-rolled AABB
- **Audio**: bevy_kira_audio
- **Assets**: bevy_asset_loader

## Project Structure

See TECH_STACK.md for full architecture. Key paths:
- `src/main.rs` — App setup, plugin registration
- `src/components.rs` — All ECS components
- `src/resources.rs` — Global singletons (score, formation, wave)
- `src/player/` — Player movement, shooting, dual fighter
- `src/enemies/` — Enemy types, formation, AI, tractor beam
- `src/waves/` — Stage progression, difficulty, challenging stages
- `src/collision/` — Collision detection systems
- `src/scoring/` — Points, lives, high score
- `src/effects/` — Explosions, starfield, animations
- `src/ui/` — HUD, menus, game over screen

## Notes

- Window: 224x288 original resolution, scaled 2-3x
- Game logic uses original coordinate space
- All enemy sprites are 16x16 pixels
- Formation: 4 Boss + 16 Butterfly + 20 Bee = 40 enemies
