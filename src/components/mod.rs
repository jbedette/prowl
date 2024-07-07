// Components Module contains the definitions for 
// the game entities that are affected by other
// the actors but are not systems

pub mod ai;
pub mod health;
pub mod money;
pub mod name;
pub mod pending_actions;
pub mod position;
pub mod renderer;
pub mod weapon;
pub mod markers;
pub mod game_resources;
pub mod active;
pub mod map;
pub mod init;

pub use map::TileMap;
pub use ai::AI;
pub use health::Health;
pub use money::Money;
pub use name::Named;
pub use position::Position;
pub use markers::{
    Player,
    Ship,
};
pub use renderer::CharRenderer;
pub use weapon::Weapon;

pub use active::Active;
pub use init::register;
