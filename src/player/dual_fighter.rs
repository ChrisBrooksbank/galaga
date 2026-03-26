// Dual fighter: secondary ship spawn, follow, and enhanced firepower.

use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::components::{Collider, DualFighter, PlayerShip};
use crate::player::spawn::PLAYER_Y;
use crate::resources::DualFighterState;
use crate::GameAudioEvent;

/// X offset of the secondary ship from the primary ship centre (px).
/// Negative = to the left, matching the classic Galaga dual-fighter layout.
pub const DUAL_SHIP_OFFSET_X: f32 = -16.0;

/// Spawns the secondary ship when `DualFighterState::active` becomes true;
/// despawns it when dual mode is deactivated.
pub fn manage_dual_fighter(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    dual_state: Res<DualFighterState>,
    player_query: Query<&Transform, With<PlayerShip>>,
    dual_query: Query<Entity, With<DualFighter>>,
) {
    let dual_exists = !dual_query.is_empty();

    if dual_state.active && !dual_exists {
        let start_x = player_query
            .single()
            .map(|t| t.translation.x + DUAL_SHIP_OFFSET_X)
            .unwrap_or(DUAL_SHIP_OFFSET_X);

        commands.spawn((
            Sprite::from_image(game_assets.player_image.clone()),
            Transform::from_xyz(start_x, PLAYER_Y, 0.0),
            DualFighter,
            Collider { half_size: Vec2::new(7.0, 7.0) },
        ));
        commands.trigger(GameAudioEvent::DualFighterJoin);
    } else if !dual_state.active && dual_exists {
        for entity in &dual_query {
            commands.entity(entity).despawn();
        }
    }
}

/// Mirrors the primary ship's X position every frame so the secondary ship
/// always stays `DUAL_SHIP_OFFSET_X` units to the left of the player.
pub fn dual_fighter_follow(
    player_query: Query<&Transform, (With<PlayerShip>, Without<DualFighter>)>,
    mut dual_query: Query<&mut Transform, With<DualFighter>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let target_x = player_transform.translation.x + DUAL_SHIP_OFFSET_X;

    for mut dual_transform in &mut dual_query {
        dual_transform.translation.x = target_x;
    }
}
