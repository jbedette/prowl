//Map 

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

    pub fn get_tile(&self, position: Vector2) -> Option<&Tile> {
        let index = self.vector2_to_index(position);
        match index {
            Some(index) => Some(&self.tiles[index]),
            None => None
        }
    }

    fn set_tile(&mut self, position: Vector2, tile: Tile) -> Result<(),()> {
        let index = self.vector2_to_index(position);
        if let Some(index) = index {
            self.tiles[index] = tile;
            Ok(())
        } else {
            Err(())
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

    // TODO MAKE GENERIC SOMEHOW DANGIT
    pub fn add_wood(&mut self, position: Vector2) -> Result<(), ()> {
        let height;
        if let Some(tile) = self.get_tile(position) {
            height = tile.height;
            let wood_tile = Tile::wood_tile(height);
            return self.set_tile(position, wood_tile)
        }
        Err(())
    }
    // TODO MAKE GENERIC SOMEHOW DANGIT
    pub fn add_metal(&mut self, position: Vector2) -> Result<(), ()> {
        let height;
        if let Some(tile) = self.get_tile(position) {
            height = tile.height;
            let metal_tile = Tile::metal_tile(height);
            return self.set_tile(position, metal_tile)
        }
        Err(())
    }
    // TODO MAKE GENERIC SOMEHOW DANGIT
    pub fn add_food(&mut self, position: Vector2) -> Result<(), ()> {
        let height;
        if let Some(tile) = self.get_tile(position) {
            height = tile.height;
            let food_tile = Tile::food_tile(height);
            return self.set_tile(position, food_tile)
        }
        Err(())
    }
    // TODO MAKE GENERIC SOMEHOW DANGIT
    pub fn add_water(&mut self, position: Vector2) -> Result<(), ()> {
        let height;
        if let Some(tile) = self.get_tile(position) {
            height = tile.height;
            let water_tile = Tile::water_tile(height);
            return self.set_tile(position, water_tile)
        }
        Err(())
    }

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
            Self::ocean_tile(height)
        }
    }

    fn land_tile(height: f64) -> Self {
        let glyph = ' ';
        let height3 = height * height * height * height;
        let r_color = (height3 * 120.0) as u8;
        let g_color = (height3 * 90.0) as u8;
        let b_color = (height3 * 50.0) as u8;
        let fg_color = Color::new(0x20, 0x30, 0x70);
        let bg_color = Color::new(r_color, g_color, b_color);
        Self {
            renderer: CharRenderer::with_bg(glyph, fg_color, bg_color),
            height,
            passable: false,
        }
    }

    fn ocean_tile(height: f64) -> Self {
        // water colors
        let glyph = ' ';
        let height2 = height * height;
        let r = (height2 * 10.0) as u8;
        let g = (height2 * 90.0) as u8;
        let b = ((height2 * 200.0) + 20.0 * (1.0 - height)) as u8;
        let fg_color = Color::new(0x20, 0x30, 0x70);
        let bg_color = Color::new(r, g, b);
        Self {
            renderer: CharRenderer::with_bg(glyph, fg_color, bg_color),
            height,
            passable: true,
        }
    }

    fn water_tile(height: f64) -> Self {
        // water colors
        let glyph = '~';
        let height3 = height * height * height;
        let r = (height3 * 20.0) as u8;
        let g = (height3 * 110.0) as u8;
        let b = (height3 * 220.0) as u8;
        let fg_r_color = (height3 * 40.0) as u8;
        let fg_g_color = (height3 * 160.0) as u8;
        let fg_b_color = (height3 * 250.0) as u8;
        let fg_color = Color::new(fg_r_color, fg_g_color, fg_b_color);
        let bg_color = Color::new(r, g, b);
        Self {
            renderer: CharRenderer::with_bg(glyph, fg_color, bg_color),
            height,
            passable: true,
        }
    }


    fn wood_tile(height: f64) -> Self {
        let glyph = '^';
        let height4 = height * height * height * height;
        let r_color = (height4 * 20.0) as u8;
        let g_color = (height4 * 80.0) as u8;
        let b_color = (height4 * 40.0) as u8;
        let fg_r_color = (height4 * 30.0) as u8;
        let fg_g_color = (height4 * 100.0) as u8;
        let fg_b_color = (height4 * 55.0) as u8;
        let fg_color = Color::new(fg_r_color, fg_g_color, fg_b_color);
        let bg_color = Color::new(r_color, g_color, b_color);
        Self {
            renderer: CharRenderer::with_bg(glyph, fg_color, bg_color),
            height,
            passable: true,
        }
    }

    fn food_tile(height: f64) -> Self {
        let glyph = '"';
        let height4 = height * height * height * height;
        let r_color = (height4 * 80.0) as u8;
        let g_color = (height4 * 110.0) as u8;
        let b_color = (height4 * 50.0) as u8;
        let fg_r_color = (height4 * 100.0) as u8;
        let fg_g_color = (height4 * 140.0) as u8;
        let fg_b_color = (height4 * 90.0) as u8;
        let fg_color = Color::new(fg_r_color, fg_g_color, fg_b_color);
        let bg_color = Color::new(r_color, g_color, b_color);
        Self {
            renderer: CharRenderer::with_bg(glyph, fg_color, bg_color),
            height,
            passable: true,
        }
    }

    fn metal_tile(height: f64) -> Self {
        let glyph = '#';
        let height4 = height * height * height * height;
        let r_color = (height4 * 80.0) as u8;
        let g_color = (height4 * 80.0) as u8;
        let b_color = (height4 * 80.0) as u8;
        let fg_r_color = (height4 * 100.0) as u8;
        let fg_g_color = (height4 * 100.0) as u8;
        let fg_b_color = (height4 * 100.0) as u8;
        let fg_color = Color::new(fg_r_color, fg_g_color, fg_b_color);
        let bg_color = Color::new(r_color, g_color, b_color);
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
