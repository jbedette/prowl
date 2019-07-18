use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct Island{
    pub size: i32,  //affect island size on map
    pub arable: i32,//affect max pop
}

impl Island{
    pub fn new(size:i32, arable:i32) -> Self {
        Self { size, arable}
    }
}