/// Renders Entities

use specs::{
    // Read,
    // Write,
    ReadStorage,
    // Entities,
    System,
    // prelude::Resources,
};

use crate::components::{
    Position,
    CharRenderer,
};

use crate::resources::RendererResource;

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, CharRenderer>,
        specs::Write<'a, RendererResource>
        // Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            positions,
            char_renderers,
            mut renderer
            // entities
            ) = data;

        renderer.clear();

        for (position, char_renderer) in (&positions, &char_renderers).join() {
            renderer.put_char(
                char_renderer.character,
                char_renderer.color,
                position.x as u16,
                position.y as u16);
        }

        renderer.flush();
    }
}
