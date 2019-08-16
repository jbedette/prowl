use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Active {
    pub yes:bool,
}
impl Active{
    pub fn new()->Self{
        Active {yes:false}
    }
    #[allow(dead_code)]
    pub fn flip(&mut self){
        self.yes = !self.yes;
    }
}



