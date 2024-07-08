// System for managing island entity's interaction with turn system
// right now they spawn ships at set intervals
// future:
//      refactor: inefficient file loading and usage
//      future: add resource generation and resoure usage

use specs::prelude::*;
use crate::event_channel::{EventChannel, SpawnShipEvent};
use crate::actors::Island;
use crate::components::{Named, Position};
use crate::shared::random::random_range;
use crate::file_io::read_file;

pub struct IslandOnTurnSystem;


impl<'a> System<'a> for IslandOnTurnSystem {
    type SystemData = (
        Write<'a, EventChannel<SpawnShipEvent>>,
        WriteStorage<'a, Island>,
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut spawn_ship_events,
            mut islands
            ) = data;
        // load all random names
        // TODO this is very inefficient
        let all_names = read_file("names.txt");

        for island in (&mut islands).join() {
            if island.ship_spawn_timer < island.ship_spawn_timer_reset {
                island.ship_spawn_timer += 1;
            } else {
                // Spawn a ship
                // get name
                let name = all_names[random_range(0, all_names.len())].clone();
                // get spawn location
                let coast_length = island.coast_tile_positions.len();
                let coast_position = island.coast_tile_positions[random_range(0, coast_length)];
                // spawn ship
                spawn_ship_events.events.push(
                    SpawnShipEvent {
                        name: Named::new(&name),
                        position: Position::new(coast_position),
                    });
                island.ship_spawn_timer = 0;
            }
        }
    }
}
