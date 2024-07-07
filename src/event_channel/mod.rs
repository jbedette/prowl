use specs::prelude::*;

// Event Channel module manages game events and their ordering

// Registers eveything with the world.
#[allow(deprecated)]
pub fn register(world: &mut World) {
    world.add_resource(EventChannel::<InteractionEvent>::default());
    world.add_resource(EventChannel::<SpawnShipEvent>::default());
}

// An event channel holds a vec of events
#[derive(Default)]
pub struct EventChannel<E>
where E: Event
{
    pub events: Vec<E>,
}

// Event is just a trait (for now?)
pub trait Event {}

#[derive(Default)]
pub struct InteractionEvent {
    pub menu_code: u32,
    pub entities: Vec<Entity>,
}
impl Event for InteractionEvent {}

#[derive(Default)]
pub struct UserInputEvent {
    // TODO what does this need???
}
impl Event for UserInputEvent {}

use crate::components::{Named, Position};

#[derive(Default)]
pub struct SpawnShipEvent {
    pub name: Named,
    pub position: Position,
}

impl Event for SpawnShipEvent {}
