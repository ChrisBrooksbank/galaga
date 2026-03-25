use bevy::prelude::*;

use crate::assets::{enemy_sprite_index, GameAssets};
use crate::components::{AnimationTimer, Collider, EnemyState, EnemyType, FormationSlot, Health, SplitterBee};
use crate::enemies::entry_patterns::build_entry_path;
use crate::enemies::formation::{enemy_type_for_slot, slot_row_col};
use crate::enemies::splitters::{random_bee_slot, splitter_type_for_stage};
use crate::resources::{Formation, ScoreBoard, WaveController};

/// Core spawn logic: populates the Formation resource and spawns all 40 enemy entities.
///
/// Called both from the `spawn_formation` system (first wave) and the stage-transition
/// system (subsequent waves).
pub fn do_spawn_formation(commands: &mut Commands, formation: &mut Formation, game_assets: &GameAssets, stage: u32) {
    // Derive entry pattern from the current stage (cycles every 3 stages).
    let pattern = ((stage.saturating_sub(1)) % 3) as usize;

    // Designate one Bee as SplitterBee if this stage has a splitter type.
    let splitter_slot = splitter_type_for_stage(stage).map(|_| random_bee_slot());

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

        let mut entity_cmd = commands.spawn((
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
        ));

        // Mark as SplitterBee if this is the designated slot.
        if splitter_slot == Some(slot_index) {
            if let Some(st) = splitter_type_for_stage(stage) {
                entity_cmd.insert(SplitterBee(st));
            }
        }

        let entity = entity_cmd.id();

        formation.slots[slot_index] = Some(entity);
    }
}

/// System wrapper: spawn the first wave of enemies when entering the Playing state.
pub fn spawn_formation(
    mut commands: Commands,
    mut formation: ResMut<Formation>,
    game_assets: Res<GameAssets>,
    score_board: Res<ScoreBoard>,
    mut wave_controller: ResMut<WaveController>,
) {
    do_spawn_formation(&mut commands, &mut formation, &game_assets, score_board.current_stage);
    wave_controller.enemies_ever_spawned = true;
}
