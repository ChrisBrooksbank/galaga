// Dive decision AI, state machine transitions, and path selection.
//
// State machine:
//   InFormation ──(dive trigger)──► Diving ──(path complete)──► InFormation
//
// Systems
// ───────
// `dive_decision_system`  — each frame, probabilistically selects one
//     InFormation enemy to start diving (respects concurrent-diver cap).
//     Assigns a `DivePath` and `DivePathProgress`, then sets state to Diving.
//
// `dive_completion_system` — detects when a Diving enemy has exhausted its
//     `DivePath` waypoints (tracked by `DivePathProgress`) and returns it to
//     formation by reverting state to InFormation and removing dive components.

use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::assets::GameAssets;
use crate::components::{
    Bullet, BulletOwner, Collider, DivePath, DivePathProgress, EnemyFireCooldown, EnemyState,
    EnemyType, FormationSlot, PlayerShip, SplitterPiece, TractorBeamRun, Velocity,
};
use crate::constants::ENEMY_BULLET_SPEED;
use crate::enemies::dive_paths::get_dive_path;
use crate::resources::{DifficultyConfig, GroupAttackCoordinator};
use crate::GameAudioEvent;

// ── Dive movement ─────────────────────────────────────────────────────────────

/// Move every `Diving` enemy along its `DivePath` waypoints.
///
/// Each frame the enemy advances toward `DivePath.0[DivePathProgress.current_waypoint]`
/// at `DifficultyConfig::dive_speed` units per second.  When it arrives it snaps to
/// the waypoint and increments `current_waypoint`, ready for `dive_completion_system`
/// to detect path exhaustion on the same or next frame.
pub fn dive_movement_system(
    time: Res<Time>,
    difficulty: Res<DifficultyConfig>,
    mut query: Query<(&mut Transform, &DivePath, &mut DivePathProgress, &EnemyState)>,
) {
    let dt = time.delta_secs();
    let speed = difficulty.dive_speed;

    for (mut transform, path, mut progress, state) in &mut query {
        if *state != EnemyState::Diving {
            continue;
        }

        let Some(&target) = path.0.get(progress.current_waypoint) else {
            continue; // path exhausted — completion system handles transition
        };

        let pos = transform.translation.truncate();
        let dir = target - pos;
        let dist = dir.length();
        let step = speed * dt;

        if dist <= step {
            // Snap to waypoint and advance to the next one.
            transform.translation.x = target.x;
            transform.translation.y = target.y;
            progress.current_waypoint += 1;
        } else {
            let movement = dir.normalize() * step;
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

// ── Dive decision ─────────────────────────────────────────────────────────────

/// Each frame, optionally pick one InFormation enemy to begin a dive.
///
/// The decision is probability-based; `dive_probability` in `DifficultyConfig`
/// is the per-frame chance (normalised against 60 fps) that any single enemy
/// will be selected.  At most `max_concurrent_divers` enemies may be diving
/// simultaneously.
pub fn dive_decision_system(
    mut commands: Commands,
    difficulty: Res<DifficultyConfig>,
    time: Res<Time>,
    query: Query<(Entity, &EnemyState, &EnemyType, &FormationSlot)>,
) {
    let dt = time.delta_secs();

    // Count how many enemies are already diving.
    let current_divers = query
        .iter()
        .filter(|(_, s, _, _)| **s == EnemyState::Diving)
        .count() as u32;

    if current_divers >= difficulty.max_concurrent_divers {
        return;
    }

    // Collect InFormation candidates (avoids borrow-checker issues with commands).
    let candidates: Vec<(Entity, EnemyType, Vec2)> = query
        .iter()
        .filter(|(_, s, _, _)| **s == EnemyState::InFormation)
        .map(|(e, _, et, slot)| (e, *et, slot.home_pos))
        .collect();

    if candidates.is_empty() {
        return;
    }

    // Roll the per-frame dive probability (scaled so that `dive_probability`
    // represents the expected probability at 60 fps).
    let mut rng = rand::thread_rng();
    let prob = (difficulty.dive_probability * dt * 60.0).min(1.0);
    if !rng.gen_bool(prob as f64) {
        return;
    }

    // Pick a random candidate.
    let idx = rng.gen_range(0..candidates.len());
    let (entity, enemy_type, home_pos) = candidates[idx];

    // Build the dive path (closed loop — ends at home_pos).
    let path = get_dive_path(enemy_type, home_pos);

    // `current_waypoint = 1`: waypoint 0 is the enemy's current (home) pos;
    // the movement system starts moving toward waypoint 1 immediately.
    let fire_timer = Timer::from_seconds(difficulty.fire_interval, TimerMode::Repeating);
    commands.entity(entity)
        .insert(DivePath(path))
        .insert(DivePathProgress { current_waypoint: 1 })
        .insert(EnemyFireCooldown(fire_timer))
        .insert(EnemyState::Diving); // replaces existing EnemyState component
    commands.trigger(GameAudioEvent::EnemyDive);
}

// ── Dive completion ───────────────────────────────────────────────────────────

/// Detect Diving enemies whose path is exhausted and return them to formation.
///
/// The dive-movement system (next task) advances `DivePathProgress` as the
/// enemy traverses each waypoint.  When all waypoints are consumed the enemy
/// is logically back at its home position, so we revert to `InFormation` and
/// strip the dive components.
pub fn dive_completion_system(
    mut commands: Commands,
    // Exclude TractorBeamRun entities: they follow a one-way path and must
    // remain at the mid-screen stop point; the tractor beam system handles
    // their return separately.
    mut query: Query<
        (Entity, &mut EnemyState, &DivePath, &DivePathProgress),
        (Without<TractorBeamRun>, Without<SplitterPiece>),
    >,
) {
    for (entity, mut state, path, progress) in &mut query {
        if *state == EnemyState::Diving && progress.current_waypoint >= path.0.len() {
            *state = EnemyState::InFormation;
            commands.entity(entity)
                .remove::<DivePath>()
                .remove::<DivePathProgress>()
                .remove::<EnemyFireCooldown>();
        }
    }
}

// ── Group attack coordinator ──────────────────────────────────────────────────

/// Launch a timed group of Bees or Butterflies in a coordinated dive attack.
///
/// Runs on its own `GroupAttackCoordinator` timer (independent of the
/// per-frame `dive_decision_system`) so that genuine wave-formations of 2–4
/// enemies can dive simultaneously.
///
/// Rules:
/// - Respects `DifficultyConfig::max_concurrent_divers`; if the cap leaves no
///   room the timer resets and we try again shortly.
/// - Group sizes: 1 (solo), 2 (pair), 3 or 4 (full group).
/// - Bee groups are preferred (60 % chance); Butterflies dive at most in pairs.
/// - Six-Bee threshold: when ≤ 6 Bees remain the attack size is capped at 1
///   so the few remaining Bees are sent one at a time in rapid succession
///   (they do not return to formation at this point, handled by a shorter
///   inter-attack delay).
pub fn group_attack_system(
    mut commands: Commands,
    time: Res<Time>,
    difficulty: Res<DifficultyConfig>,
    mut coordinator: ResMut<GroupAttackCoordinator>,
    query: Query<(Entity, &EnemyState, &EnemyType, &FormationSlot)>,
) {
    coordinator.timer.tick(time.delta());
    if !coordinator.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    // Count how many enemies are already diving.
    let current_divers = query
        .iter()
        .filter(|(_, s, _, _)| **s == EnemyState::Diving)
        .count() as u32;

    let slots_available =
        difficulty.max_concurrent_divers.saturating_sub(current_divers) as usize;

    if slots_available == 0 {
        // Cap reached — wait a moment and retry.
        coordinator.timer = Timer::from_seconds(1.0, TimerMode::Once);
        return;
    }

    // Collect InFormation candidates by type.
    let mut bee_candidates: Vec<(Entity, Vec2)> = query
        .iter()
        .filter(|(_, s, et, _)| **s == EnemyState::InFormation && **et == EnemyType::Bee)
        .map(|(e, _, _, slot)| (e, slot.home_pos))
        .collect();

    let mut butterfly_candidates: Vec<(Entity, Vec2)> = query
        .iter()
        .filter(|(_, s, et, _)| {
            **s == EnemyState::InFormation && **et == EnemyType::Butterfly
        })
        .map(|(e, _, _, slot)| (e, slot.home_pos))
        .collect();

    if bee_candidates.is_empty() && butterfly_candidates.is_empty() {
        schedule_next(&mut coordinator, &mut rng, false);
        return;
    }

    // Six-Bee threshold: ≤ 6 remaining Bees → force solo, rapid sends.
    let six_bee_mode = bee_candidates.len() <= 6 && !bee_candidates.is_empty();

    // Decide attack type: 60 % Bees, 40 % Butterflies (weighted by availability).
    let use_bees = if bee_candidates.is_empty() {
        false
    } else if butterfly_candidates.is_empty() {
        true
    } else {
        rng.gen_bool(0.6)
    };

    let (enemy_type, candidates) = if use_bees {
        bee_candidates.shuffle(&mut rng);
        (EnemyType::Bee, &mut bee_candidates)
    } else {
        butterfly_candidates.shuffle(&mut rng);
        (EnemyType::Butterfly, &mut butterfly_candidates)
    };

    // Determine how many to send.
    let max_for_type = if enemy_type == EnemyType::Butterfly { 2 } else { 4 };
    let raw_size = if six_bee_mode {
        1 // send one at a time when almost out of Bees
    } else {
        coordinator.next_attack_size
    };
    let send_count = raw_size.min(max_for_type).min(slots_available).min(candidates.len());

    for (entity, home_pos) in candidates.iter().take(send_count).copied() {
        let path = get_dive_path(enemy_type, home_pos);
        let fire_timer =
            Timer::from_seconds(difficulty.fire_interval, TimerMode::Repeating);
        commands
            .entity(entity)
            .insert(DivePath(path))
            .insert(DivePathProgress { current_waypoint: 1 })
            .insert(EnemyFireCooldown(fire_timer))
            .insert(EnemyState::Diving);
        commands.trigger(GameAudioEvent::EnemyDive);
    }

    schedule_next(&mut coordinator, &mut rng, six_bee_mode);
}

/// Reset the coordinator timer and pick the next attack size.
fn schedule_next(coordinator: &mut GroupAttackCoordinator, rng: &mut impl Rng, rapid: bool) {
    // Rapid (six-bee) mode: short 1-2 s intervals for dramatic end-game feel.
    let interval = if rapid {
        rng.gen_range(1.0_f32..2.5)
    } else {
        rng.gen_range(3.0_f32..8.0)
    };
    coordinator.timer = Timer::from_seconds(interval, TimerMode::Once);
    coordinator.next_attack_size = match rng.gen_range(0u32..6) {
        0 => 1,       // solo  (1/6)
        1 | 2 => 2,   // pair  (2/6)
        3 | 4 => 3,   // group (2/6)
        _ => 4,       // full  (1/6)
    };
}

// ── Enemy firing ───────────────────────────────────────────────────────────────

/// Fire bullets from diving enemies toward the player at difficulty-scaled intervals.
///
/// Each diving enemy has its own `EnemyFireCooldown` repeating timer.  When the
/// timer fires, a bullet is spawned aimed at the player's current position.
pub fn enemy_fire_system(
    time: Res<Time>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut enemy_query: Query<(&Transform, &mut EnemyFireCooldown, &EnemyState)>,
    player_query: Query<&Transform, With<PlayerShip>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (transform, mut cooldown, state) in &mut enemy_query {
        if *state != EnemyState::Diving {
            continue;
        }

        cooldown.0.tick(time.delta());

        if !cooldown.0.just_finished() {
            continue;
        }

        let enemy_pos = transform.translation.truncate();
        let dir = (player_pos - enemy_pos).normalize_or_zero();
        // If somehow at the exact same position, fire straight down.
        let dir = if dir == Vec2::ZERO { Vec2::NEG_Y } else { dir };

        commands.spawn((
            Sprite::from_image(game_assets.bullets_image.clone()),
            Transform::from_translation(transform.translation),
            Bullet { owner: BulletOwner::Enemy },
            Velocity(dir * ENEMY_BULLET_SPEED),
            Collider { half_size: Vec2::new(2.0, 4.0) },
        ));
        commands.trigger(GameAudioEvent::EnemyShoot);
    }
}
