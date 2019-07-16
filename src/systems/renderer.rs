/// Renders Entities
use specs::{
    // Read,
    // Write,
    ReadStorage,
    Entities,
    System,
    // prelude::Resources,
};

use crate::components::{
    CharRenderer,
    Health,
    Money,
    Named,
    Player,
    Position,
    TileMap,
};

#[allow(unused_imports)]
use crate::resources::{
    // window::RenderingConsoles,
    console::{Console, LogLevel},
    // RendererResource,
    Window,
    game_data::GameData,
};

use crate::shared::Vector2;

use tcod::{colors, colors::Color, console::*};

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, CharRenderer>,
        ReadStorage<'a, Named>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Money>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, TileMap>,
        // specs::Write<'a, RendererResource>,
        specs::Write<'a, Console>,
        specs::Write<'a, Window>,
        specs::Read<'a, GameData>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (positions,
             char_renderers,
             names,
             healths,
             moneys,
             players,
             tilemaps,
             mut console,
             mut window,
             game_data,
             entities,
             ) = data;

        let mut offset = Vector2::new(-window.size.x/3, -window.size.y/2);
        for (_player, entity) in (&players, &entities).join() {
            match positions.get(entity) {
                Some(pos) => {
                    offset.x += pos.x;
                    offset.y += pos.y;
                },
                None => (),
            }
        }
        let offset = offset;

        if window.root.window_closed() {
            // quit somehow?
            panic!() // quit somehow!
        }
        let mut root = Offscreen::new(window.size.x, window.size.y);
        renderer::prepare(&mut root);
        // Render map.
        for tilemap in (&tilemaps).join() {
            renderer::put_map(
                &mut root,
                &tilemap,
                &Vector2::zero(),
                &offset,
                &window.size);
        }
        // Render characters.
        for (position, char_renderer) in (&positions, &char_renderers).join() {
            let x = position.x - offset.x;
            let y = position.y - offset.y;
            let x = x as i32;
            let y = y as i32;
            renderer::put_char(
                &mut root,
                Vector2::new(x, y),
                &char_renderer,
            );
        }

        // CONSOLE WINDOW
        let (screen_width, screen_height) = (*window).size.to_tuple();
        let (padding_x, padding_y) = (1, 2);
        let left_edge = screen_width / 2 + 10;
        renderer::put_window(
            &mut root,
            left_edge,
            padding_y - 1,
            screen_width - padding_x,
            screen_height - padding_y,
        );
        let top_left = (left_edge, padding_y + 2);


        // STATS
        for (name, _player, health, money) in (&names, &players, &healths, &moneys).join() {
            renderer::put_text(
                &mut root,
                // &mut window.root,
                Vector2::new(1, 1),
                Vector2::new(10, 5),
                &colors::WHITE,
                &format!(
                    "{}
+ {}/{}
$ {}
T {}",
                    &name.value, &health.current, &health.max, &money.current,
                    &(*game_data).current_turn,
                ),
            );
        }
        // CONSOLE
        let mut i = 0;
        let offset = &console.y_offset;
        for log in &console.logs {
            if log.level == LogLevel::Debug {
                break;
            }
            if i < *offset { i += 1; }
            else {
                renderer::put_text(
                    &mut root,
                    Vector2::new(top_left.0 + 1, top_left.1 + (i - offset) as i32 - 2),
                    Vector2::new(screen_width - left_edge, screen_height - padding_y * 2),
                    &log.get_color(),
                    &log.message,
                );
            }
            i += 1;
        }
        // console.logs = vec![];
        renderer::put_text(
            &mut root,
            // &mut window.root,
            Vector2::new(0, screen_height - 2),
            Vector2::new(10, 5),
            &Color::new(0x00, 0x50, 0x80),
            "[arrows] to move, [esc] to quit",
        );

        window.blit(&root);
    }
}

// Actual rendering logic.
mod renderer {
    use crate::shared::Vector2;
    use crate::components::{
        TileMap,
        map::Tile,
        CharRenderer
    };
    use tcod::{colors, colors::Color, console::*};

    // pub fn prepare(r: &mut Root) {
    pub fn prepare(r: &mut Console) {
        r.set_default_foreground(colors::WHITE);
        r.clear();
    }

    pub fn put_char(r: &mut Console, screen_position: Vector2, character: &CharRenderer) {
        let (x, y) = screen_position.to_tuple();
        if x < 0 || y < 0 || y >= r.height() || x >= r.width() {
            return;
        }
        r.set_default_foreground(character.color.clone());
        let background_flag;
        match character.bg_color {
            Some(color) => {
                r.set_default_background(color);
                background_flag = BackgroundFlag::Set;
            },
            None => background_flag = BackgroundFlag::None,
        }
        r.put_char(x, y, character.character, background_flag)
    }

    pub fn put_window(r: &mut Console, x1: i32, y1: i32, x2: i32, y2: i32) {
        let color = Color::new(0x00, 0x00, 0x00);
        let bg_color = Color::new(0x40, 0x20, 0x20);
        let border_color = Color::new(0x80, 0x40, 0x30);
        let border_bg_color = Color::new(0x50, 0x30, 0x30);
        let border = CharRenderer::with_bg(' ', border_color, border_bg_color);
        let bg = CharRenderer::with_bg(' ', color, bg_color);
        // window bg
        for x in x1..x2 {
            for y in (y1)..(y2) {
                put_char(r, Vector2::new(x, y), &bg);
            }
        }
        let x2 = x2 - 1;
        let y2 = y2 - 1;
        // horizontal border
        for x in x1..(x2 + 1) {
            put_char(r, Vector2::new(x, y1), &border);
            put_char(r, Vector2::new(x, y2), &border);
        }
        // vertical border
        for y in y1..y2 {
            put_char(r, Vector2::new(x1, y), &border);
            put_char(r, Vector2::new(x2, y), &border);
        }
    }

    pub fn put_text(
        r: &mut Offscreen,
        position: Vector2,
        bounds: Vector2,
        color: &Color,
        string: &str)
    {
        if position.y > bounds.y {
            return;
        }
        r.set_default_foreground(color.clone());
        r.print_rect(
            position.x,
            position.y,
            bounds.x,
            0,
            string,
        );
    }

    // puts a Map object on to the screen.
    pub fn put_map(
        r: &mut Console,
        map: &TileMap,
        // top left of map on screen
        screen_position: &Vector2,
        // top left of map on map
        // like to scroll the map
        map_position: &Vector2,
        size: &Vector2)
    {
        let screen_size = Vector2::new(r.width(), r.height());
        let void_tile = Tile::void();
        for x in 0..size.x {
            for y in 0..size.y {
                let tile_position = Vector2::new(
                    map_position.x + x,
                    map_position.y + y);
                let blit_position = Vector2::new(
                             screen_position.x + x,
                             screen_position.y + y);
                let tile = map.get_tile(tile_position);
                let tile = match tile {
                    Some(tile) => tile,
                    None => &void_tile,
                };
                // invalid position
                // TODO check for this before loop dummyo
                if screen_position.y > screen_size.y ||
                    screen_position.y < 0 ||
                    screen_position.x > screen_size.x ||
                    screen_position.x < 0
                { break; }
                put_char(r, blit_position, &tile.renderer);
            }
        }
    }
}
