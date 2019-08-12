use specs::prelude::*;
use specs_derive::Component;
// use std::ops::{Add, Sub};

#[derive(Component, Debug, Default)]
pub struct Food {
    pub count: i32,
}

#[derive(Component, Debug, Default)]
pub struct Water {
    pub count: i32,
}

#[derive(Component, Debug, Default)]
pub struct Metal {
    pub count: i32,
}

#[derive(Component, Debug, Default)]
pub struct Wood {
    pub count: i32,
}

pub fn register_all(world: &mut World) {
    world.register::<Food>();
    world.register::<Water>();
    world.register::<Metal>();
    world.register::<Wood>();
}
