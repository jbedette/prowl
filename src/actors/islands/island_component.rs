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
        Self { size, arable }
    }
    pub fn get_max_pop(&self)-> f32{
        //return a percentage of max pop
        self.size as f32/self.arable as f32
    }
}
