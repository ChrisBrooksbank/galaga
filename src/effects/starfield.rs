// Scrolling starfield background (50-100 stars, parallax layers).

use bevy::prelude::*;
use crate::constants::{LOGICAL_WIDTH, LOGICAL_HEIGHT};

const STAR_COUNT: usize = 80;
/// Slowest star scroll speed (pixels/sec) — far layer.
const STAR_SPEED_MIN: f32 = 15.0;
/// Fastest star scroll speed (pixels/sec) — near layer.
const STAR_SPEED_MAX: f32 = 60.0;

/// Half-extents used to place/wrap stars.
const HALF_W: f32 = LOGICAL_WIDTH / 2.0;
const HALF_H: f32 = LOGICAL_HEIGHT / 2.0;

/// Marker component for a star entity. Stores its downward scroll speed.
#[derive(Component)]
pub struct Star {
    pub speed: f32,
}

/// Spawn all star entities once at startup.
pub fn spawn_starfield(mut commands: Commands) {
    // Simple deterministic pseudo-random using xorshift64.
    // We avoid pulling in the `rand` crate to keep dependencies minimal.
    let mut seed: u64 = 0xDEAD_BEEF_CAFE_1234;

    let next = |s: &mut u64| -> f32 {
        // xorshift64
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        // map to 0.0..1.0
        (*s as f32) / (u64::MAX as f32)
    };

    for _ in 0..STAR_COUNT {
        let x = next(&mut seed) * LOGICAL_WIDTH - HALF_W;
        let y = next(&mut seed) * LOGICAL_HEIGHT - HALF_H;
        let t = next(&mut seed); // 0..1 controls speed and brightness
        let speed = STAR_SPEED_MIN + t * (STAR_SPEED_MAX - STAR_SPEED_MIN);

        // Brightness: slow (far) stars are dimmer; fast (near) stars brighter.
        let brightness = 0.3 + t * 0.7;
        let color = Color::srgb(brightness, brightness, brightness);

        // Each star is a tiny 1x1 (or 2x2 for brighter ones) sprite-less quad.
        // We use a Sprite with a plain white image scaled to a small pixel size.
        let size = if t > 0.75 { 2.0 } else { 1.0 };

        commands.spawn((
            Star { speed },
            Sprite {
                color,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_xyz(x, y, -10.0), // behind all gameplay sprites
        ));
    }
}

/// Each frame move every star downward; wrap to top when it exits the bottom.
pub fn scroll_starfield(time: Res<Time>, mut query: Query<(&Star, &mut Transform)>) {
    let dt = time.delta_secs();
    for (star, mut transform) in &mut query {
        transform.translation.y -= star.speed * dt;
        // Wrap: when the star exits the bottom edge, reappear at the top.
        if transform.translation.y < -HALF_H {
            transform.translation.y = HALF_H;
        }
    }
}
