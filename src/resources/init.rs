use specs::prelude::*;


use crate::resources;
use crate::console::resource::Console;
use resources::GameData;
use resources::Window;

// world.add_resource is in the tutorials, but deprecated
// but the new thing doesn't work (?)
#[allow(deprecated)]
pub fn register(world: &mut World) {
    world.add_resource(Window::default());
    world.add_resource(Console::new());
    world.add_resource(GameData::default());
}
