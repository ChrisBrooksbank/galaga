# Galaga

A faithful recreation of the classic 1981 Namco arcade game, built from scratch in Rust with the Bevy game engine.

![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange)
![Bevy](https://img.shields.io/badge/Bevy-0.18-232326)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

**Classic Gameplay** — Full wave-based arcade action with 40-enemy formations, dive attacks, and progressive difficulty scaling across unlimited stages.

**Boss Tractor Beam & Dual Fighter** — Boss Galaga can capture your ship with a tractor beam. Destroy the boss to rescue it and fight with dual firepower.

**Challenging Stages** — Every 4th stage (3, 7, 11, ...) is a bonus round with unique flight patterns and a 10,000-point perfect bonus.

**Arcade Scoring** — Authentic point values: enemies are worth more when diving, boss escorts multiply scores up to 1600 points, and extra lives are awarded at 20,000 and every 70,000 points.

**Difficulty Scaling** — Dive speed, fire rate, attack probability, and concurrent divers all increase per stage.

**Retro Audio** — Chiptune music and 8-bit sound effects with independent volume controls.

## Controls

| Key | Action |
|-----|--------|
| Arrow Keys / A, D | Move left/right |
| Space | Shoot |
| Escape | Pause |
| Up/Down (paused) | Select volume slider |
| Left/Right (paused) | Adjust volume |

## Building & Running

Requires [Rust](https://rustup.rs/) (2024 edition).

```bash
cargo run
```

## Project Structure

```
src/
  main.rs              # App setup, state machine, system registration
  states.rs            # Game states (Menu, Playing, Paused, etc.)
  components.rs        # ECS components
  constants.rs         # Game tuning constants
  resources.rs         # Shared game resources
  assets.rs            # Asset definitions and sprite atlas indices
  player/              # Ship movement, shooting, death, dual fighter
  enemies/             # Formation, AI, dive paths, tractor beam, splitters
  collision/           # Bullet/enemy/player collision systems
  waves/               # Wave controller, difficulty scaling, challenging stages
  scoring/             # Score events and extra life logic
  effects/             # Starfield, explosions, animations
  ui/                  # Menu, HUD, pause, game over, stage intro
  audio/               # Music and SFX event handling
assets/
  sprites/             # Pixel art sprite sheets
  sounds/              # WAV sound effects
  music/               # OGG music tracks
  fonts/               # Press Start 2P font
```

## Tech Stack

- **[Bevy 0.18](https://bevyengine.org/)** — ECS game engine
- **[bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio)** — Audio playback
- **[bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader)** — Asset management

## License

MIT
