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
    pub fn transaction(&mut self, val:i64){
        let curr = self.current as i64;
        if curr + val < 0{
            self.current = 0;
        }
        else{
            self.current = (curr + val) as u64
        }
    }
}
