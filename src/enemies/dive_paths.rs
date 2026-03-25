// Predefined dive waypoint curves for each enemy type.
//
// All coordinates are in the original 224×288 game space with origin at
// centre.  X range: −112 … +112.  Y range: −144 … +144.
// Formation occupies roughly y = 16 … 80 at the top of the screen.
// The player sits at y = −120 near the bottom.
//
// Convention
// ──────────
// Every normal dive path is a closed loop: it begins **and ends** at the
// enemy's `home_pos` so that when the movement system finishes walking all
// waypoints the enemy is back in formation.
//
// The tractor-beam path is one-way: it ends at the mid-screen stop point
// (y ≈ −48, ~2/3 down the screen).  The tractor-beam system handles the
// return journey separately.

use bevy::prelude::*;

use crate::components::EnemyType;

// ── Off-screen anchor constants ───────────────────────────────────────────────

/// Just beyond the left edge (screen left = −112).
const OFF_L: f32 = -125.0;
/// Just beyond the right edge (screen right = +112).
const OFF_R: f32 = 125.0;
/// Just below the bottom edge (screen bottom = −144).
const OFF_BOTTOM: f32 = -158.0;
/// Just above the top edge (screen top = +144).
const OFF_TOP: f32 = 158.0;

/// Y coordinate where the tractor-beam Boss stops (~2/3 down from screen top).
///
/// Top = +144, bottom = −144.  Two-thirds of 288 from the top → y = −48.
pub const TRACTOR_BEAM_STOP_Y: f32 = -48.0;

// ── Public API ────────────────────────────────────────────────────────────────

/// Return the predefined dive waypoints for the given enemy type and home
/// position.
///
/// The path begins **and ends** at `home_pos` (a closed loop).  The enemy
/// movement system advances through each waypoint in order; once the last
/// waypoint is reached the enemy has returned to its formation slot.
///
/// The dive "side" (left vs. right) is inferred from `home_pos.x`: negative
/// values → left-side path; zero or positive → right-side path.
pub fn get_dive_path(enemy_type: EnemyType, home_pos: Vec2) -> Vec<Vec2> {
    let from_right = home_pos.x >= 0.0;
    match enemy_type {
        EnemyType::Bee => bee_dive(home_pos, from_right),
        EnemyType::Butterfly => butterfly_dive(home_pos, from_right),
        EnemyType::Boss => boss_dive(home_pos, from_right),
    }
}

/// Waypoints for a Boss Galaga tractor-beam run.
///
/// **One-way path**: ends at x = 0, y = [`TRACTOR_BEAM_STOP_Y`] (mid-screen).
/// The tractor-beam system is responsible for the return journey once the
/// beam sequence concludes.
pub fn boss_tractor_beam_path(home_pos: Vec2) -> Vec<Vec2> {
    let from_right = home_pos.x >= 0.0;
    // sx: +1 for right-side Boss, −1 for left-side Boss
    let sx: f32 = if from_right { 1.0 } else { -1.0 };

    vec![
        home_pos,
        Vec2::new(sx * 90.0, 60.0),                // sweep out to the side
        Vec2::new(sx * 108.0, -10.0),              // arc down the side
        Vec2::new(sx * 75.0, -60.0),               // curve back toward centre
        Vec2::new(sx * 25.0, TRACTOR_BEAM_STOP_Y), // approach stop point
        Vec2::new(0.0, TRACTOR_BEAM_STOP_Y),        // mid-screen stop
    ]
}

// ── Bee paths ─────────────────────────────────────────────────────────────────
//
// Bees execute a classic dive-bomb: plunge toward the player, exit the
// bottom of the screen, loop around the outside (off-screen), and re-enter
// from the top to return to formation.

fn bee_dive(home_pos: Vec2, from_right: bool) -> Vec<Vec2> {
    if from_right {
        bee_dive_right(home_pos)
    } else {
        bee_dive_left(home_pos)
    }
}

fn bee_dive_right(home: Vec2) -> Vec<Vec2> {
    vec![
        home,
        Vec2::new(70.0, 20.0),      // sweep right and slightly down
        Vec2::new(90.0, -60.0),     // dive along right side
        Vec2::new(75.0, -120.0),    // through player area (right)
        Vec2::new(40.0, OFF_BOTTOM),// exit screen bottom
        Vec2::new(OFF_L, -80.0),    // wrap to left side (off-screen)
        Vec2::new(OFF_L, 60.0),     // climb up left side
        Vec2::new(-50.0, OFF_TOP),  // exit top-left
        Vec2::new(0.0, OFF_TOP),    // cross top off-screen toward home
        home,
    ]
}

fn bee_dive_left(home: Vec2) -> Vec<Vec2> {
    vec![
        home,
        Vec2::new(-70.0, 20.0),
        Vec2::new(-90.0, -60.0),
        Vec2::new(-75.0, -120.0),
        Vec2::new(-40.0, OFF_BOTTOM),
        Vec2::new(OFF_R, -80.0),
        Vec2::new(OFF_R, 60.0),
        Vec2::new(50.0, OFF_TOP),
        Vec2::new(0.0, OFF_TOP),
        home,
    ]
}

// ── Butterfly paths ───────────────────────────────────────────────────────────
//
// Butterflies are faster and more evasive: they dart to one side, swoop
// down through the player zone, then zip back up the opposite side.

fn butterfly_dive(home_pos: Vec2, from_right: bool) -> Vec<Vec2> {
    if from_right {
        butterfly_dive_right(home_pos)
    } else {
        butterfly_dive_left(home_pos)
    }
}

fn butterfly_dive_right(home: Vec2) -> Vec<Vec2> {
    vec![
        home,
        Vec2::new(105.0, 30.0),    // dart hard right
        Vec2::new(110.0, -50.0),   // drop along right edge
        Vec2::new(60.0, -100.0),   // swing toward centre
        Vec2::new(0.0, -120.0),    // through player area, centre
        Vec2::new(-60.0, -90.0),   // sweep left
        Vec2::new(-105.0, 0.0),    // zip up left side
        Vec2::new(-60.0, 80.0),    // approach top-left
        Vec2::new(-5.0, OFF_TOP),  // exit top
        home,
    ]
}

fn butterfly_dive_left(home: Vec2) -> Vec<Vec2> {
    vec![
        home,
        Vec2::new(-105.0, 30.0),
        Vec2::new(-110.0, -50.0),
        Vec2::new(-60.0, -100.0),
        Vec2::new(0.0, -120.0),
        Vec2::new(60.0, -90.0),
        Vec2::new(105.0, 0.0),
        Vec2::new(60.0, 80.0),
        Vec2::new(5.0, OFF_TOP),
        home,
    ]
}

// ── Boss paths ────────────────────────────────────────────────────────────────
//
// Boss Galagas make wide, sweeping loops that cover most of the screen.
// Escort Butterflies follow the same path (offset slightly); their
// assignment is handled by the AI system — here we only define the Boss's
// own trajectory.

fn boss_dive(home_pos: Vec2, from_right: bool) -> Vec<Vec2> {
    if from_right {
        boss_dive_right(home_pos)
    } else {
        boss_dive_left(home_pos)
    }
}

fn boss_dive_right(home: Vec2) -> Vec<Vec2> {
    vec![
        home,
        Vec2::new(90.0, 55.0),      // sweep right from formation
        Vec2::new(108.0, -10.0),    // far right, start dropping
        Vec2::new(95.0, -90.0),     // right side, deep
        Vec2::new(50.0, -130.0),    // bottom right
        Vec2::new(0.0, -145.0),     // centre bottom (off-screen)
        Vec2::new(-60.0, -120.0),   // bottom left
        Vec2::new(-105.0, -30.0),   // left side, climbing
        Vec2::new(-108.0, 60.0),    // far left upper
        Vec2::new(-50.0, OFF_TOP),  // top left (off-screen)
        Vec2::new(5.0, OFF_TOP),    // top centre (off-screen)
        home,
    ]
}

fn boss_dive_left(home: Vec2) -> Vec<Vec2> {
    vec![
        home,
        Vec2::new(-90.0, 55.0),
        Vec2::new(-108.0, -10.0),
        Vec2::new(-95.0, -90.0),
        Vec2::new(-50.0, -130.0),
        Vec2::new(0.0, -145.0),
        Vec2::new(60.0, -120.0),
        Vec2::new(105.0, -30.0),
        Vec2::new(108.0, 60.0),
        Vec2::new(50.0, OFF_TOP),
        Vec2::new(-5.0, OFF_TOP),
        home,
    ]
}

// ── Splitter piece paths ──────────────────────────────────────────────────────
//
// Each of the three splitter pieces gets a distinct one-way arc path that fans
// out from the spawn position, sweeps through a wide loop, and exits off-screen.
// These are NOT closed loops — pieces are despawned when the path is exhausted.
//
// piece_index 0 → left piece  (counterclockwise arc, exits off left)
// piece_index 1 → center piece (clockwise arc,       exits off bottom)
// piece_index 2 → right piece (clockwise arc,        exits off right)

/// Return the one-way flight path for a splitter piece.
///
/// `spawn_pos` is the world position where the SplitterBee was destroyed.
/// `piece_index` selects one of three diverging trajectories (0 = left,
/// 1 = centre, 2 = right).
pub fn splitter_piece_path(spawn_pos: Vec2, piece_index: u8) -> Vec<Vec2> {
    match piece_index {
        0 => splitter_left_path(spawn_pos),
        1 => splitter_center_path(spawn_pos),
        _ => splitter_right_path(spawn_pos),
    }
}

/// Left piece: sweeps upper-left, wide counterclockwise loop, exits off left.
fn splitter_left_path(pos: Vec2) -> Vec<Vec2> {
    vec![
        pos,
        Vec2::new(pos.x - 30.0, pos.y + 15.0), // diverge up-left from spawn
        Vec2::new(-90.0, 50.0),                  // reach upper-left screen area
        Vec2::new(-105.0, -30.0),                // descend left edge
        Vec2::new(-80.0, -100.0),                // bottom-left
        Vec2::new(0.0, -125.0),                  // cross bottom centre
        Vec2::new(75.0, -85.0),                  // arc bottom-right
        Vec2::new(85.0, -5.0),                   // climb right side
        Vec2::new(45.0, 55.0),                   // arc upper-right (loop complete)
        Vec2::new(OFF_L, 45.0),                  // exit off left
    ]
}

/// Centre piece: slight right drift, clockwise loop, exits off the bottom.
fn splitter_center_path(pos: Vec2) -> Vec<Vec2> {
    vec![
        pos,
        Vec2::new(pos.x + 10.0, pos.y - 15.0), // slight right-down drift
        Vec2::new(65.0, -10.0),                  // arc right, mid-screen
        Vec2::new(85.0, -80.0),                  // right side lower
        Vec2::new(40.0, -125.0),                 // bottom-right
        Vec2::new(-35.0, -130.0),                // cross bottom to left
        Vec2::new(-75.0, -70.0),                 // left side lower
        Vec2::new(-45.0, 5.0),                   // arc back toward centre (loop complete)
        Vec2::new(0.0, OFF_BOTTOM),              // exit off bottom
    ]
}

/// Right piece: sweeps upper-right, wide clockwise loop, exits off right
/// (mirror of the left piece).
fn splitter_right_path(pos: Vec2) -> Vec<Vec2> {
    vec![
        pos,
        Vec2::new(pos.x + 30.0, pos.y + 15.0), // diverge up-right from spawn
        Vec2::new(90.0, 50.0),                   // reach upper-right screen area
        Vec2::new(105.0, -30.0),                 // descend right edge
        Vec2::new(80.0, -100.0),                 // bottom-right
        Vec2::new(0.0, -125.0),                  // cross bottom centre
        Vec2::new(-75.0, -85.0),                 // arc bottom-left
        Vec2::new(-85.0, -5.0),                  // climb left side
        Vec2::new(-45.0, 55.0),                  // arc upper-left (loop complete)
        Vec2::new(OFF_R, 45.0),                  // exit off right
    ]
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// All normal dive paths must start and end at home_pos (closed loop).
    #[test]
    fn dive_paths_are_closed_loops() {
        let homes = [
            Vec2::new(40.0, 32.0),   // right bee
            Vec2::new(-40.0, 32.0),  // left bee
            Vec2::new(40.0, 64.0),   // right butterfly
            Vec2::new(-40.0, 64.0),  // left butterfly
            Vec2::new(8.0, 80.0),    // right boss
            Vec2::new(-8.0, 80.0),   // left boss
        ];
        let types = [
            EnemyType::Bee,
            EnemyType::Bee,
            EnemyType::Butterfly,
            EnemyType::Butterfly,
            EnemyType::Boss,
            EnemyType::Boss,
        ];

        for (home, enemy_type) in homes.iter().zip(types.iter()) {
            let path = get_dive_path(*enemy_type, *home);
            assert!(
                path.len() >= 3,
                "{enemy_type:?} path too short (len={})",
                path.len()
            );
            assert_eq!(
                *path.first().unwrap(),
                *home,
                "{enemy_type:?} path must start at home_pos"
            );
            assert_eq!(
                *path.last().unwrap(),
                *home,
                "{enemy_type:?} path must end at home_pos"
            );
        }
    }

    /// Tractor beam path must start at home_pos and end at the stop point.
    #[test]
    fn tractor_beam_path_endpoints() {
        let home = Vec2::new(8.0, 80.0);
        let path = boss_tractor_beam_path(home);

        assert_eq!(*path.first().unwrap(), home, "must start at home_pos");
        assert_eq!(
            path.last().unwrap().y,
            TRACTOR_BEAM_STOP_Y,
            "must end at tractor-beam stop Y"
        );
        assert_eq!(
            path.last().unwrap().x,
            0.0,
            "must end at x = 0 (screen centre)"
        );
    }

    /// Right-side paths should stay on (or reach) the right half at some point.
    #[test]
    fn right_paths_reach_right_side() {
        let home = Vec2::new(40.0, 32.0);
        let path = get_dive_path(EnemyType::Bee, home);
        assert!(
            path.iter().any(|p| p.x > 50.0),
            "right bee path should reach the right side"
        );
    }

    /// Left-side paths should stay on (or reach) the left half at some point.
    #[test]
    fn left_paths_reach_left_side() {
        let home = Vec2::new(-40.0, 32.0);
        let path = get_dive_path(EnemyType::Bee, home);
        assert!(
            path.iter().any(|p| p.x < -50.0),
            "left bee path should reach the left side"
        );
    }
}
