use specs::prelude::*;

pub mod ai;
pub mod health;
pub mod map;
pub mod money;
pub mod name;
pub mod pending_actions;
pub mod player;
pub mod position;
pub mod renderer;
pub mod rivals;
pub mod weapon;

pub use ai::AI;
pub use health::Health;
pub use map::TileMap;
pub use money::Money;
pub use name::Named;
pub use player::Player;
pub use position::Position;
pub use renderer::CharRenderer;
pub use rivals::Rivals;
pub use weapon::Weapon;

pub fn register(world: &mut World) {
    world.register::<Named>();
    world.register::<Rivals>();
    world.register::<Health>();
    world.register::<Money>();
    world.register::<Weapon>();
    world.register::<Position>();
    world.register::<CharRenderer>();
    world.register::<Player>();
    world.register::<pending_actions::PendingActions>();
    world.register::<AI>();
    world.register::<TileMap>();
}
