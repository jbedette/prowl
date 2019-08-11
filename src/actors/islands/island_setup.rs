use specs::prelude::*;
use crate::file_io;

use crate::components::{
    map::TileMap,
    Named
};

use crate::actors::Island;
// use crate::actors::Population;
use crate::shared::{
    Vector2,
    random::random_range,
};

pub struct IslandSetupSystem;

#[allow(unused_must_use)]
impl<'a> System<'a> for IslandSetupSystem {
    type SystemData = (
        WriteStorage<'a, Island>,
        WriteStorage<'a, Named>,
        WriteStorage<'a, TileMap>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut islands,
            mut names,
            mut maps,
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
                        // add island to tile
                        let island_entity = entities.create();
                        let name = &(island_names[(random_range(0, island_names.len()))]);
                        islands.insert(
                            island_entity,
                            Island::new(
                                random_range(2, 10) as i32,
                                random_range(0, 100) as i32));
                        names.insert(island_entity, Named::new(name));
                        map.add_island(position, island_entity);
                    } else {
                        eprintln!("ERROR: INVALID POSITION IN MAP GENERATION");
                    }
                }
            }
        }

        // this is stupid. there is only one map.
        /*
        for map in (&mut maps).join() {
            for (island, position, entity) in (&islands, &positions, &entities).join() {
                /*
                map.place_island(
                    position.value,
                    Vector2::new(
                        island.size,
                        island.size
                        ));
                        */
                map.place_island(island, position.value, entity);
            }
        }
        */
    }
}
