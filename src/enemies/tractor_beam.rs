// Boss Galaga tractor beam: capture trigger, beam entity, player capture.

use bevy::prelude::*;
use rand::Rng;

use crate::components::{
    Bullet, BulletOwner, CapturedBy, CapturedShip, Collider, DivePath, DivePathProgress,
    EnemyState, EnemyType, FormationSlot, PlayerFrozen, PlayerShip, TractorBeam, TractorBeamRun,
    TractorBeamSpawned,
};
use crate::enemies::dive_paths::boss_tractor_beam_path;
use crate::player::death::RespawnTimer;
use crate::resources::{DualFighterState, ScoreBoard, TractorBeamCoordinator};
use crate::states::GameState;
use crate::GameAudioEvent;

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
    boss_query: Query<(Entity, &EnemyState, &EnemyType, &FormationSlot, Has<Children>)>,
    bullet_query: Query<&Bullet>,
    dual_fighter: Res<DualFighterState>,
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

    // As in the arcade, only one fighter can be held at a time, and a dual
    // fighter is never targeted.  (A capture strips the Boss's children, so
    // re-beaming a Boss that already holds a ship would destroy that ship.)
    if dual_fighter.active || dual_fighter.captured_ship.is_some() {
        return;
    }

    // Precondition 2: collect InFormation Bosses.
    let in_formation = |s: &EnemyState, et: &EnemyType| {
        *s == EnemyState::InFormation && *et == EnemyType::Boss
    };
    if boss_query.iter().filter(|(_, s, et, ..)| in_formation(s, et)).count() < 2 {
        return;
    }

    // Only Bosses not already carrying a captured ship can fire the beam.
    let formation_bosses: Vec<(Entity, Vec2)> = boss_query
        .iter()
        .filter(|(_, s, et, _, has_children)| in_formation(s, et) && !has_children)
        .map(|(e, _, _, slot, _)| (e, slot.home_pos))
        .collect();

    if formation_bosses.is_empty() {
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

        commands.trigger(GameAudioEvent::TractorBeamActivate);
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

// ── Player capture ─────────────────────────────────────────────────────────

/// Speed at which the player ship is pulled upward by the tractor beam (units/sec).
const CAPTURE_PULL_SPEED: f32 = 55.0;

/// Delay before the player respawns after being captured.
const CAPTURE_RESPAWN_DELAY_SECS: f32 = 2.5;

/// Pulls the frozen player ship upward toward the tractor-beam Boss.
///
/// Once the player reaches the Boss, the capture sequence completes:
///
/// 1. The player entity loses `PlayerShip` and gains `CapturedShip` +
///    `CapturedBy(boss)`.  Its sprite is tinted red.
/// 2. The player entity is re-parented to the Boss with a fixed local offset
///    so the captured ship follows the Boss as it returns to formation.
/// 3. The tractor-beam visual children are despawned.
/// 4. The Boss receives a return path back to its formation slot and
///    `TractorBeamRun` / `TractorBeamSpawned` are removed so that
///    `dive_completion_system` can handle the return-to-formation transition.
/// 5. A life is deducted; the respawn timer is started (or `GameOver` if no
///    lives remain).  `TractorBeamCoordinator::active` is cleared.
pub fn player_capture_system(
    mut commands: Commands,
    time: Res<Time>,
    mut coordinator: ResMut<TractorBeamCoordinator>,
    mut dual_fighter: ResMut<DualFighterState>,
    mut score_board: ResMut<ScoreBoard>,
    mut respawn_timer: ResMut<RespawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    boss_query: Query<
        (Entity, &Transform, &FormationSlot),
        (With<TractorBeamSpawned>, With<TractorBeamRun>, Without<PlayerShip>),
    >,
    mut player_query: Query<
        (Entity, &mut Transform, &mut Sprite),
        (With<PlayerShip>, With<PlayerFrozen>, Without<TractorBeamRun>),
    >,
) {
    let Ok((boss_entity, boss_transform, boss_slot)) = boss_query.single() else {
        return;
    };
    let Ok((player_entity, mut player_transform, mut player_sprite)) =
        player_query.single_mut()
    else {
        return;
    };

    let boss_y = boss_transform.translation.y;
    let player_y = player_transform.translation.y;

    // Tint the ship red while it is being pulled up.
    player_sprite.color = Color::srgb(1.0, 0.3, 0.3);

    // The ship is "captured" once it reaches 14 units below the boss centre.
    let capture_threshold = boss_y - 14.0;

    if player_y < capture_threshold {
        // Still in transit — pull upward.
        let step = CAPTURE_PULL_SPEED * time.delta_secs();
        player_transform.translation.y = (player_y + step).min(capture_threshold);
        return;
    }

    // ── Capture complete ────────────────────────────────────────────────────

    let home_pos = boss_slot.home_pos;
    let current_boss_pos = boss_transform.translation.truncate();

    // Convert the player entity to a captured ship parented to the Boss.
    // The local transform places it 14 px below the Boss origin so it
    // appears to be held just beneath it.
    commands
        .entity(player_entity)
        .remove::<PlayerShip>()
        .remove::<PlayerFrozen>()
        .remove::<Collider>()
        .insert(CapturedShip)
        .insert(CapturedBy(boss_entity))
        .insert(Transform::from_xyz(0.0, -14.0, 0.0));

    // Remove the beam visual children, then attach the captured ship.
    commands
        .entity(boss_entity)
        .despawn_related::<Children>()
        .add_child(player_entity)
        .remove::<TractorBeamRun>()
        .remove::<TractorBeamSpawned>()
        // Give the boss a straight return path to its formation slot.
        // dive_completion_system will revert it to InFormation once done.
        .insert(DivePath(vec![current_boss_pos, home_pos]))
        .insert(DivePathProgress { current_waypoint: 1 });
    // EnemyState stays Diving; dive_completion_system handles the InFormation transition.

    commands.trigger(GameAudioEvent::ShipCaptured);

    // Record captured ship for later dual-fighter rescue logic.
    dual_fighter.captured_ship = Some(player_entity);

    // Decrement lives and trigger respawn or game over.
    score_board.lives = score_board.lives.saturating_sub(1);
    if score_board.lives == 0 {
        next_state.set(GameState::GameOver);
    } else {
        respawn_timer.0 =
            Some(Timer::from_seconds(CAPTURE_RESPAWN_DELAY_SECS, TimerMode::Once));
    }

    coordinator.active = false;
}

// ── Interrupted-run recovery ───────────────────────────────────────────────

/// Cleans up a tractor-beam run that ended without a capture.
///
/// Two ways a run can be cut short:
///
/// * The Boss is shot down mid-run.  The beam children die with it, but the
///   player would otherwise stay `PlayerFrozen` (and red) for the rest of the
///   game, and `TractorBeamCoordinator::active` would never clear, so no
///   further tractor beams could ever start.
/// * The beam is up but no ship is caught in it (the frozen player was
///   destroyed by a bullet or diver, or was mid-respawn when the beam opened).
///   `player_capture_system` needs a frozen player, so the Boss would hover
///   at the stop point forever.  Send it home instead.
pub fn tractor_beam_watchdog(
    mut commands: Commands,
    mut coordinator: ResMut<TractorBeamCoordinator>,
    boss_query: Query<
        (Entity, &Transform, &FormationSlot, Has<TractorBeamSpawned>),
        With<TractorBeamRun>,
    >,
    mut frozen_query: Query<(Entity, &mut Sprite), (With<PlayerShip>, With<PlayerFrozen>)>,
) {
    if boss_query.is_empty() {
        coordinator.active = false;
        for (player_entity, mut sprite) in &mut frozen_query {
            sprite.color = Color::WHITE;
            commands.entity(player_entity).remove::<PlayerFrozen>();
        }
        return;
    }

    if !frozen_query.is_empty() {
        return;
    }

    for (boss_entity, transform, slot, beam_spawned) in &boss_query {
        if !beam_spawned {
            continue; // still flying to the stop point
        }
        commands
            .entity(boss_entity)
            .despawn_related::<Children>()
            .remove::<TractorBeamRun>()
            .remove::<TractorBeamSpawned>()
            .insert(DivePath(vec![transform.translation.truncate(), slot.home_pos]))
            .insert(DivePathProgress { current_waypoint: 1 });
        coordinator.active = false;
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use crate::enemies::dive_paths::TRACTOR_BEAM_STOP_Y;

    fn world_with_active_beam() -> World {
        let mut world = World::new();
        let mut coordinator = TractorBeamCoordinator::default();
        coordinator.active = true;
        world.insert_resource(coordinator);
        world
    }

    #[test]
    fn boss_killed_mid_beam_unfreezes_player() {
        let mut world = world_with_active_beam();
        let player = world
            .spawn((PlayerShip, PlayerFrozen, Sprite { color: Color::srgb(1.0, 0.3, 0.3), ..default() }))
            .id();

        world.run_system_once(tractor_beam_watchdog).unwrap();

        assert!(!world.resource::<TractorBeamCoordinator>().active);
        assert!(world.get::<PlayerFrozen>(player).is_none());
        assert_eq!(world.get::<Sprite>(player).unwrap().color, Color::WHITE);
    }

    #[test]
    fn beam_without_captive_sends_boss_home() {
        let mut world = world_with_active_beam();
        let home = Vec2::new(8.0, 80.0);
        let boss = world
            .spawn((
                Transform::from_xyz(0.0, TRACTOR_BEAM_STOP_Y, 0.0),
                FormationSlot { row: 0, col: 1, home_pos: home },
                TractorBeamRun,
                TractorBeamSpawned,
            ))
            .id();

        world.run_system_once(tractor_beam_watchdog).unwrap();

        assert!(!world.resource::<TractorBeamCoordinator>().active);
        assert!(world.get::<TractorBeamRun>(boss).is_none());
        assert_eq!(world.get::<DivePath>(boss).unwrap().0.last(), Some(&home));
    }

    #[test]
    fn boss_still_approaching_is_left_alone() {
        let mut world = world_with_active_beam();
        let boss = world
            .spawn((
                Transform::default(),
                FormationSlot { row: 0, col: 1, home_pos: Vec2::ZERO },
                TractorBeamRun,
            ))
            .id();

        world.run_system_once(tractor_beam_watchdog).unwrap();

        assert!(world.resource::<TractorBeamCoordinator>().active);
        assert!(world.get::<TractorBeamRun>(boss).is_some());
    }
}
