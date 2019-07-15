
use specs::prelude::*;

// pub mod input;
mod quit;
pub mod console;
mod window;
pub mod game_data;

// pub use renderer::RendererResource;
pub use quit::Quit;
pub use window::TCODWindow as Window;

// use input::UserInput;
use console::Console;
use game_data::GameData;

// world.add_resource is in the tutorials, but deprecated
// but the new thing doesn't work (?)
#[allow(deprecated)]
pub fn add_all(world: &mut World) {
    world.add_resource(Window::default());
    world.add_resource(Quit(false));
    world.add_resource(Console::new());
    world.add_resource(GameData::default());
}
