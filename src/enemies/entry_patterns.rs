// Enemy entry fly-in paths (3 rotating patterns).
//
// Pattern index = (stage - 1) % 3
//
// Each enemy gets:
//   - A staggered delay so they enter single-file rather than all at once.
//   - A sequence of world-space waypoints leading from off-screen to their
//     formation home position.
//
// Entry order (bottom-up, matching original Galaga):
//   Bee row 4  (slots 30–39) → Bee row 3  (slots 20–29)
//   → Butterfly row 2 (slots 12–19) → Butterfly row 1 (slots 4–11)
//   → Boss (slots 0–3)

use bevy::prelude::*;

use crate::components::{EntryPath, EnemyState};

// How quickly enemies travel along their entry path.
const ENTRY_SPEED: f32 = 110.0;

// Delay between consecutive enemies starting their entry.
const ENTRY_STAGGER_SECS: f32 = 0.12;

// ── Public API ───────────────────────────────────────────────────────────────

/// Build the `EntryPath` component for `slot_index`, given the stage pattern
/// (0–2) and the slot's formation home position.
pub fn build_entry_path(slot_index: usize, home_pos: Vec2, pattern: usize) -> EntryPath {
    let delay_secs = entry_delay(slot_index);
    let waypoints = build_waypoints(slot_index, home_pos, pattern);
    EntryPath { waypoints, current_waypoint: 0, speed: ENTRY_SPEED, delay_secs }
}

/// System: advance all `Forming` enemies along their `EntryPath`.
///
/// When an enemy reaches the last waypoint it transitions to `InFormation`
/// so that breathing and dive systems can take over.
pub fn move_forming_enemies(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut EntryPath, &mut EnemyState)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut path, mut state) in &mut query {
        if *state != EnemyState::Forming {
            continue;
        }

        // Count down the pre-entry delay.
        if path.delay_secs > 0.0 {
            path.delay_secs -= dt;
            continue;
        }

        let Some(target) = path.next_target() else {
            // Waypoints exhausted — ensure correct state.
            *state = EnemyState::InFormation;
            continue;
        };

        let pos = transform.translation.truncate();
        let dir = target - pos;
        let dist = dir.length();
        let step = path.speed * dt;

        if dist <= step {
            // Snap to waypoint and advance.
            transform.translation.x = target.x;
            transform.translation.y = target.y;
            path.advance();
            if path.is_complete() {
                *state = EnemyState::InFormation;
            }
        } else {
            let movement = dir.normalize() * step;
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

// ── Internal helpers ─────────────────────────────────────────────────────────

/// Staggered start delay for each slot index.
///
/// Entry order: Bee row 4 first, then row 3, Butterfly row 2, row 1, Boss last.
fn entry_delay(slot_index: usize) -> f32 {
    let order: f32 = match slot_index {
        30..=39 => (slot_index - 30) as f32,        // slots  0–9  in entry order
        20..=29 => 10.0 + (slot_index - 20) as f32, // slots 10–19
        12..=19 => 20.0 + (slot_index - 12) as f32, // slots 20–27
        4..=11  => 28.0 + (slot_index - 4)  as f32, // slots 28–35
        0..=3   => 36.0 + slot_index        as f32, // slots 36–39
        _       => 0.0,
    };
    order * ENTRY_STAGGER_SECS
}

/// Build the waypoint list for the given slot, home position, and pattern.
///
/// `waypoints[0]` is the off-screen spawn / entry point; the movement system
/// snaps there immediately on the first frame (distance = 0) and then moves
/// toward subsequent waypoints.
fn build_waypoints(slot_index: usize, home_pos: Vec2, pattern: usize) -> Vec<Vec2> {
    let from_right = match pattern {
        // Pattern 0: alternate by slot — creates a visual zigzag as enemies
        // pour in from both sides simultaneously.
        0 => slot_index % 2 == 0,

        // Pattern 1: all enemies whose home is on the right enter from the
        // right; left-side enemies from the left.
        1 => home_pos.x >= 0.0,

        // Pattern 2: reversed compared to pattern 1.
        2 => home_pos.x < 0.0,

        _ => slot_index % 2 == 0,
    };

    if from_right {
        right_arc(home_pos)
    } else {
        left_arc(home_pos)
    }
}

/// Off-screen-right → arc across top → home.
fn right_arc(home_pos: Vec2) -> Vec<Vec2> {
    vec![
        Vec2::new(130.0, 150.0),                    // off-screen right (spawn point)
        Vec2::new(85.0, 130.0),                     // arc top-right
        Vec2::new(20.0, home_pos.y + 25.0),         // approach from above
        home_pos,
    ]
}

/// Off-screen-left → arc across top → home.
fn left_arc(home_pos: Vec2) -> Vec<Vec2> {
    vec![
        Vec2::new(-130.0, 150.0),                   // off-screen left (spawn point)
        Vec2::new(-85.0, 130.0),                    // arc top-left
        Vec2::new(-20.0, home_pos.y + 25.0),        // approach from above
        home_pos,
    ]
}
