/// Kills entities that are out of health.

use specs::{
    ReadStorage,
    Entities,
    System
};

use crate::components::{
    Named,
    Health,
};

pub struct DeathSystem;

impl<'a> System<'a> for DeathSystem {
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, Health>,
        Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            names,
            healths,
            entities
            ) = data;

        for (health, entity) in (&healths, &entities).join() {
            if health.current <= 0 {
                let name = names.get(entity);
                let name_string = match name {
                    Some(name) => name.value.to_owned(),
                    None => String::from("UNNAMED ENTITY"),
                };
                let _result = entities.delete(entity);
                println!("{} has died!", name_string);
            }
        }
    }
}

