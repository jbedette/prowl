use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct InteractionEvents(EventChannel);

#[derive(Debug, Default)]
pub struct EventChannel {
    events: Vec<Event>,
}

#[derive(Debug, Default)]
pub struct Event {
    text: String,
    entities: Vec<Entity>,
}
