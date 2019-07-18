use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Money {
    pub current: u64,
}

impl Money {
    pub fn new(current: u64) -> Self {
        Self { current }
    }
}
