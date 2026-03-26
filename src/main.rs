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
    bullet_challenging_enemy_collision, bullet_enemy_collision,
    diving_enemy_dual_fighter_collision, diving_enemy_player_collision,
    enemy_bullet_dual_fighter_collision, enemy_bullet_player_collision,
};
use enemies::splitters::{
    bullet_splitter_piece_collision, handle_splitter_bee_killed, move_splitter_pieces,
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
use resources::{ChallengingStageData, ChallengingStageSpawner, DifficultyConfig, DualFighterState, Formation, GroupAttackCoordinator, ScoreBoard, SplitterState, TractorBeamCoordinator, WaveController};
use scoring::handle_score_event;
use effects::starfield::{scroll_starfield, spawn_starfield};
use effects::{animate_explosions, init_explosion_sprites, tick_despawn_timers};
use audio::handle_audio_events;
pub use audio::GameAudioEvent;
use states::GameState;
use ui::game_over::{despawn_game_over, game_over_input, spawn_game_over};
use ui::hud::{spawn_hud, update_hud};
use ui::menu::{blink_press_start, despawn_menu, spawn_menu};
use ui::stage_intro::{arm_stage_intro, tick_stage_intro, StageIntroTimer};
use waves::{
    challenging_stage_completion, check_stage_complete, enter_challenging_stage,
    move_challenging_enemies, spawn_challenging_stage_patterns, tick_stage_transition,
    StageClearTimer,
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
        .init_resource::<ChallengingStageSpawner>()
        .init_resource::<SplitterState>()
        .init_resource::<StageIntroTimer>()
        .add_observer(handle_audio_events)
        .add_observer(handle_score_event)
        .add_observer(handle_splitter_bee_killed)
        // Asset loading: transition Loading → Menu automatically when all assets are ready
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<GameAssets>(),
        )
        .add_systems(Startup, (setup_camera, spawn_starfield))
        // Starfield scrolls in all active game states
        .add_systems(
            Update,
            scroll_starfield.run_if(
                in_state(GameState::Menu)
                    .or(in_state(GameState::Playing))
                    .or(in_state(GameState::Paused))
                    .or(in_state(GameState::ChallengingStage))
                    .or(in_state(GameState::GameOver)),
            ),
        )
        // Reset game state, spawn player and formation when entering Playing
        .add_systems(
            OnEnter(GameState::Playing),
            (init_scoreboard, reset_wave_state, spawn_player, spawn_formation, spawn_hud, arm_stage_intro).chain(),
        )
        // HUD update: keep score, lives, and stage in sync
        .add_systems(
            Update,
            update_hud.run_if(
                in_state(GameState::Playing)
                    .or(in_state(GameState::Paused))
                    .or(in_state(GameState::ChallengingStage))
                    .or(in_state(GameState::GameOver)),
            ),
        )
        // Spawn menu UI on entering Menu state; despawn on leaving
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(OnExit(GameState::Menu), despawn_menu)
        // Menu blink and transition
        .add_systems(
            Update,
            (blink_press_start, menu_to_playing).run_if(in_state(GameState::Menu)),
        )
        // Spawn game-over UI on entering GameOver state; despawn on leaving
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
        .add_systems(OnExit(GameState::GameOver), despawn_game_over)
        .add_systems(Update, game_over_input.run_if(in_state(GameState::GameOver)))
        // Playing → Paused on Escape
        .add_systems(Update, toggle_pause.run_if(in_state(GameState::Playing)))
        // Paused → Playing on Escape
        .add_systems(Update, toggle_pause.run_if(in_state(GameState::Paused)))
        // Player movement and shooting (Playing and ChallengingStage)
        .add_systems(
            Update,
            (player_movement, player_shoot, move_bullets)
                .run_if(in_state(GameState::Playing).or(in_state(GameState::ChallengingStage))),
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
                bullet_splitter_piece_collision,
                enemy_bullet_player_collision,
                diving_enemy_player_collision,
                enemy_bullet_dual_fighter_collision,
                diving_enemy_dual_fighter_collision,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Splitter piece movement (only while Playing)
        .add_systems(
            Update,
            move_splitter_pieces.run_if(in_state(GameState::Playing)),
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
        // Stage intro overlay
        .add_systems(
            Update,
            tick_stage_intro.run_if(in_state(GameState::Playing)),
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
        // Explosion animation and despawn timers (all active gameplay states + game over)
        .add_systems(
            Update,
            (init_explosion_sprites, animate_explosions, tick_despawn_timers)
                .run_if(
                    in_state(GameState::Playing)
                        .or(in_state(GameState::ChallengingStage))
                        .or(in_state(GameState::GameOver)),
                ),
        )
        // Challenging stage: initialise on entry, spawn patterns and check completion each frame
        .add_systems(
            OnEnter(GameState::ChallengingStage),
            enter_challenging_stage,
        )
        .add_systems(
            Update,
            (
                spawn_challenging_stage_patterns,
                move_challenging_enemies,
                bullet_challenging_enemy_collision,
                challenging_stage_completion,
            )
                .chain()
                .run_if(in_state(GameState::ChallengingStage)),
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
    mut challenging_spawner: ResMut<ChallengingStageSpawner>,
    mut splitter_state: ResMut<SplitterState>,
) {
    *wave_controller = WaveController::default();
    *stage_clear_timer = StageClearTimer::default();
    *group_coordinator = GroupAttackCoordinator::default();
    *tractor_beam = TractorBeamCoordinator::default();
    *dual_fighter = DualFighterState::default();
    *challenging_stage_data = ChallengingStageData::default();
    *challenging_spawner = ChallengingStageSpawner::default();
    *splitter_state = SplitterState::default();
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
