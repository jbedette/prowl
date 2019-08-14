/// Builds other ships.
use crate::components::{
    ai::{Goal, AI},
    markers::Ship,
    pending_actions::PendingActions,
    CharRenderer, Health, Money, Named, Position, Weapon,
};
// use crate::file_io::read_file;
use crate::shared::{random::random_range};
use specs::prelude::*;
use tcod::colors::*;
// TODO maybe don't do this
use crate::event_channel::{
    EventChannel,
    SpawnShipEvent,
};


pub struct ShipSpawnerSystem;

#[allow(unused_must_use)]
impl<'a> System<'a> for ShipSpawnerSystem {
    type SystemData = (
        Write<'a, EventChannel<SpawnShipEvent>>,
        WriteStorage<'a, Named>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Weapon>,
        WriteStorage<'a, Money>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, CharRenderer>,
        WriteStorage<'a, AI>,
        WriteStorage<'a, PendingActions>,
        WriteStorage<'a, Ship>,
        Entities<'a>,
        );

    fn run(&mut self, data: Self::SystemData) {
        // use specs::Join;
        let (
            mut event_channel,
            mut names,
            mut healths,
            mut weapons,
            mut moneys,
            mut positions,
            mut renderers,
            mut ais,
            mut pending_actionses,
            mut ships,
            entities,
            ) = data;
        while let Some(event) = event_channel.events.pop() {
            let ship = entities.create();
            let name = event.name;
            let health = Health::new(100, 100);
            let weapon = Weapon::new(random_range(1, 10) as u64);
            let money = Money::new(random_range(1, 10) as u64);
            let position = event.position;
            let renderer = CharRenderer::new(
                'S',
                Color::new(
                    random_range(0x88, 0xff) as u8,
                    random_range(0x88, 0xff) as u8,
                    random_range(0x88, 0xff) as u8,
                ),
            );
            names.insert(ship, name);
            healths.insert(ship, health);
            weapons.insert(ship, weapon);
            moneys.insert(ship, money);
            positions.insert(ship, position);
            renderers.insert(ship, renderer);
            ais.insert(ship, AI::with_goal(Goal::MoveRandom));
            ships.insert(ship, Ship::default());
            pending_actionses.insert(ship, PendingActions::default());
        }
    }
}
