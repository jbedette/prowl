use specs::{
    Component,
    VecStorage
};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Weapon {
    pub damage: u64,
}

impl Weapon {
    pub fn new(damage: u64) -> Self {
        Self { damage }
    }
}

