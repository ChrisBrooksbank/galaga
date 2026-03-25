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
use collision::{bullet_enemy_collision, diving_enemy_player_collision, enemy_bullet_player_collision};
use constants::PLAYER_START_LIVES;
use enemies::entry_patterns::move_forming_enemies;
use enemies::formation::{animate_enemy_wings, apply_formation_breathing, update_formation_breathing};
use enemies::spawn::spawn_formation;
use player::{
    handle_player_death, move_bullets, player_movement, player_shoot, spawn_player, tick_respawn,
    RespawnTimer,
};
use resources::{Formation, ScoreBoard};
use scoring::handle_score_event;
use states::GameState;

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
            (init_scoreboard, spawn_player, spawn_formation).chain(),
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
        // Collision detection (only while Playing)
        .add_systems(
            Update,
            (
                bullet_enemy_collision,
                enemy_bullet_player_collision,
                diving_enemy_player_collision,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Player death and respawn (only while Playing)
        .add_systems(
            Update,
            (handle_player_death, tick_respawn).run_if(in_state(GameState::Playing)),
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
    // high_score persists across games intentionally
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
