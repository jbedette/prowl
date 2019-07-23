use specs::prelude::*;

#[derive(Debug, Default)]
pub struct InteractionEvent;
impl EventType for InteractionEvent {}

#[derive(Debug, Default)]
pub struct EventChannel<T: EventType> {
    events: Vec<Event>,
    event_type: Box<T>,
}

impl<T: EventType> EventChannel<T> {
    pub fn push(&mut self, event: Event) { self.events.push(event); }
    pub fn pop(&mut self) -> Option<Event> { self.events.pop() }
}

#[derive(Debug, Default)]
pub struct Event {
    pub text: String,
    pub entities: Vec<Entity>,
}
pub trait EventType {}

#[allow(deprecated)]
pub fn register(world: &mut World) {
    world.add_resource(EventChannel::<InteractionEvent>::default());
}

