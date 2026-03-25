use bevy::prelude::*;

use crate::constants::{
    FORMATION_BREATHING_AMPLITUDE, FORMATION_SLOT_SIZE, FORMATION_TOP_Y,
};

#[derive(Resource, Default)]
pub struct ScoreBoard {
    pub score: u32,
    pub high_score: u32,
    pub lives: u8,
    pub current_stage: u32,
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
