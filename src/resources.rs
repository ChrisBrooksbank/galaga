use bevy::prelude::*;

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
    pub breathing_phase: f32,
}

impl Default for Formation {
    fn default() -> Self {
        Self {
            slots: [None; 40],
            breathing_phase: 0.0,
        }
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
