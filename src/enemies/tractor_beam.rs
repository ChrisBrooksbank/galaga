// Boss Galaga tractor beam: capture trigger, beam entity, player capture.

use bevy::prelude::*;
use rand::Rng;

use crate::components::{
    Bullet, BulletOwner, DivePath, DivePathProgress, EnemyState, EnemyType, FormationSlot,
    TractorBeamRun,
};
use crate::enemies::dive_paths::boss_tractor_beam_path;
use crate::resources::TractorBeamCoordinator;

// ── Tractor beam trigger ───────────────────────────────────────────────────────

/// Decides whether a tractor beam run should begin.
///
/// Preconditions (all must hold):
/// 1. No tractor beam is currently active (`TractorBeamCoordinator::active == false`).
/// 2. At least 2 Boss Galagas are `InFormation`.
/// 3. No player bullet is currently in flight.
///
/// When conditions are met the system probabilistically selects one Boss and
/// sends it along `boss_tractor_beam_path`, marking it with `TractorBeamRun`
/// and setting its state to `Diving`.  `dive_movement_system` then moves the
/// boss to the mid-screen stop point; `dive_completion_system` is modified to
/// skip `TractorBeamRun` entities so the boss waits there for the beam and
/// capture sequence (implemented in subsequent tasks).
pub fn boss_tractor_decision(
    mut commands: Commands,
    time: Res<Time>,
    mut coordinator: ResMut<TractorBeamCoordinator>,
    boss_query: Query<(Entity, &EnemyState, &EnemyType, &FormationSlot)>,
    bullet_query: Query<&Bullet>,
) {
    coordinator.timer.tick(time.delta());
    if !coordinator.timer.just_finished() {
        return;
    }

    // Schedule the next check regardless of whether a run starts.
    coordinator.timer =
        Timer::from_seconds(TractorBeamCoordinator::CHECK_INTERVAL, TimerMode::Once);

    // Precondition 1: no active tractor beam run.
    if coordinator.active {
        return;
    }

    // Precondition 2: collect InFormation Bosses.
    let formation_bosses: Vec<(Entity, Vec2)> = boss_query
        .iter()
        .filter(|(_, s, et, _)| **s == EnemyState::InFormation && **et == EnemyType::Boss)
        .map(|(e, _, _, slot)| (e, slot.home_pos))
        .collect();

    if formation_bosses.len() < 2 {
        return;
    }

    // Precondition 3: no player bullet in flight.
    let player_bullet_in_flight = bullet_query.iter().any(|b| b.owner == BulletOwner::Player);
    if player_bullet_in_flight {
        return;
    }

    // Probabilistic trigger.
    let mut rng = rand::thread_rng();
    if !rng.gen_bool(TractorBeamCoordinator::TRIGGER_PROBABILITY as f64) {
        return;
    }

    // Pick a random Boss from those eligible.
    let idx = rng.gen_range(0..formation_bosses.len());
    let (entity, home_pos) = formation_bosses[idx];

    let path = boss_tractor_beam_path(home_pos);

    coordinator.active = true;

    // No EnemyFireCooldown: tractor beam bosses don't shoot during the run.
    commands
        .entity(entity)
        .insert(DivePath(path))
        .insert(DivePathProgress { current_waypoint: 1 })
        .insert(TractorBeamRun)
        .insert(EnemyState::Diving);
}
