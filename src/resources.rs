use bevy::prelude::*;

use crate::constants::{
    FORMATION_BREATHING_AMPLITUDE, FORMATION_SLOT_SIZE, FORMATION_TOP_Y,
};

#[derive(Resource)]
pub struct ScoreBoard {
    pub score: u32,
    pub high_score: u32,
    pub lives: u8,
    pub current_stage: u32,
    /// Score threshold at which the next extra life is awarded.
    pub next_extra_life: u32,
}

impl Default for ScoreBoard {
    fn default() -> Self {
        Self {
            score: 0,
            high_score: 0,
            lives: 0,
            current_stage: 0,
            next_extra_life: crate::constants::FIRST_EXTRA_LIFE_SCORE,
        }
    }
}

#[derive(Resource)]
pub struct Formation {
    /// 40-slot grid: indices 0–3 Boss, 4–19 Butterfly, 20–39 Bee
    pub slots: [Option<Entity>; 40],
    /// Home (rest) position of each slot in world-space, computed once at init.
    pub slot_positions: [Vec2; 40],
    /// Current phase of the breathing oscillation (0..TAU).
    pub breathing_phase: f32,
}

impl Formation {
    /// Build a new Formation with all slot home-positions pre-computed.
    ///
    /// Slot layout (40 total):
    ///   0– 3  Boss       row 0, 4 cols
    ///   4–11  Butterfly  row 1, 8 cols
    ///  12–19  Butterfly  row 2, 8 cols
    ///  20–29  Bee        row 3, 10 cols
    ///  30–39  Bee        row 4, 10 cols
    pub fn new() -> Self {
        let mut slot_positions = [Vec2::ZERO; 40];

        // Boss row (row 0): 4 enemies centered
        for col in 0..4usize {
            let x = (col as f32 - 1.5) * FORMATION_SLOT_SIZE;
            let y = FORMATION_TOP_Y;
            slot_positions[col] = Vec2::new(x, y);
        }

        // Butterfly row 1 (row 1): 8 enemies
        for col in 0..8usize {
            let x = (col as f32 - 3.5) * FORMATION_SLOT_SIZE;
            let y = FORMATION_TOP_Y - FORMATION_SLOT_SIZE;
            slot_positions[4 + col] = Vec2::new(x, y);
        }

        // Butterfly row 2 (row 2): 8 enemies
        for col in 0..8usize {
            let x = (col as f32 - 3.5) * FORMATION_SLOT_SIZE;
            let y = FORMATION_TOP_Y - 2.0 * FORMATION_SLOT_SIZE;
            slot_positions[12 + col] = Vec2::new(x, y);
        }

        // Bee row 3 (row 3): 10 enemies
        for col in 0..10usize {
            let x = (col as f32 - 4.5) * FORMATION_SLOT_SIZE;
            let y = FORMATION_TOP_Y - 3.0 * FORMATION_SLOT_SIZE;
            slot_positions[20 + col] = Vec2::new(x, y);
        }

        // Bee row 4 (row 4): 10 enemies
        for col in 0..10usize {
            let x = (col as f32 - 4.5) * FORMATION_SLOT_SIZE;
            let y = FORMATION_TOP_Y - 4.0 * FORMATION_SLOT_SIZE;
            slot_positions[30 + col] = Vec2::new(x, y);
        }

        Self {
            slots: [None; 40],
            slot_positions,
            breathing_phase: 0.0,
        }
    }

    /// World-space position of a slot accounting for the current breathing offset.
    pub fn current_pos(&self, slot_index: usize) -> Vec2 {
        let base = self.slot_positions[slot_index];
        let dx = (self.breathing_phase * std::f32::consts::TAU).sin()
            * FORMATION_BREATHING_AMPLITUDE;
        Vec2::new(base.x + dx, base.y)
    }
}

impl Default for Formation {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Resource, Default)]
pub struct WaveController {
    pub current_wave: u32,
    pub entry_pattern_index: usize,
    pub spawn_timer: Option<Timer>,
    /// Set to true once the first wave has been spawned; guards stage-complete detection.
    pub enemies_ever_spawned: bool,
}

#[derive(Resource)]
pub struct DifficultyConfig {
    pub dive_speed: f32,
    pub fire_interval: f32,
    pub dive_probability: f32,
    pub max_concurrent_divers: u32,
}

impl Default for DifficultyConfig {
    fn default() -> Self {
        Self {
            dive_speed: 80.0,
            fire_interval: 3.0,
            dive_probability: 0.002,
            max_concurrent_divers: 2,
        }
    }
}

#[derive(Resource, Default)]
pub struct DualFighterState {
    pub active: bool,
    pub captured_ship: Option<Entity>,
}

/// Manages the timing and active state of the Boss Galaga tractor beam.
///
/// The system checks preconditions every `CHECK_INTERVAL` seconds.  A run
/// begins when all three are satisfied: 2+ Bosses in formation, no player
/// bullet in flight, and a probabilistic roll passes.
#[derive(Resource)]
pub struct TractorBeamCoordinator {
    /// Countdown until the next eligibility check.
    pub timer: Timer,
    /// True while a tractor beam run is in progress (blocks new runs).
    pub active: bool,
}

impl TractorBeamCoordinator {
    /// Seconds between eligibility checks.
    pub const CHECK_INTERVAL: f32 = 5.0;
    /// Chance (0–1) that a run begins when all preconditions are met.
    pub const TRIGGER_PROBABILITY: f32 = 0.35;
}

impl Default for TractorBeamCoordinator {
    fn default() -> Self {
        Self {
            // First check after 10 s so the formation has time to settle.
            timer: Timer::from_seconds(10.0, TimerMode::Once),
            active: false,
        }
    }
}

/// Tracks state for the current challenging stage.
#[derive(Resource, Default)]
pub struct ChallengingStageData {
    /// True once all challenging-stage enemies have been launched on their paths.
    /// Set by the challenging-stage spawning system (Phase 9).
    pub spawning_done: bool,
    /// Number of enemies destroyed during this challenging stage.
    pub enemies_killed: u32,
    /// Total enemies for this challenging stage (always 40).
    pub total_enemies: u32,
}

/// Controls the timed spawning of enemy groups during a challenging stage.
///
/// Enemies are launched in 5 groups of 8, one group every `GROUP_DELAY_SECS`.
/// The resource is reset in `enter_challenging_stage`.
#[derive(Resource)]
pub struct ChallengingStageSpawner {
    /// Index of the next group to spawn (0–4); 5 means all groups launched.
    pub next_group: usize,
    /// Countdown until the next group is launched.
    pub timer: Timer,
}

impl ChallengingStageSpawner {
    /// Seconds between consecutive group launches.
    pub const GROUP_DELAY_SECS: f32 = 2.5;
    /// Total number of groups (5 groups × 8 enemies = 40 total).
    pub const GROUP_COUNT: usize = 5;
}

impl Default for ChallengingStageSpawner {
    fn default() -> Self {
        Self {
            next_group: 0,
            // First group spawns after a short lead-in.
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

/// Tracks the state of splitter pieces during a stage.
///
/// When a SplitterBee is killed, 3 SplitterPiece entities are spawned.
/// When all 3 are killed, a bonus is awarded.
#[derive(Resource, Default)]
pub struct SplitterState {
    /// Number of SplitterPiece enemies currently alive (reset when SplitterBee is killed).
    pub pieces_alive: u32,
    /// Number of SplitterPiece enemies killed this stage.
    pub pieces_killed: u32,
    /// Whether the all-3-killed bonus has been awarded this stage.
    pub bonus_awarded: bool,
}

/// Coordinates timed group dive attacks for Bee and Butterfly squads.
///
/// A group attack launches 1–4 enemies of the same type simultaneously,
/// creating the classic Galaga wave-attack feel.  The coordinator runs on its
/// own timer so it is independent of the per-frame `dive_decision_system`.
#[derive(Resource)]
pub struct GroupAttackCoordinator {
    /// Countdown until the next group attack is triggered.
    pub timer: Timer,
    /// How many enemies to send in the next wave (1 = solo, 2 = pair, 3–4 = group).
    pub next_attack_size: usize,
}

impl Default for GroupAttackCoordinator {
    fn default() -> Self {
        Self {
            // First attack after a short delay to let formation settle.
            timer: Timer::from_seconds(3.0, TimerMode::Once),
            next_attack_size: 2,
        }
    }
}
