use specs::{Component, VecStorage, Entity};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]

//
pub struct Nation{
    islands: Vec<Entity>, //stores vec of islands
    //ambition: Vec<Entity>,
    ruler: Entitity,
}
impl Nation{
    pub fn new(size:i32, arable:i32) -> Self {
        Self { size, arable}
    }
}