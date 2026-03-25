use bevy::prelude::*;

// --- Player ---

#[derive(Component)]
pub struct PlayerShip;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct FireCooldown(pub Timer);

// --- Enemies ---

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    Boss,
    Butterfly,
    Bee,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum EnemyState {
    Forming,
    InFormation,
    Diving,
    Captured,
}

#[derive(Component)]
pub struct FormationSlot {
    pub row: u8,
    pub col: u8,
    pub home_pos: Vec2,
}

#[derive(Component)]
pub struct Health(pub u8);

#[derive(Component)]
pub struct DivePath(pub Vec<Vec2>);

// --- Bullets ---

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum BulletOwner {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Bullet {
    pub owner: BulletOwner,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

// --- Collider ---

#[derive(Component)]
pub struct Collider {
    pub half_size: Vec2,
}

// --- Captured ship ---

#[derive(Component)]
pub struct CapturedShip;

#[derive(Component)]
pub struct CapturedBy(pub Entity);

// --- Explosion ---

#[derive(Component)]
pub struct Explosion {
    pub frame: usize,
    pub timer: Timer,
}

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

// --- Death / Respawn ---

/// Marker added to the player ship entity when it has been hit and should be destroyed.
/// The `handle_player_death` system processes entities with this marker.
#[derive(Component)]
pub struct Dying;

// --- Entry animation ---

/// Path an enemy follows during its fly-in entry animation.
///
/// `waypoints[0]` doubles as the initial off-screen spawn position; the
/// movement system advances past it immediately (distance == 0) so the
/// enemy effectively starts there and moves toward `waypoints[1]` onward.
#[derive(Component)]
pub struct EntryPath {
    /// Ordered world-space positions.  The last element must equal the
    /// enemy's formation home position.
    pub waypoints: Vec<Vec2>,
    /// Index of the next waypoint to move toward.
    pub current_waypoint: usize,
    /// Units per second along the path.
    pub speed: f32,
    /// Seconds to wait (off-screen) before starting to move.
    pub delay_secs: f32,
}

impl EntryPath {
    pub fn next_target(&self) -> Option<Vec2> {
        self.waypoints.get(self.current_waypoint).copied()
    }

    pub fn advance(&mut self) {
        self.current_waypoint += 1;
    }

    pub fn is_complete(&self) -> bool {
        self.current_waypoint >= self.waypoints.len()
    }
}

// --- Dive path progress ---

/// Tracks how far an enemy has advanced along its `DivePath`.
///
/// `current_waypoint` is the index of the **next** waypoint to move toward.
/// It starts at 1 (index 0 is the enemy's current position / home) and is
/// incremented by the dive-movement system as each waypoint is reached.
/// When `current_waypoint >= DivePath.0.len()` the dive is complete and the
/// state machine transitions the enemy back to `InFormation`.
#[derive(Component, Default)]
pub struct DivePathProgress {
    pub current_waypoint: usize,
}

// --- Enemy firing ---

/// Per-enemy repeating timer that controls when a diving enemy fires a bullet.
/// Inserted when an enemy starts diving; removed when it returns to formation.
#[derive(Component)]
pub struct EnemyFireCooldown(pub Timer);

// --- Tractor beam ---

/// Marker for a Boss Galaga executing a tractor-beam run.
///
/// The boss follows a one-way `DivePath` to the mid-screen stop point
/// (y ≈ −48) rather than a closed-loop dive.  `dive_completion_system`
/// skips entities with this marker so the boss stays at the stop point
/// until the tractor-beam sequence completes.
#[derive(Component)]
pub struct TractorBeamRun;

// --- Animation ---

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
