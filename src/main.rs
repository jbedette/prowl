use specs::{
    World,
    Builder,
    Component,
    VecStorage
};

use specs_derive::{
    Component
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Named {
    value: String
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Health {
    current: u64,
    max: u64,
}

impl Health {
    fn new(current: u64, max: u64) -> Self {
        Self {
            current,
            max
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Money {
    current: u64,
    max: u64,
}

impl Money {
    fn new(current: u64, max: u64) -> Self {
        Self {
            current,
            max
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Rival {
    entities: Vec<Entity>
}
