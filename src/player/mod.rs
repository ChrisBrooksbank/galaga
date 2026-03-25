pub mod death;
pub mod dual_fighter;
pub mod movement;
pub mod shooting;
pub mod spawn;

pub use death::{handle_player_death, tick_respawn, RespawnTimer};
pub use movement::player_movement;
pub use shooting::{move_bullets, player_shoot};
pub use spawn::spawn_player;
