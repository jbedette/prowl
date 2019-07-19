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
    console::{
        Console,
        LogLevel
    },
    // RendererResource,
    Window,
    game_data::GameData,
    // ui::{
        // UI,
        // UIPanel
    // },
};

use crate::ui::{
    panel::Panel,
};

use crate::renderer::tcod_renderer;
use tcod_renderer as renderer;

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
        ReadStorage<'a, Panel>,
        specs::Write<'a, Console>,
        // specs::Read<'a, UI>,
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
             panels,
             mut console,
             // ui,
             mut window,
             game_data,
             entities,
             ) = data;

        let mut offset = Vector2::new(-window.size.x/3, -window.size.y/2);
        for (_player, entity) in (&players, &entities).join() {
            match positions.get(entity) {
                Some(pos) => offset += pos.value,
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
        // Render characters, ignoring any that aren't in view.
        {
            let lower_right_corner = window.size + offset;
            for (position, char_renderer) in (&positions, &char_renderers)
                .join()
                .into_iter()
                .filter(|(pos, _)| {
                    (*pos).value.x >= offset.x &&
                    (*pos).value.y >= offset.y &&
                    (*pos).value.x <= lower_right_corner.x &&
                    (*pos).value.y <= lower_right_corner.y
                }) {
                    let (x, y) = (position.value - offset).to_tuple();
                    let x = x as i32;
                    let y = y as i32;
                    renderer::put_char(
                        &mut root,
                        Vector2::new(x, y),
                        &char_renderer,
                    );
                }
        }

        // CONSOLE WINDOW TODO refactor
        let (screen_width, screen_height) = (*window).size.to_tuple();
        let (padding_x, padding_y) = (1, 2);
        let left_edge = screen_width / 2 + 10;
        renderer::put_panel(
            &mut root,
            left_edge,
            padding_y - 1,
            screen_width - padding_x,
            screen_height - padding_y,
        );
        let top_left = (left_edge, padding_y + 2);

        for panel in (&panels).join() {
            renderer::put_panel(
                &mut root,
                panel.position.x,
                panel.position.y,
                panel.position.x + panel.bounds.x,
                panel.position.y + panel.bounds.y
                // panel.top_left.x,
                // panel.top_left.y,
                // panel.top_left.x + panel.bounds.x,
                // panel.top_left.y + panel.bounds.y
                )
        }


        // STATS TODO refactor
        for (name, _player, health, money) in (&names, &players, &healths, &moneys).join() {
            renderer::put_text(
                &mut root,
                // &mut window.root,
                Vector2::new(2, 2),
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
        // CONSOLE TODO refactor
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
        console.logs = vec![];
        // INSTRUCTIONS TODO replace
        renderer::put_text(
            &mut root,
            Vector2::new(2, screen_height - 2),
            Vector2::new(200, 5),
            &Color::new(0x00, 0x50, 0x80),
            "[wasd] to move, [esc] to quit",
        );

        window.blit(&root);
    }
}
