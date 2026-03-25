use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::components::EnemyType;
use crate::constants::{CHALLENGING_STAGE_INTERVAL, FIRST_CHALLENGING_STAGE, TOTAL_ENEMIES};
use crate::enemies::spawn::do_spawn_formation;
use crate::resources::{ChallengingStageData, DifficultyConfig, Formation, ScoreBoard};
use crate::states::GameState;
use crate::waves::difficulty::update_difficulty_for_stage;

/// Returns true if `stage` is a challenging stage (3, 7, 11, 15, …).
pub fn is_challenging_stage(stage: u32) -> bool {
    if stage < FIRST_CHALLENGING_STAGE {
        return false;
    }
    (stage - FIRST_CHALLENGING_STAGE) % CHALLENGING_STAGE_INTERVAL == 0
}

/// OnEnter(ChallengingStage): reset the per-stage tracker.
pub fn enter_challenging_stage(mut data: ResMut<ChallengingStageData>) {
    *data = ChallengingStageData {
        spawning_done: false,
        enemies_killed: 0,
        total_enemies: TOTAL_ENEMIES as u32,
    };
}

/// Update system: detect challenging-stage completion.
///
/// A challenging stage is complete when `spawning_done` is true and no enemy
/// entities remain on screen.  The spawning system (Phase 9 enemy patterns)
/// sets `spawning_done = true` once all 40 enemies have been launched.
///
/// On completion the stage counter advances, difficulty is updated, and the
/// next normal formation is spawned before returning to `Playing`.
pub fn challenging_stage_completion(
    mut commands: Commands,
    enemy_query: Query<(), With<EnemyType>>,
    data: ResMut<ChallengingStageData>,
    mut score_board: ResMut<ScoreBoard>,
    mut formation: ResMut<Formation>,
    mut difficulty: ResMut<DifficultyConfig>,
    game_assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Wait until spawning is complete before checking for completion.
    if !data.spawning_done {
        return;
    }
    if !enemy_query.is_empty() {
        return;
    }

    // Advance past the challenging stage to the next normal stage.
    score_board.current_stage += 1;
    update_difficulty_for_stage(&mut difficulty, score_board.current_stage);

    // Reset formation and spawn the next normal wave.
    *formation = Formation::new();
    do_spawn_formation(
        &mut commands,
        &mut formation,
        &game_assets,
        score_board.current_stage,
    );

    next_state.set(GameState::Playing);
}
