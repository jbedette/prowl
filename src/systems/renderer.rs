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

use crate::resources::{
    RendererResource,
    console::{
        Console,
        // LogLevel,
        // Log
    }
};

use crate::shared::{
    Vector2,
};

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, CharRenderer>,
        specs::Write<'a, RendererResource>,
        specs::Write<'a, Console>
        // Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            positions,
            char_renderers,
            mut renderer,
            mut console,
            // entities
            ) = data;

        renderer.prepare();

        // Render map.
        // TODO

        // Render characters.
        for (position, char_renderer) in (&positions, &char_renderers).join() {
            renderer.put_char(
                Vector2::new(position.x as i32, position.y as i32),
                char_renderer.character
                );
        }
        // Render windows.
        // TODO make this generic somehow? window object that
        // handles printing to itself??
        let (screen_width, screen_height) = renderer.get_bounds().to_tuple();
        let (padding_x, padding_y) = (1, 2);
        renderer.put_window(
            screen_width / 2,
            padding_y,
            screen_width - padding_x,
            screen_height - padding_y,
        );
        // let window_height = screen_height - padding_y * 2;
        let top_left = (screen_width / 2, padding_y + 2);
        let mut i = 0;
        for log in &console.logs {
            renderer.put_text(
                Vector2::new(
                    top_left.0 + 1,
                    top_left.1 + i - 1
                    ),
                &log.message);
            i += 1;
        }
        console.logs = vec![];
        /*
        while let Some(log) = &console.logs.pop() {
            renderer.put_text(
                Vector2::new(
                    top_left.0 + 2,
                    top_left.1 + 1 + i
                    ),
                &log.message);
            i += 1;
            if i >= window_height - 1 {
                console.logs = vec!();
            }
        }
        */
        // renderer.put_text("[wasd] to move, [q] to quit", 1, screen_height);
        // Render to console.
        renderer.flush();
    }
}
