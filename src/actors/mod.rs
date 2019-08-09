pub mod islands;
pub mod populations;

use specs::prelude::*;
pub use islands::island_component::Island;
pub use populations::population_component::Population;

pub fn register(world: &mut World) {
    world.register::<Island>();
    world.register::<Population>();
}
