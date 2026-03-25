use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::components::{Bullet, BulletOwner, DespawnTimer, Dying, Explosion, PlayerShip};
use crate::resources::ScoreBoard;
use crate::states::GameState;
use super::spawn::do_spawn_player;

const RESPAWN_DELAY_SECS: f32 = 2.5;

/// Tracks the delay before the player ship re-spawns after a death.
#[derive(Resource, Default)]
pub struct RespawnTimer(pub Option<Timer>);

/// Responds to the `Dying` marker on a `PlayerShip` entity:
/// - Clears the player's in-flight bullets
/// - Spawns an explosion marker (visual handled in Phase 10)
/// - Despawns the player ship
/// - Decrements lives; starts the respawn timer or transitions to GameOver
pub fn handle_player_death(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), (With<PlayerShip>, With<Dying>)>,
    bullet_query: Query<(Entity, &Bullet)>,
    mut score_board: ResMut<ScoreBoard>,
    mut respawn_timer: ResMut<RespawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((player_entity, transform)) = player_query.single() else {
        return;
    };

    let pos = transform.translation;

    // Clear the player's in-flight bullets.
    for (entity, bullet) in &bullet_query {
        if bullet.owner == BulletOwner::Player {
            commands.entity(entity).despawn();
        }
    }

    // Spawn explosion marker at player position.
    // Phase 10 will add the sprite/animation; DespawnTimer cleans up the entity.
    commands.spawn((
        Explosion {
            frame: 0,
            timer: Timer::from_seconds(0.08, TimerMode::Repeating),
        },
        Transform::from_translation(pos),
        DespawnTimer(Timer::from_seconds(0.5, TimerMode::Once)),
    ));

    // Despawn the player ship.
    commands.entity(player_entity).despawn();

    // Decrement lives and decide next action.
    score_board.lives = score_board.lives.saturating_sub(1);

    if score_board.lives == 0 {
        next_state.set(GameState::GameOver);
    } else {
        respawn_timer.0 = Some(Timer::from_seconds(RESPAWN_DELAY_SECS, TimerMode::Once));
    }
}

/// Ticks the respawn delay timer and re-spawns the player once it fires.
pub fn tick_respawn(
    mut commands: Commands,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    mut respawn_timer: ResMut<RespawnTimer>,
) {
    let Some(ref mut timer) = respawn_timer.0 else {
        return;
    };

    timer.tick(time.delta());
    if timer.just_finished() {
        respawn_timer.0 = None;
        do_spawn_player(&mut commands, &game_assets);
    }
}
