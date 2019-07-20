/// Kills entities that are out of health.
use specs::{Entities, ReadStorage, System, Write};

use crate::components::{Health, Named};

use crate::console::resource::{
    Log, LogLevel, Console
};

pub struct DeathSystem;

impl<'a> System<'a> for DeathSystem {
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, Health>,
        Entities<'a>,
        Write<'a, Console>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (names, healths, entities, mut console) = data;

        for (health, entity) in (&healths, &entities).join() {
            if health.current <= 0 {
                let name = names.get(entity);
                let name_string = match name {
                    Some(name) => name.value.to_owned(),
                    None => String::from("UNNAMED ENTITY"),
                };
                let _result = entities.delete(entity);
                (*console).log(Log::new(
                    LogLevel::Debug,
                    &format!("{} has died.", name_string),
                ));
            }
        }
    }
}
