// Boss Galaga tractor beam: capture trigger, beam entity, player capture.

use bevy::prelude::*;
use rand::Rng;

use crate::components::{
    Bullet, BulletOwner, DivePath, DivePathProgress, EnemyState, EnemyType, FormationSlot,
    PlayerFrozen, PlayerShip, TractorBeam, TractorBeamRun, TractorBeamSpawned,
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

// ── Tractor beam beam entity ────────────────────────────────────────────────

/// Spawns the fan-shaped cyan beam visual once the tractor-beam Boss has
/// arrived at its mid-screen stop point (path exhausted).
///
/// Three overlapping `Sprite` quads are spawned as children of the Boss:
///  - a centre vertical strip, and
///  - two angled side strips that together form a fan shape.
///
/// Each segment carries a `TractorBeam` component with an independent
/// `pulse_phase` so the segments ripple slightly out of sync.
///
/// The player ship receives `PlayerFrozen` to lock horizontal movement for
/// the duration of the capture sequence.
pub fn spawn_tractor_beam_system(
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerShip>>,
    boss_query: Query<
        (Entity, &DivePath, &DivePathProgress),
        (With<TractorBeamRun>, Without<TractorBeamSpawned>),
    >,
) {
    for (boss_entity, path, progress) in &boss_query {
        if progress.current_waypoint < path.0.len() {
            continue; // still travelling to stop point
        }

        // Mark the boss so this system doesn't re-run next frame.
        commands.entity(boss_entity).insert(TractorBeamSpawned);

        // Spawn three fan-segment sprites as children of the Boss so they
        // automatically inherit its world position.
        //
        // Coordinate convention: Y grows upward.  The beam extends downward
        // (negative Y) from the boss toward the player near y = −120.
        // Each sprite is centred at its local origin, so an offset of −40 on Y
        // places the top of an 80-px-tall sprite at the boss's centre.
        commands.entity(boss_entity).with_children(|parent| {
            // Centre strip — narrow, full length, brightest.
            parent.spawn((
                Sprite {
                    color: Color::srgba(0.0, 1.0, 1.0, 0.7),
                    custom_size: Some(Vec2::new(6.0, 80.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, -40.0, -0.5),
                TractorBeam { pulse_phase: 0.0 },
            ));

            // Left fan segment — rotated outward, slightly longer.
            parent.spawn((
                Sprite {
                    color: Color::srgba(0.0, 1.0, 1.0, 0.45),
                    custom_size: Some(Vec2::new(5.0, 86.0)),
                    ..default()
                },
                Transform::from_xyz(-7.0, -43.0, -0.6)
                    .with_rotation(Quat::from_rotation_z(0.22)),
                TractorBeam { pulse_phase: std::f32::consts::FRAC_PI_3 },
            ));

            // Right fan segment — mirror of left.
            parent.spawn((
                Sprite {
                    color: Color::srgba(0.0, 1.0, 1.0, 0.45),
                    custom_size: Some(Vec2::new(5.0, 86.0)),
                    ..default()
                },
                Transform::from_xyz(7.0, -43.0, -0.6)
                    .with_rotation(Quat::from_rotation_z(-0.22)),
                TractorBeam { pulse_phase: std::f32::consts::FRAC_PI_3 * 2.0 },
            ));
        });

        // Lock the player's horizontal movement during the beam sequence.
        if let Ok(player_entity) = player_query.single() {
            commands.entity(player_entity).insert(PlayerFrozen);
        }
    }
}

// ── Tractor beam pulse animation ───────────────────────────────────────────

/// Animates the alpha of every beam-segment sprite to create a pulsing glow.
///
/// Each segment has an independent `pulse_phase` so the fan appears to
/// "ripple" rather than blink uniformly.  Alpha oscillates between 0.2 and
/// 0.8 at ~1.5 Hz.
pub fn pulse_tractor_beam_system(
    time: Res<Time>,
    mut query: Query<(&mut TractorBeam, &mut Sprite)>,
) {
    let dt = time.delta_secs();
    for (mut beam, mut sprite) in &mut query {
        // Advance phase at 1.5 full cycles per second.
        beam.pulse_phase =
            (beam.pulse_phase + dt * std::f32::consts::TAU * 1.5) % std::f32::consts::TAU;

        // Map sine (−1..1) → alpha (0.2..0.8).
        let alpha = 0.2 + 0.6 * (beam.pulse_phase.sin() * 0.5 + 0.5);
        sprite.color = Color::srgba(0.0, 1.0, 1.0, alpha);
    }
}
