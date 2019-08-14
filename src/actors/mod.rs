pub mod islands;
pub mod populations;
pub mod ships;
pub mod player;

use specs::prelude::*;
pub use islands::island_component::Island;
pub use populations::population_component::Population;
// pub use ships::make_ship;

pub fn register(world: &mut World) {
    world.register::<Island>();
    world.register::<Population>();
}
