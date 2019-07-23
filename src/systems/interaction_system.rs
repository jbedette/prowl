use specs::prelude::*;

// use crate::ui::Panel;
// use crate::components::CharRenderer;
use crate::components::{
    Named,
};
use crate::event_channel::{
    EventChannel,
    InteractionEvent,
    // Event,
};
// use crate::console::{
use crate::console::resource::{
    Log,
    LogLevel,
    Console,
};
use crate::components::Player;


pub struct InteractionSystem;

impl<'a> System<'a> for InteractionSystem {
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, Player>,
        Write<'a, Console>,
        Write<'a, EventChannel<InteractionEvent>>,
        // Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            names,
            players,
            mut console,
            mut events,
            // entities,
            ) = data;

        while let Some(event) = events.pop() {
            let one = names.get(event.entities[0]);
            let two = names.get(event.entities[1]);
            let one = Named::name_or_noname(one);
            let two = Named::name_or_noname(two);
            if players.get(event.entities[0]).is_some() ||
                    players.get(event.entities[1]).is_some() {
                // TODO figure out how to spawn entities in this system...
                console.log(Log::new(
                        LogLevel::Game,
                        format!("{} has collided with {}", one, two),
                        ));
            }
        }
    }
}
