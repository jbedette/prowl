/// If entity has no rivals, find one.
/// Otherwise, hurt rival.

use specs::{
    ReadStorage,
    Write,
    WriteStorage,
    Entities,
    System,
    Join
};
use crate::components::{
    Named,
    Rivals,
    Health,
    Weapon
};
use crate::resources::console::{
    Console,
    Log,
    LogLevel,
};

pub struct RivalSystem;

impl<'a> System<'a> for RivalSystem {
    type SystemData = (
        ReadStorage<'a, Named>,
        WriteStorage<'a, Rivals>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Weapon>,
        Write<'a, Console>,
        Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            names,
            mut rivalses,
            mut healths,
            weapons,
            mut console,
            entities
        ) = data;

        for (entity, rivals) in (&entities, &mut rivalses).join() {
            let name = names.get(entity).unwrap();
            // must have at least one rival
            // TODO this doesn't work - see todos below
            if rivals.entities.is_empty() {
                for rival_entity in (entities).join() {
                    if entity != rival_entity {
                        rivals.entities.push(rival_entity);
                        let rival_name = names.get(rival_entity).unwrap();
                        console.log(Log {
                            level: LogLevel::Debug,
                            message: format!("{} has become rivals with {}", name.value, rival_name.value),
                        });
                        break;
                    }
                }
            } else {
                let weapon = weapons.get(entity).unwrap();
                // hurt each rival
                // TODO learn how iterators work and use one here so dead rivals can be deleted...
                for rival_entity in &rivals.entities {
                    let rival_health = healths.get_mut(*rival_entity);
                    let rival_name = names.get(*rival_entity);
                    // match rival_health {
                        // Some(health) => {
                    if let Some(health) = rival_health {
                            let damage = weapon.damage;
                            health.current -= damage as i64;
                            console.log(Log {
                                level: LogLevel::Debug,
                                message: format!("{} attacked their rival {} for {} dmg",
                                     name.value, rival_name.unwrap().value, damage),
                            });
                    }
                        // },
                        // None => {
                            // TODO delete rival somehow...
                            // eprintln!("ERROR: RIVAL NOT FOUND -- Entity Val:{:?}", rival_entity);
                        // }
                    // }
                }
            }
        }
    }
}

