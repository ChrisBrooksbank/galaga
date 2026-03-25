# Enemy AI & Mechanics

## Overview

Enemy dive attacks, AI decision-making, Boss Galaga tractor beam capture, dual fighter system, and splitter enemies.

## User Stories

- As a player, I want enemies to dive-bomb me so that there is combat challenge
- As a player, I want Boss Galaga to capture my ship so that I can attempt the dual fighter rescue
- As a player, I want a dual fighter so that I have enhanced firepower
- As a player, I want splitter enemies to appear in later stages for variety

## Requirements

### Dive Path System
- [ ] Predefined waypoint curves stored as Vec<Vec2> per path type
- [ ] Enemies interpolate between waypoints at speed determined by difficulty
- [ ] Different path sets for: Boss dives, Butterfly swoops, Bee bombs, tractor beam runs
- [ ] Enemies return to formation after completing dive (re-enter from top)

### Dive AI Decisions
- [ ] Enemy dive decision system: determines who dives, when, how many concurrently
- [ ] Dive probability increases with stage number
- [ ] Max concurrent divers increases with stage
- [ ] Bees: straightforward dive-bomb, may loop back for second pass
- [ ] Butterflies: quick left-right steering, evasive, harder to hit
- [ ] Six-Bee threshold: when 6 or fewer Bees remain, switch to wide sweeping arc pattern

### Enemy Firing
- [ ] Enemies fire projectiles during dives
- [ ] Fire rate increases with difficulty/stage
- [ ] Enemy bullets aimed generally toward player position

### Boss Galaga Attacks
- [ ] Standard bombing run: Boss dives with 0, 1, or 2 Butterfly escorts
- [ ] Escort scoring: Boss with 1 escort = 800pts, 2 escorts = 1600pts
- [ ] Boss has 2 HP (turns purple after first hit)

### Tractor Beam System
- [ ] Tractor beam run triggers only when 2+ Boss Galagas on field
- [ ] Boss performs loop, slides to mid-screen, stops ~2/3 down
- [ ] Fan-shaped blue tractor beam activates
- [ ] No escorts during tractor beam run
- [ ] Player caught in beam: ship pulled upward, changes white to red, life lost
- [ ] Captured ship joins Boss in formation

### Dual Fighter Rescue
- [ ] Destroy Boss during dive (not in formation) to rescue captured ship
- [ ] Rescued ship joins current ship to form dual fighter
- [ ] Dual fighter: 4 simultaneous bullets, doubled width hitbox
- [ ] If Boss destroyed in formation: captured ship turns hostile, attacks player
- [ ] DualFighterState resource tracks active status and captured ship entity

### Splitter / Transform Enemies
- [ ] Stage 4+: one Bee transforms into 3 special enemies per stage
- [ ] Scorpions (stages 4-6): yellow, 1000pt bonus for all 3
- [ ] Stingrays (stages 8-10): green, 2000pt bonus for all 3
- [ ] Galaxian Flagships (stages 12-14): classic design, 3000pt bonus for all 3
- [ ] Cycle repeats: Scorpions -> Stingrays -> Flagships
- [ ] Splitters dive while firing, make one loop, exit screen (do NOT return to formation)

### Difficulty Scaling
- [ ] dive_speed = BASE + (stage * INCREMENT)
- [ ] fire_rate = max(MIN_INTERVAL, BASE_INTERVAL - (stage * DECREMENT))
- [ ] dive_probability = min(MAX_PROB, BASE_PROB + (stage * INCREMENT))
- [ ] max_concurrent = min(MAX_DIVERS, BASE_DIVERS + (stage / STEP))
- [ ] DifficultyConfig resource updated each stage

### Group Attacks
- [ ] Bees and Butterflies coordinate simultaneous attacks
- [ ] Solo, pair, and group dive combinations
- [ ] When few enemies remain, they stop returning to formation and continuously dive

### Challenging Stages
- [ ] Occur at stage 3, then every 4th (3, 7, 11, 15, 19, 23...)
- [ ] 40 enemies fly in preset patterns without firing
- [ ] Groups of 8 in synchronized patterns
- [ ] Perfect (all 40 destroyed): 10,000 bonus + "PERFECT" displayed
- [ ] Partial: 100 points per enemy destroyed
- [ ] Patterns repeat after stage 31

## Acceptance Criteria

- [ ] Enemies break from formation and execute dive paths
- [ ] Boss Galaga can capture player ship via tractor beam
- [ ] Dual fighter forms when rescued ship joins player
- [ ] Splitter enemies appear on correct stages
- [ ] Difficulty noticeably increases across stages
- [ ] Challenging stages work with perfect bonus

## Out of Scope

- Visual effects for tractor beam (see visual-polish spec)
- Sound effects for dives and captures (see audio spec)
