use bevy::prelude::*;

use crate::assets::{enemy_sprite_index, GameAssets};
use crate::components::{AnimationTimer, Collider, EnemyState, EnemyType, FormationSlot, Health};
use crate::enemies::entry_patterns::build_entry_path;
use crate::enemies::formation::{enemy_type_for_slot, slot_row_col};
use crate::resources::{Formation, ScoreBoard};

/// Spawn all 40 enemies off-screen and assign them entry-animation paths.
///
/// Each enemy begins in `EnemyState::Forming` and carries an `EntryPath`
/// that guides it to its formation slot.  The `move_forming_enemies` system
/// (entry_patterns module) drives the motion; once an enemy reaches its home
/// position it transitions to `EnemyState::InFormation`.
pub fn spawn_formation(
    mut commands: Commands,
    mut formation: ResMut<Formation>,
    game_assets: Res<GameAssets>,
    score_board: Res<ScoreBoard>,
) {
    // Derive entry pattern from the current stage (cycles every 3 stages).
    let pattern = ((score_board.current_stage.saturating_sub(1)) % 3) as usize;

    for slot_index in 0..40usize {
        let enemy_type = enemy_type_for_slot(slot_index);
        let (row, col) = slot_row_col(slot_index);
        let home_pos = formation.slot_positions[slot_index];

        let sprite_index = match enemy_type {
            EnemyType::Boss => enemy_sprite_index::BOSS_GREEN_1,
            EnemyType::Butterfly => enemy_sprite_index::BUTTERFLY_1,
            EnemyType::Bee => enemy_sprite_index::BEE_1,
        };

        let health = match enemy_type {
            EnemyType::Boss => 2,
            EnemyType::Butterfly | EnemyType::Bee => 1,
        };

        // Stagger animation phase so enemies don't all flutter in sync.
        let flutter_period = 0.25_f32;
        let phase_offset = (slot_index as f32 * 0.037) % flutter_period;
        let mut anim_timer =
            AnimationTimer(Timer::from_seconds(flutter_period, TimerMode::Repeating));
        anim_timer.0.set_elapsed(std::time::Duration::from_secs_f32(phase_offset));

        // Build the entry path; waypoints[0] is the off-screen spawn position.
        let entry_path = build_entry_path(slot_index, home_pos, pattern);
        let start_pos = entry_path.waypoints[0];

        let entity = commands
            .spawn((
                Sprite::from_atlas_image(
                    game_assets.enemies_image.clone(),
                    TextureAtlas {
                        layout: game_assets.enemies_layout.clone(),
                        index: sprite_index,
                    },
                ),
                Transform::from_xyz(start_pos.x, start_pos.y, 0.0),
                enemy_type,
                EnemyState::Forming,
                FormationSlot { row, col, home_pos },
                Health(health),
                Collider { half_size: Vec2::splat(7.0) },
                anim_timer,
                entry_path,
            ))
            .id();

        formation.slots[slot_index] = Some(entity);
    }
}
