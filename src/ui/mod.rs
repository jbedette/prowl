use specs::prelude::*;
pub mod panel;
pub mod init;
pub mod markers;
pub mod ui_systems;

pub use panel::Panel;

pub fn register(world: &mut World) {
    world.register::<Panel>();
    world.register::<markers::StatusUI>();
    world.register::<markers::InteractiveUI>();
    world.register::<markers::ConsoleUI>();
}
