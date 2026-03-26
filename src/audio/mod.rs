// Audio event definitions and playback system for Galaga.
//
// Game systems trigger `GameAudioEvent` via `commands.trigger()`; the observer
// `handle_audio_events` picks them up and plays the appropriate sound through
// bevy_kira_audio.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::assets::GameAssets;

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
