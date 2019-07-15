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
    Named,
    Health,
    Money,
    Player,
};

use crate::resources::{
    // RendererResource,
    Window,
    // window::RenderingConsoles,
    console::{
        Console,
        // LogLevel,
        // Log
    }
};

use crate::shared::{
    Vector2,
};

use tcod::{
    colors,
    colors::Color,
    console::*,
};

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, CharRenderer>,
        ReadStorage<'a, Named>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Money>,
        ReadStorage<'a, Player>,
        // specs::Write<'a, RendererResource>,
        specs::Write<'a, Console>,
        specs::Write<'a, Window>
        // Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            positions,
            char_renderers,
            names,
            healths,
            moneys,
            players,
            mut console,
            mut window,
        ) = data;

        if window.root.window_closed() {
            // quit somehow?
            panic!() // quit somehow!
        }

        let mut root = Offscreen::new(window.size.x, window.size.y);

        // renderer::prepare(&mut window.root);
        renderer::prepare(&mut root);

        // Render map.
        // TODO

        // Render characters.
        for (position, char_renderer) in (&positions, &char_renderers).join() {
            renderer::put_char(
                &mut root,
                Vector2::new(position.x as i32, position.y as i32),
                &char_renderer.color,
                char_renderer.character
            );
        }
        // Render from offscreen to window
        // TODO these still render to main window...
        // Render windows.
        // TODO use TCOD panels
        let (screen_width, screen_height) = (*window).size.to_tuple();
        let (padding_x, padding_y) = (1, 2);
        renderer::put_window(
            &mut root,
            screen_width / 2,
            padding_y,
            screen_width - padding_x,
            screen_height - padding_y,
        );
        window.blit(&root);

        // STATS
        for (name, _player, health, money) in (&names, &players, &healths, &moneys).join() {
            renderer::put_text(
                // &mut root,
                &mut window.root,
                Vector2::new(1, 1),
                &colors::WHITE,
                &format!(
"{}
+ {}/{}
$ {}",
                         &name.value,
                         &health.current,
                         &health.max,
                         &money.current));
        }
        // CONSOLE
        let top_left = (screen_width / 2, padding_y + 2);
        let mut i = 0;
        for log in &console.logs {
            renderer::put_text(
                // &mut root,
                &mut window.root,
                Vector2::new(
                    top_left.0 + 1,
                    top_left.1 + i - 1
                ),
                &log.get_color(),
                &log.message);
            i += 1;
        }
        console.logs = vec![];
        renderer::put_text(
            // &mut root,
            &mut window.root,
            Vector2::new(top_left.0 + 1, screen_height - 3),
            &Color::new(0x00, 0x50, 0x80),
            "[arrows] to move, [esc] to quit");

        // TODO delete this, do it all in blit
        window.flush();
    }
}

// Actual rendering logic.
mod renderer {
    use tcod::{
        console::*,
        colors,
        colors::Color,
    };
    use crate::shared::{
        Vector2,
    };

    // pub fn prepare(r: &mut Root) {
    pub fn prepare(r: &mut Console) {
        r.set_default_foreground(colors::WHITE);
        r.clear();
    }

    pub fn put_char(
        r: &mut Console,
        screen_position: Vector2,
        color: &Color,
        character: char)
    {
        let (x, y) = screen_position.to_tuple();
        if x < 0 || y < 0 {
            println!("PUTCHAR ERROR: X/Y LESS THAN 0 -> x: {}, y: {}, char: {}",
                     x, y, character);
            return
        }
        r.set_default_foreground(color.clone());
        r.put_char(x, y, character, BackgroundFlag::None)
    }

    pub fn put_window(r: &mut Console, x1: i32, y1: i32, x2: i32, y2: i32) {
        let border = '+';
        let color = Color::new(0x00, 0x50, 0x80);
        for x in (x1 + 1)..(x2 - 1) {
            for y in (y1 + 1)..(y2 - 1) {
                put_char(r, Vector2::new(x, y), &color, ' ');
            }
        }
        for x in x1..x2 {
            put_char(r, Vector2::new(x, y1), &color, border);
            put_char(r, Vector2::new(x, y2), &color, border);
        }
        for y in y1..y2 {
            put_char(r, Vector2::new(x1, y), &color, border);
            put_char(r, Vector2::new(x2, y), &color, border);
        }
    }

    pub fn put_text(
        r: &mut Root,
        position: Vector2,
        color: &Color,
        string: &str)
    {
        r.set_default_foreground(color.clone());
        r.print_ex(
            position.x,
            position.y,
            BackgroundFlag::None,
            TextAlignment::Left,
            string
        );
    }
}
