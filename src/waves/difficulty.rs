use crate::constants::{
    BASE_DIVE_PROB, BASE_DIVE_SPEED, BASE_DIVERS, BASE_FIRE_INTERVAL, DIVER_STEP, MAX_DIVE_PROB,
    MAX_DIVERS, MIN_FIRE_INTERVAL, PROB_INCREMENT, RATE_DECREMENT, SPEED_INCREMENT,
};
use crate::resources::DifficultyConfig;

/// Apply formula-driven difficulty scaling for the given stage number.
///
/// Formulas (from enemy-ai spec):
///   dive_speed       = BASE_DIVE_SPEED + stage × SPEED_INCREMENT
///   fire_interval    = max(MIN_FIRE_INTERVAL, BASE_FIRE_INTERVAL − stage × RATE_DECREMENT)
///   dive_probability = min(MAX_DIVE_PROB, BASE_DIVE_PROB + stage × PROB_INCREMENT)
///   max_concurrent   = min(MAX_DIVERS, BASE_DIVERS + stage / DIVER_STEP)
pub fn update_difficulty_for_stage(config: &mut DifficultyConfig, stage: u32) {
    let s = stage as f32;
    config.dive_speed = BASE_DIVE_SPEED + s * SPEED_INCREMENT;
    config.fire_interval = (BASE_FIRE_INTERVAL - s * RATE_DECREMENT).max(MIN_FIRE_INTERVAL);
    config.dive_probability = (BASE_DIVE_PROB + s * PROB_INCREMENT).min(MAX_DIVE_PROB);
    config.max_concurrent_divers = (BASE_DIVERS + stage / DIVER_STEP).min(MAX_DIVERS);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_1_above_baseline() {
        let mut cfg = DifficultyConfig::default();
        update_difficulty_for_stage(&mut cfg, 1);
        assert!(cfg.dive_speed > BASE_DIVE_SPEED);
        assert!(cfg.fire_interval < BASE_FIRE_INTERVAL);
    }

    #[test]
    fn caps_are_respected() {
        let mut cfg = DifficultyConfig::default();
        update_difficulty_for_stage(&mut cfg, 999);
        assert!(cfg.fire_interval >= MIN_FIRE_INTERVAL);
        assert!(cfg.dive_probability <= MAX_DIVE_PROB);
        assert!(cfg.max_concurrent_divers <= MAX_DIVERS);
    }
}
