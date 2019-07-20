/// Actual TCOD rendering logic.
use crate::shared::Vector2;
use crate::components::{
    TileMap,
    map::Tile,
    CharRenderer
};
use tcod::{colors, colors::Color, console::*};
use crate::ui::panel::Panel;

pub fn init(r: &mut Console) {
    r.set_default_foreground(colors::WHITE);
    r.clear();
}

pub fn in_bounds(r: &mut Console, screen_position: Vector2) -> bool {
    let (x, y) = screen_position.to_tuple();
    if y >= r.height() || x >= r.width() || x < 0 || y < 0 {
        false
    } else { true }
}

pub fn put_char(
    r: &mut Console,
    screen_position: Vector2,
    character: &CharRenderer
) {
    if !in_bounds(r, screen_position) { return; }
    let (x, y) = screen_position.to_tuple();
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

pub fn put_panel(r: &mut Offscreen, panel: &Panel) {
    let bg_color = panel.border.color;
    let border_color = panel.background.color;
    r.set_default_background(border_color);
    r.rect(
        panel.position.x,
        panel.position.y,
        panel.bounds.x,
        panel.bounds.y,
        true,
        BackgroundFlag::Set,
        );
    r.set_default_foreground(bg_color);
    r.set_default_background(bg_color);
    r.print_rect(
        panel.position.x + 1,
        panel.position.y,
        panel.bounds.x - 3,
        0,
        panel.title.clone()
        );
    r.rect(
        panel.position.x + 1,
        panel.position.y + 1,
        panel.bounds.x - 2,
        panel.bounds.y - 2,
        true,
        BackgroundFlag::Set,
        );
    // TODO widgets
}

pub fn put_text(
    r: &mut Offscreen,
    position: Vector2,
    bounds: Vector2,
    color: &Color,
    string: &str)
{
    r.set_default_foreground(color.clone());
    r.print_rect(
        position.x,
        position.y,
        bounds.x,
        bounds.y,
        string,
    );
}

// puts a Map object on to the screen.
pub fn put_map(
    r: &mut Console,
    map: &TileMap,
    screen_position: &Vector2,
    map_position: &Vector2,
    size: &Vector2)
{
    use std::cmp::{max};
    println!("{}", map_position);
    let screen_size = Vector2::new(r.width(), r.height());
    let void_tile = Tile::void();
    let map_size = *size + *map_position;
    let map_offset = map_position.to_owned();
    let map_position = Vector2::new(
        max(map_position.x, 0),
        max(map_position.y, 0));

    for x in map_position.x..map_size.x {
        // TODO this could be more efficient
        for y in map_position.y..map_size.y {
            let tile_position = Vector2::new(x, y);
            let blit_position = Vector2::new(
                         screen_position.x + (x - map_offset.x),
                         screen_position.y + (y - map_offset.y));
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
