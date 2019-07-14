
use specs::prelude::*;

mod renderer;
pub mod input;
mod quit;
pub mod console;

pub use renderer::RendererResource;
pub use quit::Quit;
use input::UserInput;
use console::Console;

#[allow(deprecated)]
pub fn add_all(world: &mut World) {
    // world.add_resource is in the tutorials, but deprecated
    // but the new thing doesn't work
    world.add_resource(RendererResource::new());
    world.add_resource(UserInput::new());
    world.add_resource(Quit(false));
    world.add_resource(Console::new());
}
