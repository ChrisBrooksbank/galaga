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

// --- Animation ---

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
