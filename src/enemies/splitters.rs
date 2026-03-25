// Splitter/transform enemies: Scorpions, Stingrays, Galaxian Flagships.
//
// Starting at stage 4, one Bee in the formation is designated as a SplitterBee.
// When that Bee is killed, it spawns 3 SplitterPiece enemies that fly off screen.
// If all 3 pieces are destroyed, a stage-type bonus is awarded.
//
// Stage cycle (skipping challenging stages at 3, 7, 11, 15, ...):
//   Stages 4-6   → Scorpions  (1000 pt bonus)
//   Stages 8-10  → Stingrays  (2000 pt bonus)
//   Stages 12-14 → Flagships  (3000 pt bonus)
//   Cycle repeats from stage 16.

use bevy::prelude::*;
use rand::Rng;

use crate::assets::{enemy_sprite_index, GameAssets};
use crate::components::{
    Bullet, BulletOwner, Collider, DespawnTimer, DivePath, DivePathProgress, EnemyFireCooldown,
    EnemyState, Explosion, Health, SplitterPiece, SplitterType,
};
use crate::enemies::dive_paths::splitter_piece_path;
use crate::resources::{ScoreBoard, SplitterState};

// ── Stage-to-type mapping ─────────────────────────────────────────────────────

/// Return the SplitterType for a given stage, or `None` if this stage has no
/// splitter (stages 1-3, and challenging stages at 7, 11, 15, …).
pub fn splitter_type_for_stage(stage: u32) -> Option<SplitterType> {
    if stage < 4 {
        return None;
    }
    let adjusted = stage - 4;
    // Blocks of 4 stages: positions 0-2 are normal, position 3 is challenging.
    let pos_in_block = adjusted % 4;
    if pos_in_block == 3 {
        return None; // challenging stage — no splitter
    }
    let block_idx = adjusted / 4;
    // Each block contributes 3 normal stages to the splitter cycle.
    let normal_stage_idx = block_idx * 3 + pos_in_block;
    match normal_stage_idx % 9 {
        0..=2 => Some(SplitterType::Scorpion),
        3..=5 => Some(SplitterType::Stingray),
        _ => Some(SplitterType::Flagship),
    }
}

/// Bonus points awarded when all 3 splitter pieces are destroyed.
pub fn splitter_bonus_points(splitter_type: SplitterType) -> u32 {
    match splitter_type {
        SplitterType::Scorpion => 1000,
        SplitterType::Stingray => 2000,
        SplitterType::Flagship => 3000,
    }
}

// ── Splitter bee killed event ─────────────────────────────────────────────────

/// Triggered by the collision system when a SplitterBee entity's health reaches 0.
#[derive(Event)]
pub struct SplitterBeeKilled {
    pub position: Vec3,
    pub splitter_type: SplitterType,
}

/// Observer: spawn 3 SplitterPiece enemies when a SplitterBee is destroyed.
///
/// Each piece gets a slightly diverging velocity so they fly off screen in
/// different directions.  Task 2 will replace these with proper looping paths.
pub fn handle_splitter_bee_killed(
    trigger: On<SplitterBeeKilled>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut splitter_state: ResMut<SplitterState>,
) {
    let event = trigger.event();
    let pos = event.position;

    // Reset kill tracking for this set of 3 pieces.
    splitter_state.pieces_alive = 3;
    splitter_state.pieces_killed = 0;
    splitter_state.bonus_awarded = false;

    let sprite_index = match event.splitter_type {
        SplitterType::Scorpion => enemy_sprite_index::SCORPION_1,
        SplitterType::Stingray => enemy_sprite_index::STINGRAY_1,
        SplitterType::Flagship => enemy_sprite_index::FLAGSHIP,
    };

    // Three pieces fan out along diverging looping arc paths (left, centre, right).
    for i in 0u8..3 {
        let path = splitter_piece_path(pos.truncate(), i);
        commands.spawn((
            Sprite::from_atlas_image(
                game_assets.enemies_image.clone(),
                TextureAtlas {
                    layout: game_assets.enemies_layout.clone(),
                    index: sprite_index,
                },
            ),
            Transform::from_translation(pos),
            SplitterPiece { splitter_type: event.splitter_type, piece_index: i },
            Health(1),
            Collider { half_size: Vec2::splat(7.0) },
            DivePath(path),
            DivePathProgress { current_waypoint: 1 },
            EnemyState::Diving,
            EnemyFireCooldown(Timer::from_seconds(1.5, TimerMode::Repeating)),
        ));
    }
}

// ── Movement / completion ──────────────────────────────────────────────────────

/// Despawn SplitterPiece entities when their looping arc path is exhausted.
///
/// Movement itself is handled by `dive_movement_system` (which processes any
/// entity with `DivePath` + `DivePathProgress` + `EnemyState::Diving`).
/// `dive_completion_system` is filtered to exclude `SplitterPiece`, so pieces
/// do NOT return to formation — they are simply despawned here once the path
/// finishes (i.e. they have flown off-screen).
pub fn move_splitter_pieces(
    mut commands: Commands,
    query: Query<(Entity, &DivePath, &DivePathProgress), With<SplitterPiece>>,
    mut splitter_state: ResMut<SplitterState>,
) {
    for (entity, path, progress) in &query {
        if progress.current_waypoint >= path.0.len() {
            commands.entity(entity).despawn();
            // Piece exited off-screen without being shot — count as gone.
            splitter_state.pieces_alive = splitter_state.pieces_alive.saturating_sub(1);
        }
    }
}

// ── Collision ─────────────────────────────────────────────────────────────────

fn aabb_overlaps(pos_a: Vec2, half_a: Vec2, pos_b: Vec2, half_b: Vec2) -> bool {
    (pos_a.x - pos_b.x).abs() < half_a.x + half_b.x
        && (pos_a.y - pos_b.y).abs() < half_a.y + half_b.y
}

/// Detect player bullet vs SplitterPiece collisions.
///
/// On hit:
///   - Despawn the bullet.
///   - Reduce piece `Health` by 1; on death spawn explosion and award 100 pts.
///   - If all 3 pieces killed this stage award the type bonus.
pub fn bullet_splitter_piece_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Collider, &Bullet)>,
    mut piece_query: Query<(Entity, &Transform, &Collider, &SplitterPiece, &mut Health)>,
    mut splitter_state: ResMut<SplitterState>,
    mut score_board: ResMut<ScoreBoard>,
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

        for (piece_entity, p_tf, p_col, piece, mut health) in piece_query.iter_mut() {
            let p_pos = p_tf.translation.truncate();
            if !aabb_overlaps(b_pos, b_col.half_size, p_pos, p_col.half_size) {
                continue;
            }

            commands.entity(bullet_entity).despawn();
            used_bullets.insert(bullet_entity);

            health.0 = health.0.saturating_sub(1);
            if health.0 == 0 {
                // Spawn explosion.
                commands.spawn((
                    Explosion {
                        frame: 0,
                        timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                    },
                    Transform::from_translation(p_tf.translation),
                    DespawnTimer(Timer::from_seconds(0.5, TimerMode::Once)),
                ));

                commands.entity(piece_entity).despawn();

                // Award base points for this piece (diving Bee equivalent).
                score_board.score += 100;
                if score_board.score > score_board.high_score {
                    score_board.high_score = score_board.score;
                }

                splitter_state.pieces_killed += 1;
                splitter_state.pieces_alive = splitter_state.pieces_alive.saturating_sub(1);

                // All-3-killed bonus.
                if splitter_state.pieces_killed >= 3 && !splitter_state.bonus_awarded {
                    splitter_state.bonus_awarded = true;
                    let bonus = splitter_bonus_points(piece.splitter_type);
                    score_board.score += bonus;
                    if score_board.score > score_board.high_score {
                        score_board.high_score = score_board.score;
                    }
                }
            }
            break;
        }
    }
}

// ── Utility: pick splitter bee slot ──────────────────────────────────────────

/// Return a random Bee formation slot index (20–39) to designate as the
/// SplitterBee for this stage.  Called from `do_spawn_formation`.
pub fn random_bee_slot() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(20..40)
}
