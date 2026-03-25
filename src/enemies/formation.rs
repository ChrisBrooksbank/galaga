// Formation grid layout, slot positions, and query helpers.
//
// Slot index mapping (40 total):
//   0– 3  Boss       row 0, cols 0–3  (4 enemies)
//   4–11  Butterfly  row 1, cols 0–7  (8 enemies)
//  12–19  Butterfly  row 2, cols 0–7  (8 enemies)
//  20–29  Bee        row 3, cols 0–9  (10 enemies)
//  30–39  Bee        row 4, cols 0–9  (10 enemies)

use bevy::prelude::*;

use crate::components::{AnimationTimer, EnemyState, EnemyType, FormationSlot};
use crate::constants::FORMATION_BREATHING_FREQUENCY;
use crate::resources::Formation;

/// Convert (row, col) to a flat slot index.
///
/// Panics in debug if the row/col pair is out of range.
pub fn slot_index(row: u8, col: u8) -> usize {
    match row {
        0 => {
            debug_assert!((col as usize) < 4, "Boss row has only 4 cols");
            col as usize
        }
        1 => {
            debug_assert!((col as usize) < 8, "Butterfly row 1 has only 8 cols");
            4 + col as usize
        }
        2 => {
            debug_assert!((col as usize) < 8, "Butterfly row 2 has only 8 cols");
            12 + col as usize
        }
        3 => {
            debug_assert!((col as usize) < 10, "Bee row 3 has only 10 cols");
            20 + col as usize
        }
        4 => {
            debug_assert!((col as usize) < 10, "Bee row 4 has only 10 cols");
            30 + col as usize
        }
        _ => panic!("Invalid formation row {row}"),
    }
}

/// Convert a flat slot index back to (row, col).
pub fn slot_row_col(index: usize) -> (u8, u8) {
    match index {
        0..=3 => (0, index as u8),
        4..=11 => (1, (index - 4) as u8),
        12..=19 => (2, (index - 12) as u8),
        20..=29 => (3, (index - 20) as u8),
        30..=39 => (4, (index - 30) as u8),
        _ => panic!("Invalid slot index {index}"),
    }
}

/// Return the enemy type that occupies a given slot index.
pub fn enemy_type_for_slot(index: usize) -> EnemyType {
    match index {
        0..=3 => EnemyType::Boss,
        4..=19 => EnemyType::Butterfly,
        20..=39 => EnemyType::Bee,
        _ => panic!("Invalid slot index {index}"),
    }
}

/// Number of columns per row.
pub fn cols_for_row(row: u8) -> u8 {
    match row {
        0 => 4,
        1 | 2 => 8,
        3 | 4 => 10,
        _ => panic!("Invalid formation row {row}"),
    }
}

/// Advance the formation breathing phase each frame.
pub fn update_formation_breathing(mut formation: ResMut<Formation>, time: Res<Time>) {
    formation.breathing_phase =
        (formation.breathing_phase + FORMATION_BREATHING_FREQUENCY * time.delta_secs()).fract();
}

/// Apply the current breathing offset to all enemies sitting in the formation.
pub fn apply_formation_breathing(
    formation: Res<Formation>,
    mut query: Query<(&FormationSlot, &EnemyState, &mut Transform)>,
) {
    for (slot, state, mut transform) in &mut query {
        if *state == EnemyState::InFormation {
            let idx = slot_index(slot.row, slot.col);
            let pos = formation.current_pos(idx);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

/// Animate enemy wing flutter: toggle between 2 atlas frames per enemy.
///
/// Frame pairs are contiguous (differ only in the lowest bit), so XOR-ing the
/// current index with 1 correctly cycles within each pair:
///   Boss green:   0 ↔ 1  (BOSS_GREEN_1 / BOSS_GREEN_2)
///   Boss purple:  2 ↔ 3  (BOSS_PURPLE_1 / BOSS_PURPLE_2)
///   Butterfly:    4 ↔ 5
///   Bee:          6 ↔ 7
pub fn animate_enemy_wings(
    time: Res<Time>,
    mut query: Query<(&EnemyType, &mut AnimationTimer, &mut Sprite)>,
) {
    for (_enemy_type, mut anim_timer, mut sprite) in &mut query {
        anim_timer.0.tick(time.delta());
        if anim_timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index ^= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_index_roundtrip() {
        for index in 0..40usize {
            let (row, col) = slot_row_col(index);
            assert_eq!(slot_index(row, col), index, "roundtrip failed for index {index}");
        }
    }

    #[test]
    fn enemy_types_correct() {
        for i in 0..4 {
            assert_eq!(enemy_type_for_slot(i), EnemyType::Boss);
        }
        for i in 4..20 {
            assert_eq!(enemy_type_for_slot(i), EnemyType::Butterfly);
        }
        for i in 20..40 {
            assert_eq!(enemy_type_for_slot(i), EnemyType::Bee);
        }
    }

    #[test]
    fn slot_counts_sum_to_40() {
        // Boss: 4, Butterfly: 16, Bee: 20
        let boss_count = (0..40).filter(|&i| matches!(enemy_type_for_slot(i), EnemyType::Boss)).count();
        let butterfly_count = (0..40).filter(|&i| matches!(enemy_type_for_slot(i), EnemyType::Butterfly)).count();
        let bee_count = (0..40).filter(|&i| matches!(enemy_type_for_slot(i), EnemyType::Bee)).count();
        assert_eq!(boss_count, 4);
        assert_eq!(butterfly_count, 16);
        assert_eq!(bee_count, 20);
    }
}
