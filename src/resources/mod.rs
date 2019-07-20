use specs::prelude::*;

pub mod game_data;
pub mod window;

pub use window::TCODWindow as Window;

use crate::console::resource::Console;
use game_data::GameData;

// world.add_resource is in the tutorials, but deprecated
// but the new thing doesn't work (?)
#[allow(deprecated)]
pub fn add_all(world: &mut World) {
    world.add_resource(Window::default());
    world.add_resource(Console::new());
    world.add_resource(GameData::default());
}
