use specs::prelude::*;

// pub mod input;
pub mod console;
pub mod game_data;
mod quit;
pub mod ui;
pub mod window;

// pub use renderer::RendererResource;
pub use quit::Quit;
pub use window::TCODWindow as Window;

// use input::UserInput;
use console::Console;
use game_data::GameData;
use ui::UI;

// world.add_resource is in the tutorials, but deprecated
// but the new thing doesn't work (?)
#[allow(deprecated)]
pub fn add_all(world: &mut World) {
    world.add_resource(Window::default());
    world.add_resource(Quit(false));
    world.add_resource(Console::new());
    world.add_resource(GameData::default());
    world.add_resource(UI::default());
}
