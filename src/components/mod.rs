use specs::prelude::*;

pub mod name;
pub mod rivals;
pub mod health;
pub mod weapon;
pub mod money;
pub mod position;
pub mod renderer;
pub mod player;
pub mod ai;
pub mod pending_actions;
pub mod map;

pub use name::Named;
pub use rivals::Rivals;
pub use health::Health;
pub use weapon::Weapon;
pub use money::Money;
pub use position::Position;
pub use renderer::CharRenderer;
pub use player::Player;
pub use ai::AI;
pub use map::TileMap;

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
