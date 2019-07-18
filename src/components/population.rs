use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct Population{
    current: i32,
    max: i32, //updated by island size/arable
}

impl Population{
    pub fn new(current:i32, max:i32){
        Self {current , max}
    }
}