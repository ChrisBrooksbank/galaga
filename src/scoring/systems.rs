use bevy::prelude::*;

use crate::components::EnemyType;
use crate::constants::{
    BEE_SCORE_DIVING, BEE_SCORE_FORMATION, BOSS_SCORE_DIVING_SOLO, BOSS_SCORE_FORMATION,
    BUTTERFLY_SCORE_DIVING, BUTTERFLY_SCORE_FORMATION, EXTRA_LIFE_INTERVAL,
};
use crate::resources::ScoreBoard;
use crate::GameAudioEvent;

/// Emitted by the collision system when a player bullet destroys an enemy.
#[derive(Event)]
pub struct ScoreEvent {
    pub enemy_type: EnemyType,
    pub is_diving: bool,
}

/// Add `points` to the score, update the high score, and award any extra
/// lives crossed (first at 20,000, then every 70,000 after that).
///
/// Every source of points (enemy kills, splitter bonuses, the challenging
/// stage perfect bonus) must go through here so bonus points also count
/// toward extra lives.  Returns the number of lives awarded.
pub fn add_points(score_board: &mut ScoreBoard, points: u32) -> u32 {
    score_board.score = score_board.score.saturating_add(points);
    if score_board.score > score_board.high_score {
        score_board.high_score = score_board.score;
    }

    // next_extra_life starts at 20k and advances by 70k each time it is crossed.
    let mut lives_awarded = 0;
    while score_board.score >= score_board.next_extra_life {
        score_board.lives = score_board.lives.saturating_add(1);
        score_board.next_extra_life = score_board.next_extra_life.saturating_add(EXTRA_LIFE_INTERVAL);
        lives_awarded += 1;
    }
    lives_awarded
}

/// Observer that applies the correct point value to ScoreBoard when an enemy is destroyed.
///
/// Point values per spec:
///   Bee:       50 (formation), 100 (diving)
///   Butterfly: 80 (formation), 160 (diving)
///   Boss:     150 (formation), 400 (diving solo)
pub fn handle_score_event(
    trigger: On<ScoreEvent>,
    mut commands: Commands,
    mut score_board: ResMut<ScoreBoard>,
) {
    let event = trigger.event();
    let points: u32 = match (event.enemy_type, event.is_diving) {
        (EnemyType::Bee, false) => BEE_SCORE_FORMATION,
        (EnemyType::Bee, true) => BEE_SCORE_DIVING,
        (EnemyType::Butterfly, false) => BUTTERFLY_SCORE_FORMATION,
        (EnemyType::Butterfly, true) => BUTTERFLY_SCORE_DIVING,
        (EnemyType::Boss, false) => BOSS_SCORE_FORMATION,
        (EnemyType::Boss, true) => BOSS_SCORE_DIVING_SOLO,
    };
    if add_points(&mut score_board, points) > 0 {
        commands.trigger(GameAudioEvent::ExtraLife);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::FIRST_EXTRA_LIFE_SCORE;

    fn board(score: u32) -> ScoreBoard {
        ScoreBoard { score, lives: 3, ..ScoreBoard::default() }
    }

    #[test]
    fn bonus_points_award_extra_life() {
        let mut sb = board(FIRST_EXTRA_LIFE_SCORE - 100);
        assert_eq!(add_points(&mut sb, 10_000), 1);
        assert_eq!(sb.lives, 4);
        assert_eq!(sb.next_extra_life, FIRST_EXTRA_LIFE_SCORE + EXTRA_LIFE_INTERVAL);
        assert_eq!(sb.high_score, sb.score);
    }

    #[test]
    fn no_life_below_threshold() {
        let mut sb = board(0);
        assert_eq!(add_points(&mut sb, 50), 0);
        assert_eq!(sb.lives, 3);
    }
}
