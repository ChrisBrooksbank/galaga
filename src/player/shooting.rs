use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::components::{Bullet, BulletOwner, Collider, FireCooldown, PlayerShip, Velocity};
use crate::constants::{MAX_PLAYER_BULLETS, PLAYER_BULLET_SPEED};

/// Y offset from player center where bullet spawns.
const BULLET_SPAWN_OFFSET_Y: f32 = 12.0;

pub fn player_shoot(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut player_query: Query<(&Transform, &mut FireCooldown), With<PlayerShip>>,
    bullet_query: Query<&Bullet>,
) {
    let Ok((player_transform, mut cooldown)) = player_query.single_mut() else {
        return;
    };

    cooldown.0.tick(time.delta());

    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    if cooldown.0.elapsed_secs() < cooldown.0.duration().as_secs_f32() {
        return;
    }

    let player_bullet_count = bullet_query
        .iter()
        .filter(|b| b.owner == BulletOwner::Player)
        .count();
    if player_bullet_count >= MAX_PLAYER_BULLETS {
        return;
    }

    let bullet_pos = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y + BULLET_SPAWN_OFFSET_Y,
        0.0,
    );

    commands.spawn((
        Sprite::from_image(game_assets.bullets_image.clone()),
        Transform::from_translation(bullet_pos),
        Bullet { owner: BulletOwner::Player },
        Velocity(Vec2::new(0.0, PLAYER_BULLET_SPEED)),
        Collider { half_size: Vec2::new(2.0, 4.0) },
    ));

    cooldown.0.reset();
}
