use specs::{
    Component,
    VecStorage
};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Named {
    pub value: String
}

impl Named {
    pub fn new(value: &str) -> Self {
        let value = String::from(value);
        Self { value }
    }
}

