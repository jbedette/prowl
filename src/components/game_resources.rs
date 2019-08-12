use specs::prelude::*;
use specs_derive::Component;
// use std::ops::{Add, Sub};

#[derive(Component, Debug, Default)]
pub struct Food {
    pub count: i32,
}
impl GameResource for Food {}

#[derive(Component, Debug, Default)]
pub struct Water {
    pub count: i32,
}
impl GameResource for Water {}

#[derive(Component, Debug, Default)]
pub struct Metal {
    pub count: i32,
}
impl GameResource for Metal {}

#[derive(Component, Debug, Default)]
pub struct Wood {
    pub count: i32,
}
impl GameResource for Wood {}

pub fn register_all(world: &mut World) {
    world.register::<Food>();
    world.register::<Water>();
    world.register::<Metal>();
    world.register::<Wood>();
}

trait GameResource {}
