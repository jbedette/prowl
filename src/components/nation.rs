use specs::{Component, VecStorage, Entitity};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]

//
pub struct Nation{
    islands: Vec<Entitity>, //stores vec of islands
    ambition: Vec<Entitity>,
    ruler: Entitity,
}
impl Nation{
    pub fn new(size:i32, arable:i32) -> Self {
        Self { size, arable}
    }
}