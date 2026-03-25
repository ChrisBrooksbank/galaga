# Core Gameplay

## Overview

Foundation systems: window, player ship, bullets, enemies, formation, collision, scoring, lives, and stage progression.

## User Stories

- As a player, I want to move my ship left and right so that I can dodge enemies and aim
- As a player, I want to shoot bullets so that I can destroy enemies
- As a player, I want to see my score, lives, and stage so that I know my progress
- As a player, I want enemies to appear in formation so that I have targets
- As a player, I want to advance through stages so that the game has progression

## Requirements

### Project Setup
- [ ] Cargo.toml with Bevy 0.18, bevy_asset_loader, rand dependencies
- [ ] Window: 224x288 logical resolution, scaled 2-3x, non-resizable, title "Galaga"
- [ ] Camera setup with scaling to handle original coordinate space
- [ ] Game state enum: Menu, Playing, Paused, GameOver, ChallengingStage

### Player Ship
- [ ] Player entity with Transform, Sprite, PlayerShip marker, MovementSpeed, FireCooldown, Collider
- [ ] Horizontal-only movement via arrow keys or A/D
- [ ] Movement constrained to screen bounds
- [ ] Fire button (Space) spawns bullet with cooldown
- [ ] Max 2 bullets on screen simultaneously

### Bullets
- [ ] Bullet entity with Transform, Sprite, Velocity, BulletOwner (Player|Enemy), Collider
- [ ] Player bullets travel upward at fixed speed
- [ ] Enemy bullets travel downward at fixed speed
- [ ] Bullets despawn when leaving screen bounds

### Enemy Formation
- [ ] Formation grid: 4 Boss (row 1), 16 Butterfly (rows 2-3), 20 Bee (rows 4-5) = 40 total
- [ ] Formation resource tracking grid of Option<Entity> with row/col slots
- [ ] Formation breathing: sinusoidal expand/contract (~2px amplitude, ~1Hz)
- [ ] Enemies positioned in formation follow breathing offset
- [ ] Destroyed enemies leave gaps (formation does not compress)

### Enemy Entry
- [ ] Enemies fly onto screen in single-file lines during stage start
- [ ] 3 repeating entry patterns cycling every 3 stages
- [ ] Enemies can be shot during entry before reaching formation
- [ ] Stage starts once all enemies have reached formation positions

### Collision Detection
- [ ] AABB collision between player bullets and enemies
- [ ] AABB collision between enemy bullets and player
- [ ] AABB collision between enemy body and player
- [ ] Player bullet + enemy: destroy enemy, spawn explosion, add score
- [ ] Enemy bullet + player: lose life, spawn explosion
- [ ] Enemy body + player: lose life, destroy enemy

### Scoring
- [ ] Bee: 50 (formation), 100 (flight)
- [ ] Butterfly: 80 (formation), 160 (flight)
- [ ] Boss Galaga: 150 (formation), 400 (solo flight), 800 (1 escort), 1600 (2 escorts)
- [ ] Score displayed at top of screen
- [ ] High score tracking (session only)

### Lives System
- [ ] Start with 3 lives
- [ ] Extra life at 20,000 points, then every 70,000
- [ ] Lives displayed as ship icons at bottom-left
- [ ] Game over when lives reach 0

### Stage Progression
- [ ] Stage completes when all enemies destroyed
- [ ] Stage counter displayed on screen
- [ ] Stage flags at bottom-right (icons for 1, 5, 10, 20, 30, 50)
- [ ] Difficulty increases each stage (see enemy-ai spec)

## Acceptance Criteria

- [ ] Player can move, shoot, and destroy enemies
- [ ] Formation of 40 enemies assembles and breathes
- [ ] Collision detection works for all bullet/enemy/player interactions
- [ ] Score updates correctly for each enemy type
- [ ] Lives decrement on death, game over at 0
- [ ] Stages advance when all enemies destroyed

## Out of Scope

- Tractor beam / dual fighter (see enemy-ai spec)
- Splitter enemies (see enemy-ai spec)
- Sound effects and music (see audio spec)
- Menu and game over screens (see visual-polish spec)
