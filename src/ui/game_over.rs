// Game over screen: "GAME OVER", final score, stage reached.

use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::resources::ScoreBoard;
use crate::states::GameState;

// --- Marker component ---

/// Tags all game-over screen entities so they can be despawned on exit.
#[derive(Component)]
pub struct GameOverElement;

/// Counts down before returning to the Menu state.
#[derive(Resource)]
pub struct GameOverTimer {
    pub timer: Timer,
    pub space_enabled: bool,
}

impl Default for GameOverTimer {
    fn default() -> Self {
        Self {
            // Brief lock-out so an accidental Space at death doesn't skip straight past.
            timer: Timer::from_seconds(2.0, TimerMode::Once),
            space_enabled: false,
        }
    }
}

// --- Layout constants ---

const TITLE_FONT_SIZE: f32 = 14.0;
const BODY_FONT_SIZE: f32 = 6.0;
const SMALL_FONT_SIZE: f32 = 5.0;
const Z: f32 = 10.0;

const TITLE_Y: f32 = 60.0;
const SCORE_LABEL_Y: f32 = 20.0;
const SCORE_VALUE_Y: f32 = 8.0;
const STAGE_Y: f32 = -10.0;
const PROMPT_Y: f32 = -55.0;

// --- Systems ---

/// Spawns all game-over UI entities.  Called on `OnEnter(GameState::GameOver)`.
pub fn spawn_game_over(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    score_board: Res<ScoreBoard>,
) {
    let font = game_assets.font.clone();

    // "GAME OVER" title
    commands.spawn((
        Text2d::new("GAME OVER"),
        TextFont { font: font.clone(), font_size: TITLE_FONT_SIZE, ..default() },
        TextColor(Color::srgb(1.0, 0.1, 0.1)), // red
        Transform::from_xyz(0.0, TITLE_Y, Z),
        GameOverElement,
    ));

    // "SCORE" label
    commands.spawn((
        Text2d::new("SCORE"),
        TextFont { font: font.clone(), font_size: BODY_FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, SCORE_LABEL_Y, Z),
        GameOverElement,
    ));

    // Final score value
    commands.spawn((
        Text2d::new(format!("{}", score_board.score)),
        TextFont { font: font.clone(), font_size: BODY_FONT_SIZE, ..default() },
        TextColor(Color::srgb(1.0, 0.5, 0.0)), // orange
        Transform::from_xyz(0.0, SCORE_VALUE_Y, Z),
        GameOverElement,
    ));

    // Stage reached
    commands.spawn((
        Text2d::new(format!("STAGE  {}", score_board.current_stage)),
        TextFont { font: font.clone(), font_size: SMALL_FONT_SIZE, ..default() },
        TextColor(Color::srgb(0.7, 0.7, 1.0)), // light blue
        Transform::from_xyz(0.0, STAGE_Y, Z),
        GameOverElement,
    ));

    // "PRESS SPACE TO CONTINUE" prompt (shown after lock-out expires)
    commands.spawn((
        Text2d::new("PRESS SPACE TO CONTINUE"),
        TextFont { font: font.clone(), font_size: SMALL_FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, PROMPT_Y, Z),
        Visibility::Hidden,
        GameOverElement,
        GameOverPrompt,
    ));

    // Reset the timer resource
    commands.insert_resource(GameOverTimer::default());
}

/// Marker for the "PRESS SPACE" prompt so we can show it after the lock-out.
#[derive(Component)]
pub struct GameOverPrompt;

/// Despawns all game-over UI entities.  Called on `OnExit(GameState::GameOver)`.
pub fn despawn_game_over(
    mut commands: Commands,
    elements: Query<Entity, With<GameOverElement>>,
) {
    for entity in &elements {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<GameOverTimer>();
}

/// Ticks the lock-out timer; once elapsed reveals the prompt and accepts Space.
pub fn game_over_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut timer: ResMut<GameOverTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut prompt_query: Query<&mut Visibility, With<GameOverPrompt>>,
) {
    if !timer.space_enabled {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            timer.space_enabled = true;
            for mut vis in &mut prompt_query {
                *vis = Visibility::Inherited;
            }
        }
        return;
    }

    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Menu);
    }
}
