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
                char_renderer.character,
                char_renderer.color,
                position.x as u16,
                position.y as u16);
        }
        // Render windows.
        // TODO make this generic somehow? window object that
        // handles printing to itself??
        let (screen_width, screen_height) = renderer.get_bounds();
        let (padding_x, padding_y) = (1, 1);
        // horizontal
        /*
        let window_height = 5;
        let top = screen_height - padding_y - window_height;
        renderer.put_window(
            padding_x, // x1
            top,
            screen_width - padding_x,
            screen_height - padding_y,
        );
        let top_left = (padding_x, top + 1);
        */
        // vertical
        renderer.put_window(
            screen_width / 2,
            padding_y + 1,
            screen_width - padding_x,
            screen_height - padding_y,
        );
        let window_height = screen_height - padding_y * 2;
        let top_left = (screen_width / 2, padding_y + 2);
        let mut i = 0;
        /*
        for log in &console.logs {
            renderer.put_text(&log.message, padding_x + 2, top + 1 + i);
            i += 1;
        }
        console.logs = vec!();
        */
        while let Some(log) = &console.logs.pop() {
            renderer.put_text(&log.message, top_left.0 + 2, top_left.1 + 1 + i);
            i += 1;
            if i >= window_height - 1 {
                console.logs = vec!();
            }
        }
        renderer.put_text("[wasd] to move, [q] to quit", 1, screen_height);
        // Render to console.
        renderer.flush();
    }
}
