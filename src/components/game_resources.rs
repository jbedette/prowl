use specs::prelude::*;
use specs_derive::Component;
use std::fmt::Debug;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct GameResource<T>
where
    T: GameResourceType + Component + Send + Sync + Debug + Default,
{
    resource_type: T,
    count: u32,
    pub max: Option<u32>,
}

impl<T> GameResource<T>
where
    T: GameResourceType + Component + Send + Sync + Debug + Default,
{
    pub fn new(count: u32) -> Self {
        Self {
            resource_type: T::default(),
            count: count,
            max: None,
        }
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn transaction(&mut self, val: i32) {
        let curr = self.count as i32;
        if curr + val < 0 {
            self.count = 0;
        } else {
            self.count = (curr + val) as u32;
        }
    }

    pub fn set_count(&mut self, new_count: u32) {
        self.count = new_count;
    }

    pub fn adjust_count(&mut self, delta_count: i32) {
        /*
        if self.count > delta_count {
            self.count += delta_count;
        } else {
            self.count = 0;
        }
        */
        if delta_count >= 0 {
            self.count += delta_count as u32;
        } else {
            let delta_minus = -delta_count as u32;
            if self.count > delta_minus {
                self.count -= delta_minus;
            } else {
                self.count = 0;
            }
        }
    }

    #[allow(unused)]
    pub fn get_name(&self) -> String {
        if let Some(name) = self.resource_type.get_name() {
            name.to_owned()
        } else {
            format!("{:?}", self.resource_type).to_string()
        }
    }
}

pub trait GameResourceType {
    fn get_name(&self) -> Option<String> {
        None
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Wood {}
impl GameResourceType for Wood {}
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Metal {}
impl GameResourceType for Metal {}
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Food {}
impl GameResourceType for Food {}
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Water {}
impl GameResourceType for Water {}

pub fn register_all(world: &mut World) {
    world.register::<GameResource<Food>>();
    world.register::<GameResource<Water>>();
    world.register::<GameResource<Metal>>();
    world.register::<GameResource<Wood>>();
}
