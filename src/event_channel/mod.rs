use specs::prelude::*;

// Registers eveything with the world.
#[allow(deprecated)]
pub fn register(world: &mut World) {
    world.add_resource(EventChannel::<InteractionEvent>::default());
}

// An event channel holds a vec of events
#[derive(Default)]
pub struct EventChannel<E: Event> {
    pub events: Vec<E>,
}

// Event is just a trait (for now?)
pub trait Event {}

#[derive(Default)]
pub struct InteractionEvent {
    pub text: String,
    pub entities: Vec<Entity>,
}
impl Event for InteractionEvent {}

#[derive(Default)]
pub struct UserInputEvent {
    // TODO what does this need???
}
impl Event for UserInputEvent {}
