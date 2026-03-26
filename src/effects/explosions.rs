use bevy::prelude::*;

use crate::assets::{enemy_sprite_index, GameAssets};
use crate::components::{DespawnTimer, Explosion};

const EXPLOSION_FRAMES: [usize; 6] = [
    enemy_sprite_index::EXPLOSION_1,
    enemy_sprite_index::EXPLOSION_2,
    enemy_sprite_index::EXPLOSION_3,
    enemy_sprite_index::EXPLOSION_4,
    enemy_sprite_index::EXPLOSION_5,
    enemy_sprite_index::EXPLOSION_6,
];

/// Initializes newly-spawned `Explosion` entities with their sprite sheet components.
///
/// Runs every frame; the `Added<Explosion>` filter ensures each entity is only
/// processed once (the first frame after the component appears in the world).
pub fn init_explosion_sprites(
    mut commands: Commands,
    query: Query<Entity, Added<Explosion>>,
    game_assets: Res<GameAssets>,
) {
    for entity in &query {
        commands.entity(entity).insert(Sprite::from_atlas_image(
            game_assets.enemies_image.clone(),
            TextureAtlas {
                layout: game_assets.enemies_layout.clone(),
                index: EXPLOSION_FRAMES[0],
            },
        ));
    }
}

/// Advances the explosion animation frame each time the internal repeating timer fires.
pub fn animate_explosions(
    mut query: Query<(&mut Explosion, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut explosion, mut sprite) in &mut query {
        explosion.timer.tick(time.delta());
        if explosion.timer.just_finished() {
            let next_frame = (explosion.frame + 1).min(EXPLOSION_FRAMES.len() - 1);
            explosion.frame = next_frame;
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = EXPLOSION_FRAMES[next_frame];
            }
        }
    }
}

/// Ticks `DespawnTimer` on all entities and despawns them when the timer completes.
pub fn tick_despawn_timers(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnTimer)>,
    time: Res<Time>,
) {
    for (entity, mut despawn_timer) in &mut query {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
