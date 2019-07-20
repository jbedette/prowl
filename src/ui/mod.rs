use specs::prelude::*;
pub mod panel;
pub mod init;

pub use panel::Panel;

pub fn register(world: &mut World) {
    world.register::<Panel>();
}
