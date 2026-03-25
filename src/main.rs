use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;

pub mod assets;
pub mod audio;
pub mod collision;
pub mod components;
pub mod constants;
pub mod effects;
pub mod enemies;
pub mod player;
pub mod resources;
pub mod scoring;
pub mod states;
pub mod ui;
pub mod waves;

use assets::GameAssets;
use collision::{
    bullet_enemy_collision, diving_enemy_dual_fighter_collision, diving_enemy_player_collision,
    enemy_bullet_dual_fighter_collision, enemy_bullet_player_collision,
};
use constants::{FIRST_EXTRA_LIFE_SCORE, PLAYER_START_LIVES};
use enemies::ai::{dive_completion_system, dive_decision_system, dive_movement_system, enemy_fire_system, group_attack_system};
use enemies::tractor_beam::{boss_tractor_decision, player_capture_system, pulse_tractor_beam_system, spawn_tractor_beam_system};
use enemies::entry_patterns::move_forming_enemies;
use enemies::formation::{animate_enemy_wings, apply_formation_breathing, update_formation_breathing};
use enemies::spawn::spawn_formation;
use player::{
    dual_fighter_follow, handle_player_death, manage_dual_fighter, move_bullets, player_movement,
    player_shoot, spawn_player, tick_respawn, RespawnTimer,
};
use resources::{ChallengingStageData, DifficultyConfig, DualFighterState, Formation, GroupAttackCoordinator, ScoreBoard, TractorBeamCoordinator, WaveController};
use scoring::handle_score_event;
use states::GameState;
use waves::{
    challenging_stage_completion, check_stage_complete, enter_challenging_stage,
    tick_stage_transition, StageClearTimer,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Galaga".to_string(),
                resolution: (448u32, 576u32).into(), // 224x288 * 2x scale
                resizable: false,
                present_mode: PresentMode::Fifo, // vsync ~60fps
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .init_state::<GameState>()
        .init_resource::<ScoreBoard>()
        .init_resource::<Formation>()
        .init_resource::<RespawnTimer>()
        .init_resource::<DifficultyConfig>()
        .init_resource::<WaveController>()
        .init_resource::<GroupAttackCoordinator>()
        .init_resource::<StageClearTimer>()
        .init_resource::<TractorBeamCoordinator>()
        .init_resource::<DualFighterState>()
        .init_resource::<ChallengingStageData>()
        .add_observer(handle_score_event)
        // Asset loading: transition Loading → Menu automatically when all assets are ready
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<GameAssets>(),
        )
        .add_systems(Startup, setup_camera)
        // Reset game state, spawn player and formation when entering Playing
        .add_systems(
            OnEnter(GameState::Playing),
            (init_scoreboard, reset_wave_state, spawn_player, spawn_formation).chain(),
        )
        // Menu → Playing on Space
        .add_systems(Update, menu_to_playing.run_if(in_state(GameState::Menu)))
        // Playing → Paused on Escape
        .add_systems(Update, toggle_pause.run_if(in_state(GameState::Playing)))
        // Paused → Playing on Escape
        .add_systems(Update, toggle_pause.run_if(in_state(GameState::Paused)))
        // Player movement and shooting (only while Playing)
        .add_systems(
            Update,
            (player_movement, player_shoot, move_bullets).run_if(in_state(GameState::Playing)),
        )
        // Dual fighter: manage secondary ship and keep it aligned with the player
        .add_systems(
            Update,
            (manage_dual_fighter, dual_fighter_follow)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        // Collision detection (only while Playing)
        .add_systems(
            Update,
            (
                bullet_enemy_collision,
                enemy_bullet_player_collision,
                diving_enemy_player_collision,
                enemy_bullet_dual_fighter_collision,
                diving_enemy_dual_fighter_collision,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Player death and respawn (only while Playing)
        .add_systems(
            Update,
            (handle_player_death, tick_respawn).run_if(in_state(GameState::Playing)),
        )
        // Stage progression (only while Playing)
        .add_systems(
            Update,
            (check_stage_complete, tick_stage_transition)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        // Formation breathing animation (only while Playing)
        .add_systems(
            Update,
            (update_formation_breathing, apply_formation_breathing)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        // Entry animation: fly forming enemies to their formation slots.
        .add_systems(
            Update,
            move_forming_enemies.run_if(in_state(GameState::Playing)),
        )
        // Enemy wing-flutter animation (only while Playing)
        .add_systems(
            Update,
            animate_enemy_wings.run_if(in_state(GameState::Playing)),
        )
        // Dive state machine: select divers, move them, detect path completion (only while Playing)
        //
        // `apply_deferred` is inserted between the two dive-selection systems so that
        // commands from `dive_decision_system` (which marks an enemy as Diving) are
        // flushed before `group_attack_system` counts current divers.  Without this
        // flush, both systems could see a stale diver count on the same frame and
        // jointly exceed `DifficultyConfig::max_concurrent_divers`.
        .add_systems(
            Update,
            (
                dive_decision_system,
                ApplyDeferred,
                group_attack_system,
                boss_tractor_decision,
                spawn_tractor_beam_system,
                pulse_tractor_beam_system,
                player_capture_system,
                dive_movement_system,
                dive_completion_system,
                enemy_fire_system,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        // Challenging stage: initialise on entry, check for completion each frame
        .add_systems(
            OnEnter(GameState::ChallengingStage),
            enter_challenging_stage,
        )
        .add_systems(
            Update,
            challenging_stage_completion.run_if(in_state(GameState::ChallengingStage)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: 224.0, height: 288.0 },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn menu_to_playing(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn init_scoreboard(mut score_board: ResMut<ScoreBoard>) {
    score_board.score = 0;
    score_board.lives = PLAYER_START_LIVES;
    score_board.current_stage = 1;
    score_board.next_extra_life = FIRST_EXTRA_LIFE_SCORE;
    // high_score persists across games intentionally
}

fn reset_wave_state(
    mut wave_controller: ResMut<WaveController>,
    mut stage_clear_timer: ResMut<StageClearTimer>,
    mut difficulty: ResMut<DifficultyConfig>,
    mut group_coordinator: ResMut<GroupAttackCoordinator>,
    mut tractor_beam: ResMut<TractorBeamCoordinator>,
    mut dual_fighter: ResMut<DualFighterState>,
    mut challenging_stage_data: ResMut<ChallengingStageData>,
) {
    *wave_controller = WaveController::default();
    *stage_clear_timer = StageClearTimer::default();
    *group_coordinator = GroupAttackCoordinator::default();
    *tractor_beam = TractorBeamCoordinator::default();
    *dual_fighter = DualFighterState::default();
    *challenging_stage_data = ChallengingStageData::default();
    waves::difficulty::update_difficulty_for_stage(&mut difficulty, 1);
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}
