use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::components::{
    Bullet, BulletOwner, Collider, DualFighter, FireCooldown, PlayerFrozen, PlayerShip, Velocity,
};
use crate::constants::{LOGICAL_HEIGHT, LOGICAL_WIDTH, MAX_DUAL_BULLETS, MAX_PLAYER_BULLETS, PLAYER_BULLET_SPEED};
use crate::resources::DualFighterState;
use crate::GameAudioEvent;

/// Y offset from player centre where a bullet spawns.
const BULLET_SPAWN_OFFSET_Y: f32 = 12.0;

pub fn player_shoot(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    dual_state: Res<DualFighterState>,
    // A ship caught in a tractor beam can't fire, as in the arcade.
    mut player_query: Query<(&Transform, &mut FireCooldown), (With<PlayerShip>, Without<PlayerFrozen>)>,
    dual_query: Query<&Transform, With<DualFighter>>,
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

    let max_bullets = if dual_state.active { MAX_DUAL_BULLETS } else { MAX_PLAYER_BULLETS };

    if player_bullet_count >= max_bullets {
        return;
    }

    // Fire from the primary ship.
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
    commands.trigger(GameAudioEvent::PlayerShoot);

    // When dual mode is active, also fire from the secondary ship if the
    // bullet budget allows one more (we already fired one above).
    if dual_state.active && player_bullet_count + 1 < max_bullets {
        if let Ok(dual_transform) = dual_query.single() {
            let dual_bullet_pos = Vec3::new(
                dual_transform.translation.x,
                dual_transform.translation.y + BULLET_SPAWN_OFFSET_Y,
                0.0,
            );
            commands.spawn((
                Sprite::from_image(game_assets.bullets_image.clone()),
                Transform::from_translation(dual_bullet_pos),
                Bullet { owner: BulletOwner::Player },
                Velocity(Vec2::new(0.0, PLAYER_BULLET_SPEED)),
                Collider { half_size: Vec2::new(2.0, 4.0) },
            ));
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Velocity), With<Bullet>>,
) {
    let half_w = LOGICAL_WIDTH / 2.0;
    let half_h = LOGICAL_HEIGHT / 2.0;

    for (entity, mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        let x = transform.translation.x;
        let y = transform.translation.y;
        if y > half_h || y < -half_h || x > half_w || x < -half_w {
            commands.entity(entity).despawn();
        }
    }
}
