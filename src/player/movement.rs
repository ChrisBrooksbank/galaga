use bevy::prelude::*;

use crate::components::{MovementSpeed, PlayerShip};
use crate::constants::LOGICAL_WIDTH;

/// Half the player sprite width used for screen-edge clamping.
const PLAYER_HALF_WIDTH: f32 = 8.0;
const X_MIN: f32 = -LOGICAL_WIDTH / 2.0 + PLAYER_HALF_WIDTH;
const X_MAX: f32 = LOGICAL_WIDTH / 2.0 - PLAYER_HALF_WIDTH;

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&MovementSpeed, &mut Transform), With<PlayerShip>>,
) {
    let Ok((speed, mut transform)) = query.single_mut() else {
        return;
    };

    let mut dx = 0.0;
    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        dx -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        dx += 1.0;
    }

    transform.translation.x += dx * speed.0 * time.delta_secs();
    transform.translation.x = transform.translation.x.clamp(X_MIN, X_MAX);
}
