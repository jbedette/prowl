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
use crate::generators::generate_heightmap;

#[derive(Component)]
#[storage(VecStorage)]
pub struct TileMap {
    tiles: Vec<Tile>,
    pub size: Vector2,
    dynamic_map: Vec<Option<Entity>>,
    vec_size: usize,
    water_level: f64,
}
// use crate::actors::Island;

impl TileMap {
    #[allow(dead_code)]
    pub fn new(size: Vector2, water_level: f64) -> Self {
        let vec_size: usize = (size.x * size.y) as usize;
        // let tile = Tile::ocean();
        let seed = random_range(0, 1000) as u32;
        println!("MAP SEED: {}", seed);
        let heightmap = generate_heightmap(size, seed);
        let mut tiles = vec![];
        for height in heightmap {
            tiles.push(Tile::with_height(height, water_level));
        }
        let dynamic_map = vec![None; vec_size];
        Self {
            tiles,
            size,
            dynamic_map,
            vec_size,
            water_level,
        }
    }

    pub fn get_water_level(&self) -> f64 {
        self.water_level
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
    /*
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
    */

    pub fn add_island(
        &mut self,
        position: Vector2,
        island_entity: Entity) -> Vec<Vector2>
    {
        let mut used_positions = vec![];
        self.add_island_internal(position, island_entity, &mut used_positions);
        used_positions
    }

    fn add_island_internal(
        &mut self,
        position: Vector2,
        island_entity: Entity,
        used_positions: &mut Vec<Vector2>)
    {
        if self.get_entity(position).is_some() { return; }
        if let Some(tile) = self.get_tile(position) {
            if tile.height < self.water_level { return; }
        }
        let distance = 2;
        // add self
        self.add_to_dynamic(position, island_entity);
        used_positions.push(position);
        // try to add neighbors
        for x in -distance..distance {
            for y in -distance..distance {
                if x == 0 && y == 0 { continue; }
                let position = Vector2::new(position.x + x, position.y + y);
                if let Some(tile) = self.get_tile(position) {
                    if tile.height >= self.water_level {
                        self.add_island_internal(position, island_entity, used_positions);
                    }
                }
            }
        }
    }

    pub fn get_entity(&self, position: Vector2) -> Option<Entity> {
        if let Some(index) = self.vector2_to_index(position) {
            self.dynamic_map[index]
        } else {
            None
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
        if vector.x >= self.size.x || vector.y >= self.size.y { return None }
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
    pub height: f64,
    pub passable: bool,
}

impl Tile {
    /*
    pub fn ocean() -> Self {
        let color = Color::new(0x20, 0x30, 0x70);
        let bg_color = Color::new(0x04, 0x10, 0x40);
        Self {
            renderer: CharRenderer::with_bg('~', color, bg_color),
            passable: true,
        }
    }
    */
    pub fn with_height(height: f64, water_level: f64) -> Self {
        let mut height = height;
        let height_cutoff = water_level;
        // bounds checking TODO are these good bounds? should it be an i32 like location?
        if height < 0.0 { eprintln!("HEIGHT IS TOO LOW: {}", height ); height = 0.0; }
        else if height > 1.0 { eprintln!("HEIGHT IS TOO HIGH: {}", height ); height = 1.0 }
        // to blue value (todo scale darker)
        if height > height_cutoff {
            Self::land_tile(height)
        } else {
            Self::water_tile(height)
        }
    }

    fn land_tile(height: f64) -> Self {
        let glyph = ' ';
        let r_color = (((height * height) * 100.0)) as u8;
        let g_color = (((height * height) * 80.0)) as u8;
        let b_color = (((height * height) * 40.0)) as u8;
        let fg_color = Color::new(0x20, 0x30, 0x70);
        let bg_color = Color::new(r_color, g_color, b_color);
        Self {
            renderer: CharRenderer::with_bg(glyph, fg_color, bg_color),
            height,
            passable: false,
        }
    }

    fn water_tile(height: f64) -> Self {
        // water colors
        let r = (height * height * 10.0) as u8;
        let g = (height * height * 90.0) as u8;
        let b = ((height * height * 200.0) + 20.0 * (1.0 - height)) as u8;
        let glyph = ' ';
        let fg_color = Color::new(0x20, 0x30, 0x70);
        let bg_color = Color::new(r, g, b);
        Self {
            renderer: CharRenderer::with_bg(glyph, fg_color, bg_color),
            height,
            passable: true,
        }
    }

    pub fn void() -> Self {
        let color = Color::new(0x90, 0x70, 0x50);
        Self {
            renderer: CharRenderer::new(' ', color),
            height: 0.0,
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
