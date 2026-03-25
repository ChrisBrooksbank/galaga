use bevy::prelude::*;

use crate::components::EnemyType;

/// Emitted by the collision system when a player bullet destroys an enemy.
/// Phase 6 will add the score event handler that reads these and updates ScoreBoard.
#[derive(Event)]
pub struct ScoreEvent {
    pub enemy_type: EnemyType,
    pub is_diving: bool,
}
