use crate::components::CharRenderer;
use specs::{
    Entity,
    Component,
    VecStorage
};
use specs_derive::Component;
use tcod::colors::Color;
use crate::shared::{
    Vector2,
    random::random_range,
};

#[derive(Component)]
#[storage(VecStorage)]
pub struct TileMap {
    tiles: Vec<Tile>,
    pub size: Vector2,
    // dynamic_passable_map: Vec<bool>,
    dynamic_map: Vec<Option<Entity>>,
    vec_size: usize,
}
use crate::actors::Island;

impl TileMap {
    #[allow(dead_code)]
    pub fn new(size: Vector2) -> Self {
        let vec_size: usize = (size.x * size.y) as usize;
        let tile = Tile::ocean();
        Self {
            tiles: vec![tile; vec_size],
            size,
            //dynamic_passable_map: vec![false; vec_size],
            dynamic_map: vec![None; vec_size],
            vec_size,
        }
    }

    /*
    pub fn generate(&mut self) {
        for _ in 0..400 {
            self.place_island(Vector2::new(
                    random_range(0, self.size.x as usize) as i32,
                    random_range(0, self.size.y as usize) as i32),
                    Vector2::new(random_range(3,10) as i32,
                        random_range(3,10) as i32)
                );
        }
    }
    */

    /*
    pub fn place_island(&mut self, position: Vector2, size: Vector2) {
        for x in position.x..position.x + size.x {
            for y in position.y..position.y + size.y {
                if let Some(index) = self.vector2_to_index(Vector2::new(x, y)) {
                    self.tiles[index] = Tile::land();
                }
            }
        }
    }
    */
    // TODO this should check if island is valid (on map)
    // and return some kind of fail if island is invalid.
    pub fn place_island(
        &mut self,
        island: &Island,
        position: Vector2,
        entity: Entity)
    {
        //let size = island.size;
        let size = Vector2::new(island.size, island.size);
        for x in position.x..position.x + size.x {
            for y in position.y..position.y + size.y {
                if let Some(index) = self.vector2_to_index(Vector2::new(x, y)) {
                    self.tiles[index] = Tile::land();
                    self.dynamic_map[index] = Some(entity);
                }
            }
        }
    }


    // get ref to tile at x, y
    pub fn get_tile(&self, position: Vector2) -> Option<&Tile> {
        let index = self.vector2_to_index(position);
        match index {
            Some(index) => Some(&self.tiles[index]),
            None => None
        }
    }

    // get index from x, y
    fn vector2_to_index(&self, vector: Vector2) -> Option<usize> {
        if vector.x < 0 || vector.y < 0 { return None }
        let index = ((vector.x * self.size.x) + vector.y) as usize;
        if index < self.vec_size { Some(index) } else { None }
    }

    // get whether tile at x, y is passable
    pub fn passable_at(&self, position: Vector2) -> (bool, Option<Entity>) {
        let tile = self.get_tile(position);
        let index = self.vector2_to_index(position);
        match tile {
            // Some(tile) => self.dynamic_passable_map[index.unwrap()] &&
            Some(tile) => {
                // self.dynamic_map[index.unwrap()].is_none() &&
                // tile.passable
                let entity = self.dynamic_map[index.unwrap()];
                if entity.is_some() {
                    (false, entity)
                } else {
                    (tile.passable, None)
                }
            },
            None => (false, None),
        }
    }

    /*
    pub fn clear_dynamic(&mut self) {
        self.dynamic_map = vec![None; self.vec_size];
    }
    */

    pub fn add_to_dynamic(&mut self, position: Vector2, entity: Entity) {
        let index = self.vector2_to_index(position);
        if let Some(index) = index {
            self.dynamic_map[index] = Some(entity);
        }
    }

    pub fn remove_from_dynamic(&mut self, position: Vector2) {
        let index = self.vector2_to_index(position);
        if let Some(index) = index {
            self.dynamic_map[index] = None;
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    pub renderer: CharRenderer,
    pub passable: bool,
}

impl Tile {
    pub fn ocean() -> Self {
        let color = Color::new(0x20, 0x30, 0x70);
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
            renderer: CharRenderer::with_bg(' ', color, bg_color),
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
    /*
    pub fn new(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::Ocean => Tile::ocean(),
            TileType::Land => Tile::land(),
        }
    }
    */
}
