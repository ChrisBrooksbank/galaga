// Pause menu: displays current volume levels and lets the player adjust them.
//
// Controls while paused:
//   Up / Down   — switch between SFX and Music rows
//   Left / Right — decrease / increase the selected volume by 10 %
//   Escape      — resume (handled by toggle_pause in main.rs)

use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::resources::VolumeSettings;

// --- Marker components ---

/// Tags all pause-menu entities so they can be despawned on exit.
#[derive(Component)]
pub struct PauseElement;

/// Tags the Text2d entity showing the SFX volume.
#[derive(Component)]
pub struct SfxVolumeText;

/// Tags the Text2d entity showing the music volume.
#[derive(Component)]
pub struct MusicVolumeText;

// --- Selection state ---

/// Tracks which volume row is currently highlighted in the pause menu.
#[derive(Resource)]
pub struct PauseSelection {
    /// `true` = SFX row selected, `false` = Music row selected.
    pub is_sfx: bool,
}

impl Default for PauseSelection {
    fn default() -> Self {
        Self { is_sfx: true }
    }
}

// --- Layout constants ---

const PAUSE_Z: f32 = 20.0;
const TITLE_FONT_SIZE: f32 = 12.0;
const BODY_FONT_SIZE: f32 = 6.0;
const HINT_FONT_SIZE: f32 = 4.0;

const TITLE_Y: f32 = 30.0;
const SFX_ROW_Y: f32 = 8.0;
const MUSIC_ROW_Y: f32 = -6.0;
const HINT1_Y: f32 = -28.0;
const HINT2_Y: f32 = -40.0;

// --- Systems ---

/// Spawns the pause menu overlay.  Called on `OnEnter(GameState::Paused)`.
pub fn spawn_pause_menu(
    mut commands: Commands,
    assets: Option<Res<GameAssets>>,
    volume: Res<VolumeSettings>,
    mut selection: ResMut<PauseSelection>,
    existing: Query<Entity, With<PauseElement>>,
) {
    // Clean up any leftover entities
    for e in &existing {
        commands.entity(e).despawn();
    }

    // Reset selection to SFX each time the menu opens
    selection.is_sfx = true;

    let Some(assets) = assets else { return };
    let font = assets.font.clone();

    // "PAUSED" title
    commands.spawn((
        Text2d::new("PAUSED"),
        TextFont { font: font.clone(), font_size: TITLE_FONT_SIZE, ..default() },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
        Transform::from_xyz(0.0, TITLE_Y, PAUSE_Z),
        PauseElement,
    ));

    // SFX volume row (selected by default)
    let sfx_pct = (volume.sfx_volume * 100.0).round() as u32;
    commands.spawn((
        Text2d::new(format!("> SFX  : {:3}%", sfx_pct)),
        TextFont { font: font.clone(), font_size: BODY_FONT_SIZE, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, SFX_ROW_Y, PAUSE_Z),
        PauseElement,
        SfxVolumeText,
    ));

    // Music volume row
    let music_pct = (volume.music_volume * 100.0).round() as u32;
    commands.spawn((
        Text2d::new(format!("  MUSIC: {:3}%", music_pct)),
        TextFont { font: font.clone(), font_size: BODY_FONT_SIZE, ..default() },
        TextColor(Color::srgb(0.6, 0.6, 0.6)),
        Transform::from_xyz(0.0, MUSIC_ROW_Y, PAUSE_Z),
        PauseElement,
        MusicVolumeText,
    ));

    // Control hints
    commands.spawn((
        Text2d::new("UP/DOWN SELECT  LEFT/RIGHT ADJUST"),
        TextFont { font: font.clone(), font_size: HINT_FONT_SIZE, ..default() },
        TextColor(Color::srgb(0.45, 0.45, 0.45)),
        Transform::from_xyz(0.0, HINT1_Y, PAUSE_Z),
        PauseElement,
    ));

    commands.spawn((
        Text2d::new("ESC: RESUME"),
        TextFont { font: font.clone(), font_size: HINT_FONT_SIZE, ..default() },
        TextColor(Color::srgb(0.45, 0.45, 0.45)),
        Transform::from_xyz(0.0, HINT2_Y, PAUSE_Z),
        PauseElement,
    ));
}

/// Despawns all pause menu entities.  Called on `OnExit(GameState::Paused)`.
pub fn despawn_pause_menu(
    mut commands: Commands,
    elements: Query<Entity, With<PauseElement>>,
) {
    for e in &elements {
        commands.entity(e).despawn();
    }
}

/// Handles keyboard input in the pause menu.
///
/// Up/Down — move selection between SFX and Music rows.
/// Left/Right — adjust the selected volume by ±10 %, clamped to [0, 1].
pub fn pause_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut volume: ResMut<VolumeSettings>,
    mut selection: ResMut<PauseSelection>,
) {
    // Switch row
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowDown) {
        selection.is_sfx = !selection.is_sfx;
    }

    // Adjust volume
    let delta: f64 = if keys.just_pressed(KeyCode::ArrowLeft) {
        -0.1
    } else if keys.just_pressed(KeyCode::ArrowRight) {
        0.1
    } else {
        return;
    };

    if selection.is_sfx {
        volume.sfx_volume = (volume.sfx_volume + delta).clamp(0.0, 1.0);
    } else {
        volume.music_volume = (volume.music_volume + delta).clamp(0.0, 1.0);
    }
}

/// Refreshes the volume label text whenever VolumeSettings or the selection changes.
pub fn update_pause_volume_labels(
    volume: Res<VolumeSettings>,
    selection: Res<PauseSelection>,
    mut sfx_text: Query<&mut Text2d, (With<SfxVolumeText>, Without<MusicVolumeText>)>,
    mut music_text: Query<&mut Text2d, (With<MusicVolumeText>, Without<SfxVolumeText>)>,
) {
    if !volume.is_changed() && !selection.is_changed() {
        return;
    }

    if let Ok(mut text) = sfx_text.single_mut() {
        let pct = (volume.sfx_volume * 100.0).round() as u32;
        let cursor = if selection.is_sfx { '>' } else { ' ' };
        text.0 = format!("{} SFX  : {:3}%", cursor, pct);
    }

    if let Ok(mut text) = music_text.single_mut() {
        let pct = (volume.music_volume * 100.0).round() as u32;
        let cursor = if selection.is_sfx { ' ' } else { '>' };
        text.0 = format!("{} MUSIC: {:3}%", cursor, pct);
    }
}
