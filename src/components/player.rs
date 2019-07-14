use specs::{
    Component,
    NullStorage,
};
use specs_derive::Component;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;
