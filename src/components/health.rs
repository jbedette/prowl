use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Health {
    pub current: i64,
    pub max: i64,
}

impl Health {
    pub fn new(current: i64, max: i64) -> Self {
        Self { current, max }
    }
}
