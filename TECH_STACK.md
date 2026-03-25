# Galaga Clone — Tech Stack & Architecture Plan

## Technology Choice

| Layer | Technology | Version | Why |
|-------|-----------|---------|-----|
| Language | Rust | stable (2024+) | Memory safety, performance, no GC pauses |
| Game Engine | Bevy | 0.18 | ECS architecture maps naturally to Galaga's entity model |
| Physics/Collision | bevy_rapier2d | latest | Lightweight 2D collision detection without full physics |
| Audio | bevy_kira_audio | latest | OGG/WAV playback, volume control, web-compatible |
| Asset Loading | bevy_asset_loader | latest | Declarative asset collections, cleaner startup |

### Why Bevy over Macroquad
- **ECS fits Galaga perfectly** — 40+ enemies, bullets, effects are all entities with shared components
- **Automatic parallelism** — systems run concurrently where safe
- **State machine** — built-in game state management (Menu, Playing, Paused, GameOver)
- **Timer system** — built-in timers for wave spawning, cooldowns, animations
- **Large ecosystem** — plugins for audio, collision, asset loading
- **Active development** — Bevy 0.18 released January 2026, ~3-month release cadence

---

## Architecture Overview

### ECS Entity Model

```
Player Entity
├── Transform (position)
├── Sprite (ship texture)
├── PlayerShip (marker)
├── MovementSpeed(f32)
├── FireCooldown(Timer)
└── Collider

Enemy Entity
├── Transform (position)
├── Sprite (enemy texture + atlas)
├── AnimationTimer
├── EnemyType { Boss | Butterfly | Bee }
├── EnemyState { Forming | InFormation | Diving | Captured }
├── FormationSlot { row, col }
├── Health(u8)          // Boss=2, others=1
├── DivePath(Vec<Vec2>) // predefined dive curve
└── Collider

Bullet Entity
├── Transform (position)
├── Sprite
├── Velocity(Vec2)
├── BulletOwner { Player | Enemy }
└── Collider

CapturedShip Entity
├── Transform
├── Sprite
├── CapturedBy(Entity)  // reference to Boss that holds it
└── CapturedShip (marker)

Explosion Entity
├── Transform
├── SpriteSheet (animation frames)
├── AnimationTimer
└── DespawnTimer
```

### Resource Model (Global Singletons)

```
GameState         — Menu | Playing | Paused | GameOver | ChallengingStage
ScoreBoard        — score, high_score, lives, current_stage
Formation         — grid of Option<Entity>, breathing phase, position offsets
WaveController    — current wave, spawn timer, entry pattern index
DifficultyConfig  — dive_speed, fire_rate, aggression (scales with stage)
DualFighterState  — active: bool, captured_ship: Option<Entity>
```

### System Groups

```
┌─────────────────────────────────────────────────┐
│                  SYSTEM SCHEDULE                 │
├─────────────────────────────────────────────────┤
│                                                 │
│  Startup Systems (OnEnter)                      │
│  ├── setup_camera                               │
│  ├── load_assets (sprites, sounds, fonts)       │
│  └── spawn_starfield                            │
│                                                 │
│  Menu State                                     │
│  ├── render_menu_ui                             │
│  └── handle_menu_input                          │
│                                                 │
│  Playing State — runs every frame at 60fps      │
│  ├── Input Systems                              │
│  │   ├── player_movement                        │
│  │   ├── player_fire                            │
│  │   └── pause_toggle                           │
│  │                                              │
│  ├── Wave / Spawn Systems                       │
│  │   ├── wave_controller        (stage flow)    │
│  │   ├── enemy_entry_spawner    (fly-in paths)  │
│  │   └── splitter_transform     (stage 4+)      │
│  │                                              │
│  ├── Movement Systems                           │
│  │   ├── bullet_movement                        │
│  │   ├── formation_breathing    (expand/contract)│
│  │   ├── enemy_formation_movement               │
│  │   ├── enemy_dive_movement    (follow paths)  │
│  │   ├── tractor_beam_movement                  │
│  │   └── starfield_scroll                       │
│  │                                              │
│  ├── AI Systems                                 │
│  │   ├── enemy_dive_decision    (who dives when)│
│  │   ├── boss_tractor_decision  (beam trigger)  │
│  │   ├── enemy_fire_system      (shoot at player│
│  │   └── escort_assignment      (butterflies)   │
│  │                                              │
│  ├── Collision Systems                          │
│  │   ├── bullet_enemy_collision                 │
│  │   ├── bullet_player_collision                │
│  │   ├── enemy_player_collision                 │
│  │   └── tractor_beam_capture                   │
│  │                                              │
│  ├── Game Logic Systems                         │
│  │   ├── scoring_system                         │
│  │   ├── life_manager                           │
│  │   ├── dual_fighter_system                    │
│  │   ├── stage_completion_check                 │
│  │   └── difficulty_scaler                      │
│  │                                              │
│  ├── Animation Systems                          │
│  │   ├── sprite_animation       (frame cycling) │
│  │   ├── explosion_animation                    │
│  │   └── tractor_beam_animation                 │
│  │                                              │
│  └── Cleanup Systems                            │
│      ├── despawn_offscreen                      │
│      └── despawn_expired                        │
│                                                 │
│  Challenging Stage State                        │
│  ├── (reuses movement/collision/animation)      │
│  ├── challenge_wave_spawner                     │
│  ├── challenge_score_tracker                    │
│  └── challenge_completion                       │
│                                                 │
│  Game Over State                                │
│  ├── show_results_screen                        │
│  └── handle_restart_input                       │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## Project Structure

```
galaga/
├── Cargo.toml
├── assets/
│   ├── sprites/
│   │   ├── player.png
│   │   ├── player_dual.png
│   │   ├── enemies.png          # sprite sheet (all enemy frames)
│   │   ├── bullets.png
│   │   ├── explosions.png       # sprite sheet (explosion frames)
│   │   ├── tractor_beam.png     # sprite sheet (beam frames)
│   │   └── stage_flags.png
│   ├── sounds/
│   │   ├── shoot.wav
│   │   ├── explosion.wav
│   │   ├── enemy_dive.wav
│   │   ├── tractor_beam.wav
│   │   ├── capture.wav
│   │   ├── rescue.wav
│   │   ├── stage_start.ogg
│   │   ├── challenge_music.ogg
│   │   ├── game_over.ogg
│   │   └── perfect_bonus.wav
│   └── fonts/
│       └── arcade.ttf
├── src/
│   ├── main.rs                  # App setup, plugin registration, window config
│   ├── states.rs                # GameState enum, state transitions
│   ├── components.rs            # All ECS components
│   ├── resources.rs             # ScoreBoard, Formation, WaveController, etc.
│   ├── assets.rs                # Asset loading, sprite sheet definitions
│   ├── constants.rs             # Game constants (speeds, sizes, points, timings)
│   ├── player/
│   │   ├── mod.rs
│   │   ├── movement.rs          # Left/right input handling
│   │   ├── shooting.rs          # Fire button, bullet spawning, cooldown
│   │   └── dual_fighter.rs      # Capture/rescue/dual mechanics
│   ├── enemies/
│   │   ├── mod.rs
│   │   ├── types.rs             # Boss, Butterfly, Bee definitions
│   │   ├── formation.rs         # Grid layout, breathing, slot management
│   │   ├── entry_patterns.rs    # 3 fly-in path patterns
│   │   ├── dive_paths.rs        # Predefined dive curves (bezier/waypoints)
│   │   ├── ai.rs                # Dive decisions, aggression, group attacks
│   │   ├── tractor_beam.rs      # Boss capture mechanic
│   │   └── splitters.rs         # Scorpion/Stingray/Flagship transforms
│   ├── waves/
│   │   ├── mod.rs
│   │   ├── controller.rs        # Stage progression, wave sequencing
│   │   ├── difficulty.rs        # Scaling parameters per stage
│   │   └── challenging.rs       # Bonus stage logic
│   ├── collision/
│   │   ├── mod.rs
│   │   └── systems.rs           # All collision detection + response
│   ├── scoring/
│   │   ├── mod.rs
│   │   └── systems.rs           # Points, extra lives, high score
│   ├── effects/
│   │   ├── mod.rs
│   │   ├── explosions.rs        # Spawn + animate explosions
│   │   ├── starfield.rs         # Background star rendering + scroll
│   │   └── animations.rs        # Generic sprite animation system
│   └── ui/
│       ├── mod.rs
│       ├── hud.rs               # Score, lives, stage flags (in-game)
│       ├── menu.rs              # Title screen
│       └── game_over.rs         # Results screen
└── GALAGA_RESEARCH.md
```

---

## Key Implementation Details

### Window Configuration
```
Title:      "Galaga"
Resolution: 224 x 288 (original aspect ratio) scaled up, e.g., 448 x 576 (2x) or 672 x 864 (3x)
VSync:      AutoVsync (60fps target)
Resizable:  false
```

### Coordinate System
- Game logic uses **original 224x288 coordinate space**
- Camera scaling handles display at larger resolutions
- Origin at center; formation at top, player at bottom
- All positions, hitboxes, and paths defined in original-resolution units

### Dive Paths
Enemy dives use **predefined waypoint curves** (not runtime AI pathfinding):
- Stored as `Vec<Vec2>` waypoints per path type
- Enemies interpolate between waypoints at speed determined by difficulty
- Different path sets for: Boss dives, Butterfly swoops, Bee bombs, tractor beam runs
- Entry patterns (3 variants) also stored as waypoint paths

### Formation Breathing
- Sinusoidal expansion/contraction of formation grid
- Each slot offset = base_position + sin(time * frequency) * amplitude
- Amplitude and frequency are constants (~2px expansion, ~1Hz)

### Difficulty Scaling Formula
```
dive_speed       = BASE_DIVE_SPEED + (stage * SPEED_INCREMENT)
fire_rate        = max(MIN_FIRE_INTERVAL, BASE_FIRE_INTERVAL - (stage * RATE_DECREMENT))
dive_probability = min(MAX_DIVE_PROB, BASE_DIVE_PROB + (stage * PROB_INCREMENT))
max_concurrent   = min(MAX_DIVERS, BASE_DIVERS + (stage / DIVER_STEP))
```

### Collision Approach
Simple **AABB (axis-aligned bounding box)** collision — no need for full physics:
- Player bullets vs enemies: destroy enemy, spawn explosion, add score
- Enemy bullets vs player: lose life, spawn explosion
- Enemy body vs player: lose life, destroy enemy
- Tractor beam zone vs player: initiate capture sequence

Can use `bevy_rapier2d` sensors (no physics response, just overlap detection) or hand-rolled AABB checks — both work fine for this game's needs.

### Sprite Sheet Layout Plan
Single atlas per category for efficient batching:
- **enemies.png**: All enemy types x all animation frames in a grid
- **explosions.png**: Explosion sequence frames in a strip
- **tractor_beam.png**: Beam animation frames

### Audio Events Pattern
```rust
// Event-driven audio: game systems emit events, audio system plays sounds
enum GameAudioEvent {
    PlayerShoot,
    EnemyExplode,
    PlayerExplode,
    TractorBeamActivate,
    ShipCaptured,
    ShipRescued,
    StageStart,
    ChallengingStage,
    PerfectBonus,
    GameOver,
}
```

---

## Build & Run

```bash
# Development (fast compile, debug)
cargo run

# Release (optimized, for distribution)
cargo build --release

# Output: target/release/galaga.exe (Windows 11 native binary)
```

### Cargo.toml Key Dependencies
```toml
[dependencies]
bevy = { version = "0.18", features = ["default"] }
bevy_rapier2d = "0.28"        # or hand-rolled AABB
bevy_kira_audio = "0.21"      # rich audio playback
bevy_asset_loader = "0.22"    # declarative asset loading
rand = "0.8"                  # enemy AI randomness

[profile.dev]
opt-level = 1                 # faster dev builds

[profile.dev.package."*"]
opt-level = 3                 # optimize deps even in dev
```

---

## Development Phases

### Phase 1 — Foundation
- Project scaffold, Bevy app setup, window configuration
- Asset loading pipeline (placeholder sprites)
- Starfield background
- Player ship movement + shooting
- Basic bullet spawning and movement

### Phase 2 — Enemies & Formation
- Enemy component definitions
- Formation grid system with breathing animation
- Enemy entry patterns (fly-in paths)
- Basic enemy rendering with sprite sheets

### Phase 3 — Combat
- Collision detection (bullets vs enemies, enemies vs player)
- Explosion effects
- Scoring system + HUD
- Lives system
- Stage completion detection

### Phase 4 — Enemy AI
- Dive path system (waypoint curves)
- Dive decision AI (who dives, when, how many)
- Enemy firing during dives
- Escort formations (Butterflies with Boss)
- Difficulty scaling across stages

### Phase 5 — Special Mechanics
- Boss Galaga tractor beam (capture sequence)
- Dual fighter system (rescue + enhanced firepower)
- Splitter/transform enemies (Stage 4+)
- Challenging stages (bonus rounds)

### Phase 6 — Polish
- Sound effects + music
- Menu screen + game over screen
- Stage flag indicators
- Animation polish (enemy sprites, beam effects)
- Score display with high score persistence

### Phase 7 — Release
- Performance optimization
- Windows 11 native build + testing
- Release binary packaging
