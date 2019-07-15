use crate::components::CharRenderer;
use crate::shared::Vector2;
use tcod::colors;
use specs::{
    Component,
    VecStorage
};
use specs_derive::Component;

#[derive(Component)]
#[storage(VecStorage)]
pub struct TileMap {
    tiles: Vec<Tile>,
}

#[derive(Clone)]
pub struct Tile {
    renderer: CharRenderer,
    passable: bool,
}

/*
pub enum Tiles {
    Ocean = Tile {
        renderer: CharRenderer::new('~', colors::Color::new(0x00, 0x00, 0x50)),
        passable: true,
    }
}

impl TileMap {
    pub fn new(size: Vector2) -> Self {
        let vec_size = size.x * size.y;
        Self {
            tiles: vec![
                Tiles::Ocean; vec_size
            ]
        }
    }
}
*/
