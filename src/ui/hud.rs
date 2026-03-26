use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::resources::ScoreBoard;

// --- Marker components ---

/// Tags all HUD entities so they can be found and despawned on re-entry.
#[derive(Component)]
pub struct HudElement;

/// Tags the Text2d entity that displays the current score.
#[derive(Component)]
pub struct HudScoreText;

/// Tags the Text2d entity that displays the high score.
#[derive(Component)]
pub struct HudHighScoreText;

/// Tags the Text2d entity that displays the current stage.
#[derive(Component)]
pub struct HudStageText;

/// Tags mini ship sprite entities shown as life icons.
#[derive(Component)]
pub struct HudLifeIcon;

// --- Layout constants ---

const FONT_SIZE: f32 = 6.0;
const HUD_Z: f32 = 10.0;

// Top HUD row (y=138): labels; row below (y=128): values
const SCORE_LABEL_X: f32 = -70.0;
const SCORE_LABEL_Y: f32 = 138.0;
const SCORE_VALUE_Y: f32 = 128.0;

const HIGH_SCORE_LABEL_X: f32 = 28.0;
const HIGH_SCORE_LABEL_Y: f32 = 138.0;
const HIGH_SCORE_VALUE_Y: f32 = 128.0;

// Bottom HUD row
const LIVES_START_X: f32 = -108.0;
const LIVES_Y: f32 = -133.0;
const LIVES_SPACING: f32 = 10.0;
const LIFE_ICON_SCALE: f32 = 0.55;

const STAGE_TEXT_X: f32 = 84.0;
const STAGE_TEXT_Y: f32 = -133.0;

// --- Systems ---

/// Spawns all HUD entities.  Must be called after GameAssets is loaded.
/// Despawns any previously spawned HUD entities first to avoid duplicates
/// when re-entering the Playing state.
pub fn spawn_hud(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    score_board: Res<ScoreBoard>,
    existing: Query<Entity, With<HudElement>>,
) {
    // Despawn previous HUD (handles new-game re-entry)
    for entity in &existing {
        commands.entity(entity).despawn();
    }

    let font = game_assets.font.clone();

    // "1UP" label
    commands.spawn((
        Text2d::new("1UP"),
        TextFont { font: font.clone(), font_size: FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(SCORE_LABEL_X, SCORE_LABEL_Y, HUD_Z),
        HudElement,
    ));

    // Score value
    commands.spawn((
        Text2d::new(format!("{:>6}", score_board.score)),
        TextFont { font: font.clone(), font_size: FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(SCORE_LABEL_X, SCORE_VALUE_Y, HUD_Z),
        HudElement,
        HudScoreText,
    ));

    // "HIGH SCORE" label
    commands.spawn((
        Text2d::new("HIGH SCORE"),
        TextFont { font: font.clone(), font_size: FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(HIGH_SCORE_LABEL_X, HIGH_SCORE_LABEL_Y, HUD_Z),
        HudElement,
    ));

    // High score value
    commands.spawn((
        Text2d::new(format!("{:>6}", score_board.high_score)),
        TextFont { font: font.clone(), font_size: FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(HIGH_SCORE_LABEL_X, HIGH_SCORE_VALUE_Y, HUD_Z),
        HudElement,
        HudHighScoreText,
    ));

    // Stage text (bottom-right)
    commands.spawn((
        Text2d::new(format!("ST.{}", score_board.current_stage)),
        TextFont { font: font.clone(), font_size: FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(STAGE_TEXT_X, STAGE_TEXT_Y, HUD_Z),
        HudElement,
        HudStageText,
    ));

    // Life icons (bottom-left): show lives - 1 (excludes active ship)
    spawn_life_icons(
        &mut commands,
        &game_assets,
        score_board.lives.saturating_sub(1) as usize,
    );
}

/// Syncs score, high score, stage, and life icons each frame.
pub fn update_hud(
    mut score_text: Query<&mut Text2d, (With<HudScoreText>, Without<HudHighScoreText>, Without<HudStageText>)>,
    mut high_score_text: Query<&mut Text2d, (With<HudHighScoreText>, Without<HudScoreText>, Without<HudStageText>)>,
    mut stage_text: Query<&mut Text2d, (With<HudStageText>, Without<HudScoreText>, Without<HudHighScoreText>)>,
    life_icons: Query<Entity, With<HudLifeIcon>>,
    mut commands: Commands,
    score_board: Res<ScoreBoard>,
    game_assets: Res<GameAssets>,
) {
    // Score
    if let Ok(mut text) = score_text.single_mut() {
        let new_val = format!("{:>6}", score_board.score);
        if text.0 != new_val {
            text.0 = new_val;
        }
    }

    // High score
    if let Ok(mut text) = high_score_text.single_mut() {
        let new_val = format!("{:>6}", score_board.high_score);
        if text.0 != new_val {
            text.0 = new_val;
        }
    }

    // Stage
    if let Ok(mut text) = stage_text.single_mut() {
        let new_val = format!("ST.{}", score_board.current_stage);
        if text.0 != new_val {
            text.0 = new_val;
        }
    }

    // Life icons: rebuild when count changes
    let desired = score_board.lives.saturating_sub(1) as usize;
    let current = life_icons.iter().count();
    if current != desired {
        for entity in &life_icons {
            commands.entity(entity).despawn();
        }
        spawn_life_icons(&mut commands, &game_assets, desired);
    }
}

// --- Helpers ---

fn spawn_life_icons(commands: &mut Commands, game_assets: &GameAssets, count: usize) {
    for i in 0..count {
        commands.spawn((
            Sprite::from_image(game_assets.player_image.clone()),
            Transform::from_xyz(LIVES_START_X + i as f32 * LIVES_SPACING, LIVES_Y, HUD_Z)
                .with_scale(Vec3::splat(LIFE_ICON_SCALE)),
            HudElement,
            HudLifeIcon,
        ));
    }
}
