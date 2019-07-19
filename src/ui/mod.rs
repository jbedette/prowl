use specs::prelude::*;
pub mod panel;

pub fn register(world: &mut World) {
    world.register::<panel::Panel>();
}
