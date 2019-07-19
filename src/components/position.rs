use specs::{Component, VecStorage};
use specs_derive::Component;
use crate::shared::Vector2;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub value: Vector2,
}

impl Position {
    pub fn new(value: Vector2) -> Self {
        Self { value }
    }
}

