use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct Population{
    pub size: i32,  //affect island size on map
    pub max: f32,//affect max pop
}

impl Population{
    pub fn new(size:i32, max:f32) -> Self {
        Self { size, max }
    }
}
