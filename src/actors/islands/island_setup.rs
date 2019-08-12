use specs::prelude::*;
use crate::file_io;

use crate::{
    components::{
        map::TileMap,
        Named,
        game_resources::{
            Wood,
            Metal,
            Food,
            Water,
            GameResource,
        }
    },
    actors::Island,
    shared::{
        Vector2,
        random::random_range,
    }
};

pub struct IslandSetupSystem;

#[allow(unused_must_use)]
#[allow(dead_code)]
impl<'a> System<'a> for IslandSetupSystem {
    type SystemData = (
        WriteStorage<'a, Island>,
        WriteStorage<'a, Named>,
        WriteStorage<'a, TileMap>,
        // resources
        WriteStorage<'a, GameResource<Wood>>,
        WriteStorage<'a, GameResource<Metal>>,
        WriteStorage<'a, GameResource<Food>>,
        WriteStorage<'a, GameResource<Water>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut islands,
            mut names,
            mut maps,
            mut woods,
            mut metals,
            mut foods,
            mut waters,
            entities
        ) = data;
        let island_names = file_io::read_file("isoles.txt");

        for map in (&mut maps).join() {
            let water_level = map.get_water_level();
            for x in 0..map.size.x {
                for y in 0..map.size.y {
                    let position = Vector2::new(x, y);
                    // if this position is already claimed by another thing, just move along.
                    if map.get_entity(position).is_some() { continue; }
                    // otherwise check the tile.
                    if let Some(tile) = map.get_tile(position) {
                        if tile.height < water_level { continue; }
                        // if island is above water,
                        // add island to tile
                        let island_entity = entities.create();
                        let name = &(island_names[(random_range(0, island_names.len()))]);
                        names.insert(island_entity, Named::new(name));
                        let island_positions = map.add_island(position, island_entity);
                        // Resources -- TODO clean me up
                        {
                            let size = island_positions.len();
                            let resource_count = size / random_range(6, 12);
                            for _ in 0..resource_count {
                                let position = island_positions[random_range(0, size)];
                                let which_resource = random_range(0, 4);
                                match which_resource {
                                    0 => {
                                        map.add_wood(position);
                                        if let Some(wood) = woods.get_mut(island_entity) {
                                            wood.adjust_count(1000);
                                        } else {
                                            woods.insert(island_entity, GameResource::<Wood>::new());
                                        }
                                    },
                                    1 => {
                                        map.add_metal(position);
                                        if let Some(metal) = metals.get_mut(island_entity) {
                                            metal.adjust_count(1000);
                                        } else {
                                            metals.insert(island_entity, GameResource::<Metal>::new());
                                        }
                                    },
                                    2 => {
                                        map.add_food(position);
                                        if let Some(food) = foods.get_mut(island_entity) {
                                            food.adjust_count(1000);
                                        } else {
                                            foods.insert(island_entity, GameResource::<Food>::new());
                                        }
                                    },
                                    3 => {
                                        map.add_water(position);
                                        if let Some(water) = waters.get_mut(island_entity) {
                                            water.adjust_count(1000);
                                        } else {
                                            waters.insert(island_entity, GameResource::<Water>::new());
                                        }
                                    },
                                    _ => (),
                                    /*
                                    1 => map.add_metal(position),
                                    2 => map.add_food(position),
                                    3 => map.add_water(position),
                                    */
                                }
                            }
                        }
                        islands.insert(island_entity, Island::new(island_positions));
                    } else {
                        eprintln!("ERROR: INVALID POSITION IN MAP GENERATION");
                    }
                }
            }
        }
    }
}
