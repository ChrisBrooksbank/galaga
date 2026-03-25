use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::PresentMode;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
    Paused,
    GameOver,
    ChallengingStage,
}

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
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        // Loading → Menu immediately (no assets yet)
        .add_systems(Update, loading_to_menu.run_if(in_state(GameState::Loading)))
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

fn loading_to_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Menu);
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
