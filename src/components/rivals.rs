use specs::{
    Component,
    VecStorage,
    Entity
};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Rivals {
    pub entities: Vec<Entity>
}

impl Rivals {
    pub fn new() -> Self {
        Self {
            entities: vec![]
        }
    }
}

