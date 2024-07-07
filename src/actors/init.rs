use specs::prelude::*;


use crate::actors;

pub use actors::islands::island_component::Island;
pub use actors::populations::population_component::Population;

pub fn register(world: &mut World) {
    world.register::<Island>();
    world.register::<Population>();
}