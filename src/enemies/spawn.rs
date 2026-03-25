use bevy::prelude::*;

use crate::assets::{enemy_sprite_index, GameAssets};
use crate::components::{AnimationTimer, Collider, EnemyState, EnemyType, FormationSlot, Health};
use crate::enemies::formation::{enemy_type_for_slot, slot_row_col};
use crate::resources::Formation;

/// Spawn all 40 enemies directly at their formation home positions.
///
/// Enemies start in `InFormation` state. Entry animation (a later task) will
/// move them into a `Forming` state and fly them in from the screen edges.
pub fn spawn_formation(
    mut commands: Commands,
    mut formation: ResMut<Formation>,
    game_assets: Res<GameAssets>,
) {
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

        // Stagger the animation phase so enemies don't all flutter in sync.
        // Each slot starts at a different elapsed offset (steps of ~37ms).
        let flutter_period = 0.25_f32;
        let phase_offset = (slot_index as f32 * 0.037) % flutter_period;
        let mut anim_timer =
            AnimationTimer(Timer::from_seconds(flutter_period, TimerMode::Repeating));
        anim_timer.0.set_elapsed(std::time::Duration::from_secs_f32(phase_offset));

        let entity = commands
            .spawn((
                Sprite::from_atlas_image(
                    game_assets.enemies_image.clone(),
                    TextureAtlas {
                        layout: game_assets.enemies_layout.clone(),
                        index: sprite_index,
                    },
                ),
                Transform::from_xyz(home_pos.x, home_pos.y, 0.0),
                enemy_type,
                EnemyState::InFormation,
                FormationSlot { row, col, home_pos },
                Health(health),
                Collider { half_size: Vec2::splat(7.0) },
                anim_timer,
            ))
            .id();

        formation.slots[slot_index] = Some(entity);
    }
}
