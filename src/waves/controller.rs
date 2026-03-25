use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::components::EnemyType;
use crate::enemies::spawn::do_spawn_formation;
use crate::resources::{DifficultyConfig, Formation, ScoreBoard, WaveController};
use crate::waves::difficulty::update_difficulty_for_stage;

const STAGE_CLEAR_DELAY_SECS: f32 = 2.0;

/// Delay between the last enemy dying and the next wave spawning.
#[derive(Resource, Default)]
pub struct StageClearTimer(pub Option<Timer>);

/// System: detect when all enemies are destroyed and start the stage-clear countdown.
///
/// The `WaveController.enemies_ever_spawned` guard prevents this from triggering at
/// game startup, before the first wave has populated the field.
pub fn check_stage_complete(
    enemy_query: Query<(), With<EnemyType>>,
    wave_controller: Res<WaveController>,
    mut stage_clear_timer: ResMut<StageClearTimer>,
) {
    // Skip if a transition is already in progress.
    if stage_clear_timer.0.is_some() {
        return;
    }
    // Skip if enemies have never been spawned yet (start of game).
    if !wave_controller.enemies_ever_spawned {
        return;
    }
    if enemy_query.is_empty() {
        stage_clear_timer.0 = Some(Timer::from_seconds(STAGE_CLEAR_DELAY_SECS, TimerMode::Once));
    }
}

/// System: tick the stage-clear timer; when it fires, advance the stage and spawn the next wave.
pub fn tick_stage_transition(
    mut commands: Commands,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    mut stage_clear_timer: ResMut<StageClearTimer>,
    mut score_board: ResMut<ScoreBoard>,
    mut formation: ResMut<Formation>,
    mut difficulty: ResMut<DifficultyConfig>,
    mut wave_controller: ResMut<WaveController>,
) {
    let Some(ref mut timer) = stage_clear_timer.0 else {
        return;
    };
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }
    stage_clear_timer.0 = None;

    // Advance to the next stage.
    score_board.current_stage += 1;

    // Apply formula-driven difficulty scaling for the new stage.
    update_difficulty_for_stage(&mut difficulty, score_board.current_stage);

    // Reset the formation grid (enemy entities are already despawned by the collision system).
    *formation = Formation::new();

    // Spawn the next wave of enemies.
    do_spawn_formation(&mut commands, &mut formation, &game_assets, score_board.current_stage);
    wave_controller.enemies_ever_spawned = true;
}
