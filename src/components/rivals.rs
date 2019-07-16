use specs::{Component, Entity, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Rivals {
    pub entities: Vec<Entity>,
}

#[allow(dead_code)]
impl Rivals {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }
}
