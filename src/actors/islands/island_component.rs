use specs::{Component, VecStorage};
use specs_derive::Component;
use crate::shared::Vector2;

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct Island{
    // pub size: i32,  //affect island size on map
    // pub arable: i32,//affect max pop
    pub tile_positions: Vec<Vector2>, // positions of island on map
    pub coast_tile_positions: Vec<Vector2>,
}

impl Island{
    pub fn new(tile_positions: Vec<Vector2>) -> Self {
        let coast_tile_positions = Self::calculate_coastline(&tile_positions);
        Self { tile_positions, coast_tile_positions }
    }

    fn calculate_coastline(tile_positions: &Vec<Vector2>) -> Vec<Vector2> {
        let mut coast_tile_positions = vec![];
        for position in tile_positions {
            for x in -1..1 {
                for y in -1..1 {
                    let position = Vector2::new(position.x + x, position.y + y);
                    if !tile_positions.contains(&position) {
                        coast_tile_positions.push(position);
                    }
                }
            }
        }
        coast_tile_positions
    }
}
