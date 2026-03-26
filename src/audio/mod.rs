// Audio event definitions and playback system for Galaga.
//
// Game systems trigger `GameAudioEvent` via `commands.trigger()`; the observer
// `handle_audio_events` picks them up and plays the appropriate sound through
// bevy_kira_audio.
//
// Music is managed separately via a typed `MusicChannel`. On each `GameState`
// transition the appropriate system stops the current track and starts the new
// one (looped for ambient tracks, one-shot for jingles).

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::assets::GameAssets;
use crate::states::GameState;

// ---------------------------------------------------------------------------
// Music channel
// ---------------------------------------------------------------------------

/// Marker type for the dedicated background-music audio channel.
///
/// Registered with `app.add_audio_channel::<MusicChannel>()` in main.
/// All music playback goes through `Res<AudioChannel<MusicChannel>>` so SFX
/// and music volumes can be controlled independently.
#[derive(Resource)]
pub struct MusicChannel;

/// All sound-effect triggers used by game systems.
///
/// Trigger with `commands.trigger(GameAudioEvent::PlayerShoot)` from any
/// system; the registered observer will play the matching asset.
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAudioEvent {
    /// Player fires a bullet.
    PlayerShoot,
    /// An enemy fires a bullet.
    EnemyShoot,
    /// A regular enemy (bee/butterfly/scorpion/stingray) is destroyed.
    EnemyExplode,
    /// A Boss Galaga is destroyed.
    BossExplode,
    /// Player ship is destroyed.
    PlayerExplode,
    /// Boss Galaga activates the tractor beam.
    TractorBeamActivate,
    /// Player ship is captured by the tractor beam.
    ShipCaptured,
    /// Captured ship is rescued (player shoots the Boss holding it).
    ShipRescued,
    /// Dual fighter joins (second ship merges with player).
    DualFighterJoin,
    /// An enemy begins its dive (swoosh cue).
    EnemyDive,
    /// A bonus score event is awarded.
    BonusAwarded,
    /// Player earns an extra life.
    ExtraLife,
    /// All enemies in a regular stage are cleared.
    StageClear,
    /// A challenging stage begins.
    ChallengingStageStart,
    /// Player destroys all enemies in a challenging stage (perfect bonus).
    PerfectBonus,
    /// Game-over condition reached.
    GameOver,
}

/// Observer: plays the sound matching the triggered `GameAudioEvent`.
///
/// `GameAssets` is wrapped in `Option` because this observer can fire during
/// the Loading state before the asset collection is ready; if assets aren't
/// loaded yet the sound is simply skipped.
pub fn handle_audio_events(
    trigger: On<GameAudioEvent>,
    audio: Res<Audio>,
    assets: Option<Res<GameAssets>>,
) {
    let Some(assets) = assets else { return };

    let handle = match trigger.event() {
        GameAudioEvent::PlayerShoot => assets.sfx_shoot.clone(),
        GameAudioEvent::EnemyShoot => assets.sfx_enemy_shoot.clone(),
        GameAudioEvent::EnemyExplode => assets.sfx_explosion_small.clone(),
        GameAudioEvent::BossExplode => assets.sfx_explosion_large.clone(),
        GameAudioEvent::PlayerExplode => assets.sfx_player_death.clone(),
        GameAudioEvent::TractorBeamActivate => assets.sfx_tractor_beam.clone(),
        GameAudioEvent::ShipCaptured => assets.sfx_capture.clone(),
        GameAudioEvent::ShipRescued => assets.sfx_rescue.clone(),
        GameAudioEvent::DualFighterJoin => assets.sfx_dual_join.clone(),
        GameAudioEvent::EnemyDive => assets.sfx_dive_swoosh.clone(),
        GameAudioEvent::BonusAwarded | GameAudioEvent::PerfectBonus => assets.sfx_bonus.clone(),
        GameAudioEvent::ExtraLife => assets.sfx_extra_life.clone(),
        GameAudioEvent::StageClear | GameAudioEvent::ChallengingStageStart => {
            assets.sfx_stage_clear.clone()
        }
        GameAudioEvent::GameOver => assets.sfx_explosion_large.clone(),
    };

    audio.play(handle);
}

// ---------------------------------------------------------------------------
// Music state systems
// ---------------------------------------------------------------------------

/// Plays the title theme (looped) when the Menu state is entered.
pub fn music_on_enter_menu(
    music: Res<AudioChannel<MusicChannel>>,
    assets: Option<Res<GameAssets>>,
) {
    let Some(assets) = assets else { return };
    music.stop();
    music.play(assets.music_title_theme.clone()).looped();
}

/// Plays the gameplay loop (looped) when the Playing state is entered.
/// The stage-start jingle is handled as a one-shot SFX event elsewhere.
pub fn music_on_enter_playing(
    music: Res<AudioChannel<MusicChannel>>,
    assets: Option<Res<GameAssets>>,
) {
    let Some(assets) = assets else { return };
    music.stop();
    music.play(assets.music_gameplay_loop.clone()).looped();
}

/// Swaps to the challenging-stage music (looped) on entering ChallengingStage.
pub fn music_on_enter_challenging(
    music: Res<AudioChannel<MusicChannel>>,
    assets: Option<Res<GameAssets>>,
) {
    let Some(assets) = assets else { return };
    music.stop();
    music.play(assets.music_challenging_stage.clone()).looped();
}

/// Plays the game-over jingle (one-shot) on entering GameOver.
pub fn music_on_enter_game_over(
    music: Res<AudioChannel<MusicChannel>>,
    assets: Option<Res<GameAssets>>,
) {
    let Some(assets) = assets else { return };
    music.stop();
    music.play(assets.music_game_over.clone());
}

/// Pauses background music while the game is paused.
pub fn music_on_enter_paused(music: Res<AudioChannel<MusicChannel>>) {
    music.pause();
}

/// Resumes background music when leaving the Paused state.
pub fn music_on_exit_paused(music: Res<AudioChannel<MusicChannel>>) {
    music.resume();
}

/// Registers all music-transition systems into the Bevy app.
///
/// Call this once from `main()` after `add_audio_channel::<MusicChannel>()`.
pub fn add_music_systems(app: &mut App) {
    app.add_audio_channel::<MusicChannel>()
        .add_systems(OnEnter(GameState::Menu), music_on_enter_menu)
        .add_systems(OnEnter(GameState::Playing), music_on_enter_playing)
        .add_systems(OnEnter(GameState::ChallengingStage), music_on_enter_challenging)
        .add_systems(OnEnter(GameState::GameOver), music_on_enter_game_over)
        .add_systems(OnEnter(GameState::Paused), music_on_enter_paused)
        .add_systems(OnExit(GameState::Paused), music_on_exit_paused);
}
