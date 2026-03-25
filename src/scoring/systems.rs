use bevy::prelude::*;

use crate::components::EnemyType;
use crate::constants::EXTRA_LIFE_INTERVAL;
use crate::resources::ScoreBoard;

/// Emitted by the collision system when a player bullet destroys an enemy.
#[derive(Event)]
pub struct ScoreEvent {
    pub enemy_type: EnemyType,
    pub is_diving: bool,
}

/// Observer that applies the correct point value to ScoreBoard when an enemy is destroyed.
///
/// Point values per spec:
///   Bee:       50 (formation), 100 (diving)
///   Butterfly: 80 (formation), 160 (diving)
///   Boss:     150 (formation), 400 (diving solo) — escort bonuses added in Phase 8
///
/// Also awards extra lives: first at 20,000 points, then every 70,000 after that.
pub fn handle_score_event(
    trigger: On<ScoreEvent>,
    mut score_board: ResMut<ScoreBoard>,
) {
    let event = trigger.event();
    let points: u32 = match (event.enemy_type, event.is_diving) {
        (EnemyType::Bee, false) => 50,
        (EnemyType::Bee, true) => 100,
        (EnemyType::Butterfly, false) => 80,
        (EnemyType::Butterfly, true) => 160,
        (EnemyType::Boss, false) => 150,
        (EnemyType::Boss, true) => 400,
    };
    score_board.score += points;
    if score_board.score > score_board.high_score {
        score_board.high_score = score_board.score;
    }

    // Award extra lives for crossing thresholds (first at 20k, then every 70k).
    // next_extra_life starts at 20k and advances by 70k each time it is crossed.
    while score_board.score >= score_board.next_extra_life {
        score_board.lives = score_board.lives.saturating_add(1);
        score_board.next_extra_life += EXTRA_LIFE_INTERVAL;
    }
}
