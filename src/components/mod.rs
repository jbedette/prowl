use specs::prelude::*;

pub mod ai;
pub mod health;
pub mod map;
pub mod money;
pub mod name;
pub mod pending_actions;
pub mod position;
pub mod renderer;
pub mod weapon;
pub mod markers;
pub mod game_resources;

pub use ai::AI;
pub use health::Health;
pub use map::TileMap;
pub use money::Money;
pub use name::Named;
pub use markers::{
    Player,
    Ship,
};
pub use position::Position;
pub use renderer::CharRenderer;
pub use weapon::Weapon;
pub use game_resources::{
    Food,
    Wood,
    Water,
    Metal,
};

pub fn register(world: &mut World) {
    world.register::<Named>();
    world.register::<Health>();
    world.register::<Money>();
    world.register::<Weapon>();
    world.register::<Position>();
    world.register::<CharRenderer>();
    world.register::<Player>();
    world.register::<Ship>();
    world.register::<pending_actions::PendingActions>();
    world.register::<AI>();
    world.register::<TileMap>();
    game_resources::register_all(world);
}
