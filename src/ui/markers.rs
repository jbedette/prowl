/// Contains all "marker" (empty) components used for the UI system.
use specs::{
    Component,
    NullStorage
};
use specs_derive::Component;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct StatusUI;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct InteractiveUI;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct ConsoleUI;
