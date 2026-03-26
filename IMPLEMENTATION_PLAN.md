# Implementation Plan

## Status

- Planning iterations: 1
- Build iterations: 6
- Last updated: 2026-03-25

## Tasks

### Phase 1: Project Foundation
- [x] Initialize Rust project: `cargo new galaga`, add Bevy 0.18 + all deps to Cargo.toml (bevy_kira_audio, bevy_asset_loader, bevy_rapier2d) (spec: core-gameplay.md)
- [x] Configure Bevy app: 224x288 logical resolution, 2x pixel-perfect scaling, 60fps, window title "Galaga" (spec: core-gameplay.md)
- [x] Define GameState enum (Loading, Menu, Playing, Paused, GameOver, ChallengingStage) and wire state transitions (spec: core-gameplay.md)
- [x] Set up module structure: player/, enemies/, waves/, collision/, scoring/, effects/, ui/, audio/ (spec: core-gameplay.md)

### Phase 2: Asset Pipeline
- [x] Create assets/ directory structure: sprites/, sounds/, music/, fonts/ (spec: visual-polish.md)
- [x] Download Kenney Space Shooter Redux sprites; create/export 128x64 sprite atlas with enemies, explosions, beam frames (spec: visual-polish.md)
- [x] Download Press Start 2P font (OFL license) to assets/fonts/ (spec: visual-polish.md)
- [x] Download Kenney Sci-Fi / Impact / UI / Digital audio packs; place SFX in assets/sounds/ (spec: audio.md)
- [x] Compose 7 BeepBox chiptune tracks (menu, stage-intro, gameplay, challenging, boss-capture, game-over, high-score) and export to assets/music/ (spec: audio.md)
- [x] Implement bevy_asset_loader LoadingState: define GameAssets resource with TextureAtlasLayout, SFX handles, music handles, font handle (spec: visual-polish.md)

### Phase 3: Player Ship
- [x] Spawn player entity with sprite, Player component, Transform at bottom-center (spec: core-gameplay.md)
- [x] Player horizontal movement system: keyboard left/right, clamped to screen bounds, speed from config (spec: core-gameplay.md)
- [x] Player shoot system: Space key fires bullet, max 2 simultaneous bullets on screen (spec: core-gameplay.md)
- [x] Bullet movement system: bullets travel upward, despawn at top of screen (spec: core-gameplay.md)
- [x] Player death + respawn system: remove entity, show explosion, decrement lives, respawn if lives > 0 (spec: core-gameplay.md)

### Phase 4: Formation Layout & Animation
- [x] Define Formation resource: 40-slot grid (4 Boss rows=0, 16 Butterfly rows=1-2, 20 Bee rows=3-4), slot positions (spec: core-gameplay.md)
- [x] Spawn 40 enemy entities in formation with correct EnemyType (Boss, Butterfly, Bee) and formation slot (spec: core-gameplay.md)
- [x] Formation breathing animation: sinusoidal horizontal oscillation of the entire formation (spec: core-gameplay.md)
- [x] Enemy wing-flutter animation: 2-frame sprite cycle per enemy, timed independently (spec: visual-polish.md)
- [x] Enemy entry animation system: enemies enter in single-file lines from screen edges, follow curved path to formation slot (spec: core-gameplay.md)

### Phase 5: Collision Detection
- [x] AABB collision system: bullet vs enemy — destroy both, emit ScoreEvent, spawn explosion (spec: core-gameplay.md)
- [x] AABB collision system: enemy/bullet vs player — trigger player death, spawn explosion (spec: core-gameplay.md)
- [x] Collision system: enemy body vs player during dive — trigger player death (spec: core-gameplay.md)

### Phase 6: Scoring & Lives
- [x] ScoreBoard resource: score, high score, lives (3), stage; scoring constants per enemy type and state (diving vs formation) (spec: core-gameplay.md)
- [x] Score event handler: apply correct points (Bee 50/100, Butterfly 80/160, Boss 150/400 formation; Boss 800/1600 diving) (spec: core-gameplay.md)
- [x] Extra life system: award extra life at 20,000 points, then every 70,000 after that (spec: core-gameplay.md)
- [x] Stage progression: advance stage when all enemies destroyed; increase difficulty params (spec: core-gameplay.md)
- [x] DifficultyConfig resource with formula-driven scaling per stage (enemy speed, fire rate, dive probability, concurrent attackers) (spec: enemy-ai.md)

### Phase 7: Enemy AI — Dive System
- [x] Define predefined waypoint dive paths as Bezier curves / waypoint arrays for each enemy type and side (spec: enemy-ai.md)
- [x] Dive state machine: Idle → Diving → Returning; select random dive path on enter (spec: enemy-ai.md)
- [x] Dive movement system: move enemy along waypoints, return to formation slot when done (spec: enemy-ai.md)
- [x] Enemy firing system: enemies fire during dives at difficulty-scaled intervals; bullets travel downward (spec: enemy-ai.md)
- [x] Group attack coordinator: schedule Bee group attacks and Butterfly group attacks per wave timing (spec: enemy-ai.md)
- [x] Concurrent attacker limiter: enforce max simultaneous divers per difficulty level (spec: enemy-ai.md)

### Phase 8: Boss Galaga Tractor Beam
- [x] Tractor beam trigger: Boss Galaga (2+ on field) can initiate capture dive; requires no player bullet in flight (spec: enemy-ai.md)
- [x] Tractor beam beam entity: fan-shaped cyan visual, pulses, locks player movement during capture (spec: enemy-ai.md)
- [x] Player capture: player ship entity transitions to CapturedShip, attaches to Boss, formation continues (spec: enemy-ai.md)
- [x] Dual fighter: spawn second player ship; both ships fire when dual fighter active (4 bullets max) (spec: enemy-ai.md)
- [x] Dual fighter rescue: player destroys Boss during dive → captured ship freed → joins player as dual fighter (spec: enemy-ai.md)
- [x] Dual fighter death: losing dual fighter reverts to single ship (spec: enemy-ai.md)

### Phase 9: Challenging Stages & Splitters
- [x] Challenging stage trigger: stages 3, 7, 11, 15, ... spawn challenging stage (no formation, enemies fly fixed patterns) (spec: enemy-ai.md)
- [x] Challenging stage enemy patterns: straight lines, figure-8s, loop patterns across screen (spec: enemy-ai.md)
- [x] Perfect bonus: award 10,000 points if all challenging-stage enemies destroyed (spec: enemy-ai.md)
- [x] Splitter/transform enemies: starting stage 4, some enemies split into 2 on death (Scorpions/Stingrays) or become Galaxian Flagship (spec: enemy-ai.md)
- [x] Splitter movement: split pieces fly in diverging paths (spec: enemy-ai.md)

### Phase 10: Visual Effects & HUD
- [x] Starfield background: 50-100 stars at varying speeds (parallax layers), scrolling downward (spec: visual-polish.md)
- [ ] Explosion animation system: 4-6 frame sprite animation, auto-despawn on complete (spec: visual-polish.md)
- [ ] HUD: score (top-left), high-score (top-center), lives icons (bottom-left), stage flags (bottom-right) (spec: visual-polish.md)
- [ ] Menu screen: "GALAGA" title, "PRESS SPACE TO START", high score display; transition to Playing on Space (spec: visual-polish.md)
- [ ] Game over screen: "GAME OVER" text, final score, stage reached; return to Menu after delay (spec: visual-polish.md)
- [ ] Stage intro screen: "STAGE X" display with brief pause before enemies enter (spec: visual-polish.md)

### Phase 11: Audio System
- [ ] Integrate bevy_kira_audio; define AudioEvent enum for all 10 SFX triggers (spec: audio.md)
- [ ] SFX playback system: handle AudioEvents → play correct sound (shoot, explosion-small, explosion-large, tractor-beam, capture, rescued, bonus, extra-life, stage-clear, insert-coin) (spec: audio.md)
- [ ] Music state manager: track current MusicState, cross-fade/stop-start between tracks on GameState transitions (spec: audio.md)
- [ ] Volume control: separate SFX volume and music volume resources, adjustable from pause menu (spec: audio.md)

## Completed

- [x] Initialize Rust project: `cargo new galaga`, add Bevy 0.18 + all deps to Cargo.toml (bevy_kira_audio, bevy_asset_loader, bevy_rapier2d) (spec: core-gameplay.md)

## Notes

### Architecture Decisions
- **ECS Layout**: Components per TECH_STACK.md: `Player`, `Enemy { enemy_type, formation_slot, state }`, `Bullet { owner }`, `CapturedShip`, `Explosion { frame, timer }`, `FormationSlot { index, home_pos }`
- **Resolution**: 224x288 logical pixels (portrait, matches original arcade), 2x scale for modern screens
- **Collision**: Simple AABB (no physics engine shapes needed despite bevy_rapier2d dependency — use rapier for transform integration only if needed, otherwise hand-rolled AABB)
- **Dive Paths**: Predefined waypoint arrays, not runtime pathfinding. Each path is a `Vec<Vec2>` stored as a const or lazy_static resource.
- **State Machine**: GameState drives music and system scheduling via `run_if(in_state(...))` conditions
- **Scoring**: Diving enemies worth 2x formation score. Track whether enemy was in formation or diving at time of kill.
- **Extra Lives**: 20K first, then every 70K (not 90K as research doc suggests — specs say 20K then every 70K)
- **Sprite Atlas**: Single 128x64 atlas. TextureAtlasLayout defined in GameAssets, individual sprites referenced by index.
- **Audio**: bevy_kira_audio for positional-agnostic 2D audio. All SFX triggered via events, not direct calls.
- **Challenging Stages**: Occur at stages 3, 7, 11, 15 (every 4th after stage 3). No formation — all 40 enemies fly scripted paths. Perfect bonus = 10,000 pts.

### Dependency Order
1. Foundation → Asset Pipeline → Player → Formation → Collision → Scoring → Enemy AI → Boss/Tractor → Challenging/Splitters → Visual Polish → Audio

### Open Questions
- bevy_rapier2d: confirm if needed or if AABB hand-rolled suffices (likely hand-rolled is simpler)
- Sprite atlas source: confirm Kenney Space Shooter Redux has enough frames, or if custom pixel art needed from day 1
