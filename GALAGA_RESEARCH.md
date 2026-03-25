# Galaga - Complete Game Research

## 1. Overview & History

- **Title**: Galaga
- **Developer**: Namco
- **Release**: September 1981 (Japan), October 1981 (North America)
- **Genre**: Fixed shooter / Shoot-em-up
- **Sequel to**: Galaxian (1979)
- **Designer**: Shigeru Yokoyama
- **Significance**: One of the most recognized and influential arcade games of the golden age. Improved on Galaxian with faster gameplay, dual-bullet firing, the tractor beam capture mechanic, and bonus stages.

---

## 2. Technical Specifications

### Hardware
| Component | Specification |
|-----------|--------------|
| Main CPUs | 3x Zilog Z80 @ 3.0 MHz |
| Microcontrollers | Fujitsu MB8843 + MB8844 @ 1.536 MHz each |
| CPU1 ROM | 16K (main game logic) |
| CPU2 ROM | 4K (slave processor) |
| CPU3 ROM | 4K (slave processor) |
| Sprite ROM | 12K total |
| Sound Chip | Namco 3-channel WSG (Waveform Sound Generator) |
| Display | 19-inch color CRT monitor |
| Cabinet Types | Upright, cocktail table, cabaret/mini |

### Display
| Property | Value |
|----------|-------|
| Resolution | 288 x 224 pixels (vertical/portrait orientation) |
| Refresh Rate | 60.606061 Hz |
| Horizontal Sync | 15.7 kHz |
| Frame Time | ~16.5 ms (1/60 second) |

### Sprite System
| Property | Value |
|----------|-------|
| Hardware Sprites | 64 maximum |
| Sprite Size | 16 x 16 pixels |
| Sprite Memory | 256 bytes max for full screen update |

---

## 3. Graphics & Visual Design

### Color Palette
- **Format**: BB GGG RRR (1 byte per color: 2 bits blue, 3 bits green, 3 bits red)
- **Palette Size**: 16 colors for sprites + 16 colors for tiles (separate palettes)
- **Max Simultaneous**: 16 colors on screen at once from 32-color total palette

#### Key Colors
| Color | Hex (approx) | Usage |
|-------|-------------|-------|
| White | #DEDEDE | Player ship, text |
| Red | #FF0000 | Butterfly wings, captured ship indicator |
| Yellow | #FFFF00 | Bees, score text |
| Cyan | #00FFDE | UI elements |
| Blue | #0000DE | Tractor beam, Boss Galaga |
| Green | #00FF00 | Boss Galaga body |
| Purple | — | Boss Galaga after first hit |
| Black | #000000 | Background |

### Tilemap System
- **Tile Size**: 8 x 8 pixels
- **Screen Grid**: 28 x 36 character tiles
- **Tile Memory**: 16 bytes per tile, 2 bits per pixel
- **Character ROM** (GFX1): Contains all tilemap definitions for UI text, score, and static elements

### Rendering Pipeline (back to front)
1. **Starfield Layer** — Individual pixel-rendered stars via custom starfield generator chip (05xx @ 4M)
2. **Tilemap Layer** — 8x8 character tiles from ROM (scores, text, UI)
3. **Sprite Layer** — Up to 64 hardware sprites (enemies, player, bullets, effects)

---

## 4. Sprite Details

### Enemy Sprites
All enemy sprites are **16x16 pixels**. Each enemy type has multiple animation frames:

- **Boss Galaga**: Green body sprite + purple "damaged" variant (after first hit). Wing flutter animation.
- **Butterflies (Goei)**: White body with red wings, 2 blue stripes across midsection. Wing flutter animation during formation and dive.
- **Bees (Zako)**: Blue and yellow coloring. Simpler animation with wing movement.
- **Splitter Enemies**: Scorpions (yellow), Stingrays (green), Galaxian Flagships — each with unique sprite designs.

### Player Ship
- **Dimensions**: 16 tall x 15 wide (16x16 sprite with 1 blank pixel column)
- **Color**: White normally, red when captured by tractor beam
- **Dual Fighter**: Two ships side by side, moving as one unit

### Animations
- **Formation Breathing**: Entire formation expands and contracts rhythmically like a living organism
- **Dive Animation**: Enemies animate wing flutter while executing dive paths
- **Tractor Beam**: Fan-shaped blue energy field emanating from Boss Galaga, animated beam effect
- **Explosions**: Sprite-based explosion animation with particle/debris effects
- **Bullet Sprites**: 16x16 pixel sprites (sized to reduce sprite count usage)

### Stage Indicators
- Flag/emblem system displayed at bottom-right of screen
- Different flag designs represent milestone stages: 1, 5, 10, 20, 30, 50

---

## 5. Screen Layout

```
+--224px wide--+
|  1UP  HIGH   |  <- Score display (top)
|  SCORE SCORE |
|              |
|  [FORMATION] |  <- Enemy formation area (upper 40% of playfield)
|   4 Bosses   |
|  16 Buttrfly |
|   20 Bees    |
|              |
|              |  <- Open playfield (dive/combat zone)
|              |
|              |
|   [PLAYER]   |  <- Player movement zone (bottom)
| Lives  Flags |  <- Lives (left), Stage flags (right)
+--------------+
       288px tall (portrait)
```

- **Formation**: Centered horizontally in upper portion of screen
- **Player Zone**: Restricted to bottom horizontal strip
- **Score**: Top of screen, "1UP" left, "HIGH SCORE" center/right
- **Lives**: Bottom-left, shown as small ship icons
- **Stage Flags**: Bottom-right, flag emblems indicating progress

---

## 6. Audio & Sound

### WSG Sound Chip
| Property | Value |
|----------|-------|
| Voices | 3 independent monophonic channels |
| Waveforms | 8 pre-defined waveforms per voice |
| Voice 1 Frequency | 20-bit control |
| Voice 2 & 3 Frequency | 16-bit control each |
| Volume Levels | 16 per voice |
| Waveform Storage | 256-byte PROM (8 waveforms x 32 samples x 4-bit) |
| Sampling Rate | 96 kHz |
| Output | Mono |

### Sound Effects (~24 unique sounds)
- Shooting / fire
- Enemy explosion
- Player ship explosion
- Tractor beam activation (distinctive warbling sound)
- Dive attack swoosh
- Ship capture confirmation
- Ship rescue / dual fighter join
- Bonus text popup
- Stage clear

### Music Themes
- **Game Start Jingle**: Upbeat synthesized opening fanfare
- **Stage Start**: Brief melody before each wave begins
- **Challenging Stage**: Special music during bonus rounds
- **Fighter Captured**: Distinctive jingle when tractor beam captures ship
- **Perfect Bonus**: Fanfare for destroying all 40 enemies in challenging stage
- **Game Over**: Ending theme

### Technical Note
Simple music melodies were sped up ~10x to create many of the distinctive sound effects. Audio processing runs on a dedicated CPU separate from game logic.

---

## 7. Controls

### Arcade Cabinet
| Control | Function |
|---------|----------|
| Joystick | Horizontal (left/right) movement only |
| Fire Button | 1 button, shoots upward |
| Orientation | Galaga popularized the horizontal joystick layout |

### Firing Mechanics
- **Max bullets on screen**: 2 simultaneously (major improvement over Galaxian's 1-bullet limit)
- **Dual fighter**: 4 bullets on screen simultaneously (2 per ship)
- Rapid fire possible due to 2-bullet allowance
- Bullets travel vertically upward at fixed speed

### Movement
- Player ship moves left/right only along the bottom of the screen
- Ship cannot move vertically
- Movement is faster and more responsive than Galaxian predecessor

---

## 8. Player Ship

### Single Fighter
| Property | Detail |
|----------|--------|
| Max Bullets | 2 on screen |
| Hitbox | Standard (16x16 sprite area) |
| Vulnerability | Lower |
| Color | White |

### Dual Fighter
| Property | Detail |
|----------|--------|
| Max Bullets | 4 on screen (2 per ship) |
| Hitbox | Approximately doubled width |
| Vulnerability | Significantly higher |
| Color | White (both ships) |
| Movement | Both ships move together as one unit |

### Lives System
- Start with **3 lives**
- **First extra life**: 20,000 points
- **Subsequent extra lives**: Every 70,000 points (at 20K, 90K, 160K, 230K, etc.)
- Life lost when: destroyed by enemy fire, collision with enemy, or ship captured by tractor beam

---

## 9. Enemy Types

### Boss Galaga (Commander)
| Property | Detail |
|----------|--------|
| Appearance | Green fly-like alien; turns purple after first hit |
| HP | 2 hits to destroy |
| Formation Position | Top row, 4 per stage |
| Points (formation) | 150 |
| Points (solo flight) | 400 |
| Points (1 escort) | 800 |
| Points (2 escorts) | 1,600 |
| Special Ability | Tractor beam to capture player's ship |

### Butterfly (Goei)
| Property | Detail |
|----------|--------|
| Appearance | White body, red wings, 2 blue stripes across midsection |
| HP | 1 hit |
| Formation Position | 2 rows of 8, directly below Boss row |
| Points (formation) | 80 |
| Points (flight) | 160 |
| Behavior | Quick left-right steering while diving; harder to hit than Bees |
| Role | Top row serves as Boss Galaga escorts during dives |

### Bee (Zako)
| Property | Detail |
|----------|--------|
| Appearance | Blue and yellow bug ship |
| HP | 1 hit |
| Formation Position | Bottom 2 rows, 10 per row |
| Points (formation) | 50 |
| Points (flight) | 100 |
| Behavior | Straightforward dive-bomb attacks; may loop back upward to attack from behind |
| Special | When only 6 remain, activate wide sweeping arc dive pattern |

### Splitter / Transform Enemies (Stage 4+)
One Bee (or Butterfly if no Bees exist) transforms into 3 special enemies once per stage:

| Enemy | Stages | Appearance | Bonus (all 3 killed) |
|-------|--------|------------|---------------------|
| Sasori (Scorpion) | 4-6 | Yellow | 1,000 pts |
| Midori (Stingray) | 8-10 | Green (resembles Bosconian Spy Ship) | 2,000 pts |
| Galaxian Flagship | 12-14 | Classic Galaxian design | 3,000 pts |

- Cycle repeats: Scorpions -> Stingrays -> Flagships
- Splitters dive while firing, make one final loop, and exit screen
- They do NOT return to formation from the top (unlike regular enemies)

---

## 10. Formation Structure

### Standard Formation Layout (40 enemies total)
```
Row 1:    [B] [B] [B] [B]                    = 4 Boss Galaga
Row 2:  [F] [F] [F] [F] [F] [F] [F] [F]     = 8 Butterflies
Row 3:  [F] [F] [F] [F] [F] [F] [F] [F]     = 8 Butterflies
Row 4: [Z] [Z] [Z] [Z] [Z] [Z] [Z] [Z] [Z] [Z] = 10 Bees
Row 5: [Z] [Z] [Z] [Z] [Z] [Z] [Z] [Z] [Z] [Z] = 10 Bees
```

### Formation Behavior
- **Breathing**: Formation pulses/throbs — expands and contracts like a living entity
- **Dynamic**: Enemies continuously break away and return, creating a "hive" effect
- **Gaps**: Destroyed enemies leave gaps; formation does not compress
- **Captured Ships**: Appear alongside the Boss Galaga that captured them in formation

---

## 11. Enemy Entry Patterns

Enemies do not appear instantaneously — they fly onto screen in single-file lines and assemble into formation. This is a key strategic window for the player to shoot enemies during entry.

### Three Repeating Entrance Patterns
| Pattern | Stages | Description |
|---------|--------|-------------|
| Pattern 1 | 1, 5, 9, 13... | Two mirrored processions from sides of screen |
| Pattern 2 | 2, 6, 10, 14... | Enter from one side in double rows |
| Pattern 3 | 3, 7, 11, 15... | Enter from side in one long string |

- Patterns cycle every 3 regular stages (challenging stages have their own patterns)
- Each stage always uses the same entry pattern (allows player memorization)
- Enemies can be shot during entry before they reach formation position
- **Strategy**: Anticipating entry patterns and pre-positioning is one of the most important Galaga skills

---

## 12. Attack Patterns & AI

### Dive-Bombing Attacks
1. Enemy breaks from formation (individually, pairs, or groups)
2. Dives downward with weaving/dodging motion
3. Fires projectiles during dive
4. May loop back upward to attack from behind
5. Exits screen from sides/bottom, then re-enters at top to rejoin formation

### Boss Galaga Attack Modes

#### Standard Bombing Run
- Boss peels from formation and dives
- May bring 0, 1, or 2 Butterfly escorts
- Returns to formation after dive

#### Tractor Beam Run
- **Condition**: Only when 2+ Boss Galagas are on the playfield
- Performs single loop at top of formation
- Slides to mid-screen position
- Stops approximately 2/3 down the screen
- Activates fan-shaped blue tractor beam
- Brings NO escorts during tractor beam run
- Returns to formation with predictable loop pattern

### Butterfly Dive Behavior
- Quick left-right steering (evasive maneuvers)
- More unpredictable path than Bees
- Steers based on player position, veering slightly at dive end
- Harder to target

### Bee Dive Behavior
- More straightforward dive-bomb trajectory
- More likely to loop back upward for second pass
- Easier to predict than Butterflies
- **Six-Bee Threshold**: When only 6 Bees remain, they switch to wide sweeping arc dive patterns

### AI System
- **Pattern-based movement**: Predefined movement functions (not true AI) giving illusion of intelligent behavior
- **Function pointers**: Each enemy assigned movement functions that change between swarm dives and hovering
- **Decision trees**: Series of flags and decision trees determine next action
- **Movement**: Small incremental x/y speed adjustments per frame
- **Behavioral scaling**: Low levels have fewer AI options; higher levels unlock more complex multi-option behaviors

### Group Attack Behaviors
- Bees and Butterflies coordinate simultaneous attacks
- Combinations of solo, pair, and group dives
- Different formations during group attacks vs. solo dives
- Escort formations with Boss Galaga create multi-unit attack sequences
- **Formation depletion**: When few enemies remain, they stop returning to formation and continuously dive-bomb

---

## 13. Dual Fighter System

### Capture Sequence
1. Boss Galaga initiates tractor beam run (requires 2+ Bosses on field)
2. Fan-shaped blue energy field emanates from Boss Galaga
3. Player ship is pulled upward if caught in beam
4. Ship changes from white to red
5. Player loses a life
6. Captured ship joins Boss Galaga at top of formation

### Rescue Mechanics
- **To rescue**: Destroy the Boss Galaga holding your ship **during its dive** (not while in formation)
- **Timing**: Boss must be diving toward the player; captured ship must be near bottom of screen
- **Success**: Rescued ship joins current ship to form Dual Fighter
- **Failure**: If Boss is destroyed while in formation, captured ship turns hostile and attacks player as an enemy

### Dual Fighter Advantages
- Doubled firepower (4 simultaneous bullets)
- Crucial for achieving perfect bonus in Challenging Stages
- Higher score potential from faster enemy elimination

### Dual Fighter Disadvantages
- Significantly larger hitbox / target area
- Much harder to dodge enemy fire and collisions
- **Strategy**: Some players only use Dual Fighter during Challenging Stages and intentionally lose the extra ship before combat stages

---

## 14. Stage / Wave Structure

### Total Stages
- **255 maximum** playable stages (8-bit integer limit)
- After Stage 255: Screen displays "Stage 0" — behavior depends on DIP switch settings (game reset, incorrect play, lockup, or wrong difficulty)

### Stage Types
| Type | Frequency | Description |
|------|-----------|-------------|
| Regular Stage | Most stages | Full combat — destroy all enemies to advance |
| Challenging Stage | Stage 3, then every 4th (3, 7, 11, 15, 19, 23...) | Bonus round — 40 enemies fly in preset patterns, no enemy fire |

### Challenging Stage Details
- 40 aliens fly in preset formations without firing
- **Perfect score**: Destroy all 40 = **10,000 point bonus** + "PERFECT" displayed
- **Partial**: 100 points per alien destroyed (no special bonus)
- Groups of 8 enemies fly in synchronized patterns
- Safe opportunity to accumulate points
- After Stage 31, challenging stage patterns repeat

### Stage Progression Flow
```
Stage 1  -> Regular (Entry Pattern 1)
Stage 2  -> Regular (Entry Pattern 2)
Stage 3  -> CHALLENGING STAGE
Stage 4  -> Regular + First Splitter Enemies (Scorpions)
Stage 5  -> Regular (Entry Pattern 1)
Stage 6  -> Regular (Entry Pattern 2)
Stage 7  -> CHALLENGING STAGE
Stage 8  -> Regular + Splitter Enemies (Stingrays)
...continues cycling...
```

---

## 15. Difficulty Scaling

### Progressive Changes Across Stages
| Aspect | Early Stages | Mid Stages | Late Stages |
|--------|-------------|------------|-------------|
| Enemy Speed | Slow | Moderate | Fast |
| Dive Frequency | Low | Medium | High |
| Projectile Count | Few | More | Many |
| Aggressiveness | Passive | Moderate | Very aggressive |
| Simultaneous Attacks | 1-2 enemies | 2-4 enemies | Multiple groups |
| AI Complexity | Single behavior | Multiple options | Complex multi-option |

### Key Difficulty Milestones
- **Stage 1-3**: Introductory difficulty, learn mechanics
- **Stage 4**: Splitter enemies introduced (Scorpions)
- **Stage 8**: Second splitter type (Stingrays)
- **Stage 12**: Third splitter type (Galaxian Flagships)
- **Stage 31+**: Challenging stages repeat patterns
- **Stage 35+**: Mechanics repeat but maintain accumulated difficulty level
- **Continuous**: Enemy numbers, firing rate, dive speed, and behavioral complexity all increase progressively

### End-Game Behavior
- Remaining enemies cease returning to formation
- Continuous dive-bombing with no pause
- Maximum projectile density
- Multiple simultaneous group attacks

---

## 16. Scoring System

### Complete Point Value Table

| Target | In Formation | In Flight (Solo) | In Flight (1 Escort) | In Flight (2 Escorts) |
|--------|-------------|-------------------|----------------------|----------------------|
| Bee (Zako) | 50 | 100 | — | — |
| Butterfly (Goei) | 80 | 160 | — | — |
| Boss Galaga | 150 | 400 | 800 | 1,600 |

### Splitter Enemy Bonuses
| Enemy | Individual Kill | All 3 Killed |
|-------|----------------|-------------|
| Scorpion | — | 1,000 |
| Stingray | — | 2,000 |
| Galaxian Flagship | — | 3,000 |

### Challenging Stage Scoring
| Result | Points |
|--------|--------|
| All 40 destroyed | 10,000 bonus (+ individual points) |
| Partial destruction | 100 per enemy destroyed |

### Extra Lives
| Milestone | Points |
|-----------|--------|
| First 1-UP | 20,000 |
| Second 1-UP | 90,000 |
| Third 1-UP | 160,000 |
| Pattern | Every 70,000 after first |

### Scoring Strategy
- Diving enemies are worth **2x** formation enemies — let them dive for more points
- Boss Galaga with 2 escorts = **1,600 points** (maximum single-kill value)
- Challenging Stages with Dual Fighter maximize bonus potential
- Formation kills are safest but lowest value

---

## 17. Known Bugs & Tricks

### The "No-Fire Bug" (15-Minute Cheat)
- **What**: After approximately 15 minutes of play, Bees stop firing entirely
- **Cause**: Software bug — shots at X=0 screen coordinate are not freed properly, consuming all 8 bee-bullet slots
- **Trigger**: Occurs during end-game attack patterns when fewer than 7 Bees remain
- **Effect**: Makes the game significantly easier from that point forward
- **Exploitation**: Known speedrun and high-score strategy

### Stage 0 Overflow
- **What**: After completing Stage 255, the stage counter overflows to "Stage 0"
- **Effect**: Depends on DIP switch settings — may cause game reset, incorrect play behavior, lockup, or wrong difficulty settings
- **Cause**: 8-bit integer overflow (255 + 1 = 0)

---

## Sources

- [Galaga - Wikipedia](https://en.wikipedia.org/wiki/Galaga)
- [StrategyWiki - Galaga/Gameplay](https://strategywiki.org/wiki/Galaga/Gameplay)
- [StrategyWiki - Galaga/Walkthrough](https://strategywiki.org/wiki/Galaga/Walkthrough)
- [Computer Archaeology - Galaga](https://computerarcheology.com/Arcade/Galaga/)
- [Computer Archaeology - Galaga Color Palettes](http://computerarcheology.com/Arcade/Galaga/PROMcolors.html)
- [PixelatedArcade - Galaga Technical Specs](https://pixelatedarcade.com/games/galaga/techspecs)
- [Namco Wiki - Galaga](https://namco.fandom.com/wiki/Galaga)
- [Galaga Collection Fandom - Boss Galaga](https://galaga.fandom.com/wiki/Boss_Galaga)
- [Galaga Collection Fandom - Challenging Stage](https://galaga.fandom.com/wiki/Challenging_Stage)
- [PrimeTime Amusements - Getting Good: Galaga](https://primetimeamusements.com/getting-good-galaga/)
- [BitVint - Galaga 1981 Arcade Game](https://bitvint.com/pages/galaga)
- [Arcade Classics - Galaga Overview](https://www.arcadeclassics.net/80s-game-videos/galaga)
- [System16 - Namco Galaga Hardware](https://www.system16.com/hardware.php?id=516)
- [The Spriters Resource - Galaga Sprites](https://www.spriters-resource.com/arcade/galaga/asset/26482/)
- [MAME GitHub - Galaga Driver Source](https://github.com/mamedev/mame/blob/master/src/mame/namco/galaga.cpp)
- [Shmuplations - Galaga 30th Anniversary Developer Interview](https://shmuplations.com/galaga/)
