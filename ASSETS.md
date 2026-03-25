# Galaga Clone — Asset Plan

## Strategy

**Primary approach**: Use **CC0-licensed** pre-made assets (Kenney.nl, OpenGameArt) for rapid development, supplemented with **custom-generated** sound effects (jsfxr) and **original chiptune music** (BeepBox). Create custom pixel art sprites only where needed to match the authentic Galaga aesthetic.

**License policy**: CC0 (public domain) assets only — zero attribution requirements, zero legal risk.

---

## 1. Sprites & Visual Assets

### Recommended Source: Kenney Space Shooter Redux
- **URL**: https://kenney.nl/assets/space-shooter-redux
- **License**: CC0 (public domain)
- **Contents**: 295 sprites — ships, enemies, bullets, explosions, UI elements, 2 TTF fonts
- **Use for**: Base player ship, bullet sprites, explosion frames, UI elements

### Supplementary Sources

| Source | URL | License | Use For |
|--------|-----|---------|---------|
| Kenney Space Shooter Extension | https://kenney.nl/assets/space-shooter-extension | CC0 | Additional projectiles, missiles |
| Arcade Space Shooter Assets | https://opengameart.org/content/arcade-space-shooter-game-assets | CC0 | Retro-styled enemies, animated background |
| 200+ CC0 Spaceship Sprites | https://opengameart.org/content/200-cc0-spaceship-sprites | CC0 | Enemy variety |
| Space Ships Sprites | https://opengameart.org/content/space-ships-sprites | CC0 | Tiny pixel art ships |
| Seamless Space Backgrounds | https://opengameart.org/content/seamless-space-backgrounds | CC0 | 32 tiling backgrounds (512x512 & 1024x1024) |

### What We Need vs What Exists

| Asset | Source Available? | Plan |
|-------|-------------------|------|
| Player ship | Yes (Kenney) | Use Kenney ship, tint white to match original |
| Player ship (dual) | Partial | Duplicate player sprite side-by-side |
| Boss Galaga | No exact match | **Create custom** 16x16 pixel art (green, 2 frames) |
| Boss Galaga (hit) | No exact match | **Create custom** purple variant |
| Butterfly enemy | No exact match | **Create custom** 16x16 pixel art (red/white, 2 frames) |
| Bee enemy | No exact match | **Create custom** 16x16 pixel art (blue/yellow, 2 frames) |
| Scorpion splitter | No exact match | **Create custom** 16x16 pixel art (yellow) |
| Stingray splitter | No exact match | **Create custom** 16x16 pixel art (green) |
| Galaxian Flagship | No exact match | **Create custom** 16x16 pixel art |
| Player bullet | Yes (Kenney) | Use small laser sprite |
| Enemy bullet | Yes (Kenney) | Use small projectile sprite |
| Explosions | Yes (Kenney) | Use explosion sprite sheet |
| Tractor beam | No exact match | **Create custom** animated beam (4-6 frames) |
| Stage flags | No exact match | **Create custom** small flag icons (8x8 each) |
| Lives indicator | Partial | Miniature player ship sprite |
| Stars/background | Procedural | Generate in code (random dots) or use `bevy_starfield` crate |
| Captured ship (red) | Partial | Tint player ship sprite red |

### Custom Sprite Creation Plan

For the ~10 custom sprites needed, use one of these tools:

| Tool | Type | Cost | Best For |
|------|------|------|----------|
| **Piskel** | Browser-based | Free | Quick sprite creation, no install needed |
| **Pixelorama** | Desktop app | Free (MIT) | Advanced features, spritesheet export |
| **LibreSprite** | Desktop app | Free (GPLv2) | Aseprite-compatible, animation timeline |
| **Aseprite** | Desktop app | $20 | Industry standard (or build from source free) |

#### Sprite Sheet Layout

All custom enemy sprites should be compiled into a single `enemies.png` atlas:

```
enemies.png (128 x 64 pixels)
┌────┬────┬────┬────┬────┬────┬────┬────┐
│Boss│Boss│Boss│Boss│Btrf│Btrf│Bee │Bee │  Row 1: Main enemies (16x16 each)
│ G1 │ G2 │ P1 │ P2 │  1 │  2 │  1 │  2 │  G=green, P=purple, frame 1&2
├────┼────┼────┼────┼────┼────┼────┼────┤
│Scor│Scor│Stin│Stin│Flag│Flag│Flag│Flag│  Row 2: Splitters + misc (16x16)
│  1 │  2 │  1 │  2 │ship│ 1  │ 5  │ 10 │
├────┼────┼────┼────┼────┼────┼────┼────┤
│Expl│Expl│Expl│Expl│Expl│Expl│Beam│Beam│  Row 3: Explosions + beam
│  1 │  2 │  3 │  4 │  5 │  6 │  1 │  2 │
├────┼────┼────┼────┼────┼────┼────┼────┤
│Beam│Beam│Flag│Flag│Flag│    │    │    │  Row 4: Beam cont + remaining flags
│  3 │  4 │ 20 │ 30 │ 50 │    │    │    │
└────┴────┴────┴────┴────┴────┴────┴────┘
```

### Color Palette Reference (from original Galaga PROMs)

Source: http://computerarcheology.com/Arcade/Galaga/PROMcolors.html

| Color | Hex | RGB | Usage |
|-------|-----|-----|-------|
| White | #DEDEDE | (222, 222, 222) | Player ship, UI text |
| Red | #FF0000 | (255, 0, 0) | Butterfly wings, captured ship |
| Yellow | #FFFF00 | (255, 255, 0) | Bee enemies, score text |
| Orange | #FF9700 | (255, 151, 0) | Accents |
| Green | #00FF00 | (0, 255, 0) | Boss Galaga (normal) |
| Magenta | #FF00DE | (255, 0, 222) | Boss Galaga (damaged) |
| Cyan | #00FFDE | (0, 255, 222) | Tractor beam |
| Blue | #0068DE | (0, 104, 222) | Bee accent, tractor beam |
| Purple | #9700DE | (151, 0, 222) | Effects |
| Dark Blue | #0000DE | (0, 0, 222) | Deep accents |
| Teal | #009797 | (0, 151, 151) | Misc accents |
| Black | #000000 | (0, 0, 0) | Background |

---

## 2. Sound Effects

### Primary Source: Kenney.nl Audio Packs (all CC0)

| Pack | URL | Contents |
|------|-----|----------|
| Sci-Fi Sounds | https://kenney.nl/assets/sci-fi-sounds | 70 sci-fi effects (beams, energy, warp) |
| Impact Sounds | https://kenney.nl/assets/impact-sounds | 130 impact/collision effects |
| UI Audio | https://kenney.nl/assets/ui-audio | 50 menu/UI sounds |
| Digital Audio | https://kenney.nl/assets/digital-audio | 60 digital/synth effects |

### Supplementary Source: OpenGameArt (CC0)

| Pack | URL | Contents |
|------|-----|----------|
| 512 Sound Effects (8-bit) | https://opengameart.org/content/512-sound-effects-8-bit-style | 512 retro game SFX |
| 63 Digital SFX | https://opengameart.org/content/63-digital-sound-effects-lasers-phasers-space-etc | Lasers, phasers, zaps |
| Retro Shooter SFX | https://opengameart.org/content/retro-shooter-sound-effects | Hits, gunshots, game over |
| 8-BIT Explosions | https://opengameart.org/content/8-bit-explosions-1 | 21 explosion variations |

### Custom Generation: jsfxr

**URL**: https://sfxr.me/ (browser-based, free, exports WAV)

Best for creating custom variations when pre-made packs don't have the right feel:

| Sound Needed | jsfxr Preset | Notes |
|-------------|-------------|-------|
| Player shoot | "Laser/shoot" preset, tweak pitch up | Short, punchy |
| Enemy shoot | "Laser/shoot" preset, lower pitch | Distinct from player |
| Small explosion | "Explosion" preset, short decay | For regular enemies |
| Large explosion | "Explosion" preset, long decay | For Boss Galaga |
| Tractor beam | "Synth" preset, add vibrato | Warbling, sustained |
| Ship captured | "Powerup" preset, reversed feel | Ominous rising tone |
| Ship rescued | "Powerup" preset | Triumphant rising tone |
| Dive swoosh | "Jump" preset, modify pitch sweep | Quick descending |
| Menu select | "Blip/select" preset | Short click |
| Bonus popup | "Pickup/coin" preset | Bright, short |

### Sound Effect Mapping

| Game Event | File | Source | Format |
|------------|------|--------|--------|
| Player shoots | `shoot.wav` | jsfxr or Kenney Sci-Fi | WAV |
| Enemy shoots | `enemy_shoot.wav` | jsfxr or Kenney Sci-Fi | WAV |
| Enemy explodes | `explosion_small.wav` | Kenney Impact or OpenGameArt | WAV |
| Boss explodes | `explosion_large.wav` | Kenney Impact | WAV |
| Player explodes | `player_death.wav` | Kenney Impact | WAV |
| Tractor beam active | `tractor_beam.wav` | jsfxr custom or Kenney Sci-Fi | WAV |
| Ship captured | `capture.wav` | jsfxr custom | WAV |
| Ship rescued | `rescue.wav` | jsfxr custom | WAV |
| Dual fighter join | `dual_join.wav` | jsfxr custom | WAV |
| Enemy dive | `dive_swoosh.wav` | Kenney Sci-Fi | WAV |
| Bonus awarded | `bonus.wav` | jsfxr or Kenney UI | WAV |
| Extra life | `extra_life.wav` | jsfxr or Kenney UI | WAV |
| Menu select | `menu_select.wav` | Kenney UI Audio | WAV |
| Menu confirm | `menu_confirm.wav` | Kenney UI Audio | WAV |
| Stage clear | `stage_clear.wav` | jsfxr custom | WAV |

---

## 3. Music

### Creation Tool: BeepBox

**URL**: https://www.beepbox.co/ (browser-based, free, no signup)

BeepBox is ideal for creating authentic 8-bit/chiptune music. Songs are saved as URLs — export as WAV or MP3 for the game.

**Advanced fork**: UltraBox (https://ultrabox.blog/) — 6-op FM synthesis, 32 channels, more instruments.

### Tracks Needed

| Track | Duration | Style | Notes |
|-------|----------|-------|-------|
| Title theme | 30-60s loop | Upbeat, iconic, arcade fanfare | Main menu music, should feel nostalgic |
| Stage start jingle | 3-5s | Quick ascending fanfare | Plays once at wave start |
| Gameplay loop | 60-90s loop | Energetic, driving rhythm | Background during combat, subtle enough to not distract |
| Challenging stage | 30-60s loop | Playful, upbeat, distinct | Bonus round music |
| Fighter captured | 3-5s | Ominous, descending | Tractor beam capture moment |
| Perfect bonus | 5-8s | Triumphant, bright | All 40 enemies destroyed in bonus |
| Game over | 5-10s | Somber, final | No loop, plays once |

### Fallback: Pre-Made Music (if not composing original)

| Source | URL | License | Contents |
|--------|-----|---------|----------|
| OpenGameArt Space Shooter Music | https://opengameart.org/content/space-shooter-music | CC0 | 80s synth space shooter loops |
| Free Chiptune Pack (itch.io) | https://retroindiejosh.itch.io/free-music-pack-5 | Royalty-free | Chiptune music loops |
| 12 Sci-fi Chiptune Tracks | https://oragus.itch.io/free-asset-pack-2 | Royalty-free | 8-bit sci-fi tracks |
| Pixabay Arcade Music | https://pixabay.com/music/search/arcade/ | Royalty-free | Various arcade-style tracks |

### Alternative Music Tool: FamiTracker

**URL**: http://famitracker.com/ (Windows app, free, GPL v2)

For the most authentic NES-era sound (2 square waves, 1 triangle, 1 noise, 1 DPCM). Steeper learning curve but produces the most faithful retro audio. Exports to WAV.

---

## 4. Fonts

### Primary: Press Start 2P

- **URL**: https://fonts.google.com/specimen/Press+Start+2P
- **License**: OFL (Open Font License) — free for commercial use
- **Why**: Designed after 1980s Namco arcade games — literally the Galaga-era aesthetic
- **Rendering**: Best at 8px, 16px, and multiples of 8
- **Use for**: All in-game text (score, lives, stage number, menus, "GAME OVER", etc.)

### Fallback: Karmatic Arcade

- **URL**: https://www.1001fonts.com/karmatic-arcade-font.html
- **License**: Free for commercial use
- **Why**: Clean arcade monospace, good number rendering

### Font Discovery

- 1001 Fonts arcade collection: https://www.1001fonts.com/arcade+pixel-fonts.html (52+ free)
- FontSpace arcade: https://www.fontspace.com/category/arcade (252 free)
- DaFont bitmap: https://www.dafont.com/bitmap.php

---

## 5. Starfield Background

### Option A: Procedural (Recommended)

Generate stars in code — simplest and most authentic:

```
- Spawn 50-100 small white dot entities at random positions
- Each star has a random scroll speed (parallax depth illusion)
- Stars wrap around when they exit the screen bottom
- Optional: vary brightness (dim gray to bright white)
```

### Option B: Bevy Plugin

- **bevy_starfield**: https://crates.io/crates/bevy_starfield — procedural night sky plugin
- **bevy_hanabi**: https://crates.io/crates/bevy_hanabi — GPU particle system (overkill but available)

### Option C: Pre-Made Background

- Seamless Space Backgrounds (CC0): https://opengameart.org/content/seamless-space-backgrounds
- 32 tiling backgrounds at 512x512 and 1024x1024

---

## 6. Asset Directory Structure

```
assets/
├── sprites/
│   ├── player.png              # Player ship (from Kenney or custom)
│   ├── enemies.png             # Enemy sprite sheet (custom, see layout above)
│   ├── bullets.png             # Player + enemy bullets (from Kenney)
│   ├── explosions.png          # Explosion animation frames (from Kenney)
│   ├── tractor_beam.png        # Beam animation frames (custom)
│   ├── stage_flags.png         # Stage indicator icons (custom)
│   └── ui/
│       └── lives_icon.png      # Mini ship for lives display
├── sounds/
│   ├── shoot.wav
│   ├── enemy_shoot.wav
│   ├── explosion_small.wav
│   ├── explosion_large.wav
│   ├── player_death.wav
│   ├── tractor_beam.wav
│   ├── capture.wav
│   ├── rescue.wav
│   ├── dual_join.wav
│   ├── dive_swoosh.wav
│   ├── bonus.wav
│   ├── extra_life.wav
│   ├── menu_select.wav
│   ├── menu_confirm.wav
│   └── stage_clear.wav
├── music/
│   ├── title_theme.ogg
│   ├── stage_start.ogg
│   ├── gameplay_loop.ogg
│   ├── challenging_stage.ogg
│   ├── fighter_captured.ogg
│   ├── perfect_bonus.ogg
│   └── game_over.ogg
└── fonts/
    └── PressStart2P-Regular.ttf
```

---

## 7. Asset Creation Workflow

### Phase 1 — Immediate (download)
1. Download **Kenney Space Shooter Redux** — extract ships, bullets, explosions, UI
2. Download **Kenney Sci-Fi Sounds + Impact Sounds + UI Audio**
3. Download **OpenGameArt 512 8-bit SFX** pack as backup variety
4. Download **Press Start 2P** font from Google Fonts

### Phase 2 — Generate (tools)
5. Open **jsfxr** (https://sfxr.me/) — generate custom sounds (tractor beam, capture, rescue, dive swoosh)
6. Open **BeepBox** (https://www.beepbox.co/) — compose 7 music tracks (or use fallback pre-made tracks)
7. Export all audio as WAV (sounds) and OGG (music)

### Phase 3 — Create (pixel art)
8. Open **Piskel** (https://www.piskelapp.com/) or **Pixelorama** (https://pixelorama.org/)
9. Create enemy sprites using the color palette reference (16x16 per frame):
   - Boss Galaga: 2 green frames + 2 purple frames
   - Butterfly: 2 frames (wing flutter)
   - Bee: 2 frames (wing flutter)
   - Scorpion, Stingray, Galaxian Flagship: 2 frames each
10. Create tractor beam animation (4-6 frames, cyan)
11. Create stage flag icons (8x8 each: 1, 5, 10, 20, 30, 50)
12. Compile into sprite sheet atlas (`enemies.png`)

### Phase 4 — Integrate
13. Place all files in `assets/` directory per structure above
14. Configure Bevy `AssetServer` to load from these paths
15. Define `TextureAtlasLayout` for each sprite sheet

---

## 8. Licensing Summary

| Asset Category | Source | License | Attribution Needed |
|----------------|--------|---------|-------------------|
| Sprites (Kenney) | kenney.nl | CC0 | No |
| Sprites (custom) | Original creation | N/A (ours) | No |
| Sound effects (Kenney) | kenney.nl | CC0 | No |
| Sound effects (jsfxr) | Generated | CC0 | No |
| Sound effects (OpenGameArt) | opengameart.org | CC0 | No |
| Music (BeepBox originals) | Original creation | N/A (ours) | No |
| Music (fallback packs) | Various | CC0 / Royalty-free | Check per track |
| Font (Press Start 2P) | Google Fonts | OFL | No (but nice to credit) |
| Starfield backgrounds | opengameart.org | CC0 | No |

**All assets are free and clear for any use — no attribution legally required.**
