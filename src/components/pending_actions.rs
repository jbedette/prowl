use specs::{Component, VecStorage};
use specs_derive::Component;

use crate::shared::Vector2;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct PendingActions {
    pub actions: Vec<Action>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Action {
    Move {
        // Relative movement to complete action
        // delta: (i32, i32),
        delta: Vector2,
    },
    Buy,
    Die,
}
