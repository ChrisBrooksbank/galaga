use std::collections::HashSet;

use bevy::prelude::*;

use crate::components::{
    Bullet, BulletOwner, CapturedBy, CapturedShip, ChallengingFlightPath, Collider, DespawnTimer,
    DualFighter, Dying, EnemyState, EnemyType, Explosion, FormationSlot, Health, PlayerShip,
};
use crate::enemies::formation::{enemy_type_for_slot, slot_index};
use crate::resources::{DualFighterState, Formation};
use crate::scoring::ScoreEvent;

/// Returns true if two axis-aligned bounding boxes overlap.
fn aabb_overlaps(pos_a: Vec2, half_a: Vec2, pos_b: Vec2, half_b: Vec2) -> bool {
    (pos_a.x - pos_b.x).abs() < half_a.x + half_b.x
        && (pos_a.y - pos_b.y).abs() < half_a.y + half_b.y
}

/// Detect collisions between player bullets and enemies.
///
/// On hit:
///   - Despawn the bullet.
///   - Reduce enemy `Health` by 1.
///   - If `Health` reaches 0: spawn explosion marker, trigger `ScoreEvent`,
///     clear the formation slot, and despawn the enemy.
pub fn bullet_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Collider, &Bullet)>,
    mut enemy_query: Query<(
        Entity,
        &Transform,
        &Collider,
        &EnemyState,
        &mut Health,
        &FormationSlot,
    )>,
    mut formation: ResMut<Formation>,
    mut dual_fighter: ResMut<DualFighterState>,
    captured_query: Query<(Entity, &CapturedBy), With<CapturedShip>>,
) {
    // --- Pass 1: collect (bullet_entity, enemy_entity) hit pairs ---
    // Uses read-only iteration so the mutable borrow is free for pass 2.
    let mut bullet_hits: Vec<(Entity, Entity)> = Vec::new();
    let mut used_bullets: HashSet<Entity> = HashSet::new();

    for (bullet_entity, b_tf, b_col, bullet) in &bullet_query {
        if bullet.owner != BulletOwner::Player {
            continue;
        }
        if used_bullets.contains(&bullet_entity) {
            continue;
        }

        let b_pos = b_tf.translation.truncate();

        for (enemy_entity, e_tf, e_col, ..) in enemy_query.iter() {
            let e_pos = e_tf.translation.truncate();
            if aabb_overlaps(b_pos, b_col.half_size, e_pos, e_col.half_size) {
                bullet_hits.push((bullet_entity, enemy_entity));
                used_bullets.insert(bullet_entity);
                break; // one bullet hits at most one enemy
            }
        }
    }

    if bullet_hits.is_empty() {
        return;
    }

    // --- Pass 2: apply damage ---
    let mut despawned_enemies: HashSet<Entity> = HashSet::new();

    for (bullet_entity, enemy_entity) in bullet_hits {
        commands.entity(bullet_entity).despawn();

        if despawned_enemies.contains(&enemy_entity) {
            continue;
        }

        let Ok((_, e_tf, _, enemy_state, mut health, slot)) = enemy_query.get_mut(enemy_entity)
        else {
            continue;
        };

        health.0 = health.0.saturating_sub(1);

        if health.0 == 0 {
            let pos = e_tf.translation;
            let is_diving = *enemy_state == EnemyState::Diving;
            let idx = slot_index(slot.row, slot.col);
            let enemy_type = enemy_type_for_slot(idx);

            // Spawn explosion marker; Phase 10 adds the sprite/animation.
            commands.spawn((
                Explosion {
                    frame: 0,
                    timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                },
                Transform::from_translation(pos),
                DespawnTimer(Timer::from_seconds(0.5, TimerMode::Once)),
            ));

            // Trigger score event; Phase 6 will add the observer that applies points.
            commands.trigger(ScoreEvent { enemy_type, is_diving });

            // Clear the formation grid slot.
            if formation.slots.get(idx) == Some(&Some(enemy_entity)) {
                formation.slots[idx] = None;
            }

            commands.entity(enemy_entity).despawn();
            despawned_enemies.insert(enemy_entity);

            // Boss Galaga death: handle captured ship rescue or cleanup.
            if enemy_type == EnemyType::Boss {
                for (captured_entity, captured_by) in &captured_query {
                    if captured_by.0 != enemy_entity {
                        continue;
                    }
                    // Despawn the captured ship regardless of boss state.
                    commands.entity(captured_entity).despawn();
                    dual_fighter.captured_ship = None;
                    // Rescue only when boss was diving (not in formation).
                    if is_diving {
                        dual_fighter.active = true;
                    }
                    break;
                }
            }
        }
    }
}

/// Detect collisions between enemy bullets and the player ship.
///
/// On hit:
///   - Despawn the enemy bullet.
///   - Add `Dying` marker to the player (processed by `handle_player_death`).
pub fn enemy_bullet_player_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Collider, &Bullet)>,
    player_query: Query<(Entity, &Transform, &Collider), (With<PlayerShip>, Without<Dying>)>,
) {
    let Ok((player_entity, p_tf, p_col)) = player_query.single() else {
        return;
    };
    let p_pos = p_tf.translation.truncate();

    for (bullet_entity, b_tf, b_col, bullet) in &bullet_query {
        if bullet.owner != BulletOwner::Enemy {
            continue;
        }
        let b_pos = b_tf.translation.truncate();
        if aabb_overlaps(b_pos, b_col.half_size, p_pos, p_col.half_size) {
            commands.entity(bullet_entity).despawn();
            commands.entity(player_entity).insert(Dying);
            return; // one hit is enough
        }
    }
}

/// Detect collisions between diving enemies (body) and the player ship.
///
/// On hit:
///   - Add `Dying` marker to the player (processed by `handle_player_death`).
pub fn diving_enemy_player_collision(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &Collider, &EnemyState)>,
    player_query: Query<(Entity, &Transform, &Collider), (With<PlayerShip>, Without<Dying>)>,
) {
    let Ok((player_entity, p_tf, p_col)) = player_query.single() else {
        return;
    };
    let p_pos = p_tf.translation.truncate();

    for (e_tf, e_col, enemy_state) in &enemy_query {
        if *enemy_state != EnemyState::Diving {
            continue;
        }
        let e_pos = e_tf.translation.truncate();
        if aabb_overlaps(e_pos, e_col.half_size, p_pos, p_col.half_size) {
            commands.entity(player_entity).insert(Dying);
            return; // one hit is enough
        }
    }
}

/// Detect collisions between enemy bullets and the dual fighter ship.
///
/// On hit:
///   - Despawn the enemy bullet.
///   - Despawn the dual fighter.
///   - Set `DualFighterState.active = false` (reverts to single ship).
pub fn enemy_bullet_dual_fighter_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Collider, &Bullet)>,
    dual_query: Query<(Entity, &Transform, &Collider), With<DualFighter>>,
    mut dual_state: ResMut<DualFighterState>,
) {
    let Ok((dual_entity, d_tf, d_col)) = dual_query.single() else {
        return;
    };
    let d_pos = d_tf.translation.truncate();

    for (bullet_entity, b_tf, b_col, bullet) in &bullet_query {
        if bullet.owner != BulletOwner::Enemy {
            continue;
        }
        let b_pos = b_tf.translation.truncate();
        if aabb_overlaps(b_pos, b_col.half_size, d_pos, d_col.half_size) {
            commands.entity(bullet_entity).despawn();
            commands.spawn((
                Explosion {
                    frame: 0,
                    timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                },
                Transform::from_translation(d_tf.translation),
                DespawnTimer(Timer::from_seconds(0.5, TimerMode::Once)),
            ));
            commands.entity(dual_entity).despawn();
            dual_state.active = false;
            return;
        }
    }
}

/// Detect collisions between player bullets and challenging-stage enemies.
///
/// Challenging-stage enemies carry `ChallengingFlightPath` instead of
/// `FormationSlot`, so the standard `bullet_enemy_collision` system won't
/// reach them.  This system handles bullet→challenging-enemy hits only.
///
/// On hit:
///   - Despawn the bullet.
///   - Reduce enemy `Health` by 1.
///   - If `Health` reaches 0: spawn explosion, trigger `ScoreEvent`
///     (counted as diving = true), despawn enemy.
pub fn bullet_challenging_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Collider, &Bullet)>,
    mut enemy_query: Query<
        (Entity, &Transform, &Collider, &EnemyType, &mut Health),
        With<ChallengingFlightPath>,
    >,
) {
    let mut used_bullets: std::collections::HashSet<Entity> = std::collections::HashSet::new();

    for (bullet_entity, b_tf, b_col, bullet) in &bullet_query {
        if bullet.owner != BulletOwner::Player {
            continue;
        }
        if used_bullets.contains(&bullet_entity) {
            continue;
        }

        let b_pos = b_tf.translation.truncate();

        for (enemy_entity, e_tf, e_col, enemy_type, mut health) in enemy_query.iter_mut() {
            let e_pos = e_tf.translation.truncate();
            if !aabb_overlaps(b_pos, b_col.half_size, e_pos, e_col.half_size) {
                continue;
            }

            commands.entity(bullet_entity).despawn();
            used_bullets.insert(bullet_entity);

            health.0 = health.0.saturating_sub(1);
            if health.0 == 0 {
                commands.spawn((
                    Explosion {
                        frame: 0,
                        timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                    },
                    Transform::from_translation(e_tf.translation),
                    DespawnTimer(Timer::from_seconds(0.5, TimerMode::Once)),
                ));
                // Challenging stage enemies are always considered "diving".
                commands.trigger(ScoreEvent { enemy_type: *enemy_type, is_diving: true });
                commands.entity(enemy_entity).despawn();
            }
            break;
        }
    }
}

/// Detect collisions between diving enemies (body) and the dual fighter ship.
///
/// On hit:
///   - Despawn the dual fighter.
///   - Set `DualFighterState.active = false` (reverts to single ship).
pub fn diving_enemy_dual_fighter_collision(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &Collider, &EnemyState)>,
    dual_query: Query<(Entity, &Transform, &Collider), With<DualFighter>>,
    mut dual_state: ResMut<DualFighterState>,
) {
    let Ok((dual_entity, d_tf, d_col)) = dual_query.single() else {
        return;
    };
    let d_pos = d_tf.translation.truncate();

    for (e_tf, e_col, enemy_state) in &enemy_query {
        if *enemy_state != EnemyState::Diving {
            continue;
        }
        let e_pos = e_tf.translation.truncate();
        if aabb_overlaps(e_pos, e_col.half_size, d_pos, d_col.half_size) {
            commands.spawn((
                Explosion {
                    frame: 0,
                    timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                },
                Transform::from_translation(d_tf.translation),
                DespawnTimer(Timer::from_seconds(0.5, TimerMode::Once)),
            ));
            commands.entity(dual_entity).despawn();
            dual_state.active = false;
            return;
        }
    }
}
