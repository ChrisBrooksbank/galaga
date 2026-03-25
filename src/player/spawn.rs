use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::components::{Collider, FireCooldown, MovementSpeed, PlayerShip};
use crate::constants::{PLAYER_FIRE_COOLDOWN_SECS, PLAYER_SPEED};

/// Y position of the player ship (24px above the bottom of the 288-tall screen,
/// with origin at center so bottom = -144).
pub const PLAYER_Y: f32 = -120.0;

/// Core spawn logic — usable from both system and respawn timer callback.
pub fn do_spawn_player(commands: &mut Commands, game_assets: &GameAssets) {
    commands.spawn((
        Sprite::from_image(game_assets.player_image.clone()),
        Transform::from_xyz(0.0, PLAYER_Y, 0.0),
        PlayerShip,
        MovementSpeed(PLAYER_SPEED),
        FireCooldown(Timer::from_seconds(PLAYER_FIRE_COOLDOWN_SECS, TimerMode::Once)),
        Collider { half_size: Vec2::new(7.0, 7.0) },
    ));
}

pub fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    do_spawn_player(&mut commands, &game_assets);
}
