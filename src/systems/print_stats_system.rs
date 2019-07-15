/// Prints name and hp of every entity that has a name and hp.

use specs::{
    ReadStorage,
    System
};
use crate::components::{
    Named,
    Health,
};

pub struct PrintStatsSystem;

impl<'a> System<'a> for PrintStatsSystem {
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, Health>,
        );
    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            names,
            healths,
            ) = data;

        for (name, health) in (&names, &healths).join() {
            println!("NAME: {}", name.value);
            println!("HP  : {}/{}", health.current, health.max);
        }
    }
}

