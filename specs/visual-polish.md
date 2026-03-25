# Visual Polish

## Overview

Starfield background, explosion effects, sprite animations, HUD, menu screen, game over screen, and all visual presentation.

## User Stories

- As a player, I want a scrolling starfield so that the game feels like space
- As a player, I want explosions when enemies die so that combat feels impactful
- As a player, I want animated enemy sprites so that enemies feel alive
- As a player, I want a title screen so that the game has a proper start
- As a player, I want a game over screen showing my results

## Requirements

### Starfield Background
- [ ] 50-100 small white dot entities at random positions
- [ ] Each star has random scroll speed (parallax depth illusion)
- [ ] Stars wrap around when exiting screen bottom
- [ ] Vary brightness from dim gray to bright white

### Explosion Effects
- [ ] Explosion entity with SpriteSheet, AnimationTimer, DespawnTimer
- [ ] Sprite-based explosion animation (4-6 frames)
- [ ] Spawned at enemy/player death position
- [ ] Auto-despawn after animation completes

### Enemy Sprite Animations
- [ ] All enemies use sprite sheet atlas (enemies.png, 16x16 frames)
- [ ] Wing flutter animation: cycle between 2 frames
- [ ] AnimationTimer controls frame rate
- [ ] Boss Galaga: green sprites normally, purple after first hit
- [ ] Formation breathing visually expands/contracts the grid

### Tractor Beam Visual
- [ ] Fan-shaped cyan/blue energy field from Boss Galaga
- [ ] Animated beam effect (4-6 frames)
- [ ] Beam sprite sheet (tractor_beam.png)
- [ ] Player ship color changes white -> red when captured

### HUD (Heads-Up Display)
- [ ] "1UP" label + current score at top-left
- [ ] "HIGH SCORE" label + high score at top-center/right
- [ ] Lives as mini ship icons at bottom-left
- [ ] Stage flag icons at bottom-right (1, 5, 10, 20, 30, 50 flag types)
- [ ] Font: Press Start 2P (or similar arcade pixel font)

### Menu Screen
- [ ] "GALAGA" title text, large and centered
- [ ] "PRESS SPACE TO START" blinking text
- [ ] High score displayed
- [ ] Starfield visible in background

### Game Over Screen
- [ ] "GAME OVER" text centered
- [ ] Final score displayed
- [ ] "PRESS SPACE TO RESTART" text
- [ ] Results: shots fired, enemies hit, hit ratio

### Challenging Stage UI
- [ ] "CHALLENGING STAGE" text at start
- [ ] "PERFECT" text + 10,000 bonus display if all 40 destroyed
- [ ] Results summary: "Number of hits: XX"

### Asset Loading
- [ ] Use bevy_asset_loader for declarative asset collections
- [ ] Loading state before menu (load all sprites, fonts)
- [ ] TextureAtlasLayout definitions for each sprite sheet

## Acceptance Criteria

- [ ] Starfield scrolls with parallax effect
- [ ] Explosions animate and despawn correctly
- [ ] Enemy sprites animate in formation and during dives
- [ ] HUD shows score, lives, and stage accurately
- [ ] Menu and game over screens are functional
- [ ] All assets load without errors

## Out of Scope

- Sound effects and music (see audio spec)
- Actual gameplay logic (see core-gameplay and enemy-ai specs)
