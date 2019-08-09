use specs::prelude::*;

use crate::components::{map::TileMap, Position};

use crate::actors::Island;
use crate::actors::Population;
use crate::Vector2;

pub struct IslandSetupSystem;

impl<'a> System<'a> for IslandSetupSystem {
    type SystemData = (
        ReadStorage<'a, Island>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, TileMap>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (islands, positions, mut maps, entities) = data;

        // this is stupid. there is only one map.
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
    }
}
