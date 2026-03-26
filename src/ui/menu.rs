// Title/menu screen: "GALAGA", "PRESS SPACE TO START", high score.

use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::resources::ScoreBoard;

// --- Marker components ---

/// Tags all menu entities so they can be despawned on exit.
#[derive(Component)]
pub struct MenuElement;

/// Drives the blinking animation on the "PRESS SPACE TO START" text.
#[derive(Component)]
pub struct BlinkTimer {
    pub timer: Timer,
}

// --- Layout constants ---

const TITLE_FONT_SIZE: f32 = 14.0;
const BODY_FONT_SIZE: f32 = 6.0;
const MENU_Z: f32 = 10.0;

const TITLE_Y: f32 = 60.0;
const HIGH_SCORE_LABEL_Y: f32 = 20.0;
const HIGH_SCORE_VALUE_Y: f32 = 8.0;
const PRESS_START_Y: f32 = -40.0;
const COPYRIGHT_Y: f32 = -100.0;

const BLINK_INTERVAL_SECS: f32 = 0.5;

// --- Systems ---

/// Spawns all menu UI entities.  Called on `OnEnter(GameState::Menu)`.
pub fn spawn_menu(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    score_board: Res<ScoreBoard>,
    existing: Query<Entity, With<MenuElement>>,
) {
    // Despawn any leftover menu entities (e.g. returning from game)
    for entity in &existing {
        commands.entity(entity).despawn();
    }

    let font = game_assets.font.clone();

    // "GALAGA" title
    commands.spawn((
        Text2d::new("GALAGA"),
        TextFont { font: font.clone(), font_size: TITLE_FONT_SIZE, ..default() },
        TextColor(Color::srgb(1.0, 1.0, 0.0)), // yellow, arcade-style
        Transform::from_xyz(0.0, TITLE_Y, MENU_Z),
        MenuElement,
    ));

    // "HIGH SCORE" label
    commands.spawn((
        Text2d::new("HIGH SCORE"),
        TextFont { font: font.clone(), font_size: BODY_FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, HIGH_SCORE_LABEL_Y, MENU_Z),
        MenuElement,
    ));

    // High score value
    let high_score_str = if score_board.high_score == 0 {
        "00000".to_string()
    } else {
        format!("{}", score_board.high_score)
    };
    commands.spawn((
        Text2d::new(high_score_str),
        TextFont { font: font.clone(), font_size: BODY_FONT_SIZE, ..default() },
        TextColor(Color::srgb(1.0, 0.5, 0.0)), // orange
        Transform::from_xyz(0.0, HIGH_SCORE_VALUE_Y, MENU_Z),
        MenuElement,
    ));

    // "PRESS SPACE TO START" — starts visible, will blink
    commands.spawn((
        Text2d::new("PRESS SPACE TO START"),
        TextFont { font: font.clone(), font_size: BODY_FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, PRESS_START_Y, MENU_Z),
        MenuElement,
        BlinkTimer {
            timer: Timer::from_seconds(BLINK_INTERVAL_SECS, TimerMode::Repeating),
        },
    ));

    // Copyright / credits line
    commands.spawn((
        Text2d::new("© 1981 NAMCO LTD."),
        TextFont { font: font.clone(), font_size: BODY_FONT_SIZE, ..default() },
        TextColor(Color::srgb(0.6, 0.6, 0.6)),
        Transform::from_xyz(0.0, COPYRIGHT_Y, MENU_Z),
        MenuElement,
    ));
}

/// Despawns all menu UI entities.  Called on `OnExit(GameState::Menu)`.
pub fn despawn_menu(mut commands: Commands, elements: Query<Entity, With<MenuElement>>) {
    for entity in &elements {
        commands.entity(entity).despawn();
    }
}

/// Toggles the visibility of the "PRESS SPACE TO START" text at `BLINK_INTERVAL_SECS`.
pub fn blink_press_start(
    time: Res<Time>,
    mut query: Query<(&mut Visibility, &mut BlinkTimer)>,
) {
    for (mut vis, mut blink) in &mut query {
        blink.timer.tick(time.delta());
        if blink.timer.just_finished() {
            *vis = match *vis {
                Visibility::Hidden => Visibility::Inherited,
                _ => Visibility::Hidden,
            };
        }
    }
}
