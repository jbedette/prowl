use specs::{Component, VecStorage, Entity;};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct Ruler{
    //ambition: Vec<Entity>,
}