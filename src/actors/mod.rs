pub mod islands;
use specs::prelude::*;
pub use islands::island_component::Island;

pub fn register(world: &mut World) {
    world.register::<Island>();
}
