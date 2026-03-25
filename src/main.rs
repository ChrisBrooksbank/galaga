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
use player::spawn_player;
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
        // Asset loading: transition Loading → Menu automatically when all assets are ready
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<GameAssets>(),
        )
        .add_systems(Startup, setup_camera)
        // Spawn player when entering Playing state
        .add_systems(OnEnter(GameState::Playing), spawn_player)
        // Menu → Playing on Space
        .add_systems(Update, menu_to_playing.run_if(in_state(GameState::Menu)))
        // Playing → Paused on Escape
        .add_systems(Update, toggle_pause.run_if(in_state(GameState::Playing)))
        // Paused → Playing on Escape
        .add_systems(Update, toggle_pause.run_if(in_state(GameState::Paused)))
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
