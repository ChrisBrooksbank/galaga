# Audio

## Overview

Sound effects for all game events and music tracks for game states, using an event-driven audio system.

## User Stories

- As a player, I want to hear my shots and explosions so that combat feels satisfying
- As a player, I want music during gameplay so that the game has atmosphere
- As a player, I want distinct sounds for special events (capture, rescue, bonus) so I get audio feedback

## Requirements

### Audio System Architecture
- [ ] Event-driven audio: game systems emit GameAudioEvent, audio system plays sounds
- [ ] GameAudioEvent enum: PlayerShoot, EnemyExplode, PlayerExplode, TractorBeamActivate, ShipCaptured, ShipRescued, StageStart, ChallengingStage, PerfectBonus, GameOver
- [ ] Use bevy_kira_audio for playback
- [ ] Support WAV (effects) and OGG (music) formats

### Sound Effects
- [ ] Player shoot: short punchy laser (shoot.wav)
- [ ] Enemy shoot: distinct from player, lower pitch (enemy_shoot.wav)
- [ ] Enemy explosion: small explosion (explosion_small.wav)
- [ ] Boss explosion: larger explosion (explosion_large.wav)
- [ ] Player death: distinct explosion (player_death.wav)
- [ ] Tractor beam: warbling sustained sound (tractor_beam.wav)
- [ ] Ship captured: ominous rising tone (capture.wav)
- [ ] Ship rescued: triumphant rising tone (rescue.wav)
- [ ] Dual fighter join: confirmation sound (dual_join.wav)
- [ ] Enemy dive: swoosh sound (dive_swoosh.wav)
- [ ] Bonus awarded: bright short sound (bonus.wav)
- [ ] Extra life: distinct notification (extra_life.wav)
- [ ] Stage clear: completion sound (stage_clear.wav)

### Music Tracks
- [ ] Title theme: 30-60s loop, upbeat arcade fanfare (title_theme.ogg)
- [ ] Stage start jingle: 3-5s ascending fanfare, plays once (stage_start.ogg)
- [ ] Gameplay loop: 60-90s loop, energetic driving rhythm (gameplay_loop.ogg)
- [ ] Challenging stage: 30-60s loop, playful and distinct (challenging_stage.ogg)
- [ ] Fighter captured: 3-5s ominous descending, plays once (fighter_captured.ogg)
- [ ] Perfect bonus: 5-8s triumphant, plays once (perfect_bonus.ogg)
- [ ] Game over: 5-10s somber, plays once (game_over.ogg)

### Music State Management
- [ ] Title theme plays on Menu state
- [ ] Gameplay loop plays during Playing state
- [ ] Challenging stage music replaces gameplay loop during ChallengingStage
- [ ] Music transitions: stop current, start new on state change
- [ ] Jingles (stage start, captured, perfect, game over) interrupt/overlay as needed

### Volume & Control
- [ ] Separate volume levels for SFX and music
- [ ] SFX should not clip when multiple play simultaneously

## Acceptance Criteria

- [ ] All game events trigger appropriate sound effects
- [ ] Music plays for each game state
- [ ] Music transitions cleanly between states
- [ ] Audio doesn't clip or lag during intense gameplay
- [ ] Game runs fine if audio files are missing (graceful fallback)

## Out of Scope

- Creating the actual audio asset files (see ASSETS.md for sourcing plan)
- Volume settings UI (future enhancement)
