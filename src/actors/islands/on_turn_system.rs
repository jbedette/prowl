use specs::prelude::*;
use crate::event_channel::{EventChannel, SpawnShipEvent};
use crate::actors::Island;
use crate::components::{Named, Position};
use crate::shared::random::random_range;

pub struct IslandOnTurnSystem;

impl<'a> System<'a> for IslandOnTurnSystem {
    type SystemData = (
        Write<'a, EventChannel<SpawnShipEvent>>,
        WriteStorage<'a, Island>,
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut spawn_ship_events,
            islands
            ) = data;

        for island in islands.join() {
            let coast_length = island.coast_tile_positions.len();
            let coast_position = island.coast_tile_positions[random_range(0, coast_length)];
            spawn_ship_events.events.push(
                SpawnShipEvent {
                    name: Named::new("SHIPPYBOY"),
                    position: Position::new(coast_position),
                });
        }
    }
}
