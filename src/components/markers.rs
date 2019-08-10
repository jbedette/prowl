use specs::{Component, NullStorage};
use specs_derive::Component;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Ship;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct ConsoleWindow;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct InteractionWindow;
