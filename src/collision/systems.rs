use std::collections::HashSet;

use bevy::prelude::*;

use crate::components::{
    Bullet, BulletOwner, Collider, DespawnTimer, EnemyState, Explosion, FormationSlot, Health,
};
use crate::enemies::formation::{enemy_type_for_slot, slot_index};
use crate::resources::Formation;
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
        }
    }
}
