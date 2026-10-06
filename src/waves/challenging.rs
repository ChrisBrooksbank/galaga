use bevy::prelude::*;

use crate::assets::{enemy_sprite_index, GameAssets};
use crate::components::{
    AnimationTimer, ChallengingFlightPath, Collider, EnemyType, Health,
};
use crate::constants::{
    CHALLENGING_STAGE_INTERVAL, FIRST_CHALLENGING_STAGE, PERFECT_BONUS_SCORE, TOTAL_ENEMIES,
};
use crate::enemies::spawn::do_spawn_formation;
use crate::resources::{
    ChallengingStageData, ChallengingStageSpawner, DifficultyConfig, Formation, ScoreBoard,
};
use crate::scoring::add_points;
use crate::states::GameState;
use crate::waves::difficulty::update_difficulty_for_stage;
use crate::GameAudioEvent;

// Speed at which challenging-stage enemies fly along their paths.
const CHALLENGING_FLIGHT_SPEED: f32 = 100.0;

// Stagger between enemies within the same group.
const INTRA_GROUP_STAGGER_SECS: f32 = 0.15;

/// Returns true if `stage` is a challenging stage (3, 7, 11, 15, …).
pub fn is_challenging_stage(stage: u32) -> bool {
    if stage < FIRST_CHALLENGING_STAGE {
        return false;
    }
    (stage - FIRST_CHALLENGING_STAGE) % CHALLENGING_STAGE_INTERVAL == 0
}

/// OnEnter(ChallengingStage): reset the per-stage tracker and spawner.
pub fn enter_challenging_stage(
    mut data: ResMut<ChallengingStageData>,
    mut spawner: ResMut<ChallengingStageSpawner>,
) {
    *data = ChallengingStageData {
        spawning_done: false,
        enemies_killed: 0,
        total_enemies: TOTAL_ENEMIES as u32,
    };
    *spawner = ChallengingStageSpawner::default();
}

/// Update system: spawn the next group of enemies when the timer fires.
///
/// Five groups of 8 enemies are launched at `GROUP_DELAY_SECS` intervals,
/// each following a scripted path across the screen.  Once all groups have
/// been launched, `ChallengingStageData::spawning_done` is set to `true`.
pub fn spawn_challenging_stage_patterns(
    mut commands: Commands,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    mut spawner: ResMut<ChallengingStageSpawner>,
    mut data: ResMut<ChallengingStageData>,
) {
    // All groups already launched.
    if data.spawning_done {
        return;
    }

    spawner.timer.tick(time.delta());
    if !spawner.timer.just_finished() {
        return;
    }

    let group = spawner.next_group;
    if group >= ChallengingStageSpawner::GROUP_COUNT {
        data.spawning_done = true;
        return;
    }

    // Launch this group.
    spawn_group(&mut commands, &game_assets, group);

    spawner.next_group += 1;

    if spawner.next_group >= ChallengingStageSpawner::GROUP_COUNT {
        data.spawning_done = true;
    } else {
        // Schedule next group.
        spawner.timer =
            Timer::from_seconds(ChallengingStageSpawner::GROUP_DELAY_SECS, TimerMode::Once);
    }
}

/// Move all challenging-stage enemies along their flight paths.
///
/// Enemies are despawned when they complete their path (exit the screen).
pub fn move_challenging_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut ChallengingFlightPath)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut path) in &mut query {
        // Count down per-enemy stagger delay.
        if path.delay_secs > 0.0 {
            path.delay_secs -= dt;
            continue;
        }

        let Some(&target) = path.waypoints.get(path.current_waypoint) else {
            // Path complete — despawn (enemy flew off screen).
            commands.entity(entity).despawn();
            continue;
        };

        let pos = transform.translation.truncate();
        let dir = target - pos;
        let dist = dir.length();
        let step = path.speed * dt;

        if dist <= step {
            transform.translation.x = target.x;
            transform.translation.y = target.y;
            path.current_waypoint += 1;
        } else {
            let movement = dir.normalize() * step;
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

/// Update system: detect challenging-stage completion.
///
/// A challenging stage is complete when `spawning_done` is true and no enemy
/// entities remain on screen.  On completion the stage counter advances,
/// difficulty is updated, and the next normal formation is spawned before
/// returning to `Playing`.
pub fn challenging_stage_completion(
    mut commands: Commands,
    enemy_query: Query<(), With<EnemyType>>,
    data: ResMut<ChallengingStageData>,
    mut score_board: ResMut<ScoreBoard>,
    mut formation: ResMut<Formation>,
    mut difficulty: ResMut<DifficultyConfig>,
    game_assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Wait until spawning is complete before checking for completion.
    if !data.spawning_done {
        return;
    }
    if !enemy_query.is_empty() {
        return;
    }

    // Perfect bonus: all 40 enemies destroyed earns 10,000 bonus points.
    if data.enemies_killed >= data.total_enemies {
        commands.trigger(GameAudioEvent::PerfectBonus);
        if add_points(&mut score_board, PERFECT_BONUS_SCORE) > 0 {
            commands.trigger(GameAudioEvent::ExtraLife);
        }
    }

    // Advance past the challenging stage to the next normal stage.
    score_board.current_stage += 1;
    update_difficulty_for_stage(&mut difficulty, score_board.current_stage);

    // Reset formation and spawn the next normal wave.
    *formation = Formation::new();
    do_spawn_formation(
        &mut commands,
        &mut formation,
        &game_assets,
        score_board.current_stage,
    );

    next_state.set(GameState::Playing);
}

// ── Pattern helpers ───────────────────────────────────────────────────────────

/// Spawn a single group of 8 enemies using the pattern for `group_index`.
fn spawn_group(commands: &mut Commands, game_assets: &GameAssets, group_index: usize) {
    let (enemy_type, paths, sprite_index) = group_config(group_index);
    let health = match enemy_type {
        EnemyType::Boss => 2,
        _ => 1,
    };

    for (i, waypoints) in paths.into_iter().enumerate() {
        let start = waypoints[0];
        let delay = i as f32 * INTRA_GROUP_STAGGER_SECS;

        let flutter_period = 0.25_f32;
        let phase_offset = (i as f32 * 0.037) % flutter_period;
        let mut anim_timer =
            AnimationTimer(Timer::from_seconds(flutter_period, TimerMode::Repeating));
        anim_timer
            .0
            .set_elapsed(std::time::Duration::from_secs_f32(phase_offset));

        commands.spawn((
            Sprite::from_atlas_image(
                game_assets.enemies_image.clone(),
                TextureAtlas {
                    layout: game_assets.enemies_layout.clone(),
                    index: sprite_index,
                },
            ),
            Transform::from_xyz(start.x, start.y, 0.0),
            enemy_type,
            Health(health),
            Collider { half_size: Vec2::splat(7.0) },
            anim_timer,
            ChallengingFlightPath {
                waypoints,
                current_waypoint: 1, // 0 is the start position
                speed: CHALLENGING_FLIGHT_SPEED,
                delay_secs: delay,
            },
        ));
    }
}

/// Returns (EnemyType, 8 waypoint lists, sprite_index) for the given group.
///
/// Group patterns (5 total, 8 enemies each):
///   0 — Bees,        straight line from right to left
///   1 — Bees,        straight line from left to right
///   2 — Butterflies, wide arc from top-right to bottom-left
///   3 — Butterflies, wide arc from top-left to bottom-right
///   4 — Boss,        figure-8 loop across screen center
fn group_config(
    group_index: usize,
) -> (EnemyType, Vec<Vec<Vec2>>, usize) {
    match group_index % 5 {
        0 => (
            EnemyType::Bee,
            straight_line_right_to_left(),
            enemy_sprite_index::BEE_1,
        ),
        1 => (
            EnemyType::Bee,
            straight_line_left_to_right(),
            enemy_sprite_index::BEE_1,
        ),
        2 => (
            EnemyType::Butterfly,
            wide_arc_right_to_left(),
            enemy_sprite_index::BUTTERFLY_1,
        ),
        3 => (
            EnemyType::Butterfly,
            wide_arc_left_to_right(),
            enemy_sprite_index::BUTTERFLY_1,
        ),
        _ => (
            EnemyType::Boss,
            figure_eight_loop(),
            enemy_sprite_index::BOSS_GREEN_1,
        ),
    }
}

// ── Pattern generators ────────────────────────────────────────────────────────

/// 8 Bees fly in a tight horizontal line from the right side to the left.
/// Each bee enters at a slightly different y-offset to create a spread.
fn straight_line_right_to_left() -> Vec<Vec<Vec2>> {
    let ys: [f32; 8] = [60.0, 50.0, 40.0, 30.0, 20.0, 10.0, 0.0, -10.0];
    ys.iter()
        .map(|&y| {
            vec![
                Vec2::new(130.0, y),  // off-screen right (spawn)
                Vec2::new(-130.0, y), // off-screen left (despawn)
            ]
        })
        .collect()
}

/// 8 Bees fly in a tight horizontal line from the left side to the right.
fn straight_line_left_to_right() -> Vec<Vec<Vec2>> {
    let ys: [f32; 8] = [60.0, 50.0, 40.0, 30.0, 20.0, 10.0, 0.0, -10.0];
    ys.iter()
        .map(|&y| {
            vec![
                Vec2::new(-130.0, y), // off-screen left (spawn)
                Vec2::new(130.0, y),  // off-screen right (despawn)
            ]
        })
        .collect()
}

/// 8 Butterflies sweep in from the top-right in a downward arc, exiting bottom-left.
fn wide_arc_right_to_left() -> Vec<Vec<Vec2>> {
    let offsets: [f32; 8] = [0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0];
    offsets
        .iter()
        .map(|&offset| {
            vec![
                Vec2::new(130.0, 140.0 - offset),  // off-screen top-right
                Vec2::new(60.0, 80.0),              // upper-right
                Vec2::new(0.0, 20.0),               // center
                Vec2::new(-60.0, -40.0),            // lower-left
                Vec2::new(-130.0, -150.0 + offset), // off-screen bottom-left
            ]
        })
        .collect()
}

/// 8 Butterflies sweep in from the top-left in a downward arc, exiting bottom-right.
fn wide_arc_left_to_right() -> Vec<Vec<Vec2>> {
    let offsets: [f32; 8] = [0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0];
    offsets
        .iter()
        .map(|&offset| {
            vec![
                Vec2::new(-130.0, 140.0 - offset), // off-screen top-left
                Vec2::new(-60.0, 80.0),             // upper-left
                Vec2::new(0.0, 20.0),               // center
                Vec2::new(60.0, -40.0),             // lower-right
                Vec2::new(130.0, -150.0 + offset),  // off-screen bottom-right
            ]
        })
        .collect()
}

/// 8 Boss Galagas perform a figure-8 loop across the screen center.
///
/// The path approximates a figure-8 with two large arcs. The first 4 come
/// from the left side and loop clockwise; the other 4 come from the right
/// and loop counter-clockwise, creating a crossing pattern.
fn figure_eight_loop() -> Vec<Vec<Vec2>> {
    let mut paths = Vec::with_capacity(8);

    // Left-side group: enter left, loop down-right, up-left, exit right.
    let left_offsets: [f32; 4] = [0.0, 6.0, 12.0, 18.0];
    for offset in left_offsets {
        paths.push(vec![
            Vec2::new(-130.0, 60.0 + offset), // spawn left
            Vec2::new(-50.0, 100.0),          // upper-left arc
            Vec2::new(0.0, 60.0),             // top-center
            Vec2::new(50.0, 20.0),            // upper-right
            Vec2::new(30.0, -40.0),           // right loop bottom
            Vec2::new(0.0, -80.0),            // bottom-center
            Vec2::new(-30.0, -40.0),          // left loop bottom
            Vec2::new(-50.0, 20.0),           // return left
            Vec2::new(130.0, 60.0 + offset),  // exit right
        ]);
    }

    // Right-side group: enter right, mirror of above.
    let right_offsets: [f32; 4] = [0.0, 6.0, 12.0, 18.0];
    for offset in right_offsets {
        paths.push(vec![
            Vec2::new(130.0, 60.0 + offset),  // spawn right
            Vec2::new(50.0, 100.0),           // upper-right arc
            Vec2::new(0.0, 60.0),             // top-center
            Vec2::new(-50.0, 20.0),           // upper-left
            Vec2::new(-30.0, -40.0),          // left loop bottom
            Vec2::new(0.0, -80.0),            // bottom-center
            Vec2::new(30.0, -40.0),           // right loop bottom
            Vec2::new(50.0, 20.0),            // return right
            Vec2::new(-130.0, 60.0 + offset), // exit left
        ]);
    }

    paths
}
