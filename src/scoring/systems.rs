use bevy::prelude::*;

use crate::components::EnemyType;
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
}
