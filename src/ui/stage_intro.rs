// Stage intro overlay: displays "STAGE X" for a brief pause before enemies enter.

use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::resources::ScoreBoard;

// --- Config ---

const INTRO_DURATION_SECS: f32 = 2.0;
const TITLE_FONT_SIZE: f32 = 10.0;
const Z: f32 = 15.0;

// --- Components / Resources ---

/// Tags all stage-intro overlay entities so they can be despawned.
#[derive(Component)]
pub struct StageIntroElement;

/// Drives the stage-intro display.
/// Set `active = true` and reset the timer to show the overlay.
#[derive(Resource, Default)]
pub struct StageIntroTimer {
    pub timer: Option<Timer>,
    /// True once the text entities have been spawned for this intro.
    pub spawned: bool,
}

impl StageIntroTimer {
    /// Arm the intro so it fires on the next tick.
    pub fn trigger(&mut self) {
        self.timer = Some(Timer::from_seconds(INTRO_DURATION_SECS, TimerMode::Once));
        self.spawned = false;
    }
}

// --- Systems ---

/// System: arms the stage intro timer when entering the Playing state.
/// Called via `OnEnter(GameState::Playing)`.
pub fn arm_stage_intro(mut intro: ResMut<StageIntroTimer>) {
    intro.trigger();
}

/// System: spawns overlay text on the first tick after arming, ticks the timer,
/// and despawns the overlay once the timer expires.
/// Runs in `Update` while in `GameState::Playing`.
pub fn tick_stage_intro(
    mut commands: Commands,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    score_board: Res<ScoreBoard>,
    mut intro: ResMut<StageIntroTimer>,
    elements: Query<Entity, With<StageIntroElement>>,
) {
    if intro.timer.is_none() {
        return;
    }

    // Spawn text on the first frame this timer is active.
    if !intro.spawned {
        intro.spawned = true;

        let font = game_assets.font.clone();

        // "STAGE X" centered on screen
        commands.spawn((
            Text2d::new(format!("STAGE  {}", score_board.current_stage)),
            TextFont { font: font.clone(), font_size: TITLE_FONT_SIZE, ..default() },
            TextColor(Color::srgb(1.0, 1.0, 0.0)), // yellow
            Transform::from_xyz(0.0, 20.0, Z),
            StageIntroElement,
        ));

        // Smaller subtitle
        commands.spawn((
            Text2d::new("- START -"),
            TextFont { font, font_size: 6.0, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 1.0)),
            Transform::from_xyz(0.0, 5.0, Z),
            StageIntroElement,
        ));
    }

    // Now tick — reborrow to avoid conflicting borrows above.
    let finished = {
        let timer = intro.timer.as_mut().unwrap();
        timer.tick(time.delta());
        timer.just_finished()
    };

    if finished {
        intro.timer = None;
        intro.spawned = false;
        for entity in &elements {
            commands.entity(entity).despawn();
        }
    }
}
