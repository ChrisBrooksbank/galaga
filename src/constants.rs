// Window
pub const LOGICAL_WIDTH: f32 = 224.0;
pub const LOGICAL_HEIGHT: f32 = 288.0;
pub const WINDOW_SCALE: u32 = 2;

// Player
pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_BULLET_SPEED: f32 = 200.0;
pub const MAX_PLAYER_BULLETS: usize = 2;
pub const PLAYER_FIRE_COOLDOWN_SECS: f32 = 0.2;
pub const PLAYER_START_LIVES: u8 = 3;

// Enemy bullet
pub const ENEMY_BULLET_SPEED: f32 = 120.0;

// Formation layout
pub const FORMATION_ROWS: usize = 5;
pub const FORMATION_COLS: usize = 8; // max per row (Bee row)
pub const FORMATION_SLOT_SIZE: f32 = 16.0;
pub const FORMATION_TOP_Y: f32 = 80.0;
pub const FORMATION_BREATHING_AMPLITUDE: f32 = 2.0;
pub const FORMATION_BREATHING_FREQUENCY: f32 = 1.0; // Hz

// Enemy counts
pub const BOSS_COUNT: usize = 4;
pub const BUTTERFLY_COUNT: usize = 16;
pub const BEE_COUNT: usize = 20;
pub const TOTAL_ENEMIES: usize = BOSS_COUNT + BUTTERFLY_COUNT + BEE_COUNT;

// Scoring — formation
pub const BEE_SCORE_FORMATION: u32 = 50;
pub const BUTTERFLY_SCORE_FORMATION: u32 = 80;
pub const BOSS_SCORE_FORMATION: u32 = 150;

// Scoring — diving
pub const BEE_SCORE_DIVING: u32 = 100;
pub const BUTTERFLY_SCORE_DIVING: u32 = 160;
pub const BOSS_SCORE_DIVING_SOLO: u32 = 400;
pub const BOSS_SCORE_DIVING_ONE_ESCORT: u32 = 800;
pub const BOSS_SCORE_DIVING_TWO_ESCORTS: u32 = 1600;

// Extra lives
pub const FIRST_EXTRA_LIFE_SCORE: u32 = 20_000;
pub const EXTRA_LIFE_INTERVAL: u32 = 70_000;

// Challenging stages occur at: 3, 7, 11, 15, ...
pub const FIRST_CHALLENGING_STAGE: u32 = 3;
pub const CHALLENGING_STAGE_INTERVAL: u32 = 4;
pub const PERFECT_BONUS_SCORE: u32 = 10_000;

// Difficulty scaling
pub const BASE_DIVE_SPEED: f32 = 80.0;
pub const SPEED_INCREMENT: f32 = 2.0;
pub const BASE_FIRE_INTERVAL: f32 = 3.0;
pub const MIN_FIRE_INTERVAL: f32 = 0.5;
pub const RATE_DECREMENT: f32 = 0.05;
pub const BASE_DIVE_PROB: f32 = 0.002;
pub const MAX_DIVE_PROB: f32 = 0.02;
pub const PROB_INCREMENT: f32 = 0.0002;
pub const BASE_DIVERS: u32 = 2;
pub const MAX_DIVERS: u32 = 6;
pub const DIVER_STEP: u32 = 5;

// Dual fighter
pub const MAX_DUAL_BULLETS: usize = 4;
