use crate::components::CharRenderer;
use specs::{
    Component,
    VecStorage
};
use specs_derive::Component;
#[allow(unused_imports)]
use crate::shared::Vector2;
#[allow(unused_imports)]
use tcod::colors::Color;

#[derive(Component)]
#[storage(VecStorage)]
pub struct TileMap {
    tiles: Vec<Tile>,
    pub size: Vector2,
}

impl TileMap {
    #[allow(dead_code)]
    pub fn new(size: Vector2) -> Self {
        let vec_size: usize = (size.x * size.y) as usize;
        let tile = Tile::ocean();
        Self {
            tiles: vec![tile; vec_size],
            size
        }
    }

    pub fn place_island(&mut self, position: Vector2) {
        for x in position.x..position.x + 2 {
            for y in position.y..position.y + 2 {
                if let Some(index) = self.vector2_to_index(Vector2::new(x, y)) {
                    self.tiles[index] = Tile::land();
                }
            }
        }
    }

    pub fn get_tile(&self, position: Vector2) -> Option<&Tile> {
        let index = self.vector2_to_index(position);
        match index {
            Some(index) => Some(&self.tiles[index]),
            None => None
        }
    }

    fn vector2_to_index(&self, vector: Vector2) -> Option<usize> {
        if vector.x >= self.size.x ||
            vector.y >= self.size.y ||
            vector.x < 0 ||
            vector.y < 0 { return None }
        Some(((vector.x * self.size.x) + vector.y) as usize)
    }

    pub fn passable_at(&self, position: Vector2) -> bool {
        let tile = self.get_tile(position);
        match tile {
            Some(tile) => tile.passable,
            None => false,
        }
    }

    #[allow(dead_code)]
    pub fn new_checkered(size: Vector2) -> Self {
        let vec_size: usize = (size.x * size.y) as usize;
        let prefabs = [
            Tile::new(TileType::Ocean),
            Tile::new(TileType::Land)];
        let mut tiles = Vec::<Tile>::with_capacity(vec_size);
        for y in 0..size.y {
            for x in 0..size.x {
                if y % 2 == 1 {
                    tiles.push(prefabs[(x % 2) as usize].clone());
                } else {
                    tiles.push(prefabs[(1 - (x % 2)) as usize].clone());
                }
            }
        }
        Self {
            tiles,
            size
        }
    }
}

pub enum TileType {
    Ocean,
    Land,
}

#[derive(Clone)]
pub struct Tile {
    pub renderer: CharRenderer,
    pub passable: bool,
}

impl Tile {
    pub fn ocean() -> Self {
        let color = Color::new(0x10, 0x20, 0x50);
        let bg_color = Color::new(0x04, 0x10, 0x40);
        Self {
            renderer: CharRenderer::with_bg('~', color, bg_color),
            passable: true,
        }
    }
    pub fn land() -> Self {
        let color = Color::new(0x90, 0x70, 0x50);
        let bg_color = Color::new(0x50, 0x40, 0x20);
        Self {
            renderer: CharRenderer::with_bg('O', color, bg_color),
            passable: false,
        }
    }
    pub fn void() -> Self {
        let color = Color::new(0x90, 0x70, 0x50);
        Self {
            renderer: CharRenderer::new(' ', color),
            passable: false,
        }
    }
    pub fn new(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::Ocean => Tile::ocean(),
            TileType::Land => Tile::land(),
        }
    }
}
